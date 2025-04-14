use bevy_app::{App, Plugin};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::system::Commands;
use bevy_mod_scripting::lua::LuaScriptingPlugin;
use bevy_state::condition::in_state;
use komotool_assets::{check_scripts_loaded, handle_script_store_updates};
use komotool_utils::handler::{KomoToolScriptStore, komotool_event_handler};
use komotool_utils::startup_schedule::{PostUpdateStartup, PreUpdateStartup, UpdateStartup};
use komotool_utils::{prelude::*, send_event_systems::*};

pub struct KomoToolLuaPlugin;

impl Plugin for KomoToolLuaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LuaScriptingPlugin::default())
            .init_resource::<KomoToolScriptStore<LuaScriptingPlugin, OnPreStartUp>>()
            .init_resource::<KomoToolScriptStore<LuaScriptingPlugin, OnStartUp>>()
            .init_resource::<KomoToolScriptStore<LuaScriptingPlugin, OnPostStartUp>>()
            // Phased initialization systems
            .add_systems(
                PreUpdateStartup,
                komotool_event_handler::<LuaScriptingPlugin, OnPreStartUp>
                    .run_if(in_state(GlobalLoadingState::Loaded))
                    .after(send_pre_startup_events),
            )
            .add_systems(
                UpdateStartup,
                komotool_event_handler::<LuaScriptingPlugin, OnStartUp>
                    .run_if(in_state(GlobalLoadingState::Loaded))
                    .after(send_startup_events),
            )
            .add_systems(
                PostUpdateStartup,
                komotool_event_handler::<LuaScriptingPlugin, OnPostStartUp>
                    .run_if(in_state(GlobalLoadingState::Loaded))
                    .after(send_post_startup_events),
            )
            .add_systems(
                PostUpdateStartup,
                lua_cleanup_script_stores
                    .run_if(in_state(GlobalLoadingState::AllDone))
                    .after(advance_to_all_done),
            )
            .add_systems(
                PreUpdateStartup,
                (
                    handle_script_store_updates::<LuaScriptingPlugin, OnPreStartUp>,
                    handle_script_store_updates::<LuaScriptingPlugin, OnStartUp>,
                    handle_script_store_updates::<LuaScriptingPlugin, OnPostStartUp>,
                )
                    .before(check_scripts_loaded),
            );
    }
}

pub fn lua_cleanup_script_stores(mut commands: Commands) {
    commands.remove_resource::<KomoToolScriptStore<LuaScriptingPlugin, OnPreStartUp>>();
    commands.remove_resource::<KomoToolScriptStore<LuaScriptingPlugin, OnStartUp>>();
    commands.remove_resource::<KomoToolScriptStore<LuaScriptingPlugin, OnPostStartUp>>();

    println!("All lua script stores removed.");
}
