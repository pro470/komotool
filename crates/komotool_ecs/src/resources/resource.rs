use crate::components::Rect;
use crate::relations::RelationRegistry;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Resource;
use bevy_reflect::Reflect;
use komorebi_client::{
    FocusFollowsMouseImplementation, /*WindowContainerBehaviour,*/ MoveBehaviour,
    OperationBehaviour,
};

#[derive(Resource)]
pub struct AppState {
    pub is_paused: bool,
    pub resize_delta: i32,
    //pub new_window_behaviour: WindowContainerBehaviour,
    pub float_override: bool,
    pub cross_monitor_move_behaviour: MoveBehaviour,
    pub unmanaged_window_operation_behaviour: OperationBehaviour,
    pub work_area_offset: Option<Rect>,
    pub focus_follows_mouse: Option<FocusFollowsMouseImplementation>,
    pub mouse_follows_focus: bool,
    pub has_pending_raise_op: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            is_paused: false,
            resize_delta: 50,
            //new_window_behaviour: ff,
            float_override: false,
            cross_monitor_move_behaviour: MoveBehaviour::Insert,
            unmanaged_window_operation_behaviour: OperationBehaviour::NoOp,
            work_area_offset: None,
            focus_follows_mouse: None,
            mouse_follows_focus: false,
            has_pending_raise_op: false,
        }
    }
}

#[derive(Resource, Default, Reflect)]
pub struct FocusedMonitor(pub Option<Entity>);

#[derive(Resource, Default, Reflect)]
pub struct LastFocusedMonitor(pub Option<Entity>);

#[derive(Resource, Default, Reflect)]
pub struct FocusedWorkspaceGlobal(pub Option<Entity>);

#[derive(Resource, Default, Reflect)]
pub struct FocusedContainerGlobal(pub Option<Entity>);

#[derive(Resource, Default, Reflect)]
pub struct FocusedWindowGlobal(pub Option<Entity>);

#[derive(Resource, Default, Reflect)]
pub struct MonitorReg(pub RelationRegistry);

#[derive(Resource, Default, Reflect)]
pub struct WorkspaceReg(pub RelationRegistry);

#[derive(Resource, Default, Reflect)]
pub struct ContainerReg(pub RelationRegistry);

#[derive(Resource, Default, Reflect)]
pub struct WindowReg(pub RelationRegistry);

#[derive(Resource, Default)]
pub struct KomorebiState {
    pub current: Option<komorebi_client::State>,
    pub last: Option<komorebi_client::State>,
}
