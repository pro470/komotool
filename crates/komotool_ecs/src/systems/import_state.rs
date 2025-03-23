use crate::components::*;
use crate::resources::*;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::{Commands, Query, Res, ResMut};
use komorebi_client::{Container, Monitor, Window, Workspace};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::collections::hash_map::Entry;

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
            let mut entity = commands.spawn(komo_ws.clone());

            // Set focus if this is the monitor's focused workspace
            if idx == komo_mon.focused_workspace_idx() {
                entity.insert(Focused(1));
            }
        }
    }
}

pub fn import_komorebi_monitor_state(
    mut commands: Commands,
    mut existing_monitors: Query<&mut Monitor>,
    komorebi_state: Res<KomorebiState>,
    mut monitor_map: ResMut<MonitorToEntityMap>,
) {
    let Some(state) = &komorebi_state.current else {
        return;
    };

    // Track monitors that still exist in current state
    let mut current_serials = HashSet::new();

    // First pass: Update existing or spawn new monitors
    for (idx, komo_mon) in state.monitors.elements().iter().enumerate() {
        let Some(serial) = komo_mon.serial_number_id() else {
            continue; // Skip monitors without serial
        };

        current_serials.insert(serial.clone());

        match monitor_map.0.entry(serial.clone()) {
            Entry::Occupied(entry) => {
                let entity = *entry.get();
                
                // Update existing monitor component
                if let Ok(mut monitor) = existing_monitors.get_mut(entity) {
                    *monitor = komo_mon.clone();
                }

                // Update focus state
                let focused = idx == state.monitors.focused_idx();
                commands.entity(entity)
                    .remove::<Focused>()
                    .insert(Focused(focused as i32));
            }
            Entry::Vacant(entry) => {
                // Spawn new monitor
                let entity = commands.spawn(komo_mon.clone()).id();
                entry.insert(entity);
                
                if idx == state.monitors.focused_idx() {
                    commands.entity(entity).insert(Focused(1));
                }
            }
        }
    }

    // Second pass: Remove monitors that no longer exist
    monitor_map.0.retain(|serial, entity| {
        if current_serials.contains(serial) {
            true
        } else {
            commands.entity(*entity).despawn();
            false
        }
    });
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
                    commands.spawn(*komo_win);
                }
            }
        }
    }
}

pub fn import_komorebi_container_state(
    mut commands: Commands,
    mut existing_containers: Query<&mut Container>,
    komorebi_state: Res<KomorebiState>,
    mut container_map: ResMut<ContainerToEntityMap>,
) {
    let Some(state) = &komorebi_state.current else {
        return;
    };

    // Track containers that still exist in current state
    let mut current_ids = HashSet::new();

    // First pass: Update existing or spawn new containers
    for komo_mon in state.monitors.elements() {
        let workspaces = komo_mon.workspaces();
        for komo_ws in workspaces.iter() {
            for komo_cont in komo_ws.containers() {
                let id = komo_cont.id();
                current_ids.insert(id.clone());

                match container_map.0.entry(id.clone()) {
                    Entry::Occupied(entry) => {
                        let entity = *entry.get();
                        
                        // Update existing container component
                        if let Ok(mut container) = existing_containers.get_mut(entity) {
                            *container = komo_cont.clone();
                        }

                        // Update focus state
                        let focused_idx = komo_ws.focused_container_idx();
                        let is_focused = komo_cont.id() == komo_ws.containers()[focused_idx].id();
                        commands.entity(entity)
                            .remove::<Focused>()
                            .insert(Focused(is_focused as i32));
                    }
                    Entry::Vacant(entry) => {
                        // Spawn new container
                        let entity = commands.spawn(komo_cont.clone()).id();
                        entry.insert(entity);
                        
                        // Set initial focus if needed
                        let focused_idx = komo_ws.focused_container_idx();
                        if komo_cont.id() == komo_ws.containers()[focused_idx].id() {
                            commands.entity(entity).insert(Focused(1));
                        }
                    }
                }
            }
        }
    }

    // Second pass: Remove containers that no longer exist
    container_map.0.retain(|id, entity| {
        if current_ids.contains(id) {
            true
        } else {
            commands.entity(*entity).despawn();
            false
        }
    });
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
