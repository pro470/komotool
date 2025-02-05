use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy_mod_scripting::core::{callback_labels, event::*, handler::event_handler};
use bevy_mod_scripting::lua::LuaScriptingPlugin;

#[derive(Resource)]
pub struct LuaScripts {
    pub folder_handle: Handle<LoadedFolder>,
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
            // PreStartup phase
            .add_systems(
                PreStartup,
                (
                    load_lua_scripts,
                    send_pre_startup,
                    event_handler::<PreStartUp, LuaScriptingPlugin>,
                ),
            )
            // Startup phase
            .add_systems(
                Startup,
                (send_startup, event_handler::<StartUp, LuaScriptingPlugin>),
            )
            // PostStartup phase
            .add_systems(
                PostStartup,
                (
                    send_post_startup,
                    event_handler::<PostStartUp, LuaScriptingPlugin>,
                ),
            );
    }
}

fn load_lua_scripts(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle = asset_server.load_folder("komotool_config://Lua");

    commands.insert_resource(LuaScripts {
        folder_handle: handle,
    });
}

// Add these systems to your app setup
fn send_pre_startup(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new_for_all(PreStartUp, vec![]));
}

fn send_startup(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new_for_all(StartUp, vec![]));
}

fn send_post_startup(mut writer: EventWriter<ScriptCallbackEvent>) {
    writer.send(ScriptCallbackEvent::new_for_all(PostStartUp, vec![]));
}
