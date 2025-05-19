use super::ScriptFunctionChecker;
use super::{KomoToolScriptStore, KomoToolScriptStoreAll};
use bevy_asset::AssetPath;
use bevy_ecs::component::Tick;
use bevy_ecs::entity::Entity;
use bevy_ecs::resource::Resource;
use bevy_ecs::system::{SystemMeta, SystemParam};
use bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell;
use bevy_ecs::{system::SystemState, world::World};
use bevy_log::trace_once;
use bevy_mod_scripting::core::asset::{Language, ScriptAssetSettings};
use bevy_mod_scripting::core::error::InteropErrorInner;
use bevy_mod_scripting::core::event::{CallbackLabel, Recipients};
use bevy_mod_scripting::core::extractors::HandlerContext;
use bevy_mod_scripting::core::handler::handle_script_errors;
use bevy_mod_scripting::core::script::ScriptId;
use bevy_mod_scripting::core::{
    IntoScriptPluginParams,
    event::{IntoCallbackLabel, ScriptCallbackEvent},
    extractors::WithWorldGuard,
};
use bevy_mod_scripting::lua::LuaScriptingPlugin;
use bevy_mod_scripting::rhai::RhaiScriptingPlugin;
use indexmap::IndexSet;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[derive(SystemParam)]
pub struct HandlerContexts<'s> {
    pub lua: HandlerContext<'s, LuaScriptingPlugin>,
    pub rhai: HandlerContext<'s, RhaiScriptingPlugin>,
}

/// A system state for handling script callbacks in KomoTool
///
/// Unlike the standard EventHandlerSystemState, this version:
/// - Takes a type parameter L for callback labels
/// - Queries KomoToolScriptStore instead of individual entity script components
#[allow(deprecated)]
pub type KomoToolEventHandlerSystemState<'w, 's, P, L> = SystemState<(
    SystemResScope<'w, P, L>,
    bevy_mod_scripting::core::extractors::EventReaderScope<'s, ScriptCallbackEvent>,
    WithWorldGuard<'w, 's, HandlerContext<'s, P>>,
)>;

#[allow(deprecated)]
pub type KomoToolEventHandlerSystemStateAll<'w, 's, L> = SystemState<(
    ResScope<'w, KomoToolScriptStoreAll<L>>,
    ResScope<'w, ScriptAssetSettings>,
    WithWorldGuard<'w, 's, HandlerContexts<'s>>,
)>;

pub struct ResScope<'w, T: Resource + Default>(pub &'w mut T);

impl<T: Resource + Default> Deref for ResScope<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T: Resource + Default> DerefMut for ResScope<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

pub struct ResourceState<T: Resource + Default> {
    marker: PhantomData<T>,
}

unsafe impl<T: Resource + Default + 'static> SystemParam for ResScope<'_, T> {
    type State = T;
    type Item<'world, 'state> = ResScope<'world, T>;

    fn init_state(world: &mut World, _system_meta: &mut SystemMeta) -> Self::State {
        world.init_resource::<T>();
        T::default()
    }

    unsafe fn get_param<'world, 'state>(
        _state: &'state mut Self::State,
        _system_meta: &SystemMeta,
        world: UnsafeWorldCell<'world>,
        _change_tick: Tick,
    ) -> Self::Item<'world, 'state> {
        // Get the resource pointer
        let resource_ref = if let Some(mut ptr) = unsafe {
            if let Some(item) = world.get_resource_mut::<T>() {
                Some(item)
            } else {
                {
                    world.world_mut().init_resource::<T>();
                }
                world.get_resource_mut::<T>()
            }
        } {
            // IMPORTANT: Use the correct approach to get a reference with 'world lifetime
            // This uses unsafe to extend the lifetime, but is safe because we know
            // the resource lives for the 'world lifetime
            let raw_ptr = ptr.as_mut() as *mut T;
            unsafe { &mut *raw_ptr }
        } else {
            let raw_ptr = &mut T::default() as *mut T;
            unsafe { &mut *raw_ptr }
        };

        ResScope(resource_ref)
    }
}

