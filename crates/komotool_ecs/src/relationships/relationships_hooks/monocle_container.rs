use crate::components::MonocleContainer;
use crate::relationships::komorebi_one_to_one_relationship::{KomorebiOneToOneRelationship, KomorebiOneToOneRelationshipTarget};
use crate::relationships::monocle_container::MonocleContainerTarget;
use bevy_ecs::world::World;

pub fn register_relationships_hooks_monocle_container(world: &mut World) {
    let monocle_container_hook = world.register_component_hooks::<MonocleContainer>();

    monocle_container_hook.on_insert(MonocleContainer::on_insert);
    monocle_container_hook.on_remove(MonocleContainer::on_remove);

    let monocle_container_target_hook = world.register_component_hooks::<MonocleContainerTarget>();

    monocle_container_target_hook.on_replace(MonocleContainerTarget::on_replace);
}
