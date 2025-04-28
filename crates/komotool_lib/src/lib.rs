pub mod prelude {
    pub use super::*;
    pub use komorebi_client::*;
    pub use komotool_assets::prelude::*;
    pub use komotool_ecs::prelude::*;
    pub use komotool_framepace::*;
    pub use komotool_komorebic::*;
    pub use komotool_lua::*;
    pub use komotool_pipe::*;
    pub use komotool_rhai::*;
    pub use komotool_utils::prelude::*;
    pub use komotoolc_pipe::*;
}

pub use bevy_mod_scripting;
pub use komorebi_client;
pub use komotool_assets;
pub use komotool_ecs;
pub use komotool_framepace;
pub use komotool_komorebic;
pub use komotool_lua;
pub use komotool_pipe;
pub use komotool_rhai;
pub use komotool_utils;
pub use komotoolc_pipe;

bevy_app::plugin_group! {
    pub struct KomoToolPlugins {
        //bevy_mod_scripting::core::bindings:::AllocatorDiagnosticPlugin,
        komotool_pipe:::KomoToolPipePlugin,
        komotoolc_pipe:::KomoToolcPipePlugin,
        komotool_utils:::KomoToolUtilsPlugin,
        komotool_framepace:::KomotoolFramepacePlugin,
        komotool_ecs:::KomoToolEcsPlugin,
        komotool_assets:::KomotoolAssetsPlugin,
        bevy_mod_scripting::core:::BMSScriptingInfrastructurePlugin,
        komotool_komorebic:::KomoToolKomorebicPlugin,
        komotool_lua:::KomoToolLuaPlugin,
        komotool_rhai:::KomoToolRhaiPlugin,
    }
}
