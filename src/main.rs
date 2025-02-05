use bevy::prelude::*;
use komotool_assets::KomotoolAssetsPlugin;
use komotool_ecs::KomoToolEcsPlugin;
use komotool_komorebic::KomoToolKomorebicPlugin;
use komotool_lua::KomoToolLuaPlugin;
use komotool_pipe::KomoToolPipePlugin;
use komotool_rhai::KomoToolRhaiPlugin;
use komotool_utils::KomoToolUtilsPlugin;
use komotool_windows::KomoToolWindowsPlugin;
use komotoolc_pipe::KomoToolcPipePlugin;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(KomoToolPipePlugin)
        .add_plugins(KomoToolWindowsPlugin)
        .add_plugins(KomoToolcPipePlugin)
        .add_plugins(KomoToolUtilsPlugin)
        .add_plugins(KomoToolEcsPlugin)
        .add_plugins(KomotoolAssetsPlugin)
        .add_plugins(KomoToolLuaPlugin)
        .add_plugins(KomoToolRhaiPlugin)
        .add_plugins(KomoToolKomorebicPlugin)
        .run();
}
