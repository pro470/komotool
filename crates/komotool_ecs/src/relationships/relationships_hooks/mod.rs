pub mod container;
pub mod monitor;
pub mod window;
pub mod workspace;

use crate::prelude::{KomorebiChildOf, KomorebiChildren, relationships_hook};
use crate::relationships::HasKomorebiType;
use bevy_ecs::component::{Component, HookContext};
use bevy_ecs::entity::Entity;
use bevy_ecs::relationship::{Relationship, RelationshipHookMode};
use bevy_ecs::world::{DeferredWorld, World};

pub fn register_relationships_hooks(world: &mut World) {
    monitor::register_relationships_hooks_monitor(world);
    workspace::register_relationships_hooks_workspace(world);
    container::register_relationships_hooks_container(world);
    window::register_relationships_hooks_window(world);
}

pub fn on_remove_komorebi_relationship<Komorebitype: HasKomorebiType>(
    mut world: DeferredWorld,
    entity: Entity,
    relationship_hook_mode: RelationshipHookMode,
) -> bool {
    if !relationships_hook::<KomorebiChildOf>(relationship_hook_mode) {
        return true;
    }

    let mut needs_to_remove_chlidren = false;
    let mut needs_to_revmove_childof = false;

    let entity_ref = world.entity(entity);
    if let Some(children) =
        entity_ref
            .get::<<KomorebiChildOf as bevy_ecs::relationship::Relationship>::RelationshipTarget>()
    {
        if Komorebitype::KOMOREBI_CHILD_TYPE == children.get_komorebi_type() {
            needs_to_remove_chlidren = true;
        }
    }

    if let Some(childof) = entity_ref.get::<KomorebiChildOf>() {
        if let Some(parant_children) = world.entity(childof.get()).get::<KomorebiChildren>() {
            if Komorebitype::KOMOREBI_CHILD_TYPE == parant_children.get_komorebi_type() {
                needs_to_revmove_childof = true;
            }
        } else {
            needs_to_revmove_childof = true;
        }
    }

    if needs_to_remove_chlidren {
        world.commands().entity(entity).remove::<KomorebiChildren>();
    }

    if needs_to_revmove_childof {
        world.commands().entity(entity).remove::<KomorebiChildOf>();
    }

    false
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
