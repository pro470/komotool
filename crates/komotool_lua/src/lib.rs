use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy_mod_scripting::core::{callback_labels, event::*, handler::event_handler};
use bevy_mod_scripting::lua::LuaScriptingPlugin;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum ScriptLoadState {
    #[default]
    Loading,
    PreStartupDone,
    StartupDone,
    PostStartupDone,
    AllDone,
}

#[derive(Resource)]
struct ScriptLoadTracker {
    handle: Handle<LoadedFolder>,
}

// Add near other label definitions
callback_labels!(
PreStartUp => "pre_startup"
);

callback_labels!(
StartUp => "startup"
);

callback_labels!(
    PostStartUp => "post_startup"
);

pub struct KomoToolLuaPlugin;

impl Plugin for KomoToolLuaPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(LuaScriptingPlugin)
            .init_state::<ScriptLoadState>()
            // Original load system remains in PreStartup
            .add_systems(PreStartup, load_lua_scripts)
            // Phased initialization systems
            .add_systems(
                PreUpdate,
                check_pre_startup
                    .run_if(in_state(ScriptLoadState::Loading))
                    .before(event_handler::<PreStartUp, LuaScriptingPlugin>)
            )
            .add_systems(
                Update,
                check_startup
                    .run_if(in_state(ScriptLoadState::PreStartupDone))
                    .before(event_handler::<StartUp, LuaScriptingPlugin>)
            )
            .add_systems(
                PostUpdate,
                check_post_startup
                    .run_if(in_state(ScriptLoadState::StartupDone))
                    .before(event_handler::<PostStartUp, LuaScriptingPlugin>)
            )
            // Keep original event handlers but move to main schedules
            .add_systems(
                PreUpdate,
                event_handler::<PreStartUp, LuaScriptingPlugin>
                    .run_if(in_state(ScriptLoadState::PreStartupDone))
            )
            .add_systems(
                Update,
                event_handler::<StartUp, LuaScriptingPlugin>
                    .run_if(in_state(ScriptLoadState::StartupDone))
            )
            .add_systems(
                PostUpdate,
                event_handler::<PostStartUp, LuaScriptingPlugin>
                    .run_if(in_state(ScriptLoadState::PostStartupDone))
            )
            .add_systems(
                PostUpdate,
                advance_to_all_done
                    .run_if(in_state(ScriptLoadState::PostStartupDone))
                    .after(event_handler::<PostStartUp, LuaScriptingPlugin>)
            )
            // Add systems for the main loop phases
            .add_systems(
                PreUpdate,
                event_handler::<PreUpdate, LuaScriptingPlugin>
                    .run_if(in_state(ScriptLoadState::AllDone))
            )
            .add_systems(
                Update,
                event_handler::<Update, LuaScriptingPlugin>
                    .run_if(in_state(ScriptLoadState::AllDone))
            )
            .add_systems(
                PostUpdate,
                event_handler::<PostUpdate, LuaScriptingPlugin>
                    .run_if(in_state(ScriptLoadState::AllDone))
            );
    }
}

fn load_lua_scripts(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let handle = asset_server.load_folder("komotool_config://Lua");
    commands.insert_resource(ScriptLoadTracker { handle });
}

fn check_pre_startup(
    asset_server: Res<AssetServer>,
    tracker: Res<ScriptLoadTracker>,
    mut writer: EventWriter<ScriptCallbackEvent>,
    mut next_state: ResMut<NextState<ScriptLoadState>>,
) {
    if asset_server.get_recursive_dependency_load_state(&tracker.handle) == LoadState::Loaded {
        writer.send(ScriptCallbackEvent::new_for_all(PreStartUp, vec![]));
        next_state.set(ScriptLoadState::PreStartupDone);
    }
}

fn check_startup(
    mut writer: EventWriter<ScriptCallbackEvent>,
    mut next_state: ResMut<NextState<ScriptLoadState>>,
) {
    writer.send(ScriptCallbackEvent::new_for_all(StartUp, vec![]));
    next_state.set(ScriptLoadState::StartupDone);
}

fn check_post_startup(
    mut writer: EventWriter<ScriptCallbackEvent>,
    mut next_state: ResMut<NextState<ScriptLoadState>>,
) {
    writer.send(ScriptCallbackEvent::new_for_all(PostStartUp, vec![]));
    next_state.set(ScriptLoadState::PostStartupDone);
}

fn advance_to_all_done(
    mut next_state: ResMut<NextState<ScriptLoadState>>,
) {
    next_state.set(ScriptLoadState::AllDone);
}
