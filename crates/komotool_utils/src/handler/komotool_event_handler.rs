use bevy_ecs::{
    query::{QueryState, Ref},
    system::{Local, SystemState},
};
use bevy_mod_scripting::core::{
    event::{IntoCallbackLabel, ScriptCallbackEvent},
    extractors::{EventReaderScope, HandlerContext, WithWorldGuard},
    IntoScriptPluginParams,
};

use crate::handler::KomoToolScriptStore;

/// A system state for handling script callbacks in KomoTool
/// 
/// Unlike the standard EventHandlerSystemState, this version:
/// - Takes a type parameter L for callback labels
/// - Queries KomoToolScriptStore instead of individual entity script components
pub type KomoToolEventHandlerSystemState<'w, 's, P, L> = SystemState<(
    Local<'s, QueryState<Ref<'w, KomoToolScriptStore<P, L>>>>,
    EventReaderScope<'s, ScriptCallbackEvent>,
    WithWorldGuard<'w, 's, HandlerContext<'s, P>>,
)>
where
    P: IntoScriptPluginParams,
    L: IntoCallbackLabel;
