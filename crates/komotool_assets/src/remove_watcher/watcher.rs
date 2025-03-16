use super::super::get_or_create_komotool_config_path;
use bevy_ecs::event::{Event, EventWriter};
use bevy_ecs::system::{Commands, Res, Resource};
use crossbeam_channel::Receiver;
use notify::{Config, Event as NotifyEvent, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

// Event emitted when a file is removed
#[derive(Event)]
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
    let mut watcher = RecommendedWatcher::new(
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
    )
    .expect("Failed to create file watcher");

    // Watch the target directory RECURSIVELY as requested
    watcher
        .watch(
            &get_or_create_komotool_config_path().unwrap(),
            RecursiveMode::Recursive,
        )
        .expect("Failed to watch directory");

    // Insert the resource with the watcher and receiver
    commands.insert_resource(FileWatcher {
        rx,
        _watcher: watcher, // This keeps the watcher alive without any sleeping thread
    });
}

pub fn check_file_events(
    watcher: Res<FileWatcher>,
    mut removed_events: EventWriter<FileRemovedEvent>,
) {
    // Process all available events without blocking
    while let Ok(path) = watcher.rx.try_recv() {
        removed_events.send(FileRemovedEvent { path });
    }
}
