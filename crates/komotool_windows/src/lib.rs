mod monitor_change;
mod monitor_info;
mod window_change;
mod window_info;

use bevy_app::{App, Update, Plugin};
use monitor_change::{handle_monitor_changes, update_monitor_list, MonitorChangeEvent};
use monitor_info::MonitorList;
use window_change::{handle_window_changes, update_window_list, WindowChangeEvent};
use window_info::WindowList;

pub struct KomoToolWindowsPlugin;

impl Plugin for KomoToolWindowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WindowChangeEvent>()
            .add_event::<MonitorChangeEvent>()
            .init_resource::<WindowList>()
            .init_resource::<MonitorList>()
            .init_resource::<window_change::WindowChangeTracker>()
            .init_resource::<monitor_change::MonitorChangeTracker>()
            .add_systems(
                Update,
                (
                    update_window_list,
                    update_monitor_list,
                    handle_window_changes,
                    handle_monitor_changes,
                ),
            );
    }
}
