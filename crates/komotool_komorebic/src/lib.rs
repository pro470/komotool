use bevy_app::{App, Plugin};
use bevy_mod_scripting::core::bindings::function::namespace::NamespaceBuilder;
use bevy_reflect::Reflect;
use komorebi_client::*;

#[derive(Reflect)]
struct Komorebic;

#[derive(Default)]
pub struct KomoToolKomorebicPlugin;

impl Plugin for KomoToolKomorebicPlugin {
    fn build(&self, app: &mut App) {
        NamespaceBuilder::<Komorebic>::new(app.world_mut())
            .register("focus_window", |operation_direction: String| {
                let operation_direction: OperationDirection = match operation_direction.to_lowercase().as_str() {
                    "left" => OperationDirection::Left,
                    "right" => OperationDirection::Right,
                    "up" => OperationDirection::Up,
                    "down" => OperationDirection::Down,
                    _ => {
                        log::error!("Invalid OperationDirection: {operation_direction}");
                        return false;
                    }
                };
                let message = SocketMessage::FocusWindow(operation_direction);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send focus_window message: {}", e);
                        false
                    }
                }
            })
            .register("move_window", |operation_direction: String| {
                let operation_direction: OperationDirection = match operation_direction.to_lowercase().as_str() {
                    "left" => OperationDirection::Left,
                    "right" => OperationDirection::Right,
                    "up" => OperationDirection::Up,
                    "down" => OperationDirection::Down,
                    _ => {
                        log::error!("Invalid OperationDirection: {operation_direction}");
                        return false;
                    }
                };
                let message = SocketMessage::MoveWindow(operation_direction);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send move_window message: {}", e);
                        false
                    }
                }
            })
            .register("cycle_focus_window", |cycle_direction: String| {
                let cycle_direction: CycleDirection = match cycle_direction.to_lowercase().as_str() {
                    "previous" => CycleDirection::Previous,
                    "next" => CycleDirection::Next,
                    _ => {
                        log::error!("Invalid CycleDirection: {cycle_direction}");
                        return false;
                    }
                };
                let message = SocketMessage::CycleFocusWindow(cycle_direction);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send cycle_focus_window message: {}", e);
                        false
                    }
                }
            })
            .register("cycle_move_window", |cycle_direction: String| {
                let cycle_direction: CycleDirection = match cycle_direction.to_lowercase().as_str() {
                    "previous" => CycleDirection::Previous,
                    "next" => CycleDirection::Next,
                    _ => {
                        log::error!("Invalid CycleDirection: {cycle_direction}");
                        return false;
                    }
                };
                let message = SocketMessage::CycleMoveWindow(cycle_direction);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send cycle_move_window message: {}", e);
                        false
                    }
                }
            })
            .register("stack_window", |operation_direction: String| {
                let operation_direction: OperationDirection = match operation_direction.to_lowercase().as_str() {
                    "left" => OperationDirection::Left,
                    "right" => OperationDirection::Right,
                    "up" => OperationDirection::Up,
                    "down" => OperationDirection::Down,
                    _ => {
                        log::error!("Invalid OperationDirection: {operation_direction}");
                        return false;
                    }
                };
                let message = SocketMessage::StackWindow(operation_direction);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send stack_window message: {}", e);
                        false
                    }
                }
            })
            .register("unstack_window", || {
                let message = SocketMessage::UnstackWindow;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send unstack_window message: {}", e);
                        false
                    }
                }
            })
            .register("cycle_stack", |cycle_direction: String| {
                let cycle_direction: CycleDirection = match cycle_direction.to_lowercase().as_str() {
                    "previous" => CycleDirection::Previous,
                    "next" => CycleDirection::Next,
                    _ => {
                        log::error!("Invalid CycleDirection: {cycle_direction}");
                        return false;
                    }
                };
                let message = SocketMessage::CycleStack(cycle_direction);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send cycle_stack message: {}", e);
                        false
                    }
                }
            })
            .register("cycle_stack_index", |cycle_direction: String| {
                let cycle_direction: CycleDirection = match cycle_direction.to_lowercase().as_str() {
                    "previous" => CycleDirection::Previous,
                    "next" => CycleDirection::Next,
                    _ => {
                        log::error!("Invalid CycleDirection: {cycle_direction}");
                        return false;
                    }
                };
                let message = SocketMessage::CycleStackIndex(cycle_direction);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send cycle_stack_index message: {}", e);
                        false
                    }
                }
            })
            .register("focus_stack_window", |param: usize| {
                let message = SocketMessage::FocusStackWindow(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send focus_stack_window message: {}", e);
                        false
                    }
                }
            })
            .register("stack_all", || {
                let message = SocketMessage::StackAll;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send stack_all message: {}", e);
                        false
                    }
                }
            })
            .register("unstack_all", || {
                let message = SocketMessage::UnstackAll;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send unstack_all message: {}", e);
                        false
                    }
                }
            })
            .register("resize_window_edge", |operation_direction: String, sizing: String| {
                let operation_direction: OperationDirection = match operation_direction.to_lowercase().as_str() {
                    "left" => OperationDirection::Left,
                    "right" => OperationDirection::Right,
                    "up" => OperationDirection::Up,
                    "down" => OperationDirection::Down,
                    _ => {
                        log::error!("Invalid OperationDirection: {operation_direction}");
                        return false;
                    }
                };

                let sizing: Sizing = match sizing.to_lowercase().as_str() {
                    "increase" => Sizing::Increase,
                    "decrease" => Sizing::Decrease,
                    _ => {
                        log::error!("Invalid Sizing: {sizing}");
                        return false;
                    }
                };
                let message = SocketMessage::ResizeWindowEdge(operation_direction, sizing);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send resize_window_edge message: {}", e);
                        false
                    }
                }
            })
            .register("resize_window_axis", |axis: String, sizing: String| {
                let axis: Axis = match axis.to_lowercase().as_str() {
                    "horizontal" => Axis::Horizontal,
                    "vertical" => Axis::Vertical,
                    "horizontalandvertical" => Axis::HorizontalAndVertical,
                    _ => {
                        log::error!("Invalid Axis: {axis}");
                        return false;
                    }
                };

                let sizing: Sizing = match sizing.to_lowercase().as_str() {
                    "increase" => Sizing::Increase,
                    "decrease" => Sizing::Decrease,
                    _ => {
                        log::error!("Invalid Sizing: {sizing}");
                        return false;
                    }
                };
                let message = SocketMessage::ResizeWindowAxis(axis, sizing);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send resize_window_axis message: {}", e);
                        false
                    }
                }
            })
            .register("move_container_to_monitor_number", |param: usize| {
                let message = SocketMessage::MoveContainerToMonitorNumber(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send move_container_to_monitor_number message: {}", e);
                        false
                    }
                }
            })
            .register("cycle_move_container_to_monitor", |cycle_direction: String| {
                let cycle_direction: CycleDirection = match cycle_direction.to_lowercase().as_str() {
                    "previous" => CycleDirection::Previous,
                    "next" => CycleDirection::Next,
                    _ => {
                        log::error!("Invalid CycleDirection: {cycle_direction}");
                        return false;
                    }
                };
                let message = SocketMessage::CycleMoveContainerToMonitor(cycle_direction);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send cycle_move_container_to_monitor message: {}", e);
                        false
                    }
                }
            })
            .register("move_container_to_workspace_number", |param: usize| {
                let message = SocketMessage::MoveContainerToWorkspaceNumber(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send move_container_to_workspace_number message: {}", e);
                        false
                    }
                }
            })
            .register("move_container_to_named_workspace", |param: String| {
                let message = SocketMessage::MoveContainerToNamedWorkspace(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send move_container_to_named_workspace message: {}", e);
                        false
                    }
                }
            })
            .register("cycle_move_container_to_workspace", |cycle_direction: String| {
                let cycle_direction: CycleDirection = match cycle_direction.to_lowercase().as_str() {
                    "previous" => CycleDirection::Previous,
                    "next" => CycleDirection::Next,
                    _ => {
                        log::error!("Invalid CycleDirection: {cycle_direction}");
                        return false;
                    }
                };
                let message = SocketMessage::CycleMoveContainerToWorkspace(cycle_direction);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send cycle_move_container_to_workspace message: {}", e);
                        false
                    }
                }
            })
            .register("send_container_to_monitor_number", |param: usize| {
                let message = SocketMessage::SendContainerToMonitorNumber(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send send_container_to_monitor_number message: {}", e);
                        false
                    }
                }
            })
            .register("cycle_send_container_to_monitor", |cycle_direction: String| {
                let cycle_direction: CycleDirection = match cycle_direction.to_lowercase().as_str() {
                    "previous" => CycleDirection::Previous,
                    "next" => CycleDirection::Next,
                    _ => {
                        log::error!("Invalid CycleDirection: {cycle_direction}");
                        return false;
                    }
                };
                let message = SocketMessage::CycleSendContainerToMonitor(cycle_direction);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send cycle_send_container_to_monitor message: {}", e);
                        false
                    }
                }
            })
            .register("send_container_to_workspace_number", |param: usize| {
                let message = SocketMessage::SendContainerToWorkspaceNumber(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send send_container_to_workspace_number message: {}", e);
                        false
                    }
                }
            })
            .register("cycle_send_container_to_workspace", |cycle_direction: String| {
                let cycle_direction: CycleDirection = match cycle_direction.to_lowercase().as_str() {
                    "previous" => CycleDirection::Previous,
                    "next" => CycleDirection::Next,
                    _ => {
                        log::error!("Invalid CycleDirection: {cycle_direction}");
                        return false;
                    }
                };
                let message = SocketMessage::CycleSendContainerToWorkspace(cycle_direction);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send cycle_send_container_to_workspace message: {}", e);
                        false
                    }
                }
            })
            .register("send_container_to_monitor_workspace_number", |param_0: usize, param_1: usize| {
                let message = SocketMessage::SendContainerToMonitorWorkspaceNumber(param_0, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send send_container_to_monitor_workspace_number message: {}", e);
                        false
                    }
                }
            })
            .register("move_container_to_monitor_workspace_number", |param_0: usize, param_1: usize| {
                let message = SocketMessage::MoveContainerToMonitorWorkspaceNumber(param_0, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send move_container_to_monitor_workspace_number message: {}", e);
                        false
                    }
                }
            })
            .register("send_container_to_named_workspace", |param: String| {
                let message = SocketMessage::SendContainerToNamedWorkspace(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send send_container_to_named_workspace message: {}", e);
                        false
                    }
                }
            })
            .register("cycle_move_workspace_to_monitor", |cycle_direction: String| {
                let cycle_direction: CycleDirection = match cycle_direction.to_lowercase().as_str() {
                    "previous" => CycleDirection::Previous,
                    "next" => CycleDirection::Next,
                    _ => {
                        log::error!("Invalid CycleDirection: {cycle_direction}");
                        return false;
                    }
                };
                let message = SocketMessage::CycleMoveWorkspaceToMonitor(cycle_direction);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send cycle_move_workspace_to_monitor message: {}", e);
                        false
                    }
                }
            })
            .register("move_workspace_to_monitor_number", |param: usize| {
                let message = SocketMessage::MoveWorkspaceToMonitorNumber(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send move_workspace_to_monitor_number message: {}", e);
                        false
                    }
                }
            })
            .register("swap_workspaces_to_monitor_number", |param: usize| {
                let message = SocketMessage::SwapWorkspacesToMonitorNumber(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send swap_workspaces_to_monitor_number message: {}", e);
                        false
                    }
                }
            })
            .register("force_focus", || {
                let message = SocketMessage::ForceFocus;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send force_focus message: {}", e);
                        false
                    }
                }
            })
            .register("close", || {
                let message = SocketMessage::Close;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send close message: {}", e);
                        false
                    }
                }
            })
            .register("minimize", || {
                let message = SocketMessage::Minimize;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send minimize message: {}", e);
                        false
                    }
                }
            })
            .register("promote", || {
                let message = SocketMessage::Promote;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send promote message: {}", e);
                        false
                    }
                }
            })
            .register("promote_focus", || {
                let message = SocketMessage::PromoteFocus;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send promote_focus message: {}", e);
                        false
                    }
                }
            })
            .register("promote_window", |operation_direction: String| {
                let operation_direction: OperationDirection = match operation_direction.to_lowercase().as_str() {
                    "left" => OperationDirection::Left,
                    "right" => OperationDirection::Right,
                    "up" => OperationDirection::Up,
                    "down" => OperationDirection::Down,
                    _ => {
                        log::error!("Invalid OperationDirection: {operation_direction}");
                        return false;
                    }
                };
                let message = SocketMessage::PromoteWindow(operation_direction);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send promote_window message: {}", e);
                        false
                    }
                }
            })
            .register("eager_focus", |param: String| {
                let message = SocketMessage::EagerFocus(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send eager_focus message: {}", e);
                        false
                    }
                }
            })
            .register("toggle_float", || {
                let message = SocketMessage::ToggleFloat;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send toggle_float message: {}", e);
                        false
                    }
                }
            })
            .register("toggle_monocle", || {
                let message = SocketMessage::ToggleMonocle;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send toggle_monocle message: {}", e);
                        false
                    }
                }
            })
            .register("toggle_maximize", || {
                let message = SocketMessage::ToggleMaximize;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send toggle_maximize message: {}", e);
                        false
                    }
                }
            })
            .register("toggle_window_container_behaviour", || {
                let message = SocketMessage::ToggleWindowContainerBehaviour;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send toggle_window_container_behaviour message: {}", e);
                        false
                    }
                }
            })
            .register("toggle_float_override", || {
                let message = SocketMessage::ToggleFloatOverride;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send toggle_float_override message: {}", e);
                        false
                    }
                }
            })
            .register("window_hiding_behaviour", |hiding_behaviour: String| {
                let hiding_behaviour: HidingBehaviour = match hiding_behaviour.to_lowercase().as_str() {
                    "hide" => HidingBehaviour::Hide,
                    "minimize" => HidingBehaviour::Minimize,
                    "cloak" => HidingBehaviour::Cloak,
                    _ => {
                        log::error!("Invalid HidingBehaviour: {hiding_behaviour}");
                        return false;
                    }
                };
                let message = SocketMessage::WindowHidingBehaviour(hiding_behaviour);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send window_hiding_behaviour message: {}", e);
                        false
                    }
                }
            })
            .register("toggle_cross_monitor_move_behaviour", || {
                let message = SocketMessage::ToggleCrossMonitorMoveBehaviour;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send toggle_cross_monitor_move_behaviour message: {}", e);
                        false
                    }
                }
            })
            .register("cross_monitor_move_behaviour", |move_behaviour: String| {
                let move_behaviour: MoveBehaviour = match move_behaviour.to_lowercase().as_str() {
                    "swap" => MoveBehaviour::Swap,
                    "insert" => MoveBehaviour::Insert,
                    "noop" => MoveBehaviour::NoOp,
                    _ => {
                        log::error!("Invalid MoveBehaviour: {move_behaviour}");
                        return false;
                    }
                };
                let message = SocketMessage::CrossMonitorMoveBehaviour(move_behaviour);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send cross_monitor_move_behaviour message: {}", e);
                        false
                    }
                }
            })
            .register("unmanaged_window_operation_behaviour", |operation_behaviour: String| {
                let operation_behaviour: OperationBehaviour = match operation_behaviour.to_lowercase().as_str() {
                    "op" => OperationBehaviour::Op,
                    "noop" => OperationBehaviour::NoOp,
                    _ => {
                        log::error!("Invalid OperationBehaviour: {operation_behaviour}");
                        return false;
                    }
                };
                let message = SocketMessage::UnmanagedWindowOperationBehaviour(operation_behaviour);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send unmanaged_window_operation_behaviour message: {}", e);
                        false
                    }
                }
            })
            .register("manage_focused_window", || {
                let message = SocketMessage::ManageFocusedWindow;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send manage_focused_window message: {}", e);
                        false
                    }
                }
            })
            .register("unmanage_focused_window", || {
                let message = SocketMessage::UnmanageFocusedWindow;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send unmanage_focused_window message: {}", e);
                        false
                    }
                }
            })
            .register("adjust_container_padding", |sizing: String, param_1: i32| {
                let sizing: Sizing = match sizing.to_lowercase().as_str() {
                    "increase" => Sizing::Increase,
                    "decrease" => Sizing::Decrease,
                    _ => {
                        log::error!("Invalid Sizing: {sizing}");
                        return false;
                    }
                };
                let message = SocketMessage::AdjustContainerPadding(sizing, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send adjust_container_padding message: {}", e);
                        false
                    }
                }
            })
            .register("adjust_workspace_padding", |sizing: String, param_1: i32| {
                let sizing: Sizing = match sizing.to_lowercase().as_str() {
                    "increase" => Sizing::Increase,
                    "decrease" => Sizing::Decrease,
                    _ => {
                        log::error!("Invalid Sizing: {sizing}");
                        return false;
                    }
                };
                let message = SocketMessage::AdjustWorkspacePadding(sizing, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send adjust_workspace_padding message: {}", e);
                        false
                    }
                }
            })
            .register("change_layout", |default_layout: String| {
                let default_layout: DefaultLayout = match default_layout.to_lowercase().as_str() {
                    "bsp" => DefaultLayout::BSP,
                    "columns" => DefaultLayout::Columns,
                    "rows" => DefaultLayout::Rows,
                    "verticalstack" => DefaultLayout::VerticalStack,
                    "horizontalstack" => DefaultLayout::HorizontalStack,
                    "ultrawideverticalstack" => DefaultLayout::UltrawideVerticalStack,
                    "grid" => DefaultLayout::Grid,
                    "rightmainverticalstack" => DefaultLayout::RightMainVerticalStack,
                    _ => {
                        log::error!("Invalid DefaultLayout: {default_layout}");
                        return false;
                    }
                };
                let message = SocketMessage::ChangeLayout(default_layout);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send change_layout message: {}", e);
                        false
                    }
                }
            })
            .register("cycle_layout", |cycle_direction: String| {
                let cycle_direction: CycleDirection = match cycle_direction.to_lowercase().as_str() {
                    "previous" => CycleDirection::Previous,
                    "next" => CycleDirection::Next,
                    _ => {
                        log::error!("Invalid CycleDirection: {cycle_direction}");
                        return false;
                    }
                };
                let message = SocketMessage::CycleLayout(cycle_direction);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send cycle_layout message: {}", e);
                        false
                    }
                }
            })
            .register("change_layout_custom", |param: String| {
                let message = SocketMessage::ChangeLayoutCustom(std::path::PathBuf::from(param));
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send change_layout_custom message: {}", e);
                        false
                    }
                }
            })
            .register("flip_layout", |axis: String| {
                let axis: Axis = match axis.to_lowercase().as_str() {
                    "horizontal" => Axis::Horizontal,
                    "vertical" => Axis::Vertical,
                    "horizontalandvertical" => Axis::HorizontalAndVertical,
                    _ => {
                        log::error!("Invalid Axis: {axis}");
                        return false;
                    }
                };
                let message = SocketMessage::FlipLayout(axis);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send flip_layout message: {}", e);
                        false
                    }
                }
            })
            .register("toggle_workspace_window_container_behaviour", || {
                let message = SocketMessage::ToggleWorkspaceWindowContainerBehaviour;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send toggle_workspace_window_container_behaviour message: {}", e);
                        false
                    }
                }
            })
            .register("toggle_workspace_float_override", || {
                let message = SocketMessage::ToggleWorkspaceFloatOverride;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send toggle_workspace_float_override message: {}", e);
                        false
                    }
                }
            })
            .register("monitor_index_preference", |param_0: usize, param_1: i32, param_2: i32, param_3: i32, param_4: i32| {
                let message = SocketMessage::MonitorIndexPreference(param_0, param_1, param_2, param_3, param_4);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send monitor_index_preference message: {}", e);
                        false
                    }
                }
            })
            .register("display_index_preference", |param_0: usize, param_1: String| {
                let message = SocketMessage::DisplayIndexPreference(param_0, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send display_index_preference message: {}", e);
                        false
                    }
                }
            })
            .register("ensure_workspaces", |param_0: usize, param_1: usize| {
                let message = SocketMessage::EnsureWorkspaces(param_0, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send ensure_workspaces message: {}", e);
                        false
                    }
                }
            })
            .register("ensure_named_workspaces", |param_0: usize, param_1: Vec<String>| {
                let message = SocketMessage::EnsureNamedWorkspaces(param_0, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send ensure_named_workspaces message: {}", e);
                        false
                    }
                }
            })
            .register("new_workspace", || {
                let message = SocketMessage::NewWorkspace;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send new_workspace message: {}", e);
                        false
                    }
                }
            })
            .register("toggle_tiling", || {
                let message = SocketMessage::ToggleTiling;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send toggle_tiling message: {}", e);
                        false
                    }
                }
            })
            .register("stop", || {
                let message = SocketMessage::Stop;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send stop message: {}", e);
                        false
                    }
                }
            })
            .register("stop_ignore_restore", || {
                let message = SocketMessage::StopIgnoreRestore;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send stop_ignore_restore message: {}", e);
                        false
                    }
                }
            })
            .register("toggle_pause", || {
                let message = SocketMessage::TogglePause;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send toggle_pause message: {}", e);
                        false
                    }
                }
            })
            .register("retile", || {
                let message = SocketMessage::Retile;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send retile message: {}", e);
                        false
                    }
                }
            })
            .register("retile_with_resize_dimensions", || {
                let message = SocketMessage::RetileWithResizeDimensions;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send retile_with_resize_dimensions message: {}", e);
                        false
                    }
                }
            })
            .register("quick_save", || {
                let message = SocketMessage::QuickSave;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send quick_save message: {}", e);
                        false
                    }
                }
            })
            .register("quick_load", || {
                let message = SocketMessage::QuickLoad;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send quick_load message: {}", e);
                        false
                    }
                }
            })
            .register("save", |param: String| {
                let message = SocketMessage::Save(std::path::PathBuf::from(param));
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send save message: {}", e);
                        false
                    }
                }
            })
            .register("load", |param: String| {
                let message = SocketMessage::Load(std::path::PathBuf::from(param));
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send load message: {}", e);
                        false
                    }
                }
            })
            .register("cycle_focus_monitor", |cycle_direction: String| {
                let cycle_direction: CycleDirection = match cycle_direction.to_lowercase().as_str() {
                    "previous" => CycleDirection::Previous,
                    "next" => CycleDirection::Next,
                    _ => {
                        log::error!("Invalid CycleDirection: {cycle_direction}");
                        return false;
                    }
                };
                let message = SocketMessage::CycleFocusMonitor(cycle_direction);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send cycle_focus_monitor message: {}", e);
                        false
                    }
                }
            })
            .register("cycle_focus_workspace", |cycle_direction: String| {
                let cycle_direction: CycleDirection = match cycle_direction.to_lowercase().as_str() {
                    "previous" => CycleDirection::Previous,
                    "next" => CycleDirection::Next,
                    _ => {
                        log::error!("Invalid CycleDirection: {cycle_direction}");
                        return false;
                    }
                };
                let message = SocketMessage::CycleFocusWorkspace(cycle_direction);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send cycle_focus_workspace message: {}", e);
                        false
                    }
                }
            })
            .register("focus_monitor_number", |param: usize| {
                let message = SocketMessage::FocusMonitorNumber(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send focus_monitor_number message: {}", e);
                        false
                    }
                }
            })
            .register("focus_last_workspace", || {
                let message = SocketMessage::FocusLastWorkspace;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send focus_last_workspace message: {}", e);
                        false
                    }
                }
            })
            .register("close_workspace", || {
                let message = SocketMessage::CloseWorkspace;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send close_workspace message: {}", e);
                        false
                    }
                }
            })
            .register("focus_workspace_number", |param: usize| {
                let message = SocketMessage::FocusWorkspaceNumber(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send focus_workspace_number message: {}", e);
                        false
                    }
                }
            })
            .register("focus_workspace_numbers", |param: usize| {
                let message = SocketMessage::FocusWorkspaceNumbers(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send focus_workspace_numbers message: {}", e);
                        false
                    }
                }
            })
            .register("focus_monitor_workspace_number", |param_0: usize, param_1: usize| {
                let message = SocketMessage::FocusMonitorWorkspaceNumber(param_0, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send focus_monitor_workspace_number message: {}", e);
                        false
                    }
                }
            })
            .register("focus_named_workspace", |param: String| {
                let message = SocketMessage::FocusNamedWorkspace(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send focus_named_workspace message: {}", e);
                        false
                    }
                }
            })
            .register("container_padding", |param_0: usize, param_1: usize, param_2: i32| {
                let message = SocketMessage::ContainerPadding(param_0, param_1, param_2);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send container_padding message: {}", e);
                        false
                    }
                }
            })
            .register("named_workspace_container_padding", |param_0: String, param_1: i32| {
                let message = SocketMessage::NamedWorkspaceContainerPadding(param_0, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send named_workspace_container_padding message: {}", e);
                        false
                    }
                }
            })
            .register("focused_workspace_container_padding", |param: i32| {
                let message = SocketMessage::FocusedWorkspaceContainerPadding(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send focused_workspace_container_padding message: {}", e);
                        false
                    }
                }
            })
            .register("workspace_padding", |param_0: usize, param_1: usize, param_2: i32| {
                let message = SocketMessage::WorkspacePadding(param_0, param_1, param_2);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send workspace_padding message: {}", e);
                        false
                    }
                }
            })
            .register("named_workspace_padding", |param_0: String, param_1: i32| {
                let message = SocketMessage::NamedWorkspacePadding(param_0, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send named_workspace_padding message: {}", e);
                        false
                    }
                }
            })
            .register("focused_workspace_padding", |param: i32| {
                let message = SocketMessage::FocusedWorkspacePadding(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send focused_workspace_padding message: {}", e);
                        false
                    }
                }
            })
            .register("workspace_tiling", |param_0: usize, param_1: usize, param_2: bool| {
                let message = SocketMessage::WorkspaceTiling(param_0, param_1, param_2);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send workspace_tiling message: {}", e);
                        false
                    }
                }
            })
            .register("named_workspace_tiling", |param_0: String, param_1: bool| {
                let message = SocketMessage::NamedWorkspaceTiling(param_0, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send named_workspace_tiling message: {}", e);
                        false
                    }
                }
            })
            .register("workspace_name", |param_0: usize, param_1: usize, param_2: String| {
                let message = SocketMessage::WorkspaceName(param_0, param_1, param_2);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send workspace_name message: {}", e);
                        false
                    }
                }
            })
            .register("workspace_layout", |param_0: usize, param_1: usize, default_layout: String| {
                let default_layout: DefaultLayout = match default_layout.to_lowercase().as_str() {
                    "bsp" => DefaultLayout::BSP,
                    "columns" => DefaultLayout::Columns,
                    "rows" => DefaultLayout::Rows,
                    "verticalstack" => DefaultLayout::VerticalStack,
                    "horizontalstack" => DefaultLayout::HorizontalStack,
                    "ultrawideverticalstack" => DefaultLayout::UltrawideVerticalStack,
                    "grid" => DefaultLayout::Grid,
                    "rightmainverticalstack" => DefaultLayout::RightMainVerticalStack,
                    _ => {
                        log::error!("Invalid DefaultLayout: {default_layout}");
                        return false;
                    }
                };
                let message = SocketMessage::WorkspaceLayout(param_0, param_1, default_layout);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send workspace_layout message: {}", e);
                        false
                    }
                }
            })
            .register("named_workspace_layout", |param_0: String, default_layout: String| {
                let default_layout: DefaultLayout = match default_layout.to_lowercase().as_str() {
                    "bsp" => DefaultLayout::BSP,
                    "columns" => DefaultLayout::Columns,
                    "rows" => DefaultLayout::Rows,
                    "verticalstack" => DefaultLayout::VerticalStack,
                    "horizontalstack" => DefaultLayout::HorizontalStack,
                    "ultrawideverticalstack" => DefaultLayout::UltrawideVerticalStack,
                    "grid" => DefaultLayout::Grid,
                    "rightmainverticalstack" => DefaultLayout::RightMainVerticalStack,
                    _ => {
                        log::error!("Invalid DefaultLayout: {default_layout}");
                        return false;
                    }
                };
                let message = SocketMessage::NamedWorkspaceLayout(param_0, default_layout);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send named_workspace_layout message: {}", e);
                        false
                    }
                }
            })
            .register("workspace_layout_custom", |param_0: usize, param_1: usize, param_2: String| {
                let message = SocketMessage::WorkspaceLayoutCustom(param_0, param_1, std::path::PathBuf::from(param_2));
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send workspace_layout_custom message: {}", e);
                        false
                    }
                }
            })
            .register("named_workspace_layout_custom", |param_0: String, param_1: String| {
                let message = SocketMessage::NamedWorkspaceLayoutCustom(param_0, std::path::PathBuf::from(param_1));
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send named_workspace_layout_custom message: {}", e);
                        false
                    }
                }
            })
            .register("workspace_layout_rule", |param_0: usize, param_1: usize, param_2: usize, default_layout: String| {
                let default_layout: DefaultLayout = match default_layout.to_lowercase().as_str() {
                    "bsp" => DefaultLayout::BSP,
                    "columns" => DefaultLayout::Columns,
                    "rows" => DefaultLayout::Rows,
                    "verticalstack" => DefaultLayout::VerticalStack,
                    "horizontalstack" => DefaultLayout::HorizontalStack,
                    "ultrawideverticalstack" => DefaultLayout::UltrawideVerticalStack,
                    "grid" => DefaultLayout::Grid,
                    "rightmainverticalstack" => DefaultLayout::RightMainVerticalStack,
                    _ => {
                        log::error!("Invalid DefaultLayout: {default_layout}");
                        return false;
                    }
                };
                let message = SocketMessage::WorkspaceLayoutRule(param_0, param_1, param_2, default_layout);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send workspace_layout_rule message: {}", e);
                        false
                    }
                }
            })
            .register("named_workspace_layout_rule", |param_0: String, param_1: usize, default_layout: String| {
                let default_layout: DefaultLayout = match default_layout.to_lowercase().as_str() {
                    "bsp" => DefaultLayout::BSP,
                    "columns" => DefaultLayout::Columns,
                    "rows" => DefaultLayout::Rows,
                    "verticalstack" => DefaultLayout::VerticalStack,
                    "horizontalstack" => DefaultLayout::HorizontalStack,
                    "ultrawideverticalstack" => DefaultLayout::UltrawideVerticalStack,
                    "grid" => DefaultLayout::Grid,
                    "rightmainverticalstack" => DefaultLayout::RightMainVerticalStack,
                    _ => {
                        log::error!("Invalid DefaultLayout: {default_layout}");
                        return false;
                    }
                };
                let message = SocketMessage::NamedWorkspaceLayoutRule(param_0, param_1, default_layout);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send named_workspace_layout_rule message: {}", e);
                        false
                    }
                }
            })
            .register("workspace_layout_custom_rule", |param_0: usize, param_1: usize, param_2: usize, param_3: String| {
                let message = SocketMessage::WorkspaceLayoutCustomRule(param_0, param_1, param_2, std::path::PathBuf::from(param_3));
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send workspace_layout_custom_rule message: {}", e);
                        false
                    }
                }
            })
            .register("named_workspace_layout_custom_rule", |param_0: String, param_1: usize, param_2: String| {
                let message = SocketMessage::NamedWorkspaceLayoutCustomRule(param_0, param_1, std::path::PathBuf::from(param_2));
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send named_workspace_layout_custom_rule message: {}", e);
                        false
                    }
                }
            })
            .register("clear_workspace_layout_rules", |param_0: usize, param_1: usize| {
                let message = SocketMessage::ClearWorkspaceLayoutRules(param_0, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send clear_workspace_layout_rules message: {}", e);
                        false
                    }
                }
            })
            .register("clear_named_workspace_layout_rules", |param: String| {
                let message = SocketMessage::ClearNamedWorkspaceLayoutRules(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send clear_named_workspace_layout_rules message: {}", e);
                        false
                    }
                }
            })
            .register("reload_configuration", || {
                let message = SocketMessage::ReloadConfiguration;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send reload_configuration message: {}", e);
                        false
                    }
                }
            })
            .register("replace_configuration", |param: String| {
                let message = SocketMessage::ReplaceConfiguration(std::path::PathBuf::from(param));
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send replace_configuration message: {}", e);
                        false
                    }
                }
            })
            .register("reload_static_configuration", |param: String| {
                let message = SocketMessage::ReloadStaticConfiguration(std::path::PathBuf::from(param));
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send reload_static_configuration message: {}", e);
                        false
                    }
                }
            })
            .register("watch_configuration", |param: bool| {
                let message = SocketMessage::WatchConfiguration(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send watch_configuration message: {}", e);
                        false
                    }
                }
            })
            .register("complete_configuration", || {
                let message = SocketMessage::CompleteConfiguration;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send complete_configuration message: {}", e);
                        false
                    }
                }
            })
            .register("alt_focus_hack", |param: bool| {
                let message = SocketMessage::AltFocusHack(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send alt_focus_hack message: {}", e);
                        false
                    }
                }
            })
            .register("animation", |param_0: bool, animation_prefix: String| {
                let param_1: Option<AnimationPrefix> = match animation_prefix.to_lowercase().as_str() {
                    "movement" => Some(AnimationPrefix::Movement),
                    "transparency" => Some(AnimationPrefix::Transparency),
                    "" => None,
                    _ => {
                        log::error!("Invalid CycleDirection: {animation_prefix}");
                        return false;
                    }
                };
                let message = SocketMessage::Animation(param_0, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send animation message: {}", e);
                        false
                    }
                }
            })
            .register("animation_duration", |param_0: u64, animation_prefix: String| {
                let param_1: Option<AnimationPrefix> = match animation_prefix.to_lowercase().as_str() {
                    "movement" => Some(AnimationPrefix::Movement),
                    "transparency" => Some(AnimationPrefix::Transparency),
                    "" => None,
                    _ => {
                        log::error!("Invalid CycleDirection: {animation_prefix}");
                        return false;
                    }
                };
                let message = SocketMessage::AnimationDuration(param_0, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send animation_duration message: {}", e);
                        false
                    }
                }
            })
            .register("animation_fps", |param: u64| {
                let message = SocketMessage::AnimationFps(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send animation_fps message: {}", e);
                        false
                    }
                }
            })
            .register("animation_style", |animation_style: String, animation_prefix: String| {
                let animation_style: AnimationStyle = match animation_style.to_lowercase().as_str() {
                    "linear" => AnimationStyle::Linear,
                    "easeinsine" => AnimationStyle::EaseInSine,
                    "easeoutsine" => AnimationStyle::EaseOutSine,
                    "easeinoutsine" => AnimationStyle::EaseInOutSine,
                    "easeinquad" => AnimationStyle::EaseInQuad,
                    "easeoutquad" => AnimationStyle::EaseOutQuad,
                    "easeinoutquad" => AnimationStyle::EaseInOutQuad,
                    "easeincubic" => AnimationStyle::EaseInCubic,
                    "easeinoutcubic" => AnimationStyle::EaseInOutCubic,
                    "easeinquart" => AnimationStyle::EaseInQuart,
                    "easeoutquart" => AnimationStyle::EaseOutQuart,
                    "easeinoutquart" => AnimationStyle::EaseInOutQuart,
                    "easeinquint" => AnimationStyle::EaseInQuint,
                    "easeoutquint" => AnimationStyle::EaseOutQuint,
                    "easeinoutquint" => AnimationStyle::EaseInOutQuint,
                    "easeinexpo" => AnimationStyle::EaseInExpo,
                    "easeoutexpo" => AnimationStyle::EaseOutExpo,
                    "easeinoutexpo" => AnimationStyle::EaseInOutExpo,
                    "easeincirc" => AnimationStyle::EaseInCirc,
                    "easeoutcirc" => AnimationStyle::EaseOutCirc,
                    "easeinoutcirc" => AnimationStyle::EaseInOutCirc,
                    "easeinback" => AnimationStyle::EaseInBack,
                    "easeoutback" => AnimationStyle::EaseOutBack,
                    "easeinoutback" => AnimationStyle::EaseInOutBack,
                    "easeinelastic" => AnimationStyle::EaseInElastic,
                    "easeoutelastic" => AnimationStyle::EaseOutElastic,
                    "easeinoutelastic" => AnimationStyle::EaseInOutElastic,
                    "easeinbounce" => AnimationStyle::EaseInBounce,
                    "easeoutbounce" => AnimationStyle::EaseOutBounce,
                    "easeinoutbounce" => AnimationStyle::EaseInOutBounce,
                    _ => {
                        log::error!("Invalid AnimationStyle: {animation_style}");
                        return false;
                    }
                };
                let param_1: Option<AnimationPrefix> = match animation_prefix.to_lowercase().as_str() {
                    "movement" => Some(AnimationPrefix::Movement),
                    "transparency" => Some(AnimationPrefix::Transparency),
                    "" => None,
                    _ => {
                        log::error!("Invalid CycleDirection: {animation_prefix}");
                        return false;
                    }
                };
                let message = SocketMessage::AnimationStyle(animation_style, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send animation_style message: {}", e);
                        false
                    }
                }
            })
            .register("border", |param: bool| {
                let message = SocketMessage::Border(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send border message: {}", e);
                        false
                    }
                }
            })
            .register("border_colour", |window_kind: String, param_1: u32, param_2: u32, param_3: u32| {
                let window_kind: WindowKind = match window_kind.to_lowercase().as_str() {
                    "single" => WindowKind::Single,
                    "stack" => WindowKind::Stack,
                    "monocle" => WindowKind::Monocle,
                    "unfocused" => WindowKind::Unfocused,
                    "floating" => WindowKind::Floating,
                    _ => {
                        log::error!("Invalid WindowKind: {window_kind}");
                        return false;
                    }
                };
                let message = SocketMessage::BorderColour(window_kind, param_1, param_2, param_3);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send border_colour message: {}", e);
                        false
                    }
                }
            })
            .register("border_style", |border_style: String| {
                let border_style: BorderStyle = match border_style.to_lowercase().as_str() {
                    "system" => BorderStyle::System,
                    "rounded" => BorderStyle::Rounded,
                    "square" => BorderStyle::Square,
                    _ => {
                        log::error!("Invalid BorderStyle: {border_style}");
                        return false;
                    }
                };
                let message = SocketMessage::BorderStyle(border_style);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send border_style message: {}", e);
                        false
                    }
                }
            })
            .register("border_width", |param: i32| {
                let message = SocketMessage::BorderWidth(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send border_width message: {}", e);
                        false
                    }
                }
            })
            .register("border_offset", |param: i32| {
                let message = SocketMessage::BorderOffset(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send border_offset message: {}", e);
                        false
                    }
                }
            })
            .register("border_implementation", |border_implementation: String| {
                let border_implementation: BorderImplementation = match border_implementation.to_lowercase().as_str() {
                    "komorebi" => BorderImplementation::Komorebi,
                    "windows" => BorderImplementation::Windows,
                    _ => {
                        log::error!("Invalid BorderImplementation: {border_implementation}");
                        return false;
                    }
                };
                let message = SocketMessage::BorderImplementation(border_implementation);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send border_implementation message: {}", e);
                        false
                    }
                }
            })
            .register("transparency", |param: bool| {
                let message = SocketMessage::Transparency(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send transparency message: {}", e);
                        false
                    }
                }
            })
            .register("toggle_transparency", || {
                let message = SocketMessage::ToggleTransparency;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send toggle_transparency message: {}", e);
                        false
                    }
                }
            })
            .register("transparency_alpha", |param: u8| {
                let message = SocketMessage::TransparencyAlpha(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send transparency_alpha message: {}", e);
                        false
                    }
                }
            })
            .register("invisible_borders", |bottom: i32, left: i32, right: i32, top: i32| {
                let rect: Rect = Rect {
                    bottom,
                    left,
                    right,
                    top,
                };
                let message = SocketMessage::InvisibleBorders(rect);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send invisible_borders message: {}", e);
                        false
                    }
                }
            })
            .register("stackbar_mode", |stackbar_mode: String| {
                let stackbar_mode: StackbarMode = match stackbar_mode.to_lowercase().as_str() {
                    "always" => StackbarMode::Always,
                    "never" => StackbarMode::Never,
                    "onstack" => StackbarMode::OnStack,
                    _ => {
                        log::error!("Invalid StackbarMode: {stackbar_mode}");
                        return false;
                    }
                };
                let message = SocketMessage::StackbarMode(stackbar_mode);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send stackbar_mode message: {}", e);
                        false
                    }
                }
            })
            .register("stackbar_label", |stackbar_label: String| {
                let stackbar_label: StackbarLabel = match stackbar_label.to_lowercase().as_str() {
                    "process" => StackbarLabel::Process,
                    "title" => StackbarLabel::Title,
                    _ => {
                        log::error!("Invalid StackbarLabel: {stackbar_label}");
                        return false;
                    }
                };
                let message = SocketMessage::StackbarLabel(stackbar_label);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send stackbar_label message: {}", e);
                        false
                    }
                }
            })
            .register("stackbar_focused_text_colour", |param_0: u32, param_1: u32, param_2: u32| {
                let message = SocketMessage::StackbarFocusedTextColour(param_0, param_1, param_2);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send stackbar_focused_text_colour message: {}", e);
                        false
                    }
                }
            })
            .register("stackbar_unfocused_text_colour", |param_0: u32, param_1: u32, param_2: u32| {
                let message = SocketMessage::StackbarUnfocusedTextColour(param_0, param_1, param_2);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send stackbar_unfocused_text_colour message: {}", e);
                        false
                    }
                }
            })
            .register("stackbar_background_colour", |param_0: u32, param_1: u32, param_2: u32| {
                let message = SocketMessage::StackbarBackgroundColour(param_0, param_1, param_2);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send stackbar_background_colour message: {}", e);
                        false
                    }
                }
            })
            .register("stackbar_height", |param: i32| {
                let message = SocketMessage::StackbarHeight(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send stackbar_height message: {}", e);
                        false
                    }
                }
            })
            .register("stackbar_tab_width", |param: i32| {
                let message = SocketMessage::StackbarTabWidth(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send stackbar_tab_width message: {}", e);
                        false
                    }
                }
            })
            .register("stackbar_font_size", |param: i32| {
                let message = SocketMessage::StackbarFontSize(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send stackbar_font_size message: {}", e);
                        false
                    }
                }
            })
            .register("stackbar_font_family", |param: String| {
                let font: Option<String> = match param.as_str() {
                    "" => None,
                    _ => Some(param),
                };
                let message = SocketMessage::StackbarFontFamily(font);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send stackbar_font_family message: {}", e);
                        false
                    }
                }
            })
            .register("work_area_offset", |bottom: i32, left: i32, right: i32, top: i32| {
                let rect: Rect = Rect {
                    bottom,
                    left,
                    right,
                    top,
                };
                let message = SocketMessage::WorkAreaOffset(rect);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send work_area_offset message: {}", e);
                        false
                    }
                }
            })
            .register("monitor_work_area_offset", |param_0: usize, bottom: i32, left: i32, right: i32, top: i32| {
                let rect: Rect = Rect {
                    bottom,
                    left,
                    right,
                    top,
                };
                let message = SocketMessage::MonitorWorkAreaOffset(param_0, rect);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send monitor_work_area_offset message: {}", e);
                        false
                    }
                }
            })
            .register("resize_delta", |param: i32| {
                let message = SocketMessage::ResizeDelta(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send resize_delta message: {}", e);
                        false
                    }
                }
            })
            .register("initial_workspace_rule", |application_identifier: String, param_1: String, param_2: usize, param_3: usize| {
                let application_identifier: ApplicationIdentifier = match application_identifier.to_lowercase().as_str() {
                    "exe" => ApplicationIdentifier::Exe,
                    "class" => ApplicationIdentifier::Class,
                    "title" => ApplicationIdentifier::Title,
                    "path" => ApplicationIdentifier::Path,
                    _ => {
                        log::error!("Invalid ApplicationIdentifier: {application_identifier}");
                        return false;
                    }
                };
                let message = SocketMessage::InitialWorkspaceRule(application_identifier, param_1, param_2, param_3);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send initial_workspace_rule message: {}", e);
                        false
                    }
                }
            })
            .register("initial_named_workspace_rule", |application_identifier: String, param_1: String, param_2: String| {
                let application_identifier: ApplicationIdentifier = match application_identifier.to_lowercase().as_str() {
                    "exe" => ApplicationIdentifier::Exe,
                    "class" => ApplicationIdentifier::Class,
                    "title" => ApplicationIdentifier::Title,
                    "path" => ApplicationIdentifier::Path,
                    _ => {
                        log::error!("Invalid ApplicationIdentifier: {application_identifier}");
                        return false;
                    }
                };
                let message = SocketMessage::InitialNamedWorkspaceRule(application_identifier, param_1, param_2);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send initial_named_workspace_rule message: {}", e);
                        false
                    }
                }
            })
            .register("workspace_rule", |application_identifier: String, param_1: String, param_2: usize, param_3: usize| {
                let application_identifier: ApplicationIdentifier = match application_identifier.to_lowercase().as_str() {
                    "exe" => ApplicationIdentifier::Exe,
                    "class" => ApplicationIdentifier::Class,
                    "title" => ApplicationIdentifier::Title,
                    "path" => ApplicationIdentifier::Path,
                    _ => {
                        log::error!("Invalid ApplicationIdentifier: {application_identifier}");
                        return false;
                    }
                };
                let message = SocketMessage::WorkspaceRule(application_identifier, param_1, param_2, param_3);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send workspace_rule message: {}", e);
                        false
                    }
                }
            })
            .register("named_workspace_rule", |application_identifier: String, param_1: String, param_2: String| {
                let application_identifier: ApplicationIdentifier = match application_identifier.to_lowercase().as_str() {
                    "exe" => ApplicationIdentifier::Exe,
                    "class" => ApplicationIdentifier::Class,
                    "title" => ApplicationIdentifier::Title,
                    "path" => ApplicationIdentifier::Path,
                    _ => {
                        log::error!("Invalid ApplicationIdentifier: {application_identifier}");
                        return false;
                    }
                };
                let message = SocketMessage::NamedWorkspaceRule(application_identifier, param_1, param_2);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send named_workspace_rule message: {}", e);
                        false
                    }
                }
            })
            .register("clear_workspace_rules", |param_0: usize, param_1: usize| {
                let message = SocketMessage::ClearWorkspaceRules(param_0, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send clear_workspace_rules message: {}", e);
                        false
                    }
                }
            })
            .register("clear_named_workspace_rules", |param: String| {
                let message = SocketMessage::ClearNamedWorkspaceRules(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send clear_named_workspace_rules message: {}", e);
                        false
                    }
                }
            })
            .register("clear_all_workspace_rules", || {
                let message = SocketMessage::ClearAllWorkspaceRules;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send clear_all_workspace_rules message: {}", e);
                        false
                    }
                }
            })
            .register("enforce_workspace_rules", || {
                let message = SocketMessage::EnforceWorkspaceRules;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send enforce_workspace_rules message: {}", e);
                        false
                    }
                }
            })
            .register("ignore_rule", |application_identifier: String, param_1: String| {
                let application_identifier: ApplicationIdentifier = match application_identifier.to_lowercase().as_str() {
                    "exe" => ApplicationIdentifier::Exe,
                    "class" => ApplicationIdentifier::Class,
                    "title" => ApplicationIdentifier::Title,
                    "path" => ApplicationIdentifier::Path,
                    _ => {
                        log::error!("Invalid ApplicationIdentifier: {application_identifier}");
                        return false;
                    }
                };
                let message = SocketMessage::IgnoreRule(application_identifier, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send ignore_rule message: {}", e);
                        false
                    }
                }
            })
            .register("manage_rule", |application_identifier: String, param_1: String| {
                let application_identifier: ApplicationIdentifier = match application_identifier.to_lowercase().as_str() {
                    "exe" => ApplicationIdentifier::Exe,
                    "class" => ApplicationIdentifier::Class,
                    "title" => ApplicationIdentifier::Title,
                    "path" => ApplicationIdentifier::Path,
                    _ => {
                        log::error!("Invalid ApplicationIdentifier: {application_identifier}");
                        return false;
                    }
                };
                let message = SocketMessage::ManageRule(application_identifier, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send manage_rule message: {}", e);
                        false
                    }
                }
            })
            .register("identify_object_name_change_application", |application_identifier: String, param_1: String| {
                let application_identifier: ApplicationIdentifier = match application_identifier.to_lowercase().as_str() {
                    "exe" => ApplicationIdentifier::Exe,
                    "class" => ApplicationIdentifier::Class,
                    "title" => ApplicationIdentifier::Title,
                    "path" => ApplicationIdentifier::Path,
                    _ => {
                        log::error!("Invalid ApplicationIdentifier: {application_identifier}");
                        return false;
                    }
                };
                let message = SocketMessage::IdentifyObjectNameChangeApplication(application_identifier, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send identify_object_name_change_application message: {}", e);
                        false
                    }
                }
            })
            .register("identify_tray_application", |application_identifier: String, param_1: String| {
                let application_identifier: ApplicationIdentifier = match application_identifier.to_lowercase().as_str() {
                    "exe" => ApplicationIdentifier::Exe,
                    "class" => ApplicationIdentifier::Class,
                    "title" => ApplicationIdentifier::Title,
                    "path" => ApplicationIdentifier::Path,
                    _ => {
                        log::error!("Invalid ApplicationIdentifier: {application_identifier}");
                        return false;
                    }
                };
                let message = SocketMessage::IdentifyTrayApplication(application_identifier, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send identify_tray_application message: {}", e);
                        false
                    }
                }
            })
            .register("identify_layered_application", |application_identifier: String, param_1: String| {
                let application_identifier: ApplicationIdentifier = match application_identifier.to_lowercase().as_str() {
                    "exe" => ApplicationIdentifier::Exe,
                    "class" => ApplicationIdentifier::Class,
                    "title" => ApplicationIdentifier::Title,
                    "path" => ApplicationIdentifier::Path,
                    _ => {
                        log::error!("Invalid ApplicationIdentifier: {application_identifier}");
                        return false;
                    }
                };
                let message = SocketMessage::IdentifyLayeredApplication(application_identifier, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send identify_layered_application message: {}", e);
                        false
                    }
                }
            })
            .register("identify_border_overflow_application", |application_identifier: String, param_1: String| {
                let application_identifier: ApplicationIdentifier = match application_identifier.to_lowercase().as_str() {
                    "exe" => ApplicationIdentifier::Exe,
                    "class" => ApplicationIdentifier::Class,
                    "title" => ApplicationIdentifier::Title,
                    "path" => ApplicationIdentifier::Path,
                    _ => {
                        log::error!("Invalid ApplicationIdentifier: {application_identifier}");
                        return false;
                    }
                };
                let message = SocketMessage::IdentifyBorderOverflowApplication(application_identifier, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send identify_border_overflow_application message: {}", e);
                        false
                    }
                }
            })
            .register("state", || {
                let message = SocketMessage::State;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send state message: {}", e);
                        false
                    }
                }
            })
            .register("global_state", || {
                let message = SocketMessage::GlobalState;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send global_state message: {}", e);
                        false
                    }
                }
            })
            .register("visible_windows", || {
                let message = SocketMessage::VisibleWindows;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send visible_windows message: {}", e);
                        false
                    }
                }
            })
            .register("monitor_information", || {
                let message = SocketMessage::MonitorInformation;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send monitor_information message: {}", e);
                        false
                    }
                }
            })
            .register("query", |state_query: String| {
                let state_query: StateQuery = match state_query.to_lowercase().as_str() {
                    "focusedmonitorindex" => StateQuery::FocusedMonitorIndex,
                    "focusedworkspaceindex" => StateQuery::FocusedWorkspaceIndex,
                    "focusedcontainerindex" => StateQuery::FocusedContainerIndex,
                    "focusedwindowindex" => StateQuery::FocusedWindowIndex,
                    _ => {
                        log::error!("Invalid StateQuery: {state_query}");
                        return false;
                    }
                };
                let message = SocketMessage::Query(state_query);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send query message: {}", e);
                        false
                    }
                }
            })
            .register("focus_follows_mouse", |focus_follows_mouse_implementation: String, param_1: bool| {
                let focus_follows_mouse_implementation: FocusFollowsMouseImplementation = match focus_follows_mouse_implementation.to_lowercase().as_str() {
                    "komorebi" => FocusFollowsMouseImplementation::Komorebi,
                    "windows" => FocusFollowsMouseImplementation::Windows,
                    _ => {
                        log::error!("Invalid FocusFollowsMouseImplementation: {focus_follows_mouse_implementation}");
                        return false;
                    }
                };
                let message = SocketMessage::FocusFollowsMouse(focus_follows_mouse_implementation, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send focus_follows_mouse message: {}", e);
                        false
                    }
                }
            })
            .register("toggle_focus_follows_mouse", |focus_follows_mouse_implementation: String| {
                let focus_follows_mouse_implementation: FocusFollowsMouseImplementation = match focus_follows_mouse_implementation.to_lowercase().as_str() {
                    "komorebi" => FocusFollowsMouseImplementation::Komorebi,
                    "windows" => FocusFollowsMouseImplementation::Windows,
                    _ => {
                        log::error!("Invalid FocusFollowsMouseImplementation: {focus_follows_mouse_implementation}");
                        return false;
                    }
                };
                let message = SocketMessage::ToggleFocusFollowsMouse(focus_follows_mouse_implementation);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send toggle_focus_follows_mouse message: {}", e);
                        false
                    }
                }
            })
            .register("mouse_follows_focus", |param: bool| {
                let message = SocketMessage::MouseFollowsFocus(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send mouse_follows_focus message: {}", e);
                        false
                    }
                }
            })
            .register("toggle_mouse_follows_focus", || {
                let message = SocketMessage::ToggleMouseFollowsFocus;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send toggle_mouse_follows_focus message: {}", e);
                        false
                    }
                }
            })
            .register("remove_title_bar", |application_identifier: String, param_1: String| {
                let application_identifier: ApplicationIdentifier = match application_identifier.to_lowercase().as_str() {
                    "exe" => ApplicationIdentifier::Exe,
                    "class" => ApplicationIdentifier::Class,
                    "title" => ApplicationIdentifier::Title,
                    "path" => ApplicationIdentifier::Path,
                    _ => {
                        log::error!("Invalid ApplicationIdentifier: {application_identifier}");
                        return false;
                    }
                };
                let message = SocketMessage::RemoveTitleBar(application_identifier, param_1);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send remove_title_bar message: {}", e);
                        false
                    }
                }
            })
            .register("toggle_title_bars", || {
                let message = SocketMessage::ToggleTitleBars;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send toggle_title_bars message: {}", e);
                        false
                    }
                }
            })
            .register("add_subscriber_socket", |param: String| {
                let message = SocketMessage::AddSubscriberSocket(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send add_subscriber_socket message: {}", e);
                        false
                    }
                }
            })
            .register("add_subscriber_socket_with_options", |param_0: String, param_1: bool| {
                let subscribe_options: SubscribeOptions = SubscribeOptions  {
                    filter_state_changes: param_1,
                };
                let message = SocketMessage::AddSubscriberSocketWithOptions(param_0, subscribe_options);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send add_subscriber_socket_with_options message: {}", e);
                        false
                    }
                }
            })
            .register("remove_subscriber_socket", |param: String| {
                let message = SocketMessage::RemoveSubscriberSocket(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send remove_subscriber_socket message: {}", e);
                        false
                    }
                }
            })
            .register("add_subscriber_pipe", |param: String| {
                let message = SocketMessage::AddSubscriberPipe(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send add_subscriber_pipe message: {}", e);
                        false
                    }
                }
            })
            .register("remove_subscriber_pipe", |param: String| {
                let message = SocketMessage::RemoveSubscriberPipe(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send remove_subscriber_pipe message: {}", e);
                        false
                    }
                }
            })
            .register("application_specific_configuration_schema", || {
                let message = SocketMessage::ApplicationSpecificConfigurationSchema;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send application_specific_configuration_schema message: {}", e);
                        false
                    }
                }
            })
            .register("notification_schema", || {
                let message = SocketMessage::NotificationSchema;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send notification_schema message: {}", e);
                        false
                    }
                }
            })
            .register("socket_schema", || {
                let message = SocketMessage::SocketSchema;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send socket_schema message: {}", e);
                        false
                    }
                }
            })
            .register("static_config_schema", || {
                let message = SocketMessage::StaticConfigSchema;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send static_config_schema message: {}", e);
                        false
                    }
                }
            })
            .register("generate_static_config", || {
                let message = SocketMessage::GenerateStaticConfig;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send generate_static_config message: {}", e);
                        false
                    }
                }
            })
            .register("debug_window", |param: isize| {
                let message = SocketMessage::DebugWindow(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send debug_window message: {}", e);
                        false
                    }
                }
            });
    }
}
