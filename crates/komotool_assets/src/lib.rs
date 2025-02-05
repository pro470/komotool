use bevy::{
    asset::{io::AssetSourceBuilder, AssetPlugin},
    prelude::*,
};
use pathdiff::diff_paths;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

/// The KomotoolAssetsPlugin, which registers `.config\Komotool`
/// as a custom asset source and ensures the `AssetPlugin` is added afterward.
pub struct KomotoolAssetsPlugin;

impl Plugin for KomotoolAssetsPlugin {
    fn build(&self, app: &mut App) {
        let mut komotool_config_path = get_or_create_komotool_config_path()
            .expect("Failed to set up `.config/Komotool` directory");

        komotool_config_path = compute_relative_path_to_komotool(&komotool_config_path)
            .expect("didnt found the path to the Komotool config");

        app.register_asset_source(
            "komotool_config",
            AssetSourceBuilder::platform_default(&komotool_config_path.to_string_lossy(), None),
        );

        app.add_plugins(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..Default::default()
        });
    }
}

/// Function that retrieves the `.config\Komotool` path and ensures the directory exists.
fn get_or_create_komotool_config_path() -> std::io::Result<PathBuf> {
    let user_profile =
        env::var("USERPROFILE").expect("Failed to fetch USERPROFILE environment variable");

    let komotool_path = Path::new(&user_profile).join(".config").join("Komotool");

    if !komotool_path.exists() {
        fs::create_dir_all(&komotool_path)?;
        println!("Created directory: {}", komotool_path.display());
    }

    Ok(komotool_path)
}

/// Function that computes the relative path from the current executable directory to `.config\Komotool`.
fn compute_relative_path_to_komotool(komotool_path: &Path) -> Option<PathBuf> {
    let current_exe = env::current_exe().expect("Failed to fetch the current executable path");

    let current_exe_dir = current_exe
        .parent()
        .expect("Executable should have a parent directory");

    diff_paths(komotool_path, current_exe_dir)
}
