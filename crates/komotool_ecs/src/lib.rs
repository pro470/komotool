mod components;
pub use components::*;

use bevy::prelude::*;

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
            .register_type::<Monitor>()
            .register_type::<Window>()
            .register_type::<Container>()
            .register_type::<Workspace>()
            .register_type::<MonocleContainer>()
            .register_type::<FloatingWindow>()
            .register_type::<FocusedWindow>()
            .register_type::<FocusedContainer>()
            .register_type::<LastFocusedWorkspace>()
            .register_type::<FocusedWorkspace>()
            .register_type::<LastFocusedContainer>()
            .register_type::<MaximizedWindow>();
            
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
