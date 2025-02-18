use crate::resources::KomorebiState;
use bevy::prelude::*;
use komotool_pipe::PipeNotificationEvent;
use serde_json::{from_value, to_value};

pub fn update_komorebi_state_from_notifications(
    mut komorebi_state: ResMut<KomorebiState>,
    mut notifications: EventReader<PipeNotificationEvent>,
) {
    // Take last notification
    if let Some(last) = notifications.read().last() {
        komorebi_state.last = komorebi_state.current.take();
        
        // Deep clone via JSON serialization
        let json = to_value(&last.notification.state).expect("Failed to serialize state");
        let cloned_state = from_value(json).expect("Failed to deserialize state");
        
        komorebi_state.current = Some(cloned_state);
    }
}
