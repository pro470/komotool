mod callbacklabels;
mod loading_systems;
pub mod prelude;

use bevy::prelude::*;
pub use callbacklabels::*;
pub use loading_systems::*;
pub use prelude::*;

pub struct KomoToolUtilsPlugin;

impl Plugin for KomoToolUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingCounter>()
            .init_state::<GlobalLoadingState>()
            .add_systems(
                Update,
                update_global_state.run_if(
                    resource_changed::<LoadingCounter>.and(not(resource_added::<LoadingCounter>)),
                ),
            );
    }
}
