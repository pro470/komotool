#[cfg(feature = "DevRelease")]
use bevy::a11y::AccessibilityPlugin;
#[cfg(feature = "DevRelease")]
use bevy::core_pipeline::CorePipelinePlugin;
#[cfg(feature = "DevRelease")]
use bevy::input::InputPlugin;
#[cfg(feature = "DevRelease")]
use bevy::log::LogPlugin;
#[cfg(feature = "DevRelease")]
use bevy::pbr::PbrPlugin;
#[cfg(feature = "DevRelease")]
use bevy::picking::DefaultPickingPlugins;
#[cfg(feature = "DevRelease")]
use bevy::prelude::ImagePlugin;
#[cfg(feature = "DevRelease")]
use bevy::render::RenderPlugin;
#[cfg(feature = "DevRelease")]
use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;
#[cfg(feature = "DevRelease")]
use bevy::sprite::SpritePlugin;
#[cfg(feature = "DevRelease")]
use bevy::text::TextPlugin;
#[cfg(feature = "DevRelease")]
use bevy::window::WindowPlugin;
#[cfg(feature = "DevRelease")]
use bevy::winit::{WakeUp, WinitPlugin};
use bevy_app::TaskPoolPlugin;
use bevy_app::{App, AppExit, ScheduleRunnerPlugin};
use bevy_diagnostic::FrameCountPlugin;
#[cfg(feature = "DevRelease")]
use bevy_inspector_egui::bevy_egui::EguiPlugin;
#[cfg(feature = "DevRelease")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_scripting::ScriptFunctionsPlugin;
use bevy_mod_scripting::core::BMSScriptingInfrastructurePlugin;
use bevy_mod_scripting::core::bindings::{AllocatorDiagnosticPlugin, CoreScriptGlobalsPlugin};
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

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(StatesPlugin)
        .add_plugins(TaskPoolPlugin::default())
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
        .add_plugins(ScriptFunctionsPlugin)
        .add_plugins(CoreScriptGlobalsPlugin::default())
        .add_plugins(BMSScriptingInfrastructurePlugin)
        .add_plugins(KomoToolKomorebicPlugin)
        .add_plugins(KomoToolLuaPlugin)
        .add_plugins(KomoToolRhaiPlugin);
    #[cfg(feature = "DevRelease")]
    {
        app.add_plugins((
            LogPlugin {
                level: bevy::log::Level::INFO,
                ..Default::default()
            },
            InputPlugin,
            WindowPlugin::default(),
            AccessibilityPlugin,
            WinitPlugin::<WakeUp>::default(),
            RenderPlugin::default(),
            ImagePlugin::default(),
            PipelinedRenderingPlugin,
            CorePipelinePlugin,
            SpritePlugin,
            TextPlugin,
            PbrPlugin::default(),
            DefaultPickingPlugins,
        ))
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new());
    }
    app.run()
}
