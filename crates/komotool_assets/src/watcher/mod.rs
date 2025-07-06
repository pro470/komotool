pub mod komotool_watcher;

use bevy_asset::io::{AssetSourceEvent, AssetWatcher};
use bevy_log::warn;
use crossbeam_channel::Sender;
use komotool_schedule_runner::AppEvent;
pub use komotool_watcher::*;
use std::path::PathBuf;
use std::time::Duration;

pub fn add_komotool_watcher(
    path: PathBuf,
    sender: Sender<AssetSourceEvent>,
    debounce_wait_time: Duration,
    komotool_sender: Sender<AppEvent>,
) -> Option<Box<dyn AssetWatcher>> {
    Some(
        match FileWatcher::new(
            path.clone(),
            sender.clone(),
            debounce_wait_time,
            komotool_sender,
        ) {
            Ok(watcher) => Box::new(watcher),
            Err(e) => {
                warn!("Failed to create file watcher: {}", e);
                return None;
            }
        },
    )
}
