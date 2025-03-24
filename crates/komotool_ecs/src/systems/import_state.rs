use crate::components::*;
use crate::resources::*;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::{Commands, Query, Res, ResMut};
use indexmap::IndexSet;
use komorebi_client::{Container, Monitor, Window, Workspace};
use std::collections::{hash_map::Entry, HashSet};

pub fn import_komorebi_workspace_state(
    mut commands: Commands,
    mut existing_workspaces: Query<&mut Workspace>,
    komorebi_state: Res<KomorebiState>,
    mut workspace_map: ResMut<WorkspaceToEntityMap>,
    container_map: Res<ContainerToEntityMap>,
) {
    let Some(state) = &komorebi_state.current else {
        return;
    };

    let mut current_keys = HashSet::new();

    for komo_mon in state.monitors.elements() {
        let workspaces = komo_mon.workspaces();
        for komo_ws in workspaces.iter() {
            // Use name if available, otherwise fall back to ID
            let key = match komo_ws.name() {
                Some(name) => name.clone(),
                None => continue,
            };
            current_keys.insert(key.clone());

            let focused_idx = komo_ws.focused_container_idx();

            let container_entities = komo_ws
                .containers()
                .iter()
                .filter_map(|c| container_map.0.get(c.id().as_str()))
                .copied()
                .collect::<IndexSet<Entity>>();

            match workspace_map.0.entry(key) {
                Entry::Occupied(entry) => {
                    let entity = *entry.get();

                    if let Ok(mut workspace) = existing_workspaces.get_mut(entity) {
                        *workspace = komo_ws.clone();
                    }

                    commands
                        .entity(entity)
                        .insert(KomotoolRing(container_entities))
                        .insert(Focused(focused_idx));
                }
                Entry::Vacant(entry) => {
                    let entity = commands
                        .spawn((
                            komo_ws.clone(),
                            KomotoolRing(container_entities),
                            Focused(focused_idx),
                        ))
                        .id();
                    entry.insert(entity);
                }
            }
        }
    }

    workspace_map.0.retain(|key, entity| {
        if current_keys.contains(key) {
            true
        } else {
            commands.entity(*entity).despawn();
            false
        }
    });
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
            }
            Entry::Vacant(entry) => {
                // Spawn new monitor
                let entity = commands.spawn(komo_mon.clone()).id();
                entry.insert(entity);
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
    mut existing_windows: Query<&mut Window>,
    komorebi_state: Res<KomorebiState>,
    mut window_map: ResMut<WindowToEntityMap>,
) {
    let Some(state) = &komorebi_state.current else {
        return;
    };

    // Track windows that still exist in current state
    let mut current_hwnds = HashSet::new();

    // First pass: Update existing or spawn new windows
    for komo_mon in state.monitors.elements() {
        let workspaces = komo_mon.workspaces();
        for komo_ws in workspaces.iter() {
            for komo_cont in komo_ws.containers() {
                for komo_win in komo_cont.windows() {
                    let hwnd = komo_win.hwnd.to_string();
                    current_hwnds.insert(hwnd.clone());

                    match window_map.0.entry(hwnd) {
                        Entry::Occupied(entry) => {
                            let entity = *entry.get();

                            // Update existing window component
                            if let Ok(mut window) = existing_windows.get_mut(entity) {
                                *window = *komo_win;
                            }
                        }
                        Entry::Vacant(entry) => {
                            // Spawn new window
                            let entity = commands.spawn(*komo_win).id();
                            entry.insert(entity);
                        }
                    }
                }
            }
        }
    }

    // Second pass: Remove windows that no longer exist
    window_map.0.retain(|hwnd, entity| {
        if current_hwnds.contains(hwnd) {
            true
        } else {
            commands.entity(*entity).despawn();
            false
        }
    });
}

pub fn import_komorebi_container_state(
    mut commands: Commands,
    mut existing_containers: Query<&mut Container>,
    komorebi_state: Res<KomorebiState>,
    mut container_map: ResMut<ContainerToEntityMap>,
    window_map: Res<WindowToEntityMap>,
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

                // Get window entities for this container's windows
                let window_entities = komo_cont
                    .windows()
                    .iter()
                    .filter_map(|w| window_map.0.get(&w.hwnd.to_string()))
                    .copied()
                    .collect::<IndexSet<Entity>>();

                // Get focused index directly as usize
                let focused_idx = komo_cont.focused_window_idx();

                match container_map.0.entry(id.clone()) {
                    Entry::Occupied(entry) => {
                        let entity = *entry.get();

                        // Update existing container component
                        if let Ok(mut container) = existing_containers.get_mut(entity) {
                            *container = komo_cont.clone();
                        }

                        // Insert/update WindowRing component and Focused
                        commands
                            .entity(entity)
                            .insert(KomotoolRing(window_entities))
                            .insert(Focused(focused_idx));
                    }
                    Entry::Vacant(entry) => {
                        // Spawn new container with WindowRing and Focused
                        let entity = commands
                            .spawn((
                                komo_cont.clone(),
                                KomotoolRing(window_entities),
                                Focused(focused_idx),
                            ))
                            .id();
                        entry.insert(entity);
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
