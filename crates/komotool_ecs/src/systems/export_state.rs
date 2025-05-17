use crate::KomorebiState;
use crate::components::{FloatingWindow, Focused, MaximizedWindow, MonocleContainer};
use crate::relations::registry::RelationRegistry;
use crate::resources::{AppState, KomotoolCommandQueue, KomotoolState};
use bevy_ecs::error::{HandleError, warn};
use bevy_ecs::query::With;
use bevy_ecs::system::command::run_system_cached;
use bevy_ecs::system::{Query, Res, ResMut};
use bevy_ecs::world::{CommandQueue, World};
use komorebi_client::{
    Container, Monitor, MoveBehaviour, OperationBehaviour, Ring, SocketMessage, State, Window,
    WindowContainerBehaviour, Workspace, send_message,
};
use komotool_utils::startup_schedule::remove_komotool_startup_schedule;
use std::collections::HashMap;

#[allow(clippy::too_many_arguments)]
pub fn export_state(
    registry: Res<RelationRegistry>,
    app_state: Res<AppState>,
    monitor_query: Query<&Monitor>,
    workspace_query: Query<&Workspace>,
    container_query: Query<&Container>,
    window_query: Query<(&Window, Option<&FloatingWindow>)>,
    focused_query: Query<(), With<Focused>>,
    komorebi_state: Res<KomorebiState>,
) {
    // 1. Initialize State with global properties
    let mut state = State {
        monitors: Ring::default(),
        monitor_usr_idx_map: app_state.monitor_usr_idx_map.clone(), // Will be populated later
        is_paused: app_state.is_paused,
        resize_delta: app_state.resize_delta,
        new_window_behaviour: app_state.new_window_behaviour,
        float_override: app_state.float_override,
        cross_monitor_move_behaviour: app_state.cross_monitor_move_behaviour,
        unmanaged_window_operation_behaviour: app_state.unmanaged_window_operation_behaviour,
        work_area_offset: app_state.work_area_offset,
        focus_follows_mouse: app_state.focus_follows_mouse,
        mouse_follows_focus: app_state.mouse_follows_focus,
        has_pending_raise_op: app_state.has_pending_raise_op,
    };

    // 2. Intermediate structures for building the state
    let mut monitors_vec: Vec<Monitor> = Vec::new();
    let mut current_monitor_opt: Option<(Monitor, usize)> = None; // (Monitor data, registry_m_idx)
    let mut current_workspace_opt: Option<Workspace> = None;
    let mut current_container_opt: Option<Container> = None;

    let mut monitor_focus_idx: Option<usize> = None;
    let mut workspace_focus_idx: Option<usize> = None;
    let mut container_focus_idx: Option<usize> = None;
    let mut window_focus_idx: Option<usize> = None;

    // 3. Iterate through sorted registry records to reconstruct hierarchy
    for record in &registry.records {
        let entity = record.entity;

        let (m_idx, w_idx, c_idx, win_idx) = record.key();

        // Check focus *before* potentially switching context
        let is_focused = focused_query.get(entity).is_ok();

        // --- Monitor Level ---
        if w_idx == 0 {
            // Finish previous monitor if exists
            if let Some((mut current_monitor, _)) = current_monitor_opt.take() {
                if let Some(mut current_workspace) = current_workspace_opt.take() {
                    if let Some(current_container) = current_container_opt.take() {
                        current_workspace
                            .containers
                            .elements_mut()
                            .push_back(current_container);
                    }
                    if let Some(focus_idx) = container_focus_idx.take() {
                        current_workspace.containers.focus(focus_idx);
                    }
                    current_monitor
                        .workspaces
                        .elements_mut()
                        .push_back(current_workspace);
                }
                if let Some(focus_idx) = workspace_focus_idx.take() {
                    current_monitor.workspaces.focus(focus_idx);
                }
                monitors_vec.push(current_monitor);
            }

            // Start new monitor
            if let Ok(monitor_data) = monitor_query.get(entity) {
                let mut new_monitor = monitor_data.clone();
                new_monitor.workspaces = Ring::default(); // Ensure clean workspaces ring
                current_monitor_opt = Some((new_monitor, m_idx));
                if is_focused {
                    monitor_focus_idx = Some(monitors_vec.len());
                }
            } else {
                eprintln!("Monitor entity {:?} not found in query", entity);
                continue; // Skip processing this branch if monitor data is missing
            }
            continue; // Move to next record
        }

        // --- Workspace Level ---
        if c_idx == 0 {
            // Finish previous workspace if exists
            if let Some(mut current_workspace) = current_workspace_opt.take() {
                if let Some(current_container) = current_container_opt.take() {
                    current_workspace
                        .containers
                        .elements_mut()
                        .push_back(current_container);
                }
                if let Some(focus_idx) = container_focus_idx.take() {
                    current_workspace.containers.focus(focus_idx);
                }
                if let Some((ref mut current_monitor, _)) = current_monitor_opt {
                    current_monitor
                        .workspaces
                        .elements_mut()
                        .push_back(current_workspace);
                }
            }

            // Start new workspace
            if let Ok(workspace_data) = workspace_query.get(entity) {
                let mut new_workspace = workspace_data.clone();
                new_workspace.containers = Ring::default(); // Ensure clean containers ring
                new_workspace.floating_windows = Vec::new(); // Reset floating windows, will be populated by window records

                current_workspace_opt = Some(new_workspace);
                if is_focused {
                    if let Some((ref current_monitor, _)) = current_monitor_opt {
                        workspace_focus_idx = Some(current_monitor.workspaces.elements().len());
                    }
                }
            } else {
                eprintln!("Workspace entity {:?} not found in query", entity);
                current_workspace_opt = None; // Ensure we don't use stale data
                continue;
            }
            continue; // Move to next record
        }

        // --- Container Level ---
        if win_idx == 0 {
            // Finish previous container if exists
            if let Some(current_container) = current_container_opt.take() {
                if let Some(ref mut current_workspace) = current_workspace_opt {
                    current_workspace
                        .containers
                        .elements_mut()
                        .push_back(current_container);
                }
            }

            // Start new container
            if let Ok(container_data) = container_query.get(entity) {
                let mut new_container = container_data.clone();
                new_container.windows_mut().clear();
                current_container_opt = Some(new_container);
                if is_focused {
                    if let Some(ref current_workspace) = current_workspace_opt {
                        container_focus_idx = Some(current_workspace.containers.elements().len());
                    }
                }
            } else {
                eprintln!("Container entity {:?} not found in query", entity);
                current_container_opt = None; // Ensure we don't use stale data
                continue;
            }
            continue; // Move to next record
        }

        // --- Window Level ---
        if let Ok((window_data, floating_opt)) = window_query.get(entity) {
            let new_window = *window_data; // Window is Copy

            if floating_opt.is_some() {
                // Add to workspace floating windows
                if let Some(ref mut current_workspace) = current_workspace_opt {
                    current_workspace.floating_windows.push(new_window);
                    // Note: Floating windows don't have focus within a container ring
                }
            } else {
                // Add to current container's windows
                if let Some(ref mut current_container) = current_container_opt {
                    let current_container_len = current_container.windows().len();
                    current_container.windows_mut().push_back(new_window);
                    if is_focused {
                        window_focus_idx = Some(current_container_len);
                    }
                }
            }
        } else {
            eprintln!("Window entity {:?} not found in query", entity);
        }
    }

    // 4. Push the last remaining items
    if let Some((mut current_monitor, _)) = current_monitor_opt.take() {
        if let Some(mut current_workspace) = current_workspace_opt.take() {
            if let Some(mut current_container) = current_container_opt.take() {
                if let Some(focus_idx) = window_focus_idx.take() {
                    current_container.focus_window(focus_idx);
                }
                current_workspace
                    .containers
                    .elements_mut()
                    .push_back(current_container);
            }
            if let Some(focus_idx) = container_focus_idx.take() {
                current_workspace.containers.focus(focus_idx);
            }
            current_monitor
                .workspaces
                .elements_mut()
                .push_back(current_workspace);
        }
        if let Some(focus_idx) = workspace_focus_idx.take() {
            current_monitor.workspaces.focus(focus_idx);
        }
        monitors_vec.push(current_monitor);
    }

    // 5. Finalize state object
    state.monitors.elements_mut().extend(monitors_vec);
    if let Some(focus_idx) = monitor_focus_idx {
        state.monitors.focus(focus_idx);
    }

    // 6. Send the state to komorebi
    if let Some(komorebi_state) = komorebi_state.komorebi.as_ref() {
        if !komorebi_state.has_been_modified(&state) {
            //println!("No changes to komorebi state, skipping");
            return;
        }
    } else {
        //println("No komorebi state, skipping");
        return;
    }
    let message = SocketMessage::ApplyState(state);
    match send_message(&message) {
        Ok(_) => println!("Successfully sent ApplyState message to komorebi"),
        Err(e) => eprintln!("Failed to send ApplyState message to komorebi: {}", e),
    }
}
pub fn export_state_to_komorebi(world: &mut World) {
    let mut queue = CommandQueue::default();
    if let Some(mut komotool_commmad_queue) = world.get_resource_mut::<KomotoolCommandQueue>() {
        queue.append(&mut komotool_commmad_queue.0)
    }
    queue.apply(world);

    let komorebi_state_res = world.get_resource::<KomorebiState>();
    let komotool_state_res = world.get_resource::<KomotoolState>();

    // Use if let to ensure both states are Some before comparing
    if let (Some(komorebi_state_res), Some(komotool_state_res)) =
        (komorebi_state_res, komotool_state_res)
    {
        if let (Some(komorebi_s), Some(komotool_s)) = (
            komorebi_state_res.komorebi.as_ref(),
            komotool_state_res.current.as_ref(),
        ) {
            // Both states exist, compare them
            if komotool_s.has_been_modified(komorebi_s) {
                // States are different, send the Komotool state
                println!(
                    "Komotool state differs from Komorebi state after flush, sending ApplyState to komotool"
                );
                let message = SocketMessage::ApplyState(komotool_s.clone());
                match send_message(&message) {
                    Ok(_) => println!("Successfully sent ApplyState message to komotool"),
                    Err(e) => eprintln!("Failed to send ApplyState message to komotool: {}", e),
                }
            } else {
                // States are the same, do nothing
                println!("Komotool state matches Komorebi state after flush, skipping send.");
            }
        } else {
            // At least one of the inner Option<&State> was None, do nothing.
            // This covers cases where KomorebiState exists but komorebi is None,
            // or KomotoolState exists but current is None (e.g., after flush).
            println!(
                "Either Komorebi or Komotool state content is None after flush, skipping send."
            );
        }

        if let Some(mut komotoolstate) = world.get_resource_mut::<KomotoolState>() {
            komotoolstate.current = None;
        };
    } else {
        // At least one of the resources (KomorebiState or KomotoolState) doesn't exist, do nothing.
        println!("KomorebiState or KomotoolState resource missing after flush, skipping send.");
    }
}

pub fn make_komotool_state_some(mut komotool_state: ResMut<KomotoolState>) {
    println!("Initializing komotool state for removing KomotoolStartUp schedule");
    komotool_state.current = Some(State {
        monitors: Ring::default(),
        is_paused: true,
        monitor_usr_idx_map: HashMap::new(),
        resize_delta: 0,
        float_override: false,
        new_window_behaviour: WindowContainerBehaviour::Append,
        cross_monitor_move_behaviour: MoveBehaviour::Insert,
        unmanaged_window_operation_behaviour: OperationBehaviour::Op,
        work_area_offset: None,
        focus_follows_mouse: None,
        mouse_follows_focus: false,
        has_pending_raise_op: false,
    });
}
pub fn commands_remove_komotool_startup_schedule(
    mut komotool_command_queue: ResMut<KomotoolCommandQueue>,
) {
    komotool_command_queue
        .0
        .push(run_system_cached(remove_komotool_startup_schedule).handle_error_with(warn));
    println!("queueing for removal of startup schedule");
}
