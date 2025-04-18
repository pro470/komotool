use super::super::get_or_create_komotool_config_path;
use bevy_ecs::event::{Event, EventWriter};
use bevy_ecs::system::{Commands, Res, Resource};
use bevy_reflect::Reflect;
use crossbeam_channel::Receiver;
use notify::{Config, Event as NotifyEvent, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

// Event emitted when a file is removed
#[derive(Event, Reflect)]
pub struct FileRemovedEvent {
    pub path: String,
}

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
    mut removed_events: EventWriter<FileRemovedEvent>,
) {
    if let Some(watcher) = watcher {
        // Process all available events without blocking
        while let Ok(path) = watcher.rx.try_recv() {
            removed_events.send(FileRemovedEvent { path });
        }
    }
}
