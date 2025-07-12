use crate::components::Focused;
use crate::relationships::{
    register_komorebi_oto_relationships_hooks_inner, register_relationships_hooks_inner,
};
use bevy_ecs::world::World;

pub fn register_relationships_hooks_focused(world: &mut World) {
    register_komorebi_oto_relationships_hooks_inner::<Focused>(world);
}
