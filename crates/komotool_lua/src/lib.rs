use bevy_app::{App, Plugin, PostUpdate, PreUpdate, Update};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_mod_scripting::core::handler::event_handler;
use bevy_mod_scripting::lua::LuaScriptingPlugin;
use bevy_state::condition::in_state;
use komotool_utils::{prelude::*, send_event_systems::*};

pub struct KomoToolLuaPlugin;

impl Plugin for KomoToolLuaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LuaScriptingPlugin::default())
            // Phased initialization systems
            .add_systems(
                PreUpdate,
                event_handler::<OnPreStartUp, LuaScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::PreStartupDone))
                    .after(send_pre_startup_events),
            )
            .add_systems(
                Update,
                event_handler::<OnStartUp, LuaScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::StartupDone))
                    .after(send_startup_events),
            )
            .add_systems(
                PostUpdate,
                event_handler::<OnPostStartUp, LuaScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::PostStartupDone))
                    .after(send_post_startup_events),
            )
            // Add systems for the main loop phases
            .add_systems(
                PreUpdate,
                event_handler::<OnPreUpdate, LuaScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::AllDone)),
            )
            .add_systems(
                Update,
                event_handler::<OnUpdate, LuaScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::AllDone)),
            )
            .add_systems(
                PostUpdate,
                event_handler::<OnPostUpdate, LuaScriptingPlugin>
                    .run_if(in_state(GlobalLoadingState::AllDone)),
            );
    }
}