#[derive(SystemParam)]
pub struct SystemResScope<
    'w,
    P: IntoScriptPluginParams + ScriptFunctionChecker + Send + Sync + 'static + std::default::Default,
    L: IntoCallbackLabel + Send + Sync + 'static + std::default::Default,
> {
    pub(crate) store: ResScope<'w, KomoToolScriptStore<P, L>>,
}

macro_rules! push_err_and_continue {
    ($errors:ident, $expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => {
                $errors.push(e);
                continue;
            }
        }
    };
}

/// Passes events with the specified label to the script callback with the same name and runs the callback.
///
/// Similar to bevy_mod_scripting's event_handler but works with KomoToolScriptStore instead of entity queries.
/// If any of the resources required for the handler are missing, the system will log this issue and do nothing.
#[allow(deprecated)]
pub fn komotool_event_handler<
    P: IntoScriptPluginParams + ScriptFunctionChecker + Send + Sync + 'static + std::default::Default,
    L: IntoCallbackLabel + Send + Sync + 'static + std::default::Default,
>(
    world: &mut World,
    state: &mut KomoToolEventHandlerSystemState<P, L>,
) {
    // We wrap the inner event handler so we can immediately re-insert all the resources back.
    // Otherwise, this would happen in the next schedule
    {
        let (script_store_query, script_events, handler_ctxt) = state.get_mut(world);
        komotool_event_handler_inner::<P, L>(
            L::into_callback_label(),
            script_store_query,
            script_events,
            handler_ctxt,
        );
    }
    state.apply(world);
}

#[profiling::function]
#[allow(deprecated)]
fn komotool_event_handler_inner<
    P: IntoScriptPluginParams + ScriptFunctionChecker + Send + Sync + 'static + std::default::Default,
    L: IntoCallbackLabel + Send + Sync + 'static + std::default::Default,
>(
    callback_label: CallbackLabel,
    script_store_query: SystemResScope<P, L>,
    mut script_events: bevy_mod_scripting::core::extractors::EventReaderScope<ScriptCallbackEvent>,
    mut handler_ctxt: WithWorldGuard<HandlerContext<P>>,
) {
    if script_store_query.store.0.scripts.is_empty() {
        return;
    }

    let (guard, handler_ctxt) = handler_ctxt.get_mut();
    let mut errors = Vec::default();

    // Get the script store
    let script_store = script_store_query.store.0;

    // Process each event
    for event in script_events
        .read()
        .filter(|&e| e.label == callback_label)
        .cloned()
    {
        // Determine which scripts to process
        let scripts_to_process: IndexSet<_> = match &event.recipients {
            Recipients::Script(target_script_id)
                if script_store.scripts.contains(target_script_id) =>
            {
                // If the target script exists in the store, only process that one (Create a new HashSet for O(1) iteration)
                std::iter::once(target_script_id.clone()).collect()
            }
            _ => {
                // Otherwise, process all scripts (return a reference to the full HashSet)
                script_store.scripts.clone()
            }
        };

        for script_id in scripts_to_process {
            let entity = Entity::from_raw(0);
            let call_result = handler_ctxt.call_dynamic_label(
                &callback_label,
                &script_id,
                entity,
                event.args.clone(),
                guard.clone(),
            );

            match call_result {
                Ok(_) => {}
                Err(e) => {
                    match e.downcast_interop_inner() {
                        Some(InteropErrorInner::MissingScript { script_id }) => {
                            trace_once!(
                                "{}: Script `{}` on entity `{:?}` is either still loading, doesn't exist, or is for another language, ignoring until the corresponding script is loaded.",
                                P::LANGUAGE,
                                script_id,
                                entity
                            );
                            continue;
                        }
                        Some(InteropErrorInner::MissingContext { .. }) => {
                            // if we don't have a context for the script, it's either:
                            // 1. a script for a different language, in which case we ignore it
                            // 2. something went wrong. This should not happen though, and it's best we ignore this
                            continue;
                        }
                        _ => {}
                    }
                    let e = e
                        .with_script(script_id.clone())
                        .with_context(format!("Event handling for: Language: {}", P::LANGUAGE));
                    push_err_and_continue!(errors, Err(e));
                }
            };
        }
    }

    handle_script_errors(guard, errors.into_iter());
}
/// Passes events with the specified label to the script callback with the same name and runs the callback.
///
/// Similar to bevy_mod_scripting's event_handler but works with KomoToolScriptStore instead of entity queries.
/// If any of the resources required for the handler are missing, the system will log this issue and do nothing.
#[allow(deprecated)]
pub fn komotool_event_handler_all<
    L: IntoCallbackLabel + Send + Sync + 'static + std::default::Default,
