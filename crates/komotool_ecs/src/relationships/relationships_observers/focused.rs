use crate::relationships::focused::{
    komorebi_oto_relationship_on_insert_event, komorebi_oto_relationship_on_remove_event,
};
use bevy_ecs::world::World;

pub fn add_relationships_observers_focused(world: &mut World) {
    world.add_observer(komorebi_oto_relationship_on_insert_event);
    world.add_observer(komorebi_oto_relationship_on_remove_event);
}
