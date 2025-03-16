mod remove_watcher;

use bevy_app::{App, Plugin, PreStartup, PreUpdate, Startup, Update};
use bevy_asset::{
    AssetApp, AssetEvent, AssetId, AssetPath, AssetServer, Assets, Handle, LoadedFolder,
    RecursiveDependencyLoadState,
    {io::AssetSourceBuilder, AssetPlugin},
};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, Res, ResMut, Resource};
use bevy_ecs::event::EventReader;
use bevy_ecs::entity::Entity;
use bevy_mod_scripting::core::asset::ScriptAsset;
use bevy_mod_scripting::core::script::ScriptComponent;
use bevy_state::app::AppExtStates;
use bevy_state::condition::in_state;
use bevy_state::state::{NextState, OnEnter, OnExit, States};
use komotool_utils::prelude::*;
use komotool_utils::GlobalLoadingState;
use remove_watcher::{check_file_events, setup_file_watcher, FileRemovedEvent};
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
            .add_event::<FileRemovedEvent>()
            .init_resource::<ScriptEntityMapping>()
            .add_systems(OnEnter(ScriptLoadState::Loading), increment_loading_counter)
            .add_systems(OnExit(ScriptLoadState::Loading), decrement_loading_counter)
            .add_systems(Startup, setup_file_watcher)
            .add_systems(PreUpdate, check_file_events)
            .add_systems(PreStartup, load_scripts)
            .add_systems(
                PreUpdate,
                check_scripts_loaded.run_if(in_state(ScriptLoadState::Loading)),
            )
            .add_systems(
                Update,
                handle_script_asset_events.run_if(in_state(GlobalLoadingState::AllDone))
            );
    }
}

/// Function that retrieves the `.config\Komotool` path and ensures the directory exists.
pub fn get_or_create_komotool_config_path() -> std::io::Result<PathBuf> {
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
    let path = Path::new("scripts");
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
                    
                    // Store the mapping - typed() returns AssetId directly, not an Option
                    let asset_id = handle.id().typed::<ScriptAsset>();
                    script_mapping.handle_to_entity.insert(asset_id, entity);
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
    mut remove_event: EventReader<FileRemovedEvent>,
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

    let komotool_path = get_or_create_komotool_config_path().unwrap();

    for event in remove_event.read() {
        if !is_in_script_folder(&event.path, &komotool_path) {
            continue;
        }
        let event_path = create_komotool_asset_path(&event.path);
        let assetid = asset_server
            .get_path_id(&event_path)
            .unwrap_or_else(|| {
                println!("Failed to get path ID for file: {}", &event.path);
                AssetId::<ScriptAsset>::default().into()
            })
            .typed::<ScriptAsset>();
        // Remove the entity if the script is removed
        if let Some(entity) = script_mapping.handle_to_entity.remove(&assetid) {
            println!("Removing script entity: {:?}", entity);
            commands.entity(entity).despawn();
            println!("File removed: {}", event_path.path().to_string_lossy());
        }
    }
}

fn is_in_script_folder(file_path: &str, komotool_path: &Path) -> bool {
    // Get the absolute path of the file
    let file_path = Path::new(file_path)
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(file_path)); // Avoid errors if file doesn't exist

    // Create the scripts directory path by joining "scripts" to the Komotool path
    let scripts_path = komotool_path.join("scripts");

    // Check if the file path starts with the script directory path
    file_path.starts_with(&scripts_path)
}

fn create_komotool_asset_path(file_path: &str) -> AssetPath<'static> {
    let path = Path::new(file_path);

    // Find the Komotool directory in the path (case insensitive)
    let mut components = Vec::new();
    let mut found_komotool = false;

    for component in path.components() {
        let component_str = component.as_os_str().to_string_lossy();

        if found_komotool {
            // After finding Komotool, collect all remaining components
            components.push(component_str.to_string());
        } else if component_str.to_lowercase() == "komotool" {
            // Found the Komotool directory (case insensitive)
            found_komotool = true;
        }
    }

    // Join the components with forward slashes for the asset path
    let relative_path = components.join("/");

    let source = bevy_asset::io::AssetSourceId::from("komotool_config");
    AssetPath::from(relative_path).with_source(source)
}

}
