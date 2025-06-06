pub mod callbacklabels;
pub mod handler;
pub mod loading_systems;
pub mod send_event_systems;
pub mod startup_schedule;

pub mod prelude {
    pub use super::*;
    pub use callbacklabels::*;
    pub use handler::*;
    pub use loading_systems::*;
    pub use send_event_systems::*;
    pub use startup_schedule::*;
}

use bevy_app::{App, Last, MainScheduleOrder, Plugin, PostUpdate};
use bevy_ecs::prelude::{not, resource_added, resource_changed};
use bevy_ecs::schedule::{Condition, IntoSystemConfigs, Schedule};
use bevy_state::app::AppExtStates;
use bevy_state::condition::in_state;
use handler::KomoToolScriptStoreAll;
use handler::insert_komotool_handlers;
use loading_systems::*;
use prelude::*;
use startup_schedule::configure_single_threaded_schedules;
use startup_schedule::{PostUpdateStartup, PreUpdateStartup, UpdateStartup};

#[derive(Default)]
pub struct KomoToolUtilsPlugin;

impl Plugin for KomoToolUtilsPlugin {
    fn build(&self, app: &mut App) {
        let app = app
            .init_resource::<LoadingCounter>()
            .init_resource::<KomoToolScriptStoreAll<OnPreUpdate>>()
            .init_resource::<KomoToolScriptStoreAll<OnUpdate>>()
            .init_resource::<KomoToolScriptStoreAll<OnPostUpdate>>()
            .init_state::<GlobalLoadingState>()
            .add_schedule(Schedule::new(PreUpdateStartup))
            .add_schedule(Schedule::new(UpdateStartup))
            .add_schedule(Schedule::new(PostUpdateStartup));
        if let Some(mut main_schedule_order) =
            app.world_mut().get_resource_mut::<MainScheduleOrder>()
        {
            main_schedule_order.insert_after(PostUpdate, PreUpdateStartup);
            main_schedule_order.insert_after(PreUpdateStartup, UpdateStartup);
            main_schedule_order.insert_after(UpdateStartup, PostUpdateStartup);
        }
        app.add_systems(
            UpdateStartup,
            update_global_state.run_if(
                resource_changed::<LoadingCounter>.and(not(resource_added::<LoadingCounter>)),
            ),
        )
        .add_systems(
            PreUpdateStartup,
            send_pre_startup_events.run_if(in_state(GlobalLoadingState::Loaded)),
        )
        .add_systems(
            UpdateStartup,
            send_startup_events.run_if(in_state(GlobalLoadingState::Loaded)),
        )
        // Post-Startup Events [Original "StartupDone"]
        .add_systems(
            PostUpdateStartup,
            send_post_startup_events.run_if(in_state(GlobalLoadingState::Loaded)),
        )
        .add_systems(
            PostUpdateStartup,
            advance_to_clean_up_done
                .run_if(in_state(GlobalLoadingState::Loaded))
                .after(send_post_startup_events),
        )
        .add_systems(
            PostUpdateStartup,
            advance_to_all_done
                .run_if(in_state(GlobalLoadingState::CleanupDone))
                .before(advance_to_clean_up_done),
        )
        .add_systems(
            Last,
            remove_startup_schedules.run_if(in_state(GlobalLoadingState::AllDone)),
        )
        .add_systems(
            PostUpdateStartup,
            configure_single_threaded_schedules.run_if(in_state(GlobalLoadingState::CleanupDone)),
        )
        .add_systems(
            PreUpdateStartup,
            insert_event_sending_systems.run_if(in_state(GlobalLoadingState::CleanupDone)),
        )
        .add_systems(
            UpdateStartup,
            insert_komotool_handlers.run_if(in_state(GlobalLoadingState::CleanupDone)),
        );
    }
}
