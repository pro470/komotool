pub mod focused;
pub mod komorebi_relationship;

use crate::relationships::relationships_observers::focused::add_relationships_observers_focused;
use crate::relationships::relationships_observers::komorebi_relationship::add_relationships_observers_komorebi_relationship;
use bevy_ecs::world::World;

pub fn add_relationships_observers(world: &mut World) {
    add_relationships_observers_focused(world);
    add_relationships_observers_komorebi_relationship(world);
}