>(
    world: &mut World,
    state: &mut KomoToolEventHandlerSystemStateAll<L>,
) {
    // We wrap the inner event handler so we can immediately re-insert all the resources back.
    // Otherwise, this would happen in the next schedule
    {
        let (script_store_query, settings, handler_ctxt) = state.get_mut(world);
        komotool_event_handler_inner_all::<L>(
            L::into_callback_label(),
            script_store_query,
            settings,
            handler_ctxt,
        );
    }
    //state.apply(world);
}

#[profiling::function]
#[allow(deprecated)]
fn komotool_event_handler_inner_all<
    L: IntoCallbackLabel + Send + Sync + 'static + std::default::Default,
>(
    callback_label: CallbackLabel,
    script_store_query: ResScope<KomoToolScriptStoreAll<L>>,
    settings: ResScope<ScriptAssetSettings>,
    mut handler_ctxt: WithWorldGuard<HandlerContexts>,
) {
    if script_store_query.scripts.is_empty() {
        return;
    }

    let (guard, handler_ctxt) = handler_ctxt.get_mut();
    let mut errors = Vec::default();
    let entity = Entity::from_raw(0);
    let store: IndexSet<ScriptId>;
    {
        store = script_store_query.scripts.clone();
    }

    for script_id in store.iter() {
        let language = settings
            .0
            .select_script_language(&AssetPath::parse(script_id.as_ref()));
        let call_result = match language {
            Language::Rhai => handler_ctxt.rhai.call_dynamic_label(
                &callback_label,
                script_id,
                entity,
                Vec::new(),
                guard.clone(),
            ),
            Language::Lua => handler_ctxt.lua.call_dynamic_label(
                &callback_label,
                script_id,
                entity,
                Vec::new(),
                guard.clone(),
            ),
            Language::Rune => continue,
            Language::External(_) => continue,
            Language::Unknown => continue,
        };

        match call_result {
            Ok(_) => {}
            Err(e) => {
                match e.downcast_interop_inner() {
                    Some(InteropErrorInner::MissingScript { script_id }) => {
                        trace_once!(
                            "{}: Script `{}` on entity `{:?}` is either still loading, doesn't exist, or is for another language, ignoring until the corresponding script is loaded.",
                            &language,
                            script_id,
                            entity
                        );
                        println!("Missing script: {}", script_id);
                        continue;
                    }
                    Some(InteropErrorInner::MissingContext { .. }) => {
                        // if we don't have a context for the script, it's either:
                        // 1. a script for a different language, in which case we ignore it
                        // 2. something went wrong. This should not happen though, and it's best we ignore this
                        println!("Missing context: {}", script_id);
                        continue;
                    }
                    _ => {
                        println!("Unknown error: {}", script_id);
                    }
                }
                let e = e
                    .with_script(script_id.clone())
                    .with_context(format!("Event handling for: Language: {}", &language));
                push_err_and_continue!(errors, Err(e));
            }
        };
    }

    handle_script_errors(guard, errors.into_iter());
}
