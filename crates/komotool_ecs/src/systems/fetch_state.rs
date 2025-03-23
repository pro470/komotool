use crate::resources::KomorebiState;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::ResMut;
use komotool_pipe::PipeNotificationEvent;
use serde_json::{from_value, to_value};

pub fn update_komorebi_state_from_notifications(
    mut komorebi_state: ResMut<KomorebiState>,
    mut notifications: EventReader<PipeNotificationEvent>,
) {
    // Take last notification
    if let Some(last) = notifications.read().last() {
        komorebi_state.last = komorebi_state.current.take();
        komorebi_state.current = Some(last.notification.state.clone());
    }
}
