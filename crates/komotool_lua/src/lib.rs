use bevy::prelude::*;
use bevy_mod_scripting::*;

pub struct KomoToolLuaPlugin;

impl Plugin for KomoToolLuaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_lua_scripts);
    }
}

fn load_lua_scripts(asset_server: Res<AssetServer>) {
    // Load all Lua scripts from the komotool_config source's Lua subdirectory
    asset_server.load_folder("komotool_config://Lua").expect(
        "Failed to load Lua scripts from komotool_config://Lua - make sure the directory exists"
    );
}
