use bevy::prelude::{Commands, Query, Res, ResMut, Entity};
use crate::components::*;
use crate::resources::*;

pub fn import_komorebi_workspace_state(
    mut commands: Commands,
    mut existing_workspaces: Query<(Entity, &mut Workspace)>,
    komorebi_state: Res<KomorebiState>,
) {
    // Clear existing workspaces
    for (entity,_) in existing_workspaces.iter_mut() {
        commands.entity(entity).despawn();
    }

    let Some(state) = &komorebi_state.current else {
        return;
    };

    // Spawn new workspace entities
    for komo_mon in state.monitors.elements() {
        let workspaces = komo_mon.workspaces();
        for (idx, komo_ws) in workspaces.iter().enumerate() {
            let mut entity = commands.spawn(Workspace {
                name: komo_ws.name().clone(),
                layout: komo_ws.layout().into(),
                monocle_container_restore_idx: komo_ws.monocle_container_restore_idx(),
                maximized_window_restore_idx: komo_ws.maximized_window_restore_idx(),
                floating_windows: Vec::new(),
                layout_rules: komo_ws.layout_rules()
                    .iter()
                    .map(|(size, rule)| (*size, rule.into()))
                    .collect(),
                layout_flip: komo_ws.layout_flip().map(|a| match a {
                    komorebi_client::Axis::Horizontal => Axis::Horizontal,
                    komorebi_client::Axis::Vertical => Axis::Vertical,
                    komorebi_client::Axis::HorizontalAndVertical => Axis::HorizontalAndVertical,
                }),
                workspace_padding: komo_ws.workspace_padding(),
                container_padding: komo_ws.container_padding(),
                latest_layout: komo_ws.latest_layout()
                    .iter()
                    .map(|r| Rect {
                        left: r.left,
                        top: r.top,
                        right: r.right,
                        bottom: r.bottom,
                    })
                    .collect(),
                resize_dimensions: komo_ws.resize_dimensions()
                    .iter()
                    .map(|r| r.as_ref().map(|ri| Rect {
                        left: ri.left,
                        top: ri.top,
                        right: ri.right,
                        bottom: ri.bottom,
                    }))
                    .collect(),
                tile: *komo_ws.tile(),
                apply_window_based_work_area_offset: komo_ws.apply_window_based_work_area_offset(),
                float_override: *komo_ws.float_override(),
            });

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
) {
    // Clear existing monitors
    for (entity, _) in existing_monitors.iter_mut() {
        commands.entity(entity).despawn();
    }

    let Some(state) = &komorebi_state.current else {
        return;
    };

    // Spawn new monitor entities with getter methods
    for (idx, komo_mon) in state.monitors.elements().iter().enumerate() {
        let mut entity = commands.spawn(Monitor {
            id: komo_mon.id(),
            name: komo_mon.name().to_string(),
            device: komo_mon.device().to_string(),
            device_id: komo_mon.device_id().to_string(),
            physical_size: komo_mon.size().into(),
            work_area_size: komo_mon.work_area_size().into(),
            work_area_offset: komo_mon.work_area_offset().map(|r| (&r).into()),
            window_based_work_area_offset: komo_mon
                .window_based_work_area_offset()
                .map(|r| (&r).into()),
            window_based_work_area_offset_limit: komo_mon.window_based_work_area_offset_limit(),
        });

        if idx == state.monitors.focused_idx() {
            entity.insert(Focused(1));
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
                let mut entity = commands.spawn(Container {
                    id: komo_cont.id().to_string(),
                });

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
            cross_monitor_move_behaviour: state.cross_monitor_move_behaviour.clone(),
            unmanaged_window_operation_behaviour: state
                .unmanaged_window_operation_behaviour
                .clone(),
            work_area_offset: state.work_area_offset.map(|r| (&r).into()),
            focus_follows_mouse: state.focus_follows_mouse.clone(),
            mouse_follows_focus: state.mouse_follows_focus,
            has_pending_raise_op: state.has_pending_raise_op,
        };
    }
}
