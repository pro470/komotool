use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use komorebi_client::{send_query, SocketMessage, State};

pub fn import_komorebi_state(
    mut commands: Commands,
    mut existing_monitors: Query<(Entity, &Monitor)>,
    mut app_state: ResMut<AppState>,
) {
    // Clear existing monitors
    for (entity, _) in existing_monitors.iter_mut() {
        commands.entity(entity).despawn();
    }

    let state: State = serde_json::from_str(&send_query(&SocketMessage::State).unwrap()).unwrap();

    // Spawn new monitor entities
    for (idx, komo_mon) in state.monitors.elements().iter().enumerate() {
        let mut entity = commands.spawn(Monitor {
            id: komo_mon.id(),
            name: komo_mon.name().clone(),
            device: komo_mon.device().clone(),
            device_id: komo_mon.device_id().clone(),
            serial_number_id: komo_mon.serial_number_id().clone(),
            physical_size: komo_mon.size().into(),
            work_area_size: komo_mon.work_area_size().into(),
            work_area_offset: komo_mon.work_area_offset().into(),
            window_based_work_area_offset: komo_mon.window_based_work_area_offset().into(),
            window_based_work_area_offset_limit: komo_mon.window_based_work_area_offset_limit(),
        });

        if idx == state.monitors.focused_idx() {
            entity.insert(Focused(1));
        }
    }

    // Update AppState
    *app_state = AppState {
        is_paused: state.is_paused,
        resize_delta: state.resize_delta,
        //new_window_behaviour: state.new_window_behaviour,
        float_override: state.float_override,
        cross_monitor_move_behaviour: state.cross_monitor_move_behaviour,
        unmanaged_window_operation_behaviour: state.unmanaged_window_operation_behaviour,
        work_area_offset: state.work_area_offset.into(),
        focus_follows_mouse: state.focus_follows_mouse,
        mouse_follows_focus: state.mouse_follows_focus,
        has_pending_raise_op: state.has_pending_raise_op,
    };
}
