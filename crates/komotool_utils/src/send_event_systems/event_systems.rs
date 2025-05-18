use crate::{
    GlobalLoadingState, OnPostStartUp, OnPreStartUp,  OnStartUp, 
};
use bevy_ecs::event::EventWriter;
use bevy_ecs::system::ResMut;
use bevy_mod_scripting::core::event::ScriptCallbackEvent;
use bevy_state::state::NextState;

// Startup events
pub fn send_pre_startup_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.write(ScriptCallbackEvent::new_for_all(OnPreStartUp, vec![]));
}

pub fn send_startup_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.write(ScriptCallbackEvent::new_for_all(OnStartUp, vec![]));
}

pub fn send_post_startup_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.write(ScriptCallbackEvent::new_for_all(OnPostStartUp, vec![]));
}

pub fn advance_to_clean_up_done(mut next_state: ResMut<NextState<GlobalLoadingState>>) {
    next_state.set(GlobalLoadingState::CleanupDone);
}

pub fn advance_to_all_done(mut next_state: ResMut<NextState<GlobalLoadingState>>) {
    next_state.set(GlobalLoadingState::AllDone);
}
