use crate::{monitor_info::*, window_info::WindowList};
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Event)]
pub struct MonitorChangeEvent {
    pub added: Vec<MonitorInfo>,
    pub removed: Vec<MonitorInfo>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct MonitorIdentifier {
    handle: isize,
    device_name: String,
}

impl From<&MonitorInfo> for MonitorIdentifier {
    fn from(info: &MonitorInfo) -> Self {
        Self {
            handle: info.handle,
            device_name: info.device_name.clone(),
        }
    }
}

#[derive(Resource, Default)]
pub struct MonitorChangeTracker {
    last_state: Option<HashSet<MonitorIdentifier>>,
}

pub fn update_monitor_list(
    mut monitors: ResMut<MonitorList>,
    mut tracker: ResMut<MonitorChangeTracker>,
    mut change_events: EventWriter<MonitorChangeEvent>,
) {
    match list_monitors() {
        Ok(new_monitors) => {
            let new_identifiers: HashSet<_> =
                new_monitors.iter().map(MonitorIdentifier::from).collect();
            let old_identifiers = tracker.last_state.take().unwrap_or_default();

            let added: Vec<MonitorInfo> = new_monitors
                .iter()
                .filter(|m| !old_identifiers.contains(&MonitorIdentifier::from(*m)))
                .cloned()
                .collect();

            let removed: Vec<MonitorInfo> = old_identifiers
                .iter()
                .filter(|id| !new_identifiers.contains(id))
                .filter_map(|id| find_monitor_by_id(id, &monitors.0))
                .cloned()
                .collect();

            // Send event if changes detected
            if !added.is_empty() || !removed.is_empty() {
                // Update resources
                tracker.last_state = Some(new_identifiers);
                change_events.send(MonitorChangeEvent { added, removed });
            }
        }
        Err(e) => error!("Monitor update failed: {}", e),
    }
}

fn find_monitor_by_id<'a>(
    id: &MonitorIdentifier,
    list: &'a [MonitorInfo],
) -> Option<&'a MonitorInfo> {
    list.iter()
        .find(|m| m.handle == id.handle && m.device_name == id.device_name)
}

pub(crate) fn handle_monitor_changes(
    mut events: EventReader<MonitorChangeEvent>,
    mut monitors: ResMut<MonitorList>,
) {
    for event in events.read() {
        // Remove old monitors first
        for removed_monitor in &event.removed {
            monitors.0.retain(|m| m.handle != removed_monitor.handle);
            info!("Monitor removed: {}", removed_monitor.device_name);
        }

        // Add new monitors
        monitors.0.extend(event.added.iter().cloned());
        for added_monitor in &event.added {
            info!(
                "New monitor detected: {} ({}x{}) @ {}x{} DPI",
                added_monitor.device_name,
                added_monitor.width,
                added_monitor.height,
                added_monitor.dpi_x,
                added_monitor.dpi_y
            );
        }
    }
}
