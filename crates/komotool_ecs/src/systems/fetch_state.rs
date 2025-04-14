use crate::resources::KomorebiState;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::ResMut;
use komotool_pipe::PipeNotificationEvent;

pub fn update_komorebi_state_from_notifications(
    mut komorebi_state: ResMut<KomorebiState>,
    mut notifications: EventReader<PipeNotificationEvent>,
) {
    // Take last notification
    if let Some(last) = notifications.read().last() {
        if let Some(state) = &komorebi_state.komorebi {
           if state.has_been_modified(&last.notification.state) {
               println!("State has been modified");
               komorebi_state.komorebi = Some(last.notification.state.clone());
           }
        } else {
            komorebi_state.komorebi = Some(last.notification.state.clone());
        }
    }
}
