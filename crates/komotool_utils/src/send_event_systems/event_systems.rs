use crate::{
    GlobalLoadingState, OnPostStartUp, OnPostUpdate, OnPreStartUp, OnPreUpdate, OnStartUp, OnUpdate,
};
use bevy_app::{FixedLast, FixedPreUpdate, FixedUpdate};
use bevy_ecs::event::EventWriter;
use bevy_ecs::schedule::Schedules;
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

// Per-frame events
pub fn send_pre_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.write(ScriptCallbackEvent::new_for_all(OnPreUpdate, vec![]));
}

pub fn send_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.write(ScriptCallbackEvent::new_for_all(OnUpdate, vec![]));
}

pub fn send_post_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.write(ScriptCallbackEvent::new_for_all(OnPostUpdate, vec![]));
}

pub fn insert_event_sending_systems(mut schedule: ResMut<Schedules>) {
    schedule.add_systems(FixedLast, send_pre_update_events);
    schedule.add_systems(FixedPreUpdate, send_update_events);
    schedule.add_systems(FixedUpdate, send_post_update_events);
}
