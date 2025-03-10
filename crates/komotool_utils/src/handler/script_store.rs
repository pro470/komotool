use std::{collections::HashSet, marker::PhantomData};
use bevy_ecs::system::Resource;
use bevy_reflect::Reflect;
use bevy_mod_scripting::core::event::IntoCallbackLabel;
use bevy_mod_scripting::core::IntoScriptPluginParams;
use bevy_mod_scripting::core::script::ScriptId;

/// Type-parameterized script storage for tracking active scripts
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct KomoToolScriptStore<P, L>
where
    P: IntoScriptPluginParams,
    L: IntoCallbackLabel,
{
    /// Set of active script identifiers
    pub scripts: HashSet<ScriptId>,
    #[reflect(ignore)]
    _phantom: PhantomData<(L, P)>,
}
