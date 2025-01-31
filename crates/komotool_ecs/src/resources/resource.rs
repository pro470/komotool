use crate::relations::RelationRegistry;
use bevy::prelude::*;

#[derive(Resource)]
pub struct AppState {
    pub is_paused: bool,
    pub resize_delta: i32,
    pub focus_follows_mouse: bool,
    pub mouse_follows_focus: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            is_paused: false,
            resize_delta: 50,
            focus_follows_mouse: false,
            mouse_follows_focus: false,
        }
    }
}

#[derive(Resource, Default)]
pub struct FocusedMonitor(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct LastFocusedMonitor(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct FocusedWorkspaceGlobal(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct FocusedContainerGlobal(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct FocusedWindowGlobal(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct MonitorReg(pub RelationRegistry);

#[derive(Resource, Default)]
pub struct WorkspaceReg(pub RelationRegistry);

#[derive(Resource, Default)]
pub struct ContainerReg(pub RelationRegistry);

#[derive(Resource, Default)]
pub struct WindowReg(pub RelationRegistry);

