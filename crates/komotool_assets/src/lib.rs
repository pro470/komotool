mod remove_watcher;

use bevy_app::{App, Plugin, PreStartup, PreUpdate, Startup};
use bevy_asset::{
    AssetApp, AssetEvent, AssetId, AssetPath, AssetServer, Assets, Handle, LoadedFolder,
    RecursiveDependencyLoadState,
    {AssetPlugin, io::AssetSourceBuilder},
};
use bevy_ecs::entity::Entity;
use bevy_ecs::event::EventReader;
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::system::{Commands, Res, ResMut, Resource};
use bevy_mod_scripting::core::asset::{Language, ScriptAsset, ScriptMetadataStore};
use bevy_mod_scripting::core::event::IntoCallbackLabel;
use bevy_mod_scripting::core::script::{ScriptComponent, ScriptId};
use bevy_mod_scripting::core::{IntoScriptPluginParams, ScriptingSystemSet};
use bevy_mod_scripting::lua::LuaScriptingPlugin;
use bevy_mod_scripting::rhai::RhaiScriptingPlugin;
use bevy_reflect::Reflect;
use bevy_state::app::AppExtStates;
use bevy_state::condition::in_state;
use bevy_state::state::{NextState, OnEnter, OnExit, States};
use komotool_utils::handler::{KomoToolScriptStore, KomoToolScriptStoreAll, ScriptFunctionChecker};
use komotool_utils::prelude::*;
use komotool_utils::startup_schedule::PreUpdateStartup;
pub use remove_watcher::{check_file_events, setup_file_watcher};
use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum ScriptLoadState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Resource, Reflect)]
pub struct ScriptLoadTracker {
    handle: Handle<LoadedFolder>,
}

/// Resource to keep track of which entity corresponds to which script asset
#[derive(Resource, Default, Reflect)]
pub struct ScriptEntityMapping {
    pub handle_to_entity: HashMap<AssetId<ScriptAsset>, Entity>,
}

/// The KomotoolAssetsPlugin, which registers `.config\Komotool`
/// as a custom asset source and ensures the `AssetPlugin` is added afterward.
pub struct KomotoolAssetsPlugin;

impl Plugin for KomotoolAssetsPlugin {
    fn build(&self, app: &mut App) {
        if let Ok(komotool_config_path) = get_or_create_komotool_config_path() {
            app.register_asset_source(
                "komotool_config",
                AssetSourceBuilder::platform_default(&komotool_config_path.to_string_lossy(), None),
            );
        }

        app.add_plugins(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..Default::default()
        });

        // Add general script loading functionality
        app.init_state::<ScriptLoadState>()
            .init_resource::<ScriptEntityMapping>()
            .add_systems(OnEnter(ScriptLoadState::Loading), increment_loading_counter)
            .add_systems(OnExit(ScriptLoadState::Loading), decrement_loading_counter)
            .add_systems(Startup, setup_file_watcher)
            .add_systems(PreUpdate, check_file_events)
            .add_systems(PreStartup, load_scripts)
            .add_systems(
                PreUpdateStartup,
                check_scripts_loaded.run_if(in_state(ScriptLoadState::Loading)),
            )
            .add_systems(
                PreUpdate,
                handle_script_store_updates_all.in_set(ScriptingSystemSet::ScriptCommandDispatch),
            );
    }
}

/// Function that retrieves the `.config\Komotool` path and ensures the directory exists.
pub fn get_or_create_komotool_config_path() -> std::io::Result<PathBuf> {
    let user_profile = env::var("USERPROFILE");
    match user_profile {
        Ok(usr) => {
            let komotool_path = Path::new(&usr).join(".config").join("Komotool");

            if !komotool_path.exists() {
                fs::create_dir_all(&komotool_path)?;
                println!("Created directory: {}", komotool_path.display());
            }

            Ok(komotool_path)
        }
        Err(e) => {
            let error = format!(
                "Failed to fetch USERPROFILE environment variable. ValueError: {}",
                e
            );
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, error))
        }
    }
}

