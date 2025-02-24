mod callbacklabels;
mod loading_systems;
pub mod prelude;
mod send_event_systems;

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::{not, resource_added, resource_changed};
use bevy_ecs::schedule::{Condition, IntoSystemConfigs};
use bevy_state::app::AppExtStates;
pub use send_event_systems::*;
pub use loading_systems::*;
pub use prelude::*;

pub struct KomoToolUtilsPlugin;

impl Plugin for KomoToolUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingCounter>()
            .init_state::<GlobalLoadingState>()
            .add_systems(
                Update,
                update_global_state.run_if(
                    resource_changed::<LoadingCounter>.and(not(resource_added::<LoadingCounter>)),
                ),
            );
    }
}
