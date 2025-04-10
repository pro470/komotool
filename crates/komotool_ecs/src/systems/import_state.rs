use crate::components::Focused;
use crate::components::*;
use crate::resources::*;
use crate::RelationRegistry;
use bevy_ecs::query::QueryEntityError;
use bevy_ecs::system::{Commands, Query, Res, ResMut};
use komorebi_client::{Container, Monitor, Window, Workspace};
use std::collections::{hash_map::Entry, HashSet};

pub fn import_komorebi_workspace_state(
    mut commands: Commands,
    mut existing_workspaces: Query<&mut Workspace>,
    komorebi_state: Res<KomorebiState>,
    mut workspace_map: ResMut<WorkspaceToEntityMap>,
    registry: Res<RelationRegistry>,
    extended_marker_map: Res<ExtendedMarkerMap>,
    mut keep_alive_workspaces: ResMut<KeepAliveWorkspaces>,
) {
    let Some(state) = &komorebi_state.komorebi else {
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

            match workspace_map.0.entry(key) {
                Entry::Occupied(entry) => {
                    let entity = *entry.get();

                    // Despawn all relevant marker components
                    if let Some(record) = registry.records.get(&entity) {
                        // Despawn Monitor marker
                        if record.monitor > 0 {
                            despawn_monitor_marker_component(
                                record.monitor,
                                entity,
                                commands.reborrow(),
                                &extended_marker_map,
                            );
                        }
                        // Despawn Workspace marker
                        if record.workspace > 0 {
                            despawn_workspace_marker_component(
                                record.workspace,
                                entity,
                                commands.reborrow(),
                                &extended_marker_map,
                            );
                        }
                    }

                    if let Ok(mut workspace) = existing_workspaces.get_mut(entity) {
                        *workspace = komo_ws.clone();
                    }
                }
                Entry::Vacant(entry) => {
                    let entity = commands.spawn(komo_ws.clone()).id();
                    entry.insert(entity);
                }
            }
        }
    }

    workspace_map.0.retain(|key, entity| {
        if current_keys.contains(key) {
            // Workspace is still managed by Komorebi, keep it.
            // Markers were cleared during the update phase if it existed previously.
            // build_relation_registry will add the correct markers and focus state.
            true
        } else {
            // Workspace is no longer managed by Komorebi. Check if we should keep it alive.
            if keep_alive_workspaces.0.contains(entity) {
                // Check if the entity actually still exists and has the component
                match existing_workspaces.get(*entity) {
                    Ok(_) => {
                        // Entity exists: Keep it alive, but remove its markers and focus.
                        if let Some(record) = registry.records.get(entity) {
                            // Despawn Monitor marker
                            if record.monitor > 0 {
                                despawn_monitor_marker_component(
                                    record.monitor,
                                    *entity,
                                    commands.reborrow(),
                                    &extended_marker_map,
                                );
                            }
                            // Despawn Workspace marker
                            if record.workspace > 0 {
                                despawn_workspace_marker_component(
                                    record.workspace,
                                    *entity,
                                    commands.reborrow(),
                                    &extended_marker_map,
                                );
                            }
                        }
                        commands.entity(*entity).remove::<Focused>();
                        true // Keep the entity in the map
                    }
                    Err(error) => {
                        match error {
                            QueryEntityError::AliasedMutability(_) => {
                                // Entity exists but is mutably borrowed elsewhere. Keep it.
                                true // Keep in map
                            }
                            QueryEntityError::QueryDoesNotMatch(_, _)
                            | QueryEntityError::NoSuchEntity(_) => {
                                // Entity doesn't exist or lacks component, despite being in KeepAlive.
                                // Clean up KeepAlive entry and remove from map.
                                keep_alive_workspaces.0.remove(entity);
                                false // Remove from map (entity might already be gone)
                            }
                        }
                    }
                }
            } else {
                // Not marked to keep alive and not in current state: Despawn the entity entirely.
                commands.entity(*entity).despawn();
                false // Remove from map
            }
        }
    });
}

