use crate::window_info::{list_windows, WindowInfo, WindowList};
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Event)]
pub struct WindowChangeEvent {
    pub added: Vec<WindowInfo>,
    pub removed: Vec<WindowInfo>,
    pub changed: Vec<WindowInfo>,
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

#[derive(Resource, Default)]
pub struct WindowChangeTracker {
    last_update: Option<HashSet<WindowIdentifier>>,
}

// Updated system with change detection
pub fn update_window_list(
    mut windows: ResMut<WindowList>,
    mut tracker: ResMut<WindowChangeTracker>,
    mut change_events: EventWriter<WindowChangeEvent>,
) {
    match list_windows() {
        Ok(new_windows) => {
            let new_identifiers: HashSet<_> =
                new_windows.iter().map(WindowIdentifier::from).collect();

            let old_identifiers = tracker.last_update.take().unwrap_or_default();

            // Calculate differences
            let added: Vec<WindowInfo> = new_windows
                .iter()
                .filter(|w| !old_identifiers.contains(&WindowIdentifier::from(*w)))
                .cloned()
                .collect();

            let removed: Vec<WindowInfo> = old_identifiers
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

fn find_window_by_id<'a>(id: &WindowIdentifier, list: &'a [WindowInfo]) -> Option<&'a WindowInfo> {
    list.iter()
        .find(|w| w.hwnd == id.hwnd && w.title == id.title && w.exe_path == id.exe_path)
}

pub fn detect_changed_windows(previous: &[WindowInfo], current: &[WindowInfo]) -> Vec<WindowInfo> {
    current
        .iter()
        .filter(|curr| {
            previous.iter().any(|prev| {
                prev.hwnd == curr.hwnd
                    && (prev.title != curr.title || prev.exe_path != curr.exe_path)
            })
        })
        .cloned()
        .collect()
}

pub(crate) fn handle_window_changes(
    mut events: EventReader<WindowChangeEvent>,
    mut windows: ResMut<WindowList>,
) {
    let collected_events: Vec<_> = events.read().collect();

    for event in &collected_events {
        // Remove old windows first
        for removed_window in &event.removed {
            windows.0.retain(|w| w.hwnd != removed_window.hwnd);
        }

        // Update changed windows
        for changed_window in &event.changed {
            if let Some(index) = windows.0.iter().position(|w| w.hwnd == changed_window.hwnd) {
                windows.0[index] = changed_window.clone();
            }
        }

        // Add new windows
        windows.0.extend(event.added.iter().cloned());

        // Handle new windows
        for window in &event.added {
            info!(
                "New window detected: {} (PID: {})",
                window.title, window.pid
            );
        }

        // Handle removed windows
        for window in &event.removed {
            info!("Window closed: {} (PID: {})", window.title, window.pid);
        }

        // Handle changed windows
        for window in &event.changed {
            // Pass collected_events instead of &events
            if let Some(previous_state) = find_previous_state(window, &collected_events) {
                info!("Window changed: {} (PID: {})", window.title, window.pid);

                if previous_state.title != window.title {
                    info!(
                        "Title changed from '{}' to '{}'",
                        previous_state.title, window.title
                    );
                }
            }
        }
    }
}

// Updated helper function signature
fn find_previous_state<'a>(
    current: &WindowInfo,
    events: &'a [&WindowChangeEvent], // Changed to use slice of events
) -> Option<&'a WindowInfo> {
    events
        .iter()
        .flat_map(|e| e.removed.iter().chain(e.changed.iter()))
        .find(|w| w.hwnd == current.hwnd)
}
