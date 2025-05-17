use bevy_app::{App, Plugin};
use bevy_mod_scripting::lua::LuaScriptingPlugin;

/// Adds Lua Scripting functionality to your [`App`]
#[derive(Default)]
pub struct KomoToolLuaPlugin;

impl Plugin for KomoToolLuaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LuaScriptingPlugin::default());
    }
}
