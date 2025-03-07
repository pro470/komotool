use bevy_app::{App, Plugin, PreStartup, PreUpdate, Update};
use bevy_asset::{
    AssetApp, AssetEvent, AssetId, AssetServer, Assets, Handle, LoadedFolder, RecursiveDependencyLoadState,
    {io::AssetSourceBuilder, AssetPlugin},
};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::system::{Commands, Res, ResMut, Resource};
use bevy_ecs::event::EventReader;
use bevy_ecs::entity::Entity;
use bevy_mod_scripting::core::asset::ScriptAsset;
use bevy_mod_scripting::core::script::ScriptComponent;
use bevy_state::app::AppExtStates;
use bevy_state::condition::in_state;
use bevy_state::state::{NextState, OnEnter, OnExit, States};
use komotool_utils::prelude::*;
use std::{
    collections::HashMap,
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

/// Resource to keep track of which entity corresponds to which script asset
#[derive(Resource, Default)]
pub struct ScriptEntityMapping {
    pub handle_to_entity: HashMap<AssetId<ScriptAsset>, Entity>,
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
            )
            .init_resource::<ScriptEntityMapping>()
            .add_systems(Update, handle_script_asset_events);
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
    mut script_mapping: ResMut<ScriptEntityMapping>,
    mut next_state: ResMut<NextState<ScriptLoadState>>,
) {
    if let Some(RecursiveDependencyLoadState::Loaded) =
        asset_server.get_recursive_dependency_load_state(&tracker.handle)
    {
        if let Some(folder) = loaded_folders.get(&tracker.handle) {
            for handle in &folder.handles {
                if let Some(path) = handle.path() {
                    let script_path = path.path().to_string_lossy().to_string();
                    let entity = commands.spawn(ScriptComponent::new(vec![script_path])).id();
                    
                    // Store the mapping if this is a script asset
                    if let Some(asset_id) = handle.id().typed::<ScriptAsset>() {
                        script_mapping.handle_to_entity.insert(asset_id, entity);
                    }
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

/// System to handle hot-reloading of script assets
fn handle_script_asset_events(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut script_mapping: ResMut<ScriptEntityMapping>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Added { id } => {
                // Since we're using ScriptAsset events, we know this is already a script file
                if let Some(path) = asset_server.get_path(*id) {
                    // Create a new entity with the script component
                    let script_path = path.path().to_string_lossy().to_string();
                    println!("Adding script: {}", script_path);
                    
                    // Avoid duplication - remove existing entity if present
                    if let Some(existing_entity) = script_mapping.handle_to_entity.get(id) {
                        commands.entity(*existing_entity).despawn();
                    }
                    
                    let entity = commands.spawn(ScriptComponent::new(vec![script_path])).id();
                    
                    // Store the mapping between handle ID and entity
                    script_mapping.handle_to_entity.insert(*id, entity);
                }
            },
            AssetEvent::Modified { id } => {
                // Handle script modification if needed
                if script_mapping.handle_to_entity.contains_key(id) {
                    println!("Script modified: {:?}", asset_server.get_path(*id));
                    // For modified scripts, we don't need to do anything as bevy_mod_scripting
                    // will reload the script content automatically
                }
            },
            AssetEvent::Removed { id } => {
                // Remove the entity if the script is removed
                if let Some(entity) = script_mapping.handle_to_entity.remove(id) {
                    println!("Removing script entity: {:?}", entity);
                    commands.entity(entity).despawn();
                }
            },
            _ => {}
        }
    }
}
