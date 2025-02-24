use bevy_app::{App, Plugin, PostUpdate, PreStartup, PreUpdate, Update};
use bevy_asset::{AssetServer, Assets, Handle, LoadedFolder, RecursiveDependencyLoadState};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::system::{Commands, Res, ResMut, Resource};
use bevy_mod_scripting::core::{
    handler::event_handler, script::ScriptComponent,
};
use bevy_mod_scripting::lua::LuaScriptingPlugin;
use bevy_state::app::AppExtStates;
use bevy_state::condition::in_state;
use bevy_state::state::{NextState, OnEnter, OnExit, States};
use komotool_utils::{prelude::*, send_event_systems::*};

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum LuaScriptLoadState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Resource)]
struct LuaScriptLoadTracker {
    handle: Handle<LoadedFolder>,
}

pub struct KomoToolLuaPlugin;

impl Plugin for KomoToolLuaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LuaScriptingPlugin::default())
            .add_systems(
                OnEnter(LuaScriptLoadState::Loading),
                increment_loading_counter,
            )
            .add_systems(
                OnExit(LuaScriptLoadState::Loading),
                decrement_loading_counter,
            )
            .init_state::<LuaScriptLoadState>()
            .add_systems(PreStartup, load_lua_scripts)
            // Phased initialization systems
            .add_systems(
                PreUpdate,
                lua_check_pre_startup
                    .run_if(in_state(LuaScriptLoadState::Loading))
                    .before(event_handler::<OnPreStartUp, LuaScriptingPlugin>),
            )
            .add_systems(
                PreUpdate,
                event_handler::<OnPreStartUp, LuaScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::PreStartupDone))
                    .after(send_pre_startup_events),
            )
            .add_systems(
                Update,
                event_handler::<OnStartUp, LuaScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::StartupDone))
                    .after(send_startup_events),
            )
            .add_systems(
                PostUpdate,
                event_handler::<OnPostStartUp, LuaScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::PostStartupDone))
                    .after(send_post_startup_events),
            )
            // Add systems for the main loop phases
            .add_systems(
                PreUpdate,
                event_handler::<OnPreUpdate, LuaScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::AllDone)),
            )
            .add_systems(
                Update,
                event_handler::<OnUpdate, LuaScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::AllDone)),
            )
            .add_systems(
                PostUpdate,
                event_handler::<OnPostUpdate, LuaScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::AllDone)),
            );
    }
}

fn load_lua_scripts(asset_server: Res<AssetServer>, mut commands: Commands) {
    let path = std::path::Path::new("lua");
    let source = bevy_asset::io::AssetSourceId::from("komotool_config");
    let asset_path = bevy_asset::AssetPath::from_path(path).with_source(source);
    let handle = asset_server.load_folder(asset_path);
    commands.insert_resource(LuaScriptLoadTracker { handle });
}

fn lua_check_pre_startup(
    asset_server: Res<AssetServer>,
    tracker: Res<LuaScriptLoadTracker>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<LuaScriptLoadState>>,
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

        next_state.set(LuaScriptLoadState::Loaded);
    }
    if let Some(RecursiveDependencyLoadState::Failed(e)) =
        asset_server.get_recursive_dependency_load_state(&tracker.handle)
    {
        println!("{}", e);
    }
}
