use crate::{OnPostStartUp, OnPostUpdate, OnPreStartUp, OnPreUpdate, OnStartUp, OnUpdate};
use bevy_ecs::event::EventWriter;
use bevy_mod_scripting::core::event::ScriptCallbackEvent;

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
