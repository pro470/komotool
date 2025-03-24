use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
use bevy_mod_scripting::ScriptFunctionsPlugin;
use bevy_mod_scripting::core::bindings::AllocatorDiagnosticPlugin;
use bevy_state::app::StatesPlugin;
use bevy_time::TimePlugin;
use komotool_assets::KomotoolAssetsPlugin;
use komotool_ecs::KomoToolEcsPlugin;
use komotool_komorebic::KomoToolKomorebicPlugin;
use komotool_lua::KomoToolLuaPlugin;
use komotool_pipe::KomoToolPipePlugin;
use komotool_rhai::KomoToolRhaiPlugin;
use komotool_utils::KomoToolUtilsPlugin;
//use komotool_windows::KomoToolWindowsPlugin;
use komotool_framepace::KomotoolFramepacePlugin;
use komotoolc_pipe::KomoToolcPipePlugin;

fn main() {
    App::new()
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
        .add_plugins(KomotoolFramepacePlugin)
        .add_plugins(KomoToolEcsPlugin)
        .add_plugins(KomotoolAssetsPlugin)
        .add_plugins(KomoToolLuaPlugin)
        .add_plugins(KomoToolRhaiPlugin)
        .add_plugins(KomoToolKomorebicPlugin)
        .add_plugins(ScriptFunctionsPlugin)
        .run();
}
