use crate::relationships::register_relationships_hooks_inner;
use bevy_ecs::world::World;
use komorebi_client::Workspace;

pub fn register_relationships_hooks_workspace(world: &mut World) {
    register_relationships_hooks_inner::<Workspace>(world);
}
