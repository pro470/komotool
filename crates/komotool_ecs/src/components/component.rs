use bevy::prelude::*;

#[derive(Debug, Clone, Reflect)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Component, Reflect)]
pub enum LayoutType {
    RightMainVerticalStack,
    VerticalStack,
    HorizontalStack,
    UltrawideVerticalStack,
    Rows,
    BSP,
}

#[derive(Component, Reflect)]
pub struct Monitor {
    pub id: u32,
    pub name: String,
    pub device_id: String,
    pub serial_number: String,
    pub physical_size: Rect,
    pub work_area: Rect,
    pub work_area_offset: Rect,
}

#[derive(Component, Reflect)]
pub struct Workspace {
    pub name: String,
    pub layout: LayoutType,
    pub padding: i32,
    pub container_padding: i32,
    pub tile: bool,
}

#[derive(Component, Reflect)]
pub struct Container {
    pub id: String,
    pub focused: i32,
}

#[derive(Component, Reflect)]
pub struct Window {
    pub hwnd: i32,
    pub title: String,
    pub exe: String,
    pub class: String,
    pub rect: Rect,
}

#[derive(Component, Reflect)]
pub struct MonocleContainer(pub Option<Entity>);

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
pub struct Focused(pub i32);
