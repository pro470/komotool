use bevy::prelude::*;
use komorebi_client::*;

#[derive(Debug, Clone, Reflect)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32
}

#[derive(Component, Reflect)]
pub enum LayoutType {
    RightMainVerticalStack,
    VerticalStack,
    HorizontalStack,
    UltrawideVerticalStack,
    Rows,
    BSP
}

#[derive(Component, Reflect)]
pub struct Monitor {
    pub id: u32,
    pub name: String,
    pub device_id: String,
    pub serial_number: String,
    pub physical_size: Rect,
    pub work_area: Rect,
    pub work_area_offset: Rect
}

#[derive(Component, Reflect)]
pub struct Workspace {
    pub name: String,
    pub layout: LayoutType,
    pub padding: i32,
    pub container_padding: i32,
    pub tile: bool
}

#[derive(Component, Reflect)]
pub struct Container {
    pub id: String,
    pub focused: i32
}

#[derive(Component, Reflect)]
pub struct Window {
    pub hwnd: i32,
    pub title: String,
    pub exe: String,
    pub class: String,
    pub rect: Rect
}

#[derive(Component, Reflect)]
pub struct MonocleContainer(pub Option<Entity>); // Marker for monocle containers

#[derive(Component, Reflect)]
pub struct FloatingWindow;

#[derive(Component, Reflect)]
pub struct FocusedWindow(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct FocusedContainer(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct LastFocusedContainer(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct FocusedWorkspace(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct LastFocusedWorkspace(pub Option<Entity>);

#[derive(Component, Reflect)]
pub struct MaximizedWindow(pub Option<Entity>);

#[derive(Component, Reflect)]
struct Focused(pub i32);

#[derive(Resource)]
pub struct AppState {
    pub is_paused: bool,
    pub resize_delta: i32,
    pub focus_follows_mouse: bool,
    pub mouse_follows_focus: bool
}

#[derive(Resource, Default)]
pub struct FocusedMonitor(pub Option<Entity>);

#[derive(Resource, Default)]
struct LastFocusedMonitor(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct FocusedWorkspaceGlobal(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct FocusedContainerGlobal(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct FocusedWindowGlobal(pub Option<Entity>);

pub struct KomoToolEcsPlugin;

impl Plugin for KomoToolEcsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AppState>()
            .init_resource::<FocusedMonitor>()
            .init_resource::<FocusedWorkspaceGlobal>()
            .init_resource::<FocusedContainerGlobal>()
            .init_resource::<FocusedWindowGlobal>()
            .register_type::<>()
            .register_type::<>()
            ;
            
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            is_paused: false,
            resize_delta: 50,
            focus_follows_mouse: false,
            mouse_follows_focus: false
        }
    }
}
