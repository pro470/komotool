mod window_info;
mod window_change;

use bevy::prelude::*;
use window_change::{WindowChangeEvent, update_window_list, handle_window_changes};
use window_info::WindowList;

pub struct KomoToolWindowsPlugin;

impl Plugin for KomoToolWindowsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<WindowChangeEvent>()
            .init_resource::<WindowList>()
            .init_resource::<window_change::WindowChangeTracker>()
            .add_systems(Update, (
                handle_window_changes,
                update_window_list
            ));
    }
}
