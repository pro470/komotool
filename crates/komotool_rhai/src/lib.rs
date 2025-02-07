use bevy::asset::{LoadedFolder, RecursiveDependencyLoadState};
use bevy::prelude::*;
use bevy_mod_scripting::core::{
    asset::ScriptAssetLoader, callback_labels, event::*, handler::event_handler,
    script::ScriptComponent,
};
use bevy_mod_scripting::rhai::RhaiScriptingPlugin;

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

callback_labels!(
    OnPreStartUp => "on_pre_startup",
    OnStartUp => "on_startup",
    OnPostStartUp => "on_post_startup",
    OnPreUpdate => "on_pre_update",
    OnUpdate => "on_update",
    OnPostUpdate => "on_post_update"
);

pub struct KomoToolRhaiPlugin;

impl Plugin for KomoToolRhaiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RhaiScriptingPlugin::default())
            .init_state::<RhaiScriptLoadState>()
            .add_systems(PreStartup, load_rhai_scripts)
            // Phased initialization systems
            .add_systems(
                PreUpdate,
                check_pre_startup
                    .run_if(in_state(RhaiScriptLoadState::Loading))
                    .before(event_handler::<OnPreStartUp, RhaiScriptingPlugin>),
            )
            .add_systems(
                Update,
                check_startup
                    .run_if(in_state(RhaiScriptLoadState::PreStartupDone))
                    .before(event_handler::<OnStartUp, RhaiScriptingPlugin>),
            )
            .add_systems(
                PostUpdate,
                check_post_startup
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
                advance_to_all_done
                    .run_if(in_state(RhaiScriptLoadState::PostStartupDone))
                    .after(event_handler::<OnPostStartUp, RhaiScriptingPlugin>),
            )
            // Add systems for the main loop phases
            .add_systems(
                Last,
                send_pre_update_events.run_if(in_state(RhaiScriptLoadState::AllDone)),
            )
            .add_systems(
                PreUpdate,
                send_update_events.run_if(in_state(RhaiScriptLoadState::AllDone)),
            )
            .add_systems(
                PostUpdate,
                send_post_update_events.run_if(in_state(RhaiScriptLoadState::AllDone)),
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
    let rhaiscriptloader = ScriptAssetLoader {
        extensions: &["rhai"],
        ..Default::default()
    };
    asset_server.register_loader(rhaiscriptloader);
    let path = std::path::Path::new("rhai");
    let source = bevy::asset::io::AssetSourceId::from("komotool_config");
    let asset_path = bevy::asset::AssetPath::from_path(path).with_source(source);
    let handle = asset_server.load_folder(asset_path);
    commands.insert_resource(RhaiScriptLoadTracker { handle });
}

fn check_pre_startup(
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

        writer.send(ScriptCallbackEvent::new_for_all(OnPreStartUp, vec![]));
        next_state.set(RhaiScriptLoadState::PreStartupDone);
    }
    if let Some(RecursiveDependencyLoadState::Failed(e)) =
        asset_server.get_recursive_dependency_load_state(&tracker.handle)
    {
        println!("{}", e);
    }
}

fn check_startup(
    mut writer: EventWriter<ScriptCallbackEvent>,
    mut next_state: ResMut<NextState<RhaiScriptLoadState>>,
) {
    writer.send(ScriptCallbackEvent::new_for_all(OnStartUp, vec![]));
    next_state.set(RhaiScriptLoadState::StartupDone);
}

fn check_post_startup(
    mut writer: EventWriter<ScriptCallbackEvent>,
    mut next_state: ResMut<NextState<RhaiScriptLoadState>>,
) {
    writer.send(ScriptCallbackEvent::new_for_all(OnPostStartUp, vec![]));
    next_state.set(RhaiScriptLoadState::PostStartupDone);
}

fn advance_to_all_done(mut next_state: ResMut<NextState<RhaiScriptLoadState>>) {
    next_state.set(RhaiScriptLoadState::AllDone);
}

fn send_pre_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new_for_all(OnPreUpdate, vec![]));
}

fn send_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new_for_all(OnUpdate, vec![]));
}

fn send_post_update_events(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new_for_all(OnPostUpdate, vec![]));
}
