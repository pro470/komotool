use bevy_app::{App, Plugin};
use bevy_mod_scripting::rhai::RhaiScriptingPlugin;

/// Adds Rhai Scripting functionality to your [`App`]
#[derive(Default)]
pub struct KomoToolRhaiPlugin;

impl Plugin for KomoToolRhaiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RhaiScriptingPlugin::default());
    }
}
