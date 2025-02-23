use bevy_app::{App, Last, Plugin, PostUpdate, PreStartup, PreUpdate, Update};
use bevy_asset::{AssetServer, Assets, Handle, LoadedFolder, RecursiveDependencyLoadState};
use bevy_ecs::event::EventWriter;
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::system::{Commands, Res, ResMut, Resource};
use bevy_mod_scripting::core::{
    asset::Language, event::*, handler::event_handler, script::ScriptComponent,
};
use bevy_mod_scripting::rhai::RhaiScriptingPlugin;
use bevy_state::app::AppExtStates;
use bevy_state::condition::in_state;
use bevy_state::state::{NextState, OnEnter, OnExit, States};
use komotool_utils::prelude::*;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum RhaiScriptLoadState {
    #[default]
    Loading,
    PreStartupDone,
    StartupDone,
    PostStartupDone,
    AllDone,
}

#[derive(Resource)]
struct RhaiScriptLoadTracker {
    handle: Handle<LoadedFolder>,
}

pub struct KomoToolRhaiPlugin;

impl Plugin for KomoToolRhaiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RhaiScriptingPlugin::default())
            .add_systems(
                OnEnter(RhaiScriptLoadState::Loading),
                increment_loading_counter,
            )
            .add_systems(
                OnExit(RhaiScriptLoadState::Loading),
                decrement_loading_counter,
            )
            .init_state::<RhaiScriptLoadState>()
            .add_systems(PreStartup, load_rhai_scripts)
            // Phased initialization systems
            .add_systems(
                PreUpdate,
                rhai_check_pre_startup
                    .run_if(in_state(RhaiScriptLoadState::Loading))
                    .before(event_handler::<OnPreStartUp, RhaiScriptingPlugin>),
            )
            .add_systems(
                Update,
                rhai_check_startup
                    .run_if(in_state(RhaiScriptLoadState::PreStartupDone))
                    .run_if(in_state(GlobalLoadingState::Loaded))
                    .before(event_handler::<OnStartUp, RhaiScriptingPlugin>),
            )
            .add_systems(
                PostUpdate,
                rhai_check_post_startup
                    .run_if(in_state(RhaiScriptLoadState::StartupDone))
                    .before(event_handler::<OnPostStartUp, RhaiScriptingPlugin>),
            )
            .add_systems(
                PreUpdate,
                event_handler::<OnPreStartUp, RhaiScriptingPlugin>
                    .run_if(in_state(RhaiScriptLoadState::PreStartupDone)),
            )
            .add_systems(
                Update,
                event_handler::<OnStartUp, RhaiScriptingPlugin>
                    .run_if(in_state(RhaiScriptLoadState::StartupDone)),
            )
            .add_systems(
                PostUpdate,
                event_handler::<OnPostStartUp, RhaiScriptingPlugin>
                    .run_if(in_state(RhaiScriptLoadState::PostStartupDone)),
            )
            .add_systems(
                PostUpdate,
                rhai_advance_to_all_done
                    .run_if(in_state(RhaiScriptLoadState::PostStartupDone))
                    .after(event_handler::<OnPostStartUp, RhaiScriptingPlugin>),
            )
            // Add systems for the main loop phases
            .add_systems(
                Last,
                rhai_send_pre_update_events.run_if(in_state(RhaiScriptLoadState::AllDone)),
            )
            .add_systems(
                PreUpdate,
                rhai_send_update_events.run_if(in_state(RhaiScriptLoadState::AllDone)),
            )
            .add_systems(
                PostUpdate,
                rhai_send_post_update_events.run_if(in_state(RhaiScriptLoadState::AllDone)),
            )
            .add_systems(
                PreUpdate,
                event_handler::<OnPreUpdate, RhaiScriptingPlugin>
                    .run_if(in_state(RhaiScriptLoadState::AllDone)),
            )
            .add_systems(
                Update,
                event_handler::<OnUpdate, RhaiScriptingPlugin>
                    .run_if(in_state(RhaiScriptLoadState::AllDone)),
            )
            .add_systems(
                PostUpdate,
                event_handler::<OnPostUpdate, RhaiScriptingPlugin>
                    .run_if(in_state(RhaiScriptLoadState::AllDone)),
            );
    }
}

fn load_rhai_scripts(asset_server: Res<AssetServer>, mut commands: Commands) {
    let path = std::path::Path::new("rhai");
    let source = bevy_asset::io::AssetSourceId::from("komotool_config");
    let asset_path = bevy_asset::AssetPath::from_path(path).with_source(source);
    let handle = asset_server.load_folder(asset_path);
    commands.insert_resource(RhaiScriptLoadTracker { handle });
}

fn rhai_check_pre_startup(
    asset_server: Res<AssetServer>,
    tracker: Res<RhaiScriptLoadTracker>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut commands: Commands,
    mut writer: EventWriter<ScriptCallbackEvent>,
    mut next_state: ResMut<NextState<RhaiScriptLoadState>>,
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
            Recipients::Language(Language::Rhai),
        ));
        next_state.set(RhaiScriptLoadState::PreStartupDone);
    }
    if let Some(RecursiveDependencyLoadState::Failed(e)) =
        asset_server.get_recursive_dependency_load_state(&tracker.handle)
    {
        println!("{}", e);
    }
}

fn rhai_check_startup(
    mut writer: EventWriter<ScriptCallbackEvent>,
    mut next_state: ResMut<NextState<RhaiScriptLoadState>>,
) {
    writer.send(ScriptCallbackEvent::new(
        OnStartUp,
        vec![],
        Recipients::Language(Language::Rhai),
    ));
    next_state.set(RhaiScriptLoadState::StartupDone);
}

fn rhai_check_post_startup(
    mut writer: EventWriter<ScriptCallbackEvent>,
    mut next_state: ResMut<NextState<RhaiScriptLoadState>>,
) {
    writer.send(ScriptCallbackEvent::new(
        OnPostStartUp,
        vec![],
        Recipients::Language(Language::Rhai),
    ));
    next_state.set(RhaiScriptLoadState::PostStartupDone);
}

fn rhai_advance_to_all_done(mut next_state: ResMut<NextState<RhaiScriptLoadState>>) {
    next_state.set(RhaiScriptLoadState::AllDone);
}

fn rhai_send_pre_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new(
        OnPreUpdate,
        vec![],
        Recipients::Language(Language::Rhai),
    ));
}

fn rhai_send_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new(
        OnUpdate,
        vec![],
        Recipients::Language(Language::Rhai),
    ));
}

fn rhai_send_post_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new(
        OnPostUpdate,
        vec![],
        Recipients::Language(Language::Rhai),
    ));
}
