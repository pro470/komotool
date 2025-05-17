pub mod schedule;
pub mod single_threading;

use bevy_ecs::change_detection::ResMut;
use bevy_ecs::prelude::Schedules;
pub use schedule::*;

pub use single_threading::*;
pub fn remove_komotool_startup_schedule(mut schedules: ResMut<Schedules>) {
    if schedules.remove_entry(KomoToolStartUp).is_some() {
        println!("Removed startup schedule");
    } else {
        println!("Failed to remove startup schedule");
    };
}
