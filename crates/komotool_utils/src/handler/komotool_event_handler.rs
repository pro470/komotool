use bevy_ecs::{
    query::QueryState,
    system::{Local, SystemState},
    world::Ref,
};
use bevy_mod_scripting::core::{
    event::ScriptCallbackEvent,
    extractors::{HandlerContext, WithWorldGuard},
};

use crate::handler::KomoToolScriptStore;

/// A system state for handling script callbacks in KomoTool
/// 
/// Unlike the standard EventHandlerSystemState, this version:
/// - Takes a type parameter L for callback labels
/// - Queries KomoToolScriptStore instead of individual entity script components
#[allow(deprecated)]
pub type KomoToolEventHandlerSystemState<'w, 's, P, L> = SystemState<(
    Local<'s, QueryState<Ref<'w, KomoToolScriptStore<P, L>>>>,
    bevy_mod_scripting::core::extractors::EventReaderScope<'s, ScriptCallbackEvent>,
    WithWorldGuard<'w, 's, HandlerContext<'s, P>>,
)>;