/// Function to load all scripts from the "scripts" folder
fn load_scripts(asset_server: Res<AssetServer>, mut commands: Commands) {
    if let Ok(komotool_config_path) = get_or_create_komotool_config_path() {
        let path = komotool_config_path.join("scripts");
        if !path.exists() {
            match fs::create_dir_all(&path) {
                Ok(_) => println!("Created directory: {}", path.display()),
                Err(e) => println!("Failed to create directory: {}", e),
            };
        }
    } else {
        println!("Failed to get Komotool config path");
        return;
    }
    let path = Path::new("scripts");
    let source = bevy_asset::io::AssetSourceId::from("komotool_config");
    let asset_path = bevy_asset::AssetPath::from_path(path).with_source(source);
    let handle = asset_server.load_folder(asset_path);
    commands.insert_resource(ScriptLoadTracker { handle });
}

/// System to check if scripts are loaded and register them
pub fn check_scripts_loaded(
    asset_server: Res<AssetServer>,
    tracker: Res<ScriptLoadTracker>,
    mut next_state: ResMut<NextState<ScriptLoadState>>,
) {
    if let Some(RecursiveDependencyLoadState::Loaded) =
        asset_server.get_recursive_dependency_load_state(&tracker.handle)
    {
        println!("Scripts loaded");
        next_state.set(ScriptLoadState::Loaded);
    }
    if let Some(RecursiveDependencyLoadState::Failed(e)) =
        asset_server.get_recursive_dependency_load_state(&tracker.handle)
    {
        println!("Failed to load scripts: {}", e);
    }
}

/// System to handle hot-reloading of script assets
pub fn handle_script_asset_events(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut script_mapping: ResMut<ScriptEntityMapping>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Added { id } | AssetEvent::LoadedWithDependencies { id } => {
                // Since we're using ScriptAsset events, we know this is already a script file
                if script_mapping.handle_to_entity.contains_key(id) {
                    continue;
                }
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
            }
            AssetEvent::Modified { id } => {
                // Handle script modification if needed
                if script_mapping.handle_to_entity.contains_key(id) {
                    println!("Script modified: {:?}", asset_server.get_path(*id));
                    // For modified scripts, we don't need to do anything as bevy_mod_scripting
                    // will reload the script content automatically
                }
            }
            AssetEvent::Removed { id } => {
                // Remove the entity if the script is removed
                if let Some(entity) = script_mapping.handle_to_entity.remove(id) {
                    println!("Removing script entity: {:?}", entity);
                    commands.entity(entity).despawn();
                }
            }
            _ => {}
        }
    }
}

pub fn handle_script_store_updates<P, L>(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<ScriptAsset>>,
    metadata_store: Res<ScriptMetadataStore>,
    mut script_store: ResMut<KomoToolScriptStore<P, L>>,
) where
    P: IntoScriptPluginParams
        + ScriptFunctionChecker
        + Send
        + Sync
        + 'static
        + std::default::Default,
    L: IntoCallbackLabel + Send + Sync + 'static + std::default::Default,
{
    // Process asset events
    for event in events.read() {
        match event {
            AssetEvent::Added { id } | AssetEvent::LoadedWithDependencies { id } => {
                if let Some(script_metadata) = metadata_store.get(*id) {
                    if P::LANGUAGE != script_metadata.language {
                        continue;
                    }
                } else {
                    continue;
                }

                if let Some(path) = asset_server.get_path(*id) {
                    if let Some(script_bytes) = assets.get(*id) {
                        if P::has_function(&script_bytes.content, L::into_callback_label().as_ref())
                        {
                            // Convert to ScriptId format (path without source)
                            let script_id =
                                ScriptId::from(path.path().to_string_lossy().to_string());
                            script_store.scripts.insert(script_id);
                            println!("Adding script: {}", path.path().to_string_lossy());
                        }
                    }
                }
            }
            AssetEvent::Modified { id } => {
                if let Some(script_metadata) = metadata_store.get(*id) {
                    if P::LANGUAGE != script_metadata.language {
                        continue;
                    }
                } else {
                    continue;
                }

                if let Some(path) = asset_server.get_path(*id) {
                    let script_id = ScriptId::from(path.path().to_string_lossy().to_string());

                    // Check if script still has required functions
                    if let Some(script_bytes) = assets.get(*id) {
                        println!("Script modified: {:?}", asset_server.get_path(*id));
                        if P::has_function(&script_bytes.content, L::into_callback_label().as_ref())
                        {
                            script_store.scripts.insert(script_id);
                        } else {
                            script_store.scripts.shift_remove(&script_id);
                        }
                    }
                }
            }
            AssetEvent::Removed { id } => {
                if let Some(path) = asset_server.get_path(*id) {
                    let script_id = ScriptId::from(path.path().to_string_lossy().to_string());
                    script_store.scripts.shift_remove(&script_id);
                    println!("File removed: {}", path.path().to_string_lossy());
                }
            }
            _ => {}
        }
    }
}

