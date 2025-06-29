use crate::relationships::register_relationships_hooks_inner;
use bevy_ecs::world::World;
use komorebi_client::Container;

pub fn register_relationships_hooks_container(world: &mut World) {
    register_relationships_hooks_inner::<Container>(world);
}
