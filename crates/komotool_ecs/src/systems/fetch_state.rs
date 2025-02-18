 use crate::resources::KomorebiState;
 use bevy::prelude::*;
 use komotool_pipe::PipeNotificationEvent;

 pub fn update_komorebi_state_from_notifications(
     mut komorebi_state: ResMut<KomorebiState>,
     mut notifications: EventReader<PipeNotificationEvent>,
 ) {
     // Only take last notification for state update
     if let Some(last) = notifications.read().last() {
         komorebi_state.last = komorebi_state.current.take();
         komorebi_state.current = Some(last.notification.state.clone());
     }
 }
