use super::KomoToolScriptStore;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Res;
use bevy_ecs::{system::SystemState, world::World};
use bevy_log::trace_once;
use bevy_mod_scripting::core::error::InteropErrorInner;
use bevy_mod_scripting::core::event::{CallbackLabel, Recipients};
use bevy_mod_scripting::core::handler::handle_script_errors;
use bevy_mod_scripting::core::{
    event::{IntoCallbackLabel, ScriptCallbackEvent},
    extractors::{HandlerContext, WithWorldGuard},
    IntoScriptPluginParams,
};
use indexmap::IndexSet;

/// A system state for handling script callbacks in KomoTool
///
/// Unlike the standard EventHandlerSystemState, this version:
/// - Takes a type parameter L for callback labels
/// - Queries KomoToolScriptStore instead of individual entity script components
#[allow(deprecated)]
pub type KomoToolEventHandlerSystemState<'w, 's, P, L> = SystemState<(
    Res<'w, KomoToolScriptStore<P, L>>,
    bevy_mod_scripting::core::extractors::EventReaderScope<'s, ScriptCallbackEvent>,
    WithWorldGuard<'w, 's, HandlerContext<'s, P>>,
)>;

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
    P: IntoScriptPluginParams + ScriptFunctionChecker + Send + Sync + 'static,
    L: IntoCallbackLabel + Send + Sync + 'static,
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

#[allow(deprecated)]
fn komotool_event_handler_inner<
    P: IntoScriptPluginParams + ScriptFunctionChecker + Send + Sync + 'static,
    L: IntoCallbackLabel + Send + Sync + 'static,
>(
    callback_label: CallbackLabel,
    script_store_query: Res<KomoToolScriptStore<P, L>>,
    mut script_events: bevy_mod_scripting::core::extractors::EventReaderScope<ScriptCallbackEvent>,
    mut handler_ctxt: WithWorldGuard<HandlerContext<P>>,
) {
    let (guard, handler_ctxt) = handler_ctxt.get_mut();
    let mut errors = Vec::default();

    // Get the script store
    let script_store = script_store_query;

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
                script_id.clone(),
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
                                script_id, entity
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
