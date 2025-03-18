use bevy_app::{App, Plugin, PostUpdate, PreUpdate, Update};
use bevy_ecs::schedule::{IntoSystemConfigs, Schedules};
use bevy_ecs::system::{Commands, ResMut};
use bevy_mod_scripting::core::ScriptingSystemSet;
use bevy_mod_scripting::lua::LuaScriptingPlugin;
use bevy_state::condition::in_state;
use komotool_assets::{check_scripts_loaded, handle_script_store_updates, handle_script_store_updates_all};
use komotool_utils::startup_schedule::{PostUpdateStartup, PreUpdateStartup, UpdateStartup};
use komotool_utils::{prelude::*, send_event_systems::*};
use komotool_utils::handler::{komotool_event_handler, KomoToolScriptStore};

pub struct KomoToolLuaPlugin;

impl Plugin for KomoToolLuaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LuaScriptingPlugin::default())
            .init_resource::<KomoToolScriptStore<LuaScriptingPlugin, OnUpdate>>()
            .init_resource::<KomoToolScriptStore<LuaScriptingPlugin, OnPreUpdate>>()
            .init_resource::<KomoToolScriptStore<LuaScriptingPlugin, OnPostUpdate>>()
            .init_resource::<KomoToolScriptStore<LuaScriptingPlugin, OnPreStartUp>>()
            .init_resource::<KomoToolScriptStore<LuaScriptingPlugin, OnStartUp>>()
            .init_resource::<KomoToolScriptStore<LuaScriptingPlugin, OnPostStartUp>>()
            // Phased initialization systems
            .add_systems(
                PreUpdateStartup,
                komotool_event_handler::<LuaScriptingPlugin, OnPreStartUp>
                    .run_if(in_state(GlobalLoadingState::PreStartupDone))
                    .after(send_pre_startup_events),
            )
            .add_systems(
                UpdateStartup,
                komotool_event_handler::<LuaScriptingPlugin, OnStartUp>
                    .run_if(in_state(GlobalLoadingState::StartupDone))
                    .after(send_startup_events),
            )
            .add_systems(
                PostUpdateStartup,
                komotool_event_handler::<LuaScriptingPlugin, OnPostStartUp>
                    .run_if(in_state(GlobalLoadingState::PostStartupDone))
                    .after(send_post_startup_events),
            )
            // Add systems for the main loop phases
            .add_systems(
                UpdateStartup,
                insert_komotool_lua_handlers.run_if(in_state(GlobalLoadingState::CleanupDone)),
            )
            .add_systems(
                PostUpdateStartup,
                lua_cleanup_script_stores.run_if(in_state(GlobalLoadingState::AllDone))
                    .after(advance_to_all_done)
            )
            .add_systems(PreUpdate, handle_script_store_updates_all::<LuaScriptingPlugin>.in_set(ScriptingSystemSet::ScriptCommandDispatch))
            .add_systems(
                PreUpdateStartup,
                (
                    handle_script_store_updates::<LuaScriptingPlugin, OnPreStartUp>,
                    handle_script_store_updates::<LuaScriptingPlugin, OnStartUp>,
                    handle_script_store_updates::<LuaScriptingPlugin, OnPostStartUp>,
                    ).before(check_scripts_loaded)
            )

        ;
    }
}

pub fn insert_komotool_lua_handlers(mut schedule: ResMut<Schedules>) {
    schedule.add_systems(PreUpdate, komotool_event_handler::<LuaScriptingPlugin, OnPreUpdate>);
    schedule.add_systems(Update, komotool_event_handler::<LuaScriptingPlugin, OnUpdate>);
    schedule.add_systems(
        PostUpdate,
        komotool_event_handler::<LuaScriptingPlugin, OnPostUpdate>,
    );
}

pub fn lua_cleanup_script_stores(
    mut commands: Commands
) {
    commands.remove_resource::<KomoToolScriptStore<LuaScriptingPlugin, OnPreStartUp>>();
    commands.remove_resource::<KomoToolScriptStore<LuaScriptingPlugin, OnStartUp>>();
    commands.remove_resource::<KomoToolScriptStore<LuaScriptingPlugin, OnPostStartUp>>();

    println!("All lua script stores removed.");
}