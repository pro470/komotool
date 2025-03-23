use crate::components::*;
use crate::resources::*;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::{Commands, Query, Res, ResMut};
use komorebi_client::{Container, Monitor, Window, Workspace};

pub fn import_komorebi_workspace_state(
    mut commands: Commands,
    mut existing_workspaces: Query<(Entity, &mut Workspace)>,
    komorebi_state: Res<KomorebiState>,
) {
    // Clear existing workspaces
    for (entity, _) in existing_workspaces.iter_mut() {
        commands.entity(entity).despawn();
    }

    let Some(state) = &komorebi_state.current else {
        return;
    };

    // Spawn new workspace entities
    for komo_mon in state.monitors.elements() {
        let workspaces = komo_mon.workspaces();
        for (idx, komo_ws) in workspaces.iter().enumerate() {
            let mut entity = commands.spawn(komo_ws);

            // Set focus if this is the monitor's focused workspace
            if idx == komo_mon.focused_workspace_idx() {
                entity.insert(Focused(1));
            }
        }
    }
}

pub fn import_komorebi_monitor_state(
    mut commands: Commands,
    mut existing_monitors: Query<(Entity, &mut Monitor)>,
    komorebi_state: Res<KomorebiState>,
    mut monitor_map: ResMut<MonitorToEntityMap>,
) {
    monitor_map.0.clear();
    for (entity, _) in existing_monitors.iter_mut() {
        commands.entity(entity).despawn();
    }

    let Some(state) = &komorebi_state.current else {
        return;
    };

    for (idx, komo_mon) in state.monitors.elements().iter().enumerate() {
        let entity = commands.spawn(komo_mon).id();
        
        // Use serial_number_id as key with getter access
        if let Some(serial) = komo_mon.serial_number_id() {
            monitor_map.0.insert(serial.clone(), entity);
        }

        if idx == state.monitors.focused_idx() {
            commands.entity(entity).insert(Focused(1));
        }
    }
}

pub fn import_komorebi_window_state(
    mut commands: Commands,
    mut existing_windows: Query<(Entity, &mut Window)>,
    komorebi_state: Res<KomorebiState>,
) {
    // Clear existing windows
    for (entity, _) in existing_windows.iter_mut() {
        commands.entity(entity).despawn();
    }

    let Some(state) = &komorebi_state.current else {
        return;
    };

    // Spawn new window entities
    for komo_mon in state.monitors.elements() {
        let workspaces = komo_mon.workspaces();
        for komo_ws in workspaces.iter() {
            for komo_cont in komo_ws.containers() {
                for komo_win in komo_cont.windows() {
                    commands.spawn(komo_win);
                }
            }
        }
    }
}

pub fn import_komorebi_container_state(
    mut commands: Commands,
    mut existing_containers: Query<(Entity, &mut Container)>,
    komorebi_state: Res<KomorebiState>,
) {
    // Clear existing containers
    for (entity, _) in existing_containers.iter_mut() {
        commands.entity(entity).despawn();
    }

    let Some(state) = &komorebi_state.current else {
        return;
    };

    // Spawn new container entities
    for komo_mon in state.monitors.elements() {
        let workspaces = komo_mon.workspaces();
        for komo_ws in workspaces.iter() {
            for komo_cont in komo_ws.containers() {
                let mut entity = commands.spawn(komo_cont);

                // Set focus if this is the workspace's focused container
                let focused_idx = komo_ws.focused_container_idx();
                if komo_cont.id() == komo_ws.containers()[focused_idx].id() {
                    entity.insert(Focused(1));
                }
            }
        }
    }
}

pub fn import_komorebi_appstate_state(
    mut app_state: ResMut<AppState>,
    komorebi_state: Res<KomorebiState>,
) {
    if let Some(state) = &komorebi_state.current {
        // Update AppState
        *app_state = AppState {
            is_paused: state.is_paused,
            resize_delta: state.resize_delta,
            float_override: state.float_override,
            new_window_behaviour: state.new_window_behaviour,
            cross_monitor_move_behaviour: state.cross_monitor_move_behaviour,
            unmanaged_window_operation_behaviour: state.unmanaged_window_operation_behaviour,
            work_area_offset: state.work_area_offset,
            focus_follows_mouse: state.focus_follows_mouse,
            mouse_follows_focus: state.mouse_follows_focus,
            has_pending_raise_op: state.has_pending_raise_op,
        };
    }
}
