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

use bevy_app::{App, First, Last, Plugin};
use bevy_ecs::prelude::resource_changed;
use bevy_ecs::schedule::IntoSystemConfigs;
use components::*;
use komorebi_client::{Container, Monitor, Window, Workspace};
use register_komorebi_types::register_komorebi_types;
use relations::*;
use resources::*;
use systems::*;

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
                First,
                (
                    // Process notifications first
                    update_komorebi_state_from_notifications
                        .after(komotool_pipe::handle_pipe_notifications),
                    // Then run all imports in parallel
                    (
                        (
                            import_komorebi_window_state,
                            import_komorebi_container_state,
                            import_komorebi_workspace_state,
                            import_komorebi_monitor_state,
                        )
                            .before(build_relation_registry),
                        import_komorebi_appstate_state,
                        build_relation_registry,
                    )
                        .after(update_komorebi_state_from_notifications)
                        .run_if(resource_changed::<KomorebiState>),
                ),
            )
            .add_systems(
                Last,
                export_state_to_komorebi.before(komotool_framepace::framerate_limiter),
            );
        register_container_types(app);
        register_monitor_types(app);
        register_window_types(app);
        register_workspace_types(app);
        register_komorebi_types(app);
    }
}
