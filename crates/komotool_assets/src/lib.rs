use bevy::{
    asset::{io::AssetSourceBuilder, AssetPlugin},
    prelude::*,
};
use pathdiff::diff_paths;
use std::{
    env,
    fs,
    path::{Path, PathBuf},
};

/// The KomotoolAssetsPlugin, which registers `.config\Komotool`
/// as a custom asset source and ensures the `AssetPlugin` is added afterward.
pub struct KomotoolAssetsPlugin;

impl Plugin for KomotoolAssetsPlugin {
    fn build(&self, app: &mut App) {
        // Compute the `.config\Komotool` directory path and ensure it exists.
        let komotool_config_path = get_or_create_komotool_config_path()
            .expect("Failed to set up `.config/Komotool` directory");

        // Compute the relative path from the current executable directory to `.config\Komotool` (for debugging purposes).
        if let Some(relative_path) = compute_relative_path_to_komotool(&komotool_config_path) {
            println!(
                "Relative path from the current binary to `.config/Komotool`: {}",
                relative_path.display()
            );
        } else {
            eprintln!("Failed to compute relative path from the current executable to `.config/Komotool`.");
        }

        // Register `.config\Komotool` as a custom asset source called "komotool_config".
        app.register_asset_source(
            "komotool_config",
            AssetSourceBuilder::platform_default(&komotool_config_path.to_string_lossy(), None),
        );

        // Add the AssetPlugin (after registering the custom asset source).
        app.add_plugins(AssetPlugin {
            ..Default::default()
        });
    }
}

/// Function that retrieves the `.config\Komotool` path and ensures the directory exists.
fn get_or_create_komotool_config_path() -> std::io::Result<PathBuf> {
    // Step 1: Get the user's home directory using the USERPROFILE environment variable.
    let user_profile = env::var("USERPROFILE").expect("Failed to fetch USERPROFILE environment variable");

    // Step 2: Construct the absolute path to `.config\Komotool`.
    let komotool_path = Path::new(&user_profile).join(".config").join("Komotool");

    // Step 3: Ensure the directory exists (create it if it does not).
    if !komotool_path.exists() {
        fs::create_dir_all(&komotool_path)?;
        println!("Created directory: {}", komotool_path.display());
    }

    Ok(komotool_path)
}

/// Function that computes the relative path from the current executable directory to `.config\Komotool`.
fn compute_relative_path_to_komotool(komotool_path: &Path) -> Option<PathBuf> {
    // Step 1: Get the path to the current executable.
    let current_exe = env::current_exe().expect("Failed to fetch the current executable path");

    // Step 2: Get the parent directory of the executable.
    let current_exe_dir = current_exe.parent().expect("Executable should have a parent directory");

    // Step 3: Compute and return the relative path.
    diff_paths(komotool_path, current_exe_dir)
}
