use super::ScriptFunctionChecker;
use bevy_ecs::resource::Resource;
use bevy_mod_scripting::core::IntoScriptPluginParams;
use bevy_mod_scripting::core::event::IntoCallbackLabel;
use bevy_mod_scripting::core::script::ScriptId;
use bevy_reflect::Reflect;
use indexmap::IndexSet;
use std::marker::PhantomData;

/// Type-parameterized script storage for tracking active scripts
#[derive(Resource, Default, Reflect)]
pub struct KomoToolScriptStore<P, L>
where
    P: IntoScriptPluginParams
        + ScriptFunctionChecker
        + Send
        + Sync
        + 'static
        + std::default::Default,
    L: IntoCallbackLabel + Send + Sync + 'static + std::default::Default,
{
    /// Set of active script identifiers
    #[reflect(ignore)]
    pub scripts: IndexSet<ScriptId>,
    _phantom: PhantomData<(L, P)>,
}

/// Type-parameterized script storage for tracking active scripts
#[derive(Resource, Default, Reflect)]
pub struct KomoToolScriptStoreAll<L>
where
    L: IntoCallbackLabel + Send + Sync + 'static + std::default::Default,
{
    /// Set of active script identifiers
    #[reflect(ignore)]
    pub scripts: IndexSet<ScriptId>,
    #[reflect(ignore)]
    _phantom: PhantomData<L>,
}