pub fn import_komorebi_monitor_state(
    mut commands: Commands,
    mut existing_monitors: Query<&mut Monitor>,
    komorebi_state: Res<KomorebiState>,
    mut monitor_map: ResMut<MonitorToEntityMap>,
    registry: Res<RelationRegistry>,
    extended_marker_map: Res<ExtendedMarkerMap>,
    mut keep_alive_monitors: ResMut<KeepAliveMonitors>,
) {
    let Some(state) = &komorebi_state.komorebi else {
        return;
    };

    let mut current_serials = HashSet::new();

    for komo_mon in state.monitors.elements() {
        let Some(serial) = komo_mon.serial_number_id() else {
            continue;
        };
        current_serials.insert(serial.clone());

        match monitor_map.0.entry(serial.clone()) {
            Entry::Occupied(entry) => {
                let entity = *entry.get();

                // Despawn the old marker component if it exists
                if let Some(record) = registry.records.get(&entity) {
                    if record.monitor > 0 {
                        despawn_monitor_marker_component(
                            record.monitor,
                            entity,
                            commands.reborrow(),
                            &extended_marker_map,
                        );
                    }
                }

                if let Ok(mut monitor) = existing_monitors.get_mut(entity) {
                    *monitor = komo_mon.clone();
                }
            }
            Entry::Vacant(entry) => {
                let entity = commands.spawn(komo_mon.clone()).id();
                entry.insert(entity);
            }
        }
    }

    monitor_map.0.retain(|serial, entity| {
        if current_serials.contains(serial) {
            // Monitor is still managed by Komorebi, keep it.
            // Markers were cleared during the update phase if it existed previously.
            // build_relation_registry will add the correct markers and focus state.
            true
        } else {
            // Monitor is no longer managed by Komorebi. Check if we should keep it alive.
            if keep_alive_monitors.0.contains(entity) {
                // Check if the entity actually still exists and has the component
                match existing_monitors.get(*entity) {
                    Ok(_) => {
                        // Entity exists: Keep it alive, but remove its marker and focus.
                        if let Some(record) = registry.records.get(entity) {
                            if record.monitor > 0 {
                                despawn_monitor_marker_component(
                                    record.monitor,
                                    *entity,
                                    commands.reborrow(),
                                    &extended_marker_map,
                                );
                            }
                        }
                        commands.entity(*entity).remove::<Focused>();
                        true // Keep the entity in the map
                    }
                    Err(error) => {
                        match error {
                            QueryEntityError::AliasedMutability(_) => {
                                // Entity exists but is mutably borrowed elsewhere. Keep it.
                                true // Keep in map
                            }
                            QueryEntityError::QueryDoesNotMatch(_, _)
                            | QueryEntityError::NoSuchEntity(_) => {
                                // Entity doesn't exist or lacks component, despite being in KeepAlive.
                                // Clean up KeepAlive entry and remove from map.
                                keep_alive_monitors.0.remove(entity);
                                false // Remove from map (entity might already be gone)
                            }
                        }
                    }
                }
            } else {
                // Not marked to keep alive and not in current state: Despawn the entity entirely.
                commands.entity(*entity).despawn();
                false // Remove from map
            }
        }
    });
}

