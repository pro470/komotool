use bevy_app::{App, Plugin, PreStartup, PreUpdate};
use bevy_asset::{
    AssetApp, AssetServer, Assets, Handle, LoadedFolder, RecursiveDependencyLoadState,
    {io::AssetSourceBuilder, AssetPlugin},
};
use bevy_ecs::system::{Commands, Res, ResMut, Resource};
use bevy_mod_scripting::core::script::ScriptComponent;
use bevy_state::app::AppExtStates;
use bevy_state::condition::in_state;
use bevy_state::state::{NextState, OnEnter, OnExit, States};
use komotool_utils::prelude::*;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum ScriptLoadState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Resource)]
struct ScriptLoadTracker {
    handle: Handle<LoadedFolder>,
}

/// The KomotoolAssetsPlugin, which registers `.config\Komotool`
/// as a custom asset source and ensures the `AssetPlugin` is added afterward.
pub struct KomotoolAssetsPlugin;

impl Plugin for KomotoolAssetsPlugin {
    fn build(&self, app: &mut App) {
        let komotool_config_path = get_or_create_komotool_config_path()
            .expect("Failed to set up `.config/Komotool` directory");

        app.register_asset_source(
            "komotool_config",
            AssetSourceBuilder::platform_default(&komotool_config_path.to_string_lossy(), None),
        );

        app.add_plugins(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..Default::default()
        });
        
        // Add general script loading functionality
        app.init_state::<ScriptLoadState>()
            .add_systems(OnEnter(ScriptLoadState::Loading), increment_loading_counter)
            .add_systems(OnExit(ScriptLoadState::Loading), decrement_loading_counter)
            .add_systems(PreStartup, load_scripts)
            .add_systems(
                PreUpdate,
                check_scripts_loaded.run_if(in_state(ScriptLoadState::Loading)),
            );
    }
}

/// Function that retrieves the `.config\Komotool` path and ensures the directory exists.
fn get_or_create_komotool_config_path() -> std::io::Result<PathBuf> {
    let user_profile =
        env::var("USERPROFILE").expect("Failed to fetch USERPROFILE environment variable");

    let komotool_path = Path::new(&user_profile).join(".config").join("Komotool");

    if !komotool_path.exists() {
        fs::create_dir_all(&komotool_path)?;
        println!("Created directory: {}", komotool_path.display());
    }

    Ok(komotool_path)
}

/// Function to load all scripts from the "scripts" folder
fn load_scripts(asset_server: Res<AssetServer>, mut commands: Commands) {
    let path = std::path::Path::new("scripts");
    let source = bevy_asset::io::AssetSourceId::from("komotool_config");
    let asset_path = bevy_asset::AssetPath::from_path(path).with_source(source);
    let handle = asset_server.load_folder(asset_path);
    commands.insert_resource(ScriptLoadTracker { handle });
}

/// System to check if scripts are loaded and register them
fn check_scripts_loaded(
    asset_server: Res<AssetServer>,
    tracker: Res<ScriptLoadTracker>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<ScriptLoadState>>,
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

        next_state.set(ScriptLoadState::Loaded);
    }
    if let Some(RecursiveDependencyLoadState::Failed(e)) =
        asset_server.get_recursive_dependency_load_state(&tracker.handle)
    {
        println!("Failed to load scripts: {}", e);
    }
}
