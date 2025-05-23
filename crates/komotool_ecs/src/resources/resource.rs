use bevy_ecs::component::ComponentId;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Resource;
use bevy_reflect::Reflect;
use komorebi_client::{
    FocusFollowsMouseImplementation, MoveBehaviour, OperationBehaviour, Rect, StaticConfig,
    WindowContainerBehaviour,
};
use std::collections::{HashMap, HashSet};

#[derive(Resource, Reflect)]
pub struct AppState {
    pub is_paused: bool,
    pub monitor_usr_idx_map: HashMap<usize, usize>,
    pub resize_delta: i32,
    pub new_window_behaviour: WindowContainerBehaviour,
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
            is_paused: true,
            monitor_usr_idx_map: HashMap::new(),
            resize_delta: 50,
            new_window_behaviour: WindowContainerBehaviour::Create,
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
pub struct MonitorToEntityMap(pub HashMap<String, Entity>);

#[derive(Resource, Default, Reflect)]
pub struct WorkspaceToEntityMap(pub HashMap<String, Entity>);

#[derive(Resource, Default, Reflect)]
pub struct ContainerToEntityMap(pub HashMap<String, Entity>);

#[derive(Resource, Default, Reflect)]
pub struct WindowToEntityMap(pub HashMap<String, Entity>);

#[derive(Resource, Default, Reflect)]
pub struct KomorebiState {
    pub komorebi: Option<komorebi_client::State>,
}

#[derive(Resource, Default, Reflect)]
pub struct KomotoolState {
    pub current: Option<komorebi_client::State>,
}

#[derive(Resource, Default, Reflect)]
pub struct KomotoolStaticConfig {
    pub config: Option<StaticConfig>,
}

#[derive(Resource, Default, Reflect)]
pub struct KomorebiStaticConfig {
    pub config: Option<StaticConfig>,
}

#[derive(Resource, Default, Reflect)]
pub struct ExtendedMarkerMap {
    pub makers: HashMap<usize, ComponentId>,
}

#[derive(Resource, Default, Reflect)]
pub struct KeepAliveMonitors(pub HashSet<Entity>);

#[derive(Resource, Default, Reflect)]
pub struct KeepAliveWorkspaces(pub HashSet<Entity>);

#[derive(Resource, Default, Reflect)]
pub struct KeepAliveContainers(pub HashSet<Entity>);