pub fn import_komorebi_window_state(
    mut commands: Commands,
    mut existing_windows: Query<&mut Window>,
    komorebi_state: Res<KomorebiState>,
    mut window_map: ResMut<WindowToEntityMap>,
    registry: Res<RelationRegistry>,
    extended_marker_map: Res<ExtendedMarkerMap>,
) {
    let Some(state) = &komorebi_state.komorebi else {
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

                            // Despawn all relevant marker components
                            if let Some(record) = registry.records.get(&entity) {
                                // Despawn Monitor marker
                                if record.monitor > 0 {
                                    despawn_monitor_marker_component(
                                        record.monitor,
                                        entity,
                                        commands.reborrow(),
                                        &extended_marker_map,
                                    );
                                }
                                // Despawn Workspace marker
                                if record.workspace > 0 {
                                    despawn_workspace_marker_component(
                                        record.workspace,
                                        entity,
                                        commands.reborrow(),
                                        &extended_marker_map,
                                    );
                                }
                                // Despawn Container marker
                                if record.container > 0 {
                                    despawn_container_marker_component(
                                        record.container,
                                        entity,
                                        commands.reborrow(),
                                        &extended_marker_map,
                                    );
                                }
                                // Despawn Window marker
                                if record.window > 0 {
                                    despawn_window_marker_component(
                                        record.window,
                                        entity,
                                        commands.reborrow(),
                                        &extended_marker_map,
                                    );
                                }
                            }

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

    // Second pass: Retain/Remove windows based on presence in current state and process status
    window_map.0.retain(|_hwnd, entity| {
        if current_hwnds.contains(_hwnd) {
            // Window is still managed by Komorebi, keep it.
            // Markers were cleared during the update phase if it existed previously.
            // build_relation_registry will add the correct markers and focus state.
            true
        } else {
            // Window is no longer managed by Komorebi. Check if its process still exists.
            match existing_windows.get(*entity) {
                Ok(window) => {
                    match window.exe() {
                        Ok(_) => {
                            // Process exists: Keep entity but remove markers and focus.
                            // Markers *should* have been removed if it was previously managed,
                            // but clear them again just in case it's an edge case.
                            if let Some(record) = registry.records.get(entity) {
                                // Despawn Monitor marker
                                if record.monitor > 0 {
                                    despawn_monitor_marker_component(
                                        record.monitor,
                                        *entity,
                                        commands.reborrow(),
                                        &extended_marker_map,
                                    );
                                }
                                // Despawn Workspace marker
                                if record.workspace > 0 {
                                    despawn_workspace_marker_component(
                                        record.workspace,
                                        *entity,
                                        commands.reborrow(),
                                        &extended_marker_map,
                                    );
                                }
                                // Despawn Container marker
                                if record.container > 0 {
                                    despawn_container_marker_component(
                                        record.container,
                                        *entity,
                                        commands.reborrow(),
                                        &extended_marker_map,
                                    );
                                }
                                // Despawn Window marker
                                if record.window > 0 {
                                    despawn_window_marker_component(
                                        record.window,
                                        *entity,
                                        commands.reborrow(),
                                        &extended_marker_map,
                                    );
                                }
                            }
                            // Ensure focus is removed as it's no longer managed
                            commands.entity(*entity).remove::<Focused>();
                            true // Keep the entity in the map
                        }
                        Err(_) => {
                            // Process doesn't exist: Despawn the entity entirely.
                            commands.entity(*entity).despawn();
                            false // Remove from map
                        }
                    }
                }
                Err(error) => {
                    match error {
                        QueryEntityError::AliasedMutability(_) => {
                            // Entity exists and has the component, but is mutably borrowed elsewhere.
                            // Keep the entity, don't despawn.
                            true // Keep in map
                        }
                        QueryEntityError::QueryDoesNotMatch(_, _)
                        | QueryEntityError::NoSuchEntity(_) => {
                            // Entity doesn't have the component or doesn't exist. Despawn.
                            commands.entity(*entity).despawn();
                            false // Remove from map
                        }
                    }
                }
            }
        }
    });
}

pub fn import_komorebi_container_state(
    mut commands: Commands,
    mut existing_containers: Query<&mut Container>,
    komorebi_state: Res<KomorebiState>,
    mut container_map: ResMut<ContainerToEntityMap>,
    registry: Res<RelationRegistry>,
    extended_marker_map: Res<ExtendedMarkerMap>,
    mut keep_alive_containers: ResMut<KeepAliveContainers>,
) {
    let Some(state) = &komorebi_state.komorebi else {
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

                        // Despawn all relevant marker components
                        if let Some(record) = registry.records.get(&entity) {
                            // Despawn Monitor marker
                            if record.monitor > 0 {
                                despawn_monitor_marker_component(
                                    record.monitor,
                                    entity,
                                    commands.reborrow(),
                                    &extended_marker_map,
                                );
                            }
                            // Despawn Workspace marker
                            if record.workspace > 0 {
                                despawn_workspace_marker_component(
                                    record.workspace,
                                    entity,
                                    commands.reborrow(),
                                    &extended_marker_map,
                                );
                            }
                            // Despawn Container marker
                            if record.container > 0 {
                                despawn_container_marker_component(
                                    record.container,
                                    entity,
                                    commands.reborrow(),
                                    &extended_marker_map,
                                );
                            }
                        }

                        // Update existing container component
                        if let Ok(mut container) = existing_containers.get_mut(entity) {
                            *container = komo_cont.clone();
                        }

                        // Insert/update WindowRing component
                    }
                    Entry::Vacant(entry) => {
                        // Spawn new container with WindowRing
                        let entity = commands.spawn(komo_cont.clone()).id();
                        entry.insert(entity);
                    }
                }
            }
        }
    }

    // Second pass: Remove containers that no longer exist
    container_map.0.retain(|id, entity| {
        if current_ids.contains(id) {
            // Container is still managed by Komorebi, keep it.
            // Markers were cleared during the update phase if it existed previously.
            // build_relation_registry will add the correct markers and focus state.
            true
        } else {
            // Container is no longer managed by Komorebi. Check if we should keep it alive.
            if keep_alive_containers.0.contains(entity) {
                // Check if the entity actually still exists and has the component
                match existing_containers.get(*entity) {
                    Ok(_) => {
                        // Entity exists: Keep it alive, but remove its markers and focus.
                        if let Some(record) = registry.records.get(entity) {
                            // Despawn Monitor marker
                            if record.monitor > 0 {
                                despawn_monitor_marker_component(
                                    record.monitor,
                                    *entity,
                                    commands.reborrow(),
                                    &extended_marker_map,
                                );
                            }
                            // Despawn Workspace marker
                            if record.workspace > 0 {
                                despawn_workspace_marker_component(
                                    record.workspace,
                                    *entity,
                                    commands.reborrow(),
                                    &extended_marker_map,
                                );
                            }
                            // Despawn Container marker
                            if record.container > 0 {
                                despawn_container_marker_component(
                                    record.container,
                                    *entity,
                                    commands.reborrow(),
                                    &extended_marker_map,
                                );
                            }
                        }
                        commands.entity(*entity).remove::<Focused>();
                        true // Keep the entity in the map
                    }
                    Err(error) => {
                        match error {
                            QueryEntityError::AliasedMutability(_) => {
                                // Entity exists but is mutably borrowed elsewhere. Keep it.
                                true // Keep in map
                            }
                            QueryEntityError::QueryDoesNotMatch(_, _)
                            | QueryEntityError::NoSuchEntity(_) => {
                                // Entity doesn't exist or lacks component, despite being in KeepAlive.
                                // Clean up KeepAlive entry and remove from map.
                                keep_alive_containers.0.remove(entity);
                                false // Remove from map (entity might already be gone)
                            }
                        }
                    }
                }
            } else {
                // Not marked to keep alive and not in current state: Despawn the entity entirely.
                commands.entity(*entity).despawn();
                false // Remove from map
            }
        }
    });
}

