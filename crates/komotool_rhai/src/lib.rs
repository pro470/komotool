use bevy_app::{App, Plugin};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::system::Commands;
use bevy_mod_scripting::rhai::RhaiScriptingPlugin;
use bevy_state::condition::in_state;
use komotool_assets::{check_scripts_loaded, handle_script_store_updates};
use komotool_utils::handler::{komotool_event_handler, KomoToolScriptStore};
use komotool_utils::prelude::*;
use komotool_utils::send_event_systems::{
    advance_to_all_done, send_post_startup_events, send_pre_startup_events, send_startup_events,
};
use komotool_utils::startup_schedule::{PostUpdateStartup, PreUpdateStartup, UpdateStartup};

pub struct KomoToolRhaiPlugin;

impl Plugin for KomoToolRhaiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RhaiScriptingPlugin::default())
            .init_resource::<KomoToolScriptStore<RhaiScriptingPlugin, OnPreStartUp>>()
            .init_resource::<KomoToolScriptStore<RhaiScriptingPlugin, OnStartUp>>()
            .init_resource::<KomoToolScriptStore<RhaiScriptingPlugin, OnPostStartUp>>()
            // Phased initialization systems
            .add_systems(
                PreUpdateStartup,
                komotool_event_handler::<RhaiScriptingPlugin, OnPreStartUp>
                    .run_if(in_state(GlobalLoadingState::PreStartupDone))
                    .after(send_pre_startup_events),
            )
            .add_systems(
                UpdateStartup,
                komotool_event_handler::<RhaiScriptingPlugin, OnStartUp>
                    .run_if(in_state(GlobalLoadingState::StartupDone))
                    .after(send_startup_events),
            )
            .add_systems(
                PostUpdateStartup,
                komotool_event_handler::<RhaiScriptingPlugin, OnPostStartUp>
                    .run_if(in_state(GlobalLoadingState::PostStartupDone))
                    .after(send_post_startup_events),
            )
            .add_systems(
                PostUpdateStartup,
                rhai_cleanup_script_stores
                    .run_if(in_state(GlobalLoadingState::AllDone))
                    .after(advance_to_all_done),
            )
            .add_systems(
                PreUpdateStartup,
                (
                    handle_script_store_updates::<RhaiScriptingPlugin, OnPreStartUp>,
                    handle_script_store_updates::<RhaiScriptingPlugin, OnStartUp>,
                    handle_script_store_updates::<RhaiScriptingPlugin, OnPostStartUp>,
                )
                    .before(check_scripts_loaded),
            );
    }
}

pub fn rhai_cleanup_script_stores(mut commands: Commands) {
    commands.remove_resource::<KomoToolScriptStore<RhaiScriptingPlugin, OnPreStartUp>>();
    commands.remove_resource::<KomoToolScriptStore<RhaiScriptingPlugin, OnStartUp>>();
    commands.remove_resource::<KomoToolScriptStore<RhaiScriptingPlugin, OnPostStartUp>>();

    println!("All rhai script stores removed.");
}
