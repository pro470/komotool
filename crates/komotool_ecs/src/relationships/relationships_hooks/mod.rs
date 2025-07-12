pub mod container;
mod focused;
pub mod monitor;
pub mod window;
pub mod workspace;

use crate::components::Focused;
use crate::prelude::{KomorebiChildOf, KomorebiChildren, relationships_hook};
use crate::relationships::HasKomorebiType;
use crate::relationships::komorebi_one_to_one_relationship::{
    KomorebiOneToOneRelationship, KomorebiOneToOneRelationshipTarget,
};
use bevy_ecs::component::{Component, HookContext};
use bevy_ecs::entity::Entity;
use bevy_ecs::relationship::{Relationship, RelationshipHookMode};
use bevy_ecs::world::{DeferredWorld, World};

pub fn register_relationships_hooks(world: &mut World) {
    monitor::register_relationships_hooks_monitor(world);
    workspace::register_relationships_hooks_workspace(world);
    container::register_relationships_hooks_container(world);
    window::register_relationships_hooks_window(world);
    focused::register_relationships_hooks_focused(world);
}

pub fn on_remove_komorebi_relationship<Komorebitype: HasKomorebiType>(
    mut world: DeferredWorld,
    entity: Entity,
    relationship_hook_mode: RelationshipHookMode,
) {
    if !relationships_hook::<KomorebiChildOf>(relationship_hook_mode) {
        return;
    }
    world.commands().entity(entity).remove::<KomorebiChildren>();

    world.commands().entity(entity).remove::<KomorebiChildOf>();

    world.commands().entity(entity).remove::<Focused>();
}

pub fn register_relationships_hooks_inner<Komorebitype: HasKomorebiType + Component>(
    world: &mut World,
) {
    let komorebi_hook = world.register_component_hooks::<Komorebitype>();

    komorebi_hook.on_remove(
        |mut world: DeferredWorld,
         HookContext {
             entity,
             relationship_hook_mode,
             ..
         }| {
            on_remove_komorebi_relationship::<Komorebitype>(
                world.reborrow(),
                entity,
                relationship_hook_mode,
            );
        },
    );
}

pub fn register_komorebi_oto_relationships_hooks_inner<
    Relationship: KomorebiOneToOneRelationship,
>(
    world: &mut World,
) {
    let komorebi_oto_hook = world.register_component_hooks::<Relationship>();

    komorebi_oto_hook
        .on_insert(<Relationship as KomorebiOneToOneRelationship>::on_insert)
        .on_remove(<Relationship as KomorebiOneToOneRelationship>::on_remove);

    let komorebi_oto_target_hook =
        world.register_component_hooks::<Relationship::Komorebionetoonerelationshiptarget>();

    komorebi_oto_target_hook.on_replace(
        <Relationship::Komorebionetoonerelationshiptarget as KomorebiOneToOneRelationshipTarget>::on_replace
    );
}