pub fn import_komorebi_appstate_state(
    mut app_state: ResMut<AppState>,
    komorebi_state: Res<KomorebiState>,
) {
    if let Some(state) = &komorebi_state.komorebi {
        // Update AppState
        *app_state = AppState {
            is_paused: state.is_paused,
            monitor_usr_idx_map: state.monitor_usr_idx_map.clone(),
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

pub fn build_relation_registry(
    mut commands: Commands,
    komorebi_state: Res<KomorebiState>,
    monitor_map: Res<MonitorToEntityMap>,
    workspace_map: Res<WorkspaceToEntityMap>,
    container_map: Res<ContainerToEntityMap>,
    window_map: Res<WindowToEntityMap>,
    mut registry: ResMut<RelationRegistry>,
    extended_marker_map: Res<ExtendedMarkerMap>,
) {
    registry.records.clear();

    let Some(state) = &komorebi_state.komorebi else {
        return;
    };

    for (monitor_idx, komo_mon) in state.monitors.elements().iter().enumerate() {
        let Some(serial) = komo_mon.serial_number_id() else {
            continue;
        };
        let Some(monitor_entity) = monitor_map.0.get(serial) else {
            continue;
        };
        let monitor_marker_idx = monitor_idx + 1; // 1-based index

        // Insert Monitor Markers
        insert_monitor_marker_component(
            monitor_marker_idx,
            *monitor_entity,
            commands.reborrow(),
            &extended_marker_map,
        );

        // Check and insert/remove monitor focus
        if state.monitors.focused_idx() == monitor_idx {
            commands.entity(*monitor_entity).insert(Focused);
        } else {
            commands.entity(*monitor_entity).remove::<Focused>();
        }

        registry.insert(*monitor_entity, monitor_marker_idx, 0, 0, 0);

        for (workspace_idx, komo_ws) in komo_mon.workspaces().iter().enumerate() {
            let Some(name) = komo_ws.name() else {
                continue;
            };
            let Some(workspace_entity) = workspace_map.0.get(name) else {
                continue;
            };
            let workspace_marker_idx = workspace_idx + 1; // 1-based index

            // Insert Workspace Markers
            insert_monitor_marker_component(
                monitor_marker_idx,
                *workspace_entity,
                commands.reborrow(),
                &extended_marker_map,
            );
            insert_workspace_marker_component(
                workspace_marker_idx,
                *workspace_entity,
                commands.reborrow(),
                &extended_marker_map,
            );

            // Check and insert/remove workspace focus
            if komo_mon.focused_workspace_idx() == workspace_idx {
                commands.entity(*workspace_entity).insert(Focused);
            } else {
                commands.entity(*workspace_entity).remove::<Focused>();
            }

            registry.insert(
                *workspace_entity,
                monitor_marker_idx,
                workspace_marker_idx,
                0,
                0,
            );

            for (container_idx, komo_cont) in komo_ws.containers().iter().enumerate() {
                let Some(container_entity) = container_map.0.get(komo_cont.id()) else {
                    continue;
                };
                let container_marker_idx = container_idx + 1; // 1-based index

                // Insert Container Markers
                insert_monitor_marker_component(
                    monitor_marker_idx,
                    *container_entity,
                    commands.reborrow(),
                    &extended_marker_map,
                );
                insert_workspace_marker_component(
                    workspace_marker_idx,
                    *container_entity,
                    commands.reborrow(),
                    &extended_marker_map,
                );
                insert_container_marker_component(
                    container_marker_idx,
                    *container_entity,
                    commands.reborrow(),
                    &extended_marker_map,
                );

                // Check and insert/remove container focus
                if komo_ws.focused_container_idx() == container_idx {
                    commands.entity(*container_entity).insert(Focused);
                } else {
                    commands.entity(*container_entity).remove::<Focused>();
                }

                registry.insert(
                    *container_entity,
                    monitor_marker_idx,
                    workspace_marker_idx,
                    container_marker_idx,
                    0,
                );

                for (window_idx, komo_win) in komo_cont.windows().iter().enumerate() {
                    let hwnd_str = komo_win.hwnd.to_string();
                    let Some(window_entity) = window_map.0.get(&hwnd_str) else {
                        continue;
                    };
                    let window_marker_idx = window_idx + 1; // 1-based index

                    // Insert Window Markers
                    insert_monitor_marker_component(
                        monitor_marker_idx,
                        *window_entity,
                        commands.reborrow(),
                        &extended_marker_map,
                    );
                    insert_workspace_marker_component(
                        workspace_marker_idx,
                        *window_entity,
                        commands.reborrow(),
                        &extended_marker_map,
                    );
                    insert_container_marker_component(
                        container_marker_idx,
                        *window_entity,
                        commands.reborrow(),
                        &extended_marker_map,
                    );
                    insert_window_marker_component(
                        window_marker_idx,
                        *window_entity,
                        commands.reborrow(),
                        &extended_marker_map,
                    );

                    // Check and insert/remove window focus
                    if komo_cont.focused_window_idx() == window_idx {
                        commands.entity(*window_entity).insert(Focused);
                    } else {
                        commands.entity(*window_entity).remove::<Focused>();
                    }

                    registry.insert(
                        *window_entity,
                        monitor_marker_idx,
                        workspace_marker_idx,
                        container_marker_idx,
                        window_marker_idx,
                    );
                }
            }
        }
    }
}
