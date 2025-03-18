use super::ScriptFunctionChecker;
use bevy_ecs::system::Resource;
use bevy_mod_scripting::core::event::IntoCallbackLabel;
use bevy_mod_scripting::core::script::ScriptId;
use bevy_mod_scripting::core::IntoScriptPluginParams;
use indexmap::IndexSet;
use std::marker::PhantomData;

/// Type-parameterized script storage for tracking active scripts
#[derive(Resource, Default)]
pub struct KomoToolScriptStore<P, L>
where
    P: IntoScriptPluginParams + ScriptFunctionChecker + Send + Sync + 'static + std::default::Default,
    L: IntoCallbackLabel + Send + Sync + 'static + std::default::Default,
{
    /// Set of active script identifiers
    pub scripts: IndexSet<ScriptId>,
    _phantom: PhantomData<(L, P)>,
}

