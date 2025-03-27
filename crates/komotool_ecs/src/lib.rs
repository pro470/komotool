mod components;
mod relations;
mod resources;
mod systems;

use bevy_app::{App, First, Plugin};
use bevy_ecs::prelude::resource_changed;
use bevy_ecs::schedule::IntoSystemConfigs;
pub use components::*;
use komorebi_client::{Container, Monitor, Window, Workspace};
pub use relations::*;
pub use resources::*;
pub use systems::*;

pub struct KomoToolEcsPlugin;

impl Plugin for KomoToolEcsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppState>()
            .init_resource::<RelationRegistry>()
            .init_resource::<ExtendedMakerMap>()
            .init_resource::<KomorebiState>()
            .init_resource::<FocusedMonitor>()
            .init_resource::<FocusedWorkspaceGlobal>()
            .init_resource::<FocusedContainerGlobal>()
            .init_resource::<FocusedWindowGlobal>()
            .init_resource::<MonitorToEntityMap>()
            .init_resource::<WorkspaceToEntityMap>()
            .init_resource::<ContainerToEntityMap>()
            .init_resource::<WindowToEntityMap>()
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
                    // Process notifications first
                    update_komorebi_state_from_notifications
                        .after(komotool_pipe::handle_pipe_notifications),
                    // Then run all imports in parallel
                    (
                        import_komorebi_window_state,
                        import_komorebi_container_state,
                        import_komorebi_workspace_state,
                        import_komorebi_monitor_state,
                        import_komorebi_appstate_state,
                    )
                        .after(update_komorebi_state_from_notifications)
                        .before(build_relation_registry)
                        .run_if(resource_changed::<KomorebiState>),
                    build_relation_registry,
                ),
            );
        register_container_types(app);
        register_monitor_types(app);
        register_window_types(app);
        register_workspace_types(app);
    }
}
