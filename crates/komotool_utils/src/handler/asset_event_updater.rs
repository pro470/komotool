use bevy_asset::{AssetEvent, AssetServer};
use bevy_ecs::prelude::*;
use bevy_mod_scripting::core::{
    event::IntoCallbackLabel,
    script::{ScriptAsset, ScriptId},
    IntoScriptPluginParams,
};
use crate::get_or_create_komotool_config_path;
use super::KomoToolScriptStore;

pub trait ScriptFunctionChecker {
    /// Check if a script implementation contains a specific function
    fn has_function(script_bytes: &[u8], function_name: &str) -> bool;
}

pub fn handle_script_store_updates<P, L>(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    mut remove_events: EventReader<FileRemovedEvent>,
    asset_server: Res<AssetServer>,
    mut script_store: ResMut<KomoToolScriptStore<P, L>>,
) where
    P: IntoScriptPluginParams + ScriptFunctionChecker + Send + Sync + 'static,
    L: IntoCallbackLabel + Send + Sync + 'static,
{
    // Process asset events
    for event in events.read() {
        match event {
            AssetEvent::Added { id } | AssetEvent::LoadedWithDependencies { id } => {
                if let Some(path) = asset_server.get_path(*id) {
                    // Convert to ScriptId format (path without source)
                    let script_id = ScriptId::from(path.path().to_string_lossy().to_string());
                    script_store.scripts.insert(script_id);
                }
            }
            AssetEvent::Modified { id } => {
                if let Some(path) = asset_server.get_path(*id) {
                    let script_id = ScriptId::from(path.path().to_string_lossy().to_string());
                    
                    // Check if script still has required functions
                    if let Some(script_bytes) = asset_server.get_handle(path).get_asset_bytes() {
                        if P::has_function(&script_bytes, L::into_callback_label().as_str()) {
                            script_store.scripts.insert(script_id);
                        } else {
                            script_store.scripts.remove(&script_id);
                        }
                    }
                }
            }
            AssetEvent::Removed { id } => {
                if let Some(path) = asset_server.get_path(*id) {
                    let script_id = ScriptId::from(path.path().to_string_lossy().to_string());
                    script_store.scripts.remove(&script_id);
                }
            }
            _ => {}
        }
    }

    // Process file removal events
    let komotool_path = get_or_create_komotool_config_path().unwrap();
    for event in remove_events.read() {
        if !is_in_script_folder(&event.path, &komotool_path) {
            continue;
        }
        
        let asset_path = create_komotool_asset_path(&event.path);
        let script_id = ScriptId::from(
            asset_path.path().to_string_lossy().to_string()
        );
        script_store.scripts.remove(&script_id);
    }
}
