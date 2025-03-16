use crate::{
    GlobalLoadingState, OnPostStartUp, OnPostUpdate, OnPreStartUp, OnPreUpdate, OnStartUp, OnUpdate,
};
use bevy_app::{Last, PreUpdate, Update};
use bevy_ecs::event::EventWriter;
use bevy_ecs::schedule::Schedules;
use bevy_ecs::system::ResMut;
use bevy_mod_scripting::core::event::ScriptCallbackEvent;
use bevy_state::state::NextState;

// Startup events
pub fn send_pre_startup_events(
    mut writer: EventWriter<ScriptCallbackEvent>,
    mut next_state: ResMut<NextState<GlobalLoadingState>>,
) {
    writer.send(ScriptCallbackEvent::new_for_all(OnPreStartUp, vec![]));
    next_state.set(GlobalLoadingState::PreStartupDone);
}

pub fn send_startup_events(
    mut writer: EventWriter<ScriptCallbackEvent>,
    mut next_state: ResMut<NextState<GlobalLoadingState>>,
) {
    writer.send(ScriptCallbackEvent::new_for_all(OnStartUp, vec![]));
    next_state.set(GlobalLoadingState::StartupDone);
}

pub fn send_post_startup_events(
    mut writer: EventWriter<ScriptCallbackEvent>,
    mut next_state: ResMut<NextState<GlobalLoadingState>>,
) {
    writer.send(ScriptCallbackEvent::new_for_all(OnPostStartUp, vec![]));
    next_state.set(GlobalLoadingState::PostStartupDone);
}

pub fn advance_to_clean_up_done(mut next_state: ResMut<NextState<GlobalLoadingState>>) {
    next_state.set(GlobalLoadingState::CleanupDone);
}

pub fn advance_to_all_done(mut next_state: ResMut<NextState<GlobalLoadingState>>) {
    next_state.set(GlobalLoadingState::AllDone);
}

// Per-frame events
pub fn send_pre_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new_for_all(OnPreUpdate, vec![]));
}

pub fn send_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new_for_all(OnUpdate, vec![]));
}

pub fn send_post_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new_for_all(OnPostUpdate, vec![]));
}

pub fn insert_event_sending_systems(mut schedule: ResMut<Schedules>) {
    schedule.add_systems(Last, send_pre_update_events);
    schedule.add_systems(PreUpdate, send_update_events);
    schedule.add_systems(Update, send_post_update_events);
}