pub fn handle_script_store_updates_all_labels<P>(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<ScriptAsset>>,
    metadata_store: Res<ScriptMetadataStore>,
    mut update: ResMut<KomoToolScriptStore<P, OnUpdate>>,
    mut preupdate: ResMut<KomoToolScriptStore<P, OnPreUpdate>>,
    mut postupdate: ResMut<KomoToolScriptStore<P, OnPostUpdate>>,
) where
    P: IntoScriptPluginParams
        + ScriptFunctionChecker
        + Send
        + Sync
        + 'static
        + std::default::Default,
{
    // Process asset events
    for event in events.read() {
        match event {
            AssetEvent::Added { id } | AssetEvent::LoadedWithDependencies { id } => {
                // Skip if script is not for this plugin language
                if let Some(script_metadata) = metadata_store.get(*id) {
                    if P::LANGUAGE != script_metadata.language {
                        continue;
                    }
                } else {
                    continue;
                }

                if let Some(path) = asset_server.get_path(*id) {
                    if let Some(script_bytes) = assets.get(*id) {
                        // Get all functions in the script once
                        let script_functions = P::get_functions(&script_bytes.content);

                        // Convert to ScriptId format (path without source)
                        let script_id = ScriptId::from(path.path().to_string_lossy().to_string());

                        // Check and update each store
                        if script_functions.contains(OnUpdate::into_callback_label().as_ref()) {
                            update.scripts.insert(script_id.clone());
                            println!("Added to OnUpdate: {}", script_id);
                        }

                        if script_functions.contains(OnPreUpdate::into_callback_label().as_ref()) {
                            preupdate.scripts.insert(script_id.clone());
                            println!("Added to OnPreUpdate: {}", script_id);
                        }

                        if script_functions.contains(OnPostUpdate::into_callback_label().as_ref()) {
                            postupdate.scripts.insert(script_id.clone());
                            println!("Added to OnPostUpdate: {}", script_id);
                        }

                        println!("Processed new script: {}", path.path().to_string_lossy());
                    }
                }
            }
            AssetEvent::Modified { id } => {
                // Skip if script is not for this plugin language
                if let Some(script_metadata) = metadata_store.get(*id) {
                    if P::LANGUAGE != script_metadata.language {
                        continue;
                    }
                } else {
                    continue;
                }

                if let Some(path) = asset_server.get_path(*id) {
                    let script_id = ScriptId::from(path.path().to_string_lossy().to_string());

                    // Check if script still has required functions
                    if let Some(script_bytes) = assets.get(*id) {
                        println!("Script modified: {:?}", path);

                        // Get all functions in the script once
                        let script_functions = P::get_functions(&script_bytes.content);

                        // Update each store based on function presence
                        if script_functions.contains(OnUpdate::into_callback_label().as_ref()) {
                            update.scripts.insert(script_id.clone());
                        } else {
                            update.scripts.shift_remove(&script_id);
                        }

                        if script_functions.contains(OnPreUpdate::into_callback_label().as_ref()) {
                            preupdate.scripts.insert(script_id.clone());
                        } else {
                            preupdate.scripts.shift_remove(&script_id);
                        }

                        if script_functions.contains(OnPostUpdate::into_callback_label().as_ref()) {
                            postupdate.scripts.insert(script_id.clone());
                        } else {
                            postupdate.scripts.shift_remove(&script_id);
                        }
                    }
                }
            }
            AssetEvent::Removed { id } => {
                if let Some(path) = asset_server.get_path(*id) {
                    let script_id = ScriptId::from(path.path().to_string_lossy().to_string());

                    // Remove from all stores
                    update.scripts.shift_remove(&script_id);
                    preupdate.scripts.shift_remove(&script_id);
                    postupdate.scripts.shift_remove(&script_id);

                    println!("File removed: {}", path.path().to_string_lossy());
                }
            }
            _ => {}
        }
    }
}

pub fn is_in_script_folder(file_path: &str, komotool_path: &Path) -> bool {
    // Get the absolute path of the file
    let file_path = Path::new(file_path)
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(file_path)); // Avoid errors if file doesn't exist

    // Create the scripts directory path by joining "scripts" to the Komotool path
    let scripts_path = komotool_path.join("scripts");

    // Check if the file path starts with the script directory path
    file_path.starts_with(&scripts_path)
}

