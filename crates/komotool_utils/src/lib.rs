pub mod callbacklabels;
pub mod handler;
pub mod komotool_schedule;
pub mod send_event_systems;
pub mod startup_schedule;

pub mod prelude {
    pub use super::*;
    pub use callbacklabels::*;
    pub use handler::*;
    pub use komotool_schedule::*;
    pub use send_event_systems::*;
    pub use startup_schedule::*;
}

use bevy_app::{App, MainScheduleOrder, Plugin, PreUpdate};
use bevy_ecs::schedule::{IntoScheduleConfigs, Schedule};
use handler::KomoToolScriptStoreAll;
use prelude::*;
use startup_schedule::configure_single_threaded_schedules;
use startup_schedule::{PostUpdateStartup, PreUpdateStartup, UpdateStartup};

#[derive(Default)]
pub struct KomoToolUtilsPlugin;

impl Plugin for KomoToolUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KomoToolScriptStoreAll<OnPreUpdate>>()
            .init_resource::<KomoToolScriptStoreAll<OnUpdate>>()
            .init_resource::<KomoToolScriptStoreAll<OnPostUpdate>>()
            .init_resource::<KomoToolScriptStoreAll<OnPreStartUp>>()
            .init_resource::<KomoToolScriptStoreAll<OnStartUp>>()
            .init_resource::<KomoToolScriptStoreAll<OnPostStartUp>>()
            .add_schedule(Schedule::new(PreUpdateStartup))
            .add_schedule(Schedule::new(UpdateStartup))
            .add_schedule(Schedule::new(PostUpdateStartup))
            .add_schedule(Schedule::new(KomoToolStartUpFinished))
            .add_schedule(Schedule::new(KomoToolStartUp))
            .add_schedule(Schedule::new(KomoToolPreUpdate))
            .add_schedule(Schedule::new(KomoToolUpdate))
            .add_schedule(Schedule::new(KomoToolPostUpdate))
            .add_systems(PreUpdateStartup, send_pre_startup_events)
            .add_systems(UpdateStartup, send_startup_events)
            .add_systems(PostUpdateStartup, send_post_startup_events)
            // KomoTool Handlers
            .add_systems(
                PreUpdateStartup,
                komotool_event_handler_all::<OnPreStartUp>.after(send_pre_startup_events),
            )
            .add_systems(
                UpdateStartup,
                komotool_event_handler_all::<OnStartUp>.after(send_startup_events),
            )
            .add_systems(
                PostUpdateStartup,
                komotool_event_handler_all::<OnPostStartUp>.after(send_post_startup_events),
            )
            .add_systems(KomoToolStartUpFinished, configure_single_threaded_schedules)
            .add_systems(
                KomoToolPreUpdate,
                send_pre_update_events
                    .before_ignore_deferred(komotool_event_handler_all::<OnPreUpdate>),
            )
            .add_systems(
                KomoToolUpdate,
                send_update_events.before_ignore_deferred(komotool_event_handler_all::<OnUpdate>),
            )
            .add_systems(
                KomoToolPostUpdate,
                send_post_update_events
                    .before_ignore_deferred(komotool_event_handler_all::<OnPostUpdate>),
            )
            .add_systems(KomoToolPreUpdate, komotool_event_handler_all::<OnPreUpdate>)
            .add_systems(KomoToolUpdate, komotool_event_handler_all::<OnUpdate>)
            .add_systems(
                KomoToolPostUpdate,
                komotool_event_handler_all::<OnPostUpdate>,
            );
        if let Some(mut mainscheduleorder) = app.world_mut().get_resource_mut::<MainScheduleOrder>()
        {
            mainscheduleorder.insert_after(PreUpdate, KomoToolStartUp);
        }
    }
}
