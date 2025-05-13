use super::super::get_or_create_komotool_config_path;
use crate::create_komotool_asset_path;
use bevy_asset::{AssetEvent, AssetServer};
use bevy_ecs::event::EventWriter;
use bevy_ecs::resource::Resource;
use bevy_ecs::system::{Commands, Res};
use bevy_mod_scripting::core::asset::{Language, ScriptAsset, ScriptAssetSettings};
use crossbeam_channel::Receiver;
use notify::{Config, Event as NotifyEvent, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

// Resource to hold the file watcher and event receiver
#[derive(Resource)]
pub struct FileWatcher {
    rx: Receiver<String>,
    _watcher: RecommendedWatcher, // Keeps the watcher alive
}

pub fn setup_file_watcher(mut commands: Commands) {
    // Use crossbeam channels as requested
    let (tx, rx) = crossbeam_channel::unbounded();

    // Configure the watcher to watch for removal events
    let mut watcher = match RecommendedWatcher::new(
        move |res: Result<NotifyEvent, notify::Error>| {
            if let Ok(event) = res {
                if let EventKind::Remove(_) = event.kind {
                    for path in event.paths {
                        if let Some(path_str) = path.to_str() {
                            let _ = tx.send(path_str.to_string());
                        }
                    }
                }
            }
        },
        Config::default(),
    ) {
        Ok(watcher) => watcher,
        Err(e) => {
            println!("Failed to create file watcher: {}", e);
            return;
        }
    };

    // Watch the target directory RECURSIVELY as requested
    if let Ok(komotool_path) = get_or_create_komotool_config_path() {
        match watcher.watch(&komotool_path, RecursiveMode::Recursive) {
            Ok(_) => (),
            Err(e) => {
                println!("Failed to watch directory: {}", e);
                return;
            }
        }
    }

    // Insert the resource with the watcher and receiver
    commands.insert_resource(FileWatcher {
        rx,
        _watcher: watcher, // This keeps the watcher alive without any sleeping thread
    });
}

pub fn check_file_events(
    watcher: Option<Res<FileWatcher>>,
    mut event: EventWriter<AssetEvent<ScriptAsset>>,
    asset_server: Res<AssetServer>,
    settings: Res<ScriptAssetSettings>,
) {
    if let Some(watcher) = watcher {
        // Process all available events without blocking
        while let Ok(path) = watcher.rx.try_recv() {
            let asset_path = create_komotool_asset_path(&path);

            if let Some(id) = asset_server.get_path_id(&asset_path) {
                match settings.select_script_language(&asset_path) {
                    Language::Lua | Language::Rhai => {
                        let typed_id = id.try_typed::<ScriptAsset>();
                        match typed_id {
                            Ok(typed) => {
                                event.write(AssetEvent::Removed { id: typed });
                            }
                            Err(e) => {
                                println!("Failed to get typed id: {}", e);
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
