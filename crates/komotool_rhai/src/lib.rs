use bevy_app::{App, Plugin, PostUpdate, PreUpdate, Update};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_mod_scripting::core::handler::event_handler;
use bevy_mod_scripting::rhai::RhaiScriptingPlugin;
use bevy_state::condition::in_state;
use komotool_utils::prelude::*;
use komotool_utils::send_event_systems::{
    send_post_startup_events, send_pre_startup_events, send_startup_events,
};

pub struct KomoToolRhaiPlugin;

impl Plugin for KomoToolRhaiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RhaiScriptingPlugin::default())
            // Phased initialization systems
            .add_systems(
                PreUpdate,
                event_handler::<OnPreStartUp, RhaiScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::PreStartupDone))
                    .after(send_pre_startup_events),
            )
            .add_systems(
                Update,
                event_handler::<OnStartUp, RhaiScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::StartupDone))
                    .after(send_startup_events),
            )
            .add_systems(
                PostUpdate,
                event_handler::<OnPostStartUp, RhaiScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::PostStartupDone))
                    .after(send_post_startup_events),
            )
            // Add systems for the main loop phases
            .add_systems(
                PreUpdate,
                event_handler::<OnPreUpdate, RhaiScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::AllDone)),
            )
            .add_systems(
                Update,
                event_handler::<OnUpdate, RhaiScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::AllDone)),
            )
            .add_systems(
                PostUpdate,
                event_handler::<OnPostUpdate, RhaiScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::AllDone)),
            );
    }
}
