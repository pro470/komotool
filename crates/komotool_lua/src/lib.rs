use bevy::asset::{LoadedFolder, RecursiveDependencyLoadState};
use bevy::prelude::*;
use bevy_mod_scripting::core::{
    asset::ScriptAssetLoader, callback_labels, event::*, handler::event_handler, script::ScriptComponent,
};
use bevy_mod_scripting::lua::LuaScriptingPlugin;

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

// Add near other label definitions
callback_labels!(
PreStartUp => "on_pre_startup"
);

callback_labels!(
StartUp => "on_startup"
);

callback_labels!(
    PostStartUp => "on_post_startup"
);

callback_labels!(
    PreUpdate => "on_pre_update"
);

callback_labels!(
    Update => "on_update"
);

callback_labels!(
    PostUpdate => "on_post_update"
);

pub struct KomoToolLuaPlugin;

impl Plugin for KomoToolLuaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LuaScriptingPlugin::default())
            .init_state::<LuaScriptLoadState>()
            // Original load system remains in PreStartup
            .add_systems(PreStartup, load_lua_scripts)
            // Phased initialization systems
            .add_systems(
                bevy::prelude::PreUpdate,
                check_pre_startup
                    .run_if(in_state(LuaScriptLoadState::Loading))
                    .before(event_handler::<PreStartUp, LuaScriptingPlugin>),
            )
            .add_systems(
                bevy::prelude::Update,
                check_startup
                    .run_if(in_state(LuaScriptLoadState::PreStartupDone))
                    .before(event_handler::<StartUp, LuaScriptingPlugin>),
            )
            .add_systems(
                bevy::prelude::PostUpdate,
                check_post_startup
                    .run_if(in_state(LuaScriptLoadState::StartupDone))
                    .before(event_handler::<PostStartUp, LuaScriptingPlugin>),
            )
            // Keep original event handlers but move to main schedules
            .add_systems(
                bevy::prelude::PreUpdate,
                event_handler::<PreStartUp, LuaScriptingPlugin>
                    .run_if(in_state(LuaScriptLoadState::PreStartupDone)),
            )
            .add_systems(
                bevy::prelude::Update,
                event_handler::<StartUp, LuaScriptingPlugin>
                    .run_if(in_state(LuaScriptLoadState::StartupDone)),
            )
            .add_systems(
                bevy::prelude::PostUpdate,
                event_handler::<PostStartUp, LuaScriptingPlugin>
                    .run_if(in_state(LuaScriptLoadState::PostStartupDone)),
            )
            .add_systems(
                bevy::prelude::PostUpdate,
                advance_to_all_done
                    .run_if(in_state(LuaScriptLoadState::PostStartupDone))
                    .after(event_handler::<PostStartUp, LuaScriptingPlugin>),
            )
            // Add systems for the main loop phases
            .add_systems(
                bevy::prelude::Last,
                send_pre_update_events
                    .run_if(in_state(LuaScriptLoadState::AllDone))
                    .before(event_handler::<PreUpdate, LuaScriptingPlugin>),
            )
            .add_systems(
                bevy::prelude::PreUpdate,
                send_update_events
                    .run_if(in_state(LuaScriptLoadState::AllDone))
                    .before(event_handler::<Update, LuaScriptingPlugin>),
            )
            .add_systems(
                bevy::prelude::Update,
                send_post_update_events
                    .run_if(in_state(LuaScriptLoadState::AllDone))
                    .before(event_handler::<PostUpdate, LuaScriptingPlugin>),
            )
            .add_systems(
                bevy::prelude::PreUpdate,
                event_handler::<PreUpdate, LuaScriptingPlugin>
                    .run_if(in_state(LuaScriptLoadState::AllDone)),
            )
            .add_systems(
                bevy::prelude::Update,
                event_handler::<Update, LuaScriptingPlugin>
                    .run_if(in_state(LuaScriptLoadState::AllDone)),
            )
            .add_systems(
                bevy::prelude::PostUpdate,
                event_handler::<PostUpdate, LuaScriptingPlugin>
                    .run_if(in_state(LuaScriptLoadState::AllDone)),
            );
    }
}

fn load_lua_scripts(asset_server: Res<AssetServer>, mut commands: Commands) {
    let luascriptloader = ScriptAssetLoader { extensions: &["lua"], ..Default::default() };
    asset_server.register_loader(luascriptloader);
    let path = std::path::Path::new("lua");
    let source = bevy::asset::io::AssetSourceId::from("komotool_config");
    let asset_path = bevy::asset::AssetPath::from_path(path).with_source(source);
    let handle = asset_server.load_folder(asset_path);
    commands.insert_resource(LuaScriptLoadTracker { handle });
}

fn check_pre_startup(
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
                    commands.spawn(ScriptComponent::new(vec![path.to_string()]));
                }
            }
        }

        writer.send(ScriptCallbackEvent::new_for_all(PreStartUp, vec![]));
        next_state.set(LuaScriptLoadState::PreStartupDone);
    }
    if let Some(RecursiveDependencyLoadState::Failed(e)) =
        asset_server.get_recursive_dependency_load_state(&tracker.handle)
    {
        println!("{}", e);
    }
}

fn check_startup(
    mut writer: EventWriter<ScriptCallbackEvent>,
    mut next_state: ResMut<NextState<LuaScriptLoadState>>,
) {
    writer.send(ScriptCallbackEvent::new_for_all(StartUp, vec![]));
    next_state.set(LuaScriptLoadState::StartupDone);
}

fn check_post_startup(
    mut writer: EventWriter<ScriptCallbackEvent>,
    mut next_state: ResMut<NextState<LuaScriptLoadState>>,
) {
    writer.send(ScriptCallbackEvent::new_for_all(PostStartUp, vec![]));
    next_state.set(LuaScriptLoadState::PostStartupDone);
}

fn advance_to_all_done(mut next_state: ResMut<NextState<LuaScriptLoadState>>) {
    next_state.set(LuaScriptLoadState::AllDone);
}

fn send_pre_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new_for_all(PreUpdate, vec![]));
}

fn send_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new_for_all(Update, vec![]));
}

fn send_post_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new_for_all(PostUpdate, vec![]));
}
