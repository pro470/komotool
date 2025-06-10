use crate::RelationRegistry;
use crate::components::Focused;
use crate::components::*;
use crate::prelude::WorkspaceChildOf;
use crate::relationships::{ContainerChildOf, MonitorChildOf, WindowManagerChildOf};
#[cfg(not(debug_assertions))]
use crate::relationships::{
    ContainerChildren, MonitorChildren, WindowManagerChildren, WorkspaceChildren,
};
use crate::resources::*;
use bevy_ecs::entity::{ContainsEntity, Entity};
use bevy_ecs::query::{QueryEntityError, With};
use bevy_ecs::system::{Commands, Query, Res, ResMut, Single};
use komorebi_client::{Container, Monitor, Window, Workspace};
use std::collections::{HashSet, hash_map::Entry};

pub fn import_komorebi_workspace_state(
    mut commands: Commands,
    mut existing_workspaces: Query<&mut Workspace>,
    komorebi_state: Res<KomorebiState>,
    mut workspace_map: ResMut<WorkspaceToEntityMap>,
    monitor_map: Res<MonitorToEntityMap>,
    mut keep_alive_workspaces: ResMut<KeepAliveWorkspaces>,
) {
    let Some(state) = &komorebi_state.komorebi else {
        return;
    };

    let mut current_keys = HashSet::new();

    for komo_mon in state.monitors.elements() {
        let Some(serial) = komo_mon.serial_number_id() else {
            continue;
        };
        let monitor_entity = monitor_map.0.get(serial).unwrap_or(&Entity::PLACEHOLDER);
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

                    if let Ok(mut workspace) = existing_workspaces.get_mut(entity) {
                        *workspace = komo_ws.clone();
                        commands
                            .entity(*monitor_entity)
                            .add_one_related::<MonitorChildOf>(entity);
                        #[cfg(not(debug_assertions))]
                        {
                            // This code will only be included in release builds
                            commands.entity(entity).remove::<WorkspaceChildren>();
                        }
                    }
                }
                Entry::Vacant(entry) => {
                    let entity = commands
                        .spawn((komo_ws.clone(), MonitorChildOf(*monitor_entity)))
                        .id();
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
                        commands.entity(*entity).remove::<Focused>();
                        commands.entity(*entity).remove::<MonitorChildOf>();
                        true // Keep the entity in the map
                    }
                    Err(error) => {
                        match error {
                            QueryEntityError::AliasedMutability(_) => {
                                // Entity exists but is mutably borrowed elsewhere. Keep it.
                                commands.entity(*entity).remove::<Focused>();
                                commands.entity(*entity).remove::<MonitorChildOf>();
                                true // Keep in map
                            }
                            QueryEntityError::QueryDoesNotMatch(_, _)
                            | QueryEntityError::EntityDoesNotExist(_) => {
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
    mut keep_alive_monitors: ResMut<KeepAliveMonitors>,
    window_manager_entity: Single<Entity, With<WindowManager>>,
) {
    let Some(state) = &komorebi_state.komorebi else {
        return;
    };

    let mut current_serials = HashSet::new();

    let window_manager_entity = window_manager_entity.entity();
    #[cfg(not(debug_assertions))]
    {
        // This code will only be included in release builds
        commands
            .entity(window_manager_entity)
            .remove::<WindowManagerChildren>();
    }

    for komo_mon in state.monitors.elements() {
        let Some(serial) = komo_mon.serial_number_id() else {
            continue;
        };
        current_serials.insert(serial.clone());

        match monitor_map.0.entry(serial.clone()) {
            Entry::Occupied(entry) => {
                let entity = *entry.get();

                if let Ok(mut monitor) = existing_monitors.get_mut(entity) {
                    *monitor = komo_mon.clone();
                    commands
                        .entity(window_manager_entity)
                        .add_one_related::<WindowManagerChildOf>(entity);
                    #[cfg(not(debug_assertions))]
                    {
                        // This code will only be included in release builds
                        commands.entity(entity).remove::<MonitorChildren>();
                    }
                }
            }
            Entry::Vacant(entry) => {
                let entity = commands.spawn(komo_mon.clone()).id();
                entry.insert(entity);
                commands
                    .entity(window_manager_entity)
                    .add_one_related::<WindowManagerChildOf>(entity);
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
                        commands.entity(*entity).remove::<Focused>();
                        commands.entity(*entity).remove::<WindowManagerChildOf>();
                        true // Keep the entity in the map
                    }
                    Err(error) => {
                        match error {
                            QueryEntityError::AliasedMutability(_) => {
                                commands.entity(*entity).remove::<Focused>();
                                commands.entity(*entity).remove::<WindowManagerChildOf>();
                                // Entity exists but is mutably borrowed elsewhere. Keep it.
                                true // Keep in map
                            }
                            QueryEntityError::QueryDoesNotMatch(_, _)
                            | QueryEntityError::EntityDoesNotExist(_) => {
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
    container_map: Res<ContainerToEntityMap>,
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
                let container_entity = match container_map.0.get(komo_cont.id()) {
                    Some(entity) => *entity,
                    None => continue,
                };

                for komo_win in komo_cont.windows() {
                    let hwnd = komo_win.hwnd.to_string();
                    current_hwnds.insert(hwnd.clone());

                    match window_map.0.entry(hwnd) {
                        Entry::Occupied(entry) => {
                            let entity = *entry.get();

                            // Update existing window component
                            if let Ok(mut window) = existing_windows.get_mut(entity) {
                                *window = *komo_win;
                                commands.entity(entity).remove::<MaximizedWindow>();
                                commands
                                    .entity(container_entity)
                                    .add_one_related::<ContainerChildOf>(entity);
                            }
                        }
                        Entry::Vacant(entry) => {
                            // Spawn new window
                            let entity = commands.spawn(*komo_win).id();
                            entry.insert(entity);
                            commands
                                .entity(container_entity)
                                .add_one_related::<ContainerChildOf>(entity);
                        }
                    }
                }
            }

            if let Some(max) = komo_ws.maximized_window() {
                let hwnd = max.hwnd;
                current_hwnds.insert(hwnd.to_string().clone());
                match window_map.0.entry(hwnd.to_string().clone()) {
                    Entry::Occupied(entry) => {
                        let entity = *entry.get();
                        commands.entity(entity).insert(MaximizedWindow);
                    }

                    Entry::Vacant(entry) => {
                        let entity = commands.spawn((*max, MaximizedWindow)).id();
                        entry.insert(entity);
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
                            // Ensure focus is removed as it's no longer managed
                            commands.entity(*entity).remove::<Focused>();
                            commands.entity(*entity).remove::<ContainerChildOf>();
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
                            commands.entity(*entity).remove::<Focused>();
                            commands.entity(*entity).remove::<ContainerChildOf>();
                            true // Keep in map
                        }
                        QueryEntityError::QueryDoesNotMatch(_, _)
                        | QueryEntityError::EntityDoesNotExist(_) => {
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
    workspace_map: Res<WorkspaceToEntityMap>,
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
            let Some(name) = komo_ws.name() else {
                continue;
            };
            let workspace_entity = workspace_map.0.get(name).unwrap_or(&Entity::PLACEHOLDER);
            for komo_cont in komo_ws.containers() {
                let id = komo_cont.id();
                current_ids.insert(id.clone());

                match container_map.0.entry(id.clone()) {
                    Entry::Occupied(entry) => {
                        let entity = *entry.get();

                        // Update existing container component
                        if let Ok(mut container) = existing_containers.get_mut(entity) {
                            *container = komo_cont.clone();
                            commands
                                .entity(*workspace_entity)
                                .add_one_related::<WorkspaceChildOf>(entity);
                            #[cfg(not(debug_assertions))]
                            {
                                // This code will only be included in release builds
                                commands.entity(entity).remove::<ContainerChildren>();
                            }
                            commands.entity(entity).remove::<MonocleContainer>();
                        }

                        // Insert/update WindowRing component
                    }
                    Entry::Vacant(entry) => {
                        // Spawn new container with WindowRing
                        let entity = commands.spawn(komo_cont.clone()).id();
                        entry.insert(entity);
                        commands
                            .entity(*workspace_entity)
                            .add_one_related::<WorkspaceChildOf>(entity);
                    }
                }
            }
            if let Some(monocle) = komo_ws.monocle_container() {
                let id = monocle.id();
                current_ids.insert(id.clone());
                match container_map.0.entry(id.clone()) {
                    Entry::Occupied(entry) => {
                        let entity = *entry.get();
                        commands.entity(entity).insert(MonocleContainer);
                    }

                    Entry::Vacant(entry) => {
                        // Spawn new container with MonocleContainer
                        let entity = commands.spawn((monocle.clone(), MonocleContainer)).id();
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
                        commands.entity(*entity).remove::<Focused>();
                        commands.entity(*entity).remove::<WorkspaceChildOf>();
                        true // Keep the entity in the map
                    }
                    Err(error) => {
                        match error {
                            QueryEntityError::AliasedMutability(_) => {
                                // Entity exists but is mutably borrowed elsewhere. Keep it.
                                commands.entity(*entity).remove::<Focused>();
                                commands.entity(*entity).remove::<WorkspaceChildOf>();
                                true // Keep in map
                            }
                            QueryEntityError::QueryDoesNotMatch(_, _)
                            | QueryEntityError::EntityDoesNotExist(_) => {
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
    window_extended_marker_map: Res<WindowExtendedMarkerMap>,
    container_extended_marker_map: Res<ContainerExtendedMarkerMap>,
    workspace_extended_marker_map: Res<WorkspaceExtendedMarkerMap>,
    monitor_extended_marker_map: Res<MonitorExtendedMarkerMap>,
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
            &monitor_extended_marker_map,
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
                &monitor_extended_marker_map,
            );
            insert_workspace_marker_component(
                workspace_marker_idx,
                *workspace_entity,
                commands.reborrow(),
                &workspace_extended_marker_map,
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
                    &monitor_extended_marker_map,
                );
                insert_workspace_marker_component(
                    workspace_marker_idx,
                    *container_entity,
                    commands.reborrow(),
                    &workspace_extended_marker_map,
                );
                insert_container_marker_component(
                    container_marker_idx,
                    *container_entity,
                    commands.reborrow(),
                    &container_extended_marker_map,
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
                        &monitor_extended_marker_map,
                    );
                    insert_workspace_marker_component(
                        workspace_marker_idx,
                        *window_entity,
                        commands.reborrow(),
                        &workspace_extended_marker_map,
                    );
                    insert_container_marker_component(
                        container_marker_idx,
                        *window_entity,
                        commands.reborrow(),
                        &container_extended_marker_map,
                    );
                    insert_window_marker_component(
                        window_marker_idx,
                        *window_entity,
                        commands.reborrow(),
                        &window_extended_marker_map,
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
            if let Some(monocle) = komo_ws.monocle_container() {
                if let Some(monocle_entity) = container_map.0.get(monocle.id()) {
                    insert_monitor_marker_component(
                        monitor_marker_idx,
                        *monocle_entity,
                        commands.reborrow(),
                        &monitor_extended_marker_map,
                    );
                    insert_workspace_marker_component(
                        workspace_marker_idx,
                        *monocle_entity,
                        commands.reborrow(),
                        &workspace_extended_marker_map,
                    );
                };
            }

            if let Some(maximized) = komo_ws.maximized_window() {
                if let Some(maximized_entity) = window_map.0.get(&maximized.hwnd.to_string()) {
                    insert_monitor_marker_component(
                        monitor_marker_idx,
                        *maximized_entity,
                        commands.reborrow(),
                        &monitor_extended_marker_map,
                    );
                    insert_workspace_marker_component(
                        workspace_marker_idx,
                        *maximized_entity,
                        commands.reborrow(),
                        &workspace_extended_marker_map,
                    );
                }
            }
        }
    }
}

pub fn spawn_window_manager(mut commands: Commands) {
    commands.spawn(WindowManager);
}
