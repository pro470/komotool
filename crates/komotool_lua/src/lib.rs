use bevy::asset::{LoadedFolder, RecursiveDependencyLoadState};
use bevy::prelude::*;
use bevy_mod_scripting::core::{
    asset::Language,
    event::*,
    handler::event_handler,
    script::ScriptComponent,
};
use bevy_mod_scripting::lua::LuaScriptingPlugin;
use komotool_utils::prelude::*;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum LuaScriptLoadState {
    #[default]
    Loading,
    PreStartupDone,
    StartupDone,
    PostStartupDone,
    AllDone,
}

#[derive(Resource)]
struct LuaScriptLoadTracker {
    handle: Handle<LoadedFolder>,
}

pub struct KomoToolLuaPlugin;

impl Plugin for KomoToolLuaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LuaScriptingPlugin::default())
            .add_systems(OnEnter(LuaScriptLoadState::Loading), increment_loading_counter)
            .add_systems(OnExit(LuaScriptLoadState::Loading), decrement_loading_counter)
            .init_state::<LuaScriptLoadState>()
            .add_systems(PreStartup, load_lua_scripts)
            // Phased initialization systems
            .add_systems(
                PreUpdate,
                lua_check_pre_startup
                    .run_if(in_state(LuaScriptLoadState::Loading))
                    .before(event_handler::<OnPreStartUp, LuaScriptingPlugin>),
            )
            .add_systems(
                Update,
                lua_check_startup
                    .run_if(in_state(LuaScriptLoadState::PreStartupDone))
                    .run_if(in_state(GlobalLoadingState::Loaded))
                    .before(event_handler::<OnStartUp, LuaScriptingPlugin>),
            )
            .add_systems(
                PostUpdate,
                lua_check_post_startup
                    .run_if(in_state(LuaScriptLoadState::StartupDone))
                    .before(event_handler::<OnPostStartUp, LuaScriptingPlugin>),
            )
            .add_systems(
                PreUpdate,
                event_handler::<OnPreStartUp, LuaScriptingPlugin>
                    .run_if(in_state(LuaScriptLoadState::PreStartupDone)),
            )
            .add_systems(
                Update,
                event_handler::<OnStartUp, LuaScriptingPlugin>
                    .run_if(in_state(LuaScriptLoadState::StartupDone)),
            )
            .add_systems(
                PostUpdate,
                event_handler::<OnPostStartUp, LuaScriptingPlugin>
                    .run_if(in_state(LuaScriptLoadState::PostStartupDone)),
            )
            .add_systems(
                PostUpdate,
                lua_advance_to_all_done
                    .run_if(in_state(LuaScriptLoadState::PostStartupDone))
                    .after(event_handler::<OnPostStartUp, LuaScriptingPlugin>),
            )
            // Add systems for the main loop phases
            .add_systems(
                Last,
                lua_send_pre_update_events.run_if(in_state(LuaScriptLoadState::AllDone)),
            )
            .add_systems(
                PreUpdate,
                lua_send_update_events.run_if(in_state(LuaScriptLoadState::AllDone)),
            )
            .add_systems(
                Update,
                lua_send_post_update_events.run_if(in_state(LuaScriptLoadState::AllDone)),
            )
            .add_systems(
                PreUpdate,
                event_handler::<OnPreUpdate, LuaScriptingPlugin>
                    .run_if(in_state(LuaScriptLoadState::AllDone)),
            )
            .add_systems(
                Update,
                event_handler::<OnUpdate, LuaScriptingPlugin>
                    .run_if(in_state(LuaScriptLoadState::AllDone)),
            )
            .add_systems(
                PostUpdate,
                event_handler::<OnPostUpdate, LuaScriptingPlugin>
                    .run_if(in_state(LuaScriptLoadState::AllDone)),
            );
    }
}

fn load_lua_scripts(asset_server: Res<AssetServer>, mut commands: Commands) {
    let path = std::path::Path::new("lua");
    let source = bevy::asset::io::AssetSourceId::from("komotool_config");
    let asset_path = bevy::asset::AssetPath::from_path(path).with_source(source);
    let handle = asset_server.load_folder(asset_path);
    commands.insert_resource(LuaScriptLoadTracker { handle });
}

fn lua_check_pre_startup(
    asset_server: Res<AssetServer>,
    tracker: Res<LuaScriptLoadTracker>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut commands: Commands,
    mut writer: EventWriter<ScriptCallbackEvent>,
    mut next_state: ResMut<NextState<LuaScriptLoadState>>,
) {
    if let Some(RecursiveDependencyLoadState::Loaded) =
        asset_server.get_recursive_dependency_load_state(&tracker.handle)
    {
        if let Some(folder) = loaded_folders.get(&tracker.handle) {
            for handle in &folder.handles {
                if let Some(path) = handle.path() {
                    commands.spawn(ScriptComponent::new(vec![path
                        .path()
                        .to_string_lossy()
                        .to_string()]));
                }
            }
        }

        writer.send(ScriptCallbackEvent::new(
            OnPreStartUp,
            vec![],
            Recipients::Language(Language::Lua),
        ));
        next_state.set(LuaScriptLoadState::PreStartupDone);
    }
    if let Some(RecursiveDependencyLoadState::Failed(e)) =
        asset_server.get_recursive_dependency_load_state(&tracker.handle)
    {
        println!("{}", e);
    }
}

fn lua_check_startup(
    mut writer: EventWriter<ScriptCallbackEvent>,
    mut next_state: ResMut<NextState<LuaScriptLoadState>>,
) {
    writer.send(ScriptCallbackEvent::new(
        OnStartUp,
        vec![],
        Recipients::Language(Language::Lua),
    ));
    next_state.set(LuaScriptLoadState::StartupDone);
}

fn lua_check_post_startup(
    mut writer: EventWriter<ScriptCallbackEvent>,
    mut next_state: ResMut<NextState<LuaScriptLoadState>>,
) {
    writer.send(ScriptCallbackEvent::new(
        OnPostStartUp,
        vec![],
        Recipients::Language(Language::Lua),
    ));
    next_state.set(LuaScriptLoadState::PostStartupDone);
}

fn lua_advance_to_all_done(mut next_state: ResMut<NextState<LuaScriptLoadState>>) {
    next_state.set(LuaScriptLoadState::AllDone);
}

fn lua_send_pre_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new(
        OnPreUpdate,
        vec![],
        Recipients::Language(Language::Lua),
    ));
}

fn lua_send_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new(
        OnUpdate,
        vec![],
        Recipients::Language(Language::Lua),
    ));
}

fn lua_send_post_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new(
        OnPostUpdate,
        vec![],
        Recipients::Language(Language::Lua),
    ));
}
