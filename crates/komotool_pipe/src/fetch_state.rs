use crate::PipeNotificationEvent;
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::Commands;
use bevy_platform::time::Instant;
use komotool_ecs::resources::KomorebiState;
use komotool_ecs::systems::{
    build_relation_registry, export_state_to_komorebi, import_komorebi_appstate_state,
    import_komorebi_container_state, import_komorebi_monitor_state, import_komorebi_window_state,
    import_komorebi_workspace_state,
};
use komotool_framepace::IdleFramePaceState;
use komotool_utils::komotool_schedule::{KomoToolPostUpdate, KomoToolPreUpdate, KomoToolUpdate};

pub fn update_komorebi_state_from_notifications(
    mut komorebi_state: ResMut<KomorebiState>,
    mut notifications: EventReader<PipeNotificationEvent>,
    mut idle: ResMut<IdleFramePaceState>,
    mut commands: Commands,
) {
    // Take last notification
    if let Some(last) = notifications.read().last() {
        if let Some(state) = &komorebi_state.komorebi {
            if state.has_been_modified(&last.notification.state) {
                println!("State has been modified");
                komorebi_state.komorebi = Some(last.notification.state.clone());
                idle.last_activity = Instant::now();
                update_komotool_state(commands.reborrow());
            }
        } else {
            komorebi_state.komorebi = Some(last.notification.state.clone());
            update_komotool_state(commands.reborrow());
        }
    }
}

pub fn update_komotool_state(mut commands: Commands) {
    commands.run_system_cached(import_komorebi_window_state);
    commands.run_system_cached(import_komorebi_container_state);
    commands.run_system_cached(import_komorebi_workspace_state);
    commands.run_system_cached(import_komorebi_monitor_state);
    commands.run_system_cached(import_komorebi_appstate_state);
    commands.run_system_cached(build_relation_registry);
    commands.run_schedule(KomoToolPreUpdate);
    commands.run_schedule(KomoToolUpdate);
    commands.run_schedule(KomoToolPostUpdate);
    commands.run_system_cached(export_state_to_komorebi);
}
