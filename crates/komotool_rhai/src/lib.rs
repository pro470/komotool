use bevy_app::{App, Plugin, PostUpdate, PreStartup, PreUpdate, Update};
use bevy_asset::{AssetServer, Assets, Handle, LoadedFolder, RecursiveDependencyLoadState};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::system::{Commands, Res, ResMut, Resource};
use bevy_mod_scripting::core::{handler::event_handler, script::ScriptComponent};
use bevy_mod_scripting::rhai::RhaiScriptingPlugin;
use bevy_state::app::AppExtStates;
use bevy_state::condition::in_state;
use bevy_state::state::{NextState, OnEnter, OnExit, States};
use komotool_utils::prelude::*;
use komotool_utils::send_event_systems::{
    send_post_startup_events, send_pre_startup_events, send_startup_events,
};

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum RhaiScriptLoadState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Resource)]
struct RhaiScriptLoadTracker {
    handle: Handle<LoadedFolder>,
}

pub struct KomoToolRhaiPlugin;

impl Plugin for KomoToolRhaiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RhaiScriptingPlugin::default())
            .add_systems(
                OnEnter(RhaiScriptLoadState::Loading),
                increment_loading_counter,
            )
            .add_systems(
                OnExit(RhaiScriptLoadState::Loading),
                decrement_loading_counter,
            )
            .init_state::<RhaiScriptLoadState>()
            .add_systems(PreStartup, load_rhai_scripts)
            // Phased initialization systems
            .add_systems(
                PreUpdate,
                rhai_check_pre_startup
                    .run_if(in_state(RhaiScriptLoadState::Loading))
                    .before(event_handler::<OnPreStartUp, RhaiScriptingPlugin>),
            )
            .add_systems(
                PreUpdate,
                event_handler::<OnPreStartUp, RhaiScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::PreStartupDone))
                    .after(send_pre_startup_events),
            )
            .add_systems(
                Update,
                event_handler::<OnStartUp, RhaiScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::StartupDone))
                    .after(send_startup_events),
            )
            .add_systems(
                PostUpdate,
                event_handler::<OnPostStartUp, RhaiScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::PostStartupDone))
                    .after(send_post_startup_events),
            )
            // Add systems for the main loop phases
            .add_systems(
                PreUpdate,
                event_handler::<OnPreUpdate, RhaiScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::AllDone)),
            )
            .add_systems(
                Update,
                event_handler::<OnUpdate, RhaiScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::AllDone)),
            )
            .add_systems(
                PostUpdate,
                event_handler::<OnPostUpdate, RhaiScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::AllDone)),
            );
    }
}

fn load_rhai_scripts(asset_server: Res<AssetServer>, mut commands: Commands) {
    let path = std::path::Path::new("rhai");
    let source = bevy_asset::io::AssetSourceId::from("komotool_config");
    let asset_path = bevy_asset::AssetPath::from_path(path).with_source(source);
    let handle = asset_server.load_folder(asset_path);
    commands.insert_resource(RhaiScriptLoadTracker { handle });
}

fn rhai_check_pre_startup(
    asset_server: Res<AssetServer>,
    tracker: Res<RhaiScriptLoadTracker>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<RhaiScriptLoadState>>,
) {
    if let Some(RecursiveDependencyLoadState::Loaded) =
        asset_server.get_recursive_dependency_load_state(&tracker.handle)
    {
        if let Some(folder) = loaded_folders.get(&tracker.handle) {
            for handle in &folder.handles {
                if let Some(path) = handle.path() {
                    commands.spawn(ScriptComponent::new(vec![path
                        .path()
                        .to_string_lossy()
                        .to_string()]));
                }
            }
        }

        next_state.set(RhaiScriptLoadState::Loaded);
    }
    if let Some(RecursiveDependencyLoadState::Failed(e)) =
        asset_server.get_recursive_dependency_load_state(&tracker.handle)
    {
        println!("{}", e);
    }
}
