mod window_change;
mod window_info;

use bevy::prelude::*;
use window_change::{handle_window_changes, update_window_list, WindowChangeEvent};
use window_info::WindowList;

pub struct KomoToolWindowsPlugin;

impl Plugin for KomoToolWindowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WindowChangeEvent>()
            .init_resource::<WindowList>()
            .init_resource::<window_change::WindowChangeTracker>()
            .add_systems(Update, (
                update_window_list,
                handle_window_changes
            ));
    }
}
