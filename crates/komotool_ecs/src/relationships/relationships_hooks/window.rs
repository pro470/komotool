use crate::relationships::register_relationships_hooks_inner;
use bevy_ecs::world::World;
use komorebi_client::Window;

pub fn register_relationships_hooks_window(world: &mut World) {
    register_relationships_hooks_inner::<Window>(world);
}
