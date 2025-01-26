use crate::window_info::{WindowInfo, WindowList, list_windows};
use bevy::prelude::*;
use windows::{core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};
use std::collections::HashSet;
use crate::window_info;

#[derive(Event)]
pub struct WindowChangeEvent {
    pub added: Vec<WindowInfo>,
    pub removed: Vec<WindowInfo>,
    pub changed: Vec<WindowInfo>,
}

#[derive(Resource)]
pub struct WindowState {
    previous_windows: HashSet<WindowIdentifier>,
    current_windows: HashSet<WindowIdentifier>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct WindowIdentifier {
    hwnd: isize,
    title: String,
    exe_path: String,
}

impl From<&WindowInfo> for WindowIdentifier {
    fn from(info: &WindowInfo) -> Self {
        Self {
            hwnd: info.hwnd,
            title: info.title.clone(),
            exe_path: info.exe_path.clone(),
        }
    }
}

#[derive(Resource)]
pub struct WindowChangeTracker {
    last_update: Option<HashSet<WindowIdentifier>>,
}

impl Default for WindowChangeTracker {
    fn default() -> Self {
        Self {
            last_update: None,
        }
    }
}

// Updated system with change detection
pub fn update_window_list(
    mut windows: ResMut<WindowList>,
    mut tracker: ResMut<WindowChangeTracker>,
    mut change_events: EventWriter<WindowChangeEvent>,
) {
    match list_windows() {
        Ok(new_windows) => {
            let new_identifiers: HashSet<_> = new_windows
                .iter()
                .map(WindowIdentifier::from)
                .collect();

            let old_identifiers = tracker.last_update.take().unwrap_or_default();

            // Calculate differences
            let added = new_windows
                .iter()
                .filter(|w| !old_identifiers.contains(&WindowIdentifier::from(*w)))
                .cloned()
                .collect();

            let removed = old_identifiers
                .iter()
                .filter(|id| !new_identifiers.contains(id))
                .filter_map(|id| find_window_by_id(id, &windows.0))
                .cloned()
                .collect();

            let changed = detect_changed_windows(&windows.0, &new_windows);

            // Update resources
            windows.0 = new_windows;
            tracker.last_update = Some(new_identifiers);

            // Send event if changes detected
            if !added.is_empty() || !removed.is_empty() || !changed.is_empty() {
                change_events.send(WindowChangeEvent {
                    added,
                    removed,
                    changed,
                });
            }
        }
        Err(e) => error!("Window update failed: {}", e),
    }
}

fn find_window_by_id(id: &WindowIdentifier, list: &[WindowInfo]) -> Option<&WindowInfo> {
    list.iter().find(|w|
        w.hwnd == id.hwnd &&
            w.title == id.title &&
            w.exe_path == id.exe_path
    )
}

pub fn detect_changed_windows(previous: &[WindowInfo], current: &[WindowInfo]) -> Vec<WindowInfo> {
    current
        .iter()
        .filter(|curr| {
            previous.iter().any(|prev|
                prev.hwnd == curr.hwnd &&
                    (prev.title != curr.title || prev.exe_path != curr.exe_path)
            )
        })
        .cloned()
        .collect()
}
fn handle_new_windows(
    mut events: EventReader<WindowChangeEvent>,
) {
    for event in events.read() {
        for window in &event.added {
            info!("New window: {}", window.title);
        }
    }
}

fn handle_closed_windows(
    mut events: EventReader<WindowChangeEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        for window in &event.removed {
            commands.entity(window.entity).despawn();
        }
    }
}

pub(crate) fn handle_window_changes(
    mut events: EventReader<WindowChangeEvent>,
) {
    for event in events.read() {
        if !event.added.is_empty() {
            info!("New windows added: {}", event.added.len());
        }

        if !event.removed.is_empty() {
            info!("Windows closed: {}", event.removed.len());
        }

        if !event.changed.is_empty() {
            info!("Windows changed: {}", event.changed.len());
        }
    }
}