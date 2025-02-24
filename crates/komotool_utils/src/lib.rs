mod callbacklabels;
mod loading_systems;
pub mod prelude;
pub mod send_event_systems;

use bevy_app::{App, Last, Plugin, PostUpdate, PreUpdate, Update};
use bevy_ecs::prelude::{not, resource_added, resource_changed};
use bevy_ecs::schedule::{Condition, IntoSystemConfigs};
use bevy_state::app::AppExtStates;
use bevy_state::condition::in_state;
use send_event_systems::*;
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
            )
            .add_systems(
                PreUpdate,
                send_pre_startup_events.run_if(in_state(GlobalLoadingState::Loaded)),
            )
            .add_systems(
                Update,
                send_startup_events.run_if(in_state(GlobalLoadingState::PreStartupDone)),
            )
            // Post-Startup Events [Original "StartupDone"]
            .add_systems(
                PostUpdate,
                send_post_startup_events.run_if(in_state(GlobalLoadingState::StartupDone)),
            )
            .add_systems(
                PostUpdate,
                advance_to_all_done.run_if(in_state(GlobalLoadingState::PostStartupDone))
                    .after(send_post_startup_events))
            .add_systems(
                Last,
                send_pre_update_events.run_if(in_state(GlobalLoadingState::AllDone)),
            )
            .add_systems(
                PreUpdate,
                send_update_events.run_if(in_state(GlobalLoadingState::AllDone)),
            )
            .add_systems(
                Update,
                send_post_update_events.run_if(in_state(GlobalLoadingState::AllDone)),
            );
    }
}
