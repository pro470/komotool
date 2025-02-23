use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_state::app::StatesPlugin;
use bevy_core::{TaskPoolPlugin, TypeRegistrationPlugin, FrameCountPlugin};
use bevy_time::TimePlugin;
use bevy_mod_scripting::core::bindings::AllocatorDiagnosticPlugin;
use komotool_assets::KomotoolAssetsPlugin;
use komotool_ecs::KomoToolEcsPlugin;
use komotool_komorebic::KomoToolKomorebicPlugin;
use komotool_lua::KomoToolLuaPlugin;
use komotool_pipe::KomoToolPipePlugin;
use komotool_rhai::KomoToolRhaiPlugin;
use komotool_utils::KomoToolUtilsPlugin;
//use komotool_windows::KomoToolWindowsPlugin;
use komotoolc_pipe::KomoToolcPipePlugin;
use komotool_framepace::KomotoolFramepacePlugin;

fn main() {
    App::new()
        .add_plugins(KomotoolFramepacePlugin)
        .add_plugins(StatesPlugin)
        .add_plugins(TaskPoolPlugin::default())
        .add_plugins(TypeRegistrationPlugin)
        .add_plugins(FrameCountPlugin)
        .add_plugins(TimePlugin)
        .add_plugins(ScheduleRunnerPlugin::default())
        .add_plugins(AllocatorDiagnosticPlugin)
        .add_plugins(KomoToolPipePlugin)
        //.add_plugins(KomoToolWindowsPlugin)
        .add_plugins(KomoToolcPipePlugin)
        .add_plugins(KomoToolUtilsPlugin)
        .add_plugins(KomoToolEcsPlugin)
        .add_plugins(KomotoolAssetsPlugin)
        .add_plugins(KomoToolLuaPlugin)
        .add_plugins(KomoToolRhaiPlugin)
        .add_plugins(KomoToolKomorebicPlugin)
        .run();
}
