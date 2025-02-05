use bevy::prelude::*;
use bevy_mod_scripting::*;
use bevy::asset::LoadedFolder;

#[derive(Resource)]
pub struct LuaScripts {
    pub folder_handle: Handle<LoadedFolder>,
}

pub struct KomoToolLuaPlugin;

impl Plugin for KomoToolLuaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_lua_scripts);
    }
}

fn load_lua_scripts(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle = asset_server.load_folder("komotool_config://Lua")
        .expect("Failed to load Lua scripts from komotool_config://Lua - make sure the directory exists");
    
    commands.insert_resource(LuaScripts {
        folder_handle: handle
    });
}
