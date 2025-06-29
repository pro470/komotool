use crate::relationships::register_relationships_hooks_inner;
use bevy_ecs::world::World;
use komorebi_client::Monitor;

pub fn register_relationships_hooks_monitor(world: &mut World) {
    register_relationships_hooks_inner::<Monitor>(world);
}
