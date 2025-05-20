pub mod components;
pub mod register_komorebi_types;
pub mod relations;
pub mod resources;
pub mod systems;

pub mod prelude {
    pub use super::*;
    pub use components::*;
    pub use register_komorebi_types::*;
    pub use relations::*;
    pub use resources::*;
    pub use systems::*;
}

use crate::systems::{commands_remove_komotool_startup_schedule, export_state_to_komorebi};
use bevy_app::{App, Plugin};
use bevy_ecs::schedule::IntoScheduleConfigs;
use components::*;
use komorebi_client::{Container, Monitor, Window, Workspace};
use komotool_utils::startup_schedule::KomoToolStartUpFinished;
use register_komorebi_types::register_komorebi_types;
use relations::*;
use resources::*;

#[derive(Default)]
pub struct KomoToolEcsPlugin;

impl Plugin for KomoToolEcsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppState>()
            .init_resource::<RelationRegistry>()
            .init_resource::<ExtendedMarkerMap>()
            .init_resource::<KomorebiState>()
            .init_resource::<KomotoolState>()
            .init_resource::<MonitorToEntityMap>()
            .init_resource::<WorkspaceToEntityMap>()
            .init_resource::<ContainerToEntityMap>()
            .init_resource::<WindowToEntityMap>()
            .init_resource::<KomotoolStaticConfig>()
            .init_resource::<KomorebiStaticConfig>()
            .init_resource::<KeepAliveMonitors>()
            .init_resource::<KeepAliveWorkspaces>()
            .init_resource::<KeepAliveContainers>()
            .init_resource::<KomotoolCommandQueue>()
            .register_type::<Monitor>()
            .register_type::<Window>()
            .register_type::<Container>()
            .register_type::<Workspace>()
            .register_type::<MonocleContainer>()
            .register_type::<FloatingWindow>()
            .register_type::<Focused>()
            .register_type::<FocusedGlobal>()
            .register_type::<MaximizedWindow>()
            .register_type::<LastFocused>()
            .add_systems(
                KomoToolStartUpFinished,
                export_state_to_komorebi
                    .before_ignore_deferred(commands_remove_komotool_startup_schedule),
            )
            .add_systems(
                KomoToolStartUpFinished,
                commands_remove_komotool_startup_schedule,
            );
        register_container_types(app);
        register_monitor_types(app);
        register_window_types(app);
        register_workspace_types(app);
        register_komorebi_types(app);
    }
}
