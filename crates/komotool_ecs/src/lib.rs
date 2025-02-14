mod components;
mod relations;
mod resources;
mod systems;

use bevy::{app::First, prelude::*};
pub use components::*;
pub use relations::*;
pub use resources::*;
pub use systems::*;

pub struct KomoToolEcsPlugin;

impl Plugin for KomoToolEcsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppState>()
            .init_resource::<KomorebiState>()
            .init_resource::<FocusedMonitor>()
            .init_resource::<FocusedWorkspaceGlobal>()
            .init_resource::<FocusedContainerGlobal>()
            .init_resource::<FocusedWindowGlobal>()
            .init_resource::<MonitorReg>()
            .init_resource::<WorkspaceReg>()
            .init_resource::<ContainerReg>()
            .init_resource::<WindowReg>()
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
            .register_type::<MaximizedWindow>()
            .add_systems(
                First,
                (
                    fetch_komorebi_state,
                    import_komorebi_monitor_state.after(fetch_komorebi_state),
                    import_komorebi_appstate_state.after(fetch_komorebi_state),
                ),
            );
    }
}