pub fn handle_script_store_updates_all(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    assets: Res<Assets<ScriptAsset>>,
    asset_server: Res<AssetServer>,
    metadata_store: Res<ScriptMetadataStore>,
    mut update: ResMut<KomoToolScriptStoreAll<OnUpdate>>,
    mut preupdate: ResMut<KomoToolScriptStoreAll<OnPreUpdate>>,
    mut postupdate: ResMut<KomoToolScriptStoreAll<OnPostUpdate>>,
) {
    // Process asset events
    for event in events.read() {
        match event {
            AssetEvent::Added { id } | AssetEvent::LoadedWithDependencies { id } => {
                let language = if let Some(script_metadata) = metadata_store.get(*id) {
                    script_metadata.language.clone()
                } else {
                    Language::Unknown
                };

                if let Some(script_bytes) = assets.get(*id) {
                    // Get all functions in the script once
                    let script_functions = match language {
                        Language::Lua => LuaScriptingPlugin::get_functions(&script_bytes.content),
                        Language::Rhai => RhaiScriptingPlugin::get_functions(&script_bytes.content),
                        Language::Unknown => continue,
                        _ => continue,
                    };

                    // Convert to ScriptId format (path without source)
                    let script_id = ScriptId::from(
                        script_bytes.asset_path.path().to_string_lossy().to_string(),
                    );

                    // Check and update each store
                    if script_functions.contains(OnUpdate::into_callback_label().as_ref()) {
                        update.scripts.insert(script_id.clone());
                        println!("Added to OnUpdate: {}", script_id);
                    }

                    if script_functions.contains(OnPreUpdate::into_callback_label().as_ref()) {
                        preupdate.scripts.insert(script_id.clone());
                        println!("Added to OnPreUpdate: {}", script_id);
                    }

                    if script_functions.contains(OnPostUpdate::into_callback_label().as_ref()) {
                        postupdate.scripts.insert(script_id.clone());
                        println!("Added to OnPostUpdate: {}", script_id);
                    }

                    println!(
                        "Processed new script: {}",
                        script_bytes.asset_path.path().to_string_lossy()
                    );
                }
            }
            AssetEvent::Modified { id } => {
                let language = if let Some(script_metadata) = metadata_store.get(*id) {
                    script_metadata.language.clone()
                } else {
                    Language::Unknown
                };

                // Check if script still has required functions
                if let Some(script_bytes) = assets.get(*id) {
                    println!("Script modified: {:?}", script_bytes.asset_path);

                    let script_id = ScriptId::from(
                        script_bytes.asset_path.path().to_string_lossy().to_string(),
                    );

                    // Get all functions in the script once
                    let script_functions = match language {
                        Language::Lua => LuaScriptingPlugin::get_functions(&script_bytes.content),
                        Language::Rhai => RhaiScriptingPlugin::get_functions(&script_bytes.content),
                        Language::Unknown => continue,
                        _ => continue,
                    };

                    // Update each store based on function presence
                    if script_functions.contains(OnUpdate::into_callback_label().as_ref()) {
                        update.scripts.insert(script_id.clone());
                    } else {
                        update.scripts.shift_remove(&script_id);
                    }

                    if script_functions.contains(OnPreUpdate::into_callback_label().as_ref()) {
                        preupdate.scripts.insert(script_id.clone());
                    } else {
                        preupdate.scripts.shift_remove(&script_id);
                    }

                    if script_functions.contains(OnPostUpdate::into_callback_label().as_ref()) {
                        postupdate.scripts.insert(script_id.clone());
                    } else {
                        postupdate.scripts.shift_remove(&script_id);
                    }
                }
            }
            AssetEvent::Removed { id } => {
                if let Some(path) = asset_server.get_path(*id) {
                    let script_id = ScriptId::from(path.path().to_string_lossy().to_string());

                    // Remove from all stores
                    update.scripts.shift_remove(&script_id);
                    preupdate.scripts.shift_remove(&script_id);
                    postupdate.scripts.shift_remove(&script_id);

                    println!("File removed: {}", path.path().to_string_lossy());
                }
            }
            _ => {}
        }
    }
}

pub fn create_komotool_asset_path(file_path: &str) -> AssetPath<'static> {
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
