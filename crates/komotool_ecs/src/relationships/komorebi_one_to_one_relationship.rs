use crate::relationships::{
    GetIndex, KomorebiChildOf, KomorebiChildren, KomorebiType,
};
use bevy_ecs::change_detection::MaybeLocation;
use bevy_ecs::component::{Component, HookContext, Mutable};
use bevy_ecs::entity::{ContainsEntity, Entity};
use bevy_ecs::relationship::{Relationship, RelationshipHookMode};
use bevy_ecs::world::{DeferredWorld, EntityWorldMut};
use bevy_log::warn;
use bevy_reflect::Reflect;

pub trait KomorebiOneToOneRelationship: Component + Reflect {
    type Komorebionetoonerelationshiptarget: KomorebiOneToOneRelationshipTarget<
        Komorebionetoonerelationship = Self,
    >;
    const CHILD: KomorebiType;
    const PARENT: KomorebiType;

    fn on_insert(world: DeferredWorld, hookcontext: HookContext);

    fn on_remove(world: DeferredWorld, hookcontext: HookContext);
}

pub trait KomorebiOneToOneRelationshipTarget: Component<Mutability = Mutable> + Reflect {
    type Komorebionetoonerelationship: KomorebiOneToOneRelationship<
        Komorebionetoonerelationshiptarget = Self,
    >;

    fn set_entity(&mut self, entity: Entity);

    fn get_entity(&self) -> Entity;

    fn set_index(&mut self, index: usize);

    fn get_index(&self) -> usize;

    fn from(entity: Entity, index: usize) -> Self;

    fn on_replace(world: DeferredWorld, hookcontext: HookContext);
}

pub enum KomorebiOTOonInsertResult {
    Ok(Entity, usize),
    Err,
}

pub enum KomorebiOTOonRemoveResult {
    Ok(Entity, usize),
    Err,
}

pub fn komorebi_oto_relationship_on_insert<Relationship: KomorebiOneToOneRelationship>(
    mut world: DeferredWorld,
    HookContext {
        entity,
        caller,
        relationship_hook_mode,
        ..
    }: HookContext,
) -> KomorebiOTOonInsertResult {
    if let RelationshipHookMode::Skip = relationship_hook_mode {
        return KomorebiOTOonInsertResult::Err;
    }

    if let Some(komorebi_child_of) = world.entity(entity).get::<KomorebiChildOf>() {
        let mut target_entity = komorebi_child_of.get();

        if let Some(komrebi_children) = world.entity(target_entity).get::<KomorebiChildren>() {
            let komorebi_type = komrebi_children.get_komorebi_type();
            if !Relationship::CHILD.verify_komorebi_type(world.entity(entity))
                && !matches!(Relationship::CHILD, KomorebiType::NotDefined)
            {
                world.commands().entity(entity).remove::<Relationship>();
                return KomorebiOTOonInsertResult::Err;
            }
            if let Some(index) = komrebi_children.get_index_of(&entity) {
                if let Some(new_target_entity) = get_komorebi_oto_parent::<Relationship>(
                    komorebi_type,
                    world.reborrow(),
                    caller,
                    target_entity,
                    entity,
                    true,
                ) {
                    target_entity = new_target_entity;
                } else {
                    return KomorebiOTOonInsertResult::Err;
                }

                if let Ok(mut target_entity_mut) = world.get_entity_mut(target_entity) {
                    if let Some(mut relationship_target) =
                        target_entity_mut
                            .get_mut::<Relationship::Komorebionetoonerelationshiptarget>()
                    {
                        let old_entity = relationship_target.get_entity();
                        relationship_target.set_entity(entity);
                        relationship_target.set_index(index);

                        if old_entity != entity {
                            world.commands().entity(old_entity).remove::<Relationship>();
                        }
                    } else {
                        let relationship_target = <<Relationship as KomorebiOneToOneRelationship>::Komorebionetoonerelationshiptarget as KomorebiOneToOneRelationshipTarget>::from(entity, index);

                        world
                            .commands()
                            .entity(target_entity)
                            .insert(relationship_target);
                    }
                } else {
                    warn!(
                        "{}Failed to get the target entity: {}.",
                        caller
                            .map(|location| format!("{location}: "))
                            .unwrap_or_default(),
                        target_entity
                    );
                    world.commands().entity(entity).remove::<Relationship>();
                    return KomorebiOTOonInsertResult::Err;
                }
                KomorebiOTOonInsertResult::Ok(target_entity, index)
            } else {
                warn!(
                    "{}Failed to get index for {}.",
                    caller
                        .map(|location| format!("{location}: "))
                        .unwrap_or_default(),
                    core::any::type_name::<Relationship::Komorebionetoonerelationshiptarget>()
                );
                world.commands().entity(entity).remove::<Relationship>();
                KomorebiOTOonInsertResult::Err
            }
        } else {
            warn!(
                "{}Failed to get {} for {}.",
                caller
                    .map(|location| format!("{location}: "))
                    .unwrap_or_default(),
                core::any::type_name::<KomorebiChildren>(),
                core::any::type_name::<Relationship::Komorebionetoonerelationshiptarget>()
            );
            world.commands().entity(entity).remove::<Relationship>();
            KomorebiOTOonInsertResult::Err
        }
    } else {
        warn!(
            "{}Failed to get {} for {}.",
            caller
                .map(|location| format!("{location}: "))
                .unwrap_or_default(),
            core::any::type_name::<KomorebiChildOf>(),
            core::any::type_name::<Relationship::Komorebionetoonerelationshiptarget>()
        );
        world.commands().entity(entity).remove::<Relationship>();
        KomorebiOTOonInsertResult::Err
    }
}

pub fn komorebi_oto_relationship_on_remove<Relationship: KomorebiOneToOneRelationship>(
    mut world: DeferredWorld,
    HookContext {
        entity,
        caller,
        relationship_hook_mode,
        ..
    }: HookContext,
) -> KomorebiOTOonRemoveResult {
    if let RelationshipHookMode::Skip = relationship_hook_mode {
        return KomorebiOTOonRemoveResult::Err;
    }

    if let Some(komorebi_child_of) = world.entity(entity).get::<KomorebiChildOf>() {
        let mut target_entity = komorebi_child_of.get();
        if let Some(new_target_entity) = get_komorebi_oto_parent::<Relationship>(
            Relationship::CHILD,
            world.reborrow(),
            caller,
            target_entity,
            entity,
            false,
        ) {
            target_entity = new_target_entity;
        } else {
            return KomorebiOTOonRemoveResult::Err;
        }
        if let Ok(target_entity_mut) = world.get_entity(target_entity)
            && let Some(relationship_target) =
                target_entity_mut.get::<Relationship::Komorebionetoonerelationshiptarget>()
            {
                let old_entity = relationship_target.get_entity();
                let index = relationship_target.get_index();
                if old_entity == entity {
                    if let Ok(mut entity) = world.commands().get_entity(target_entity) {
                        entity.queue(|mut entity: EntityWorldMut| {
                            entity.remove::<Relationship::Komorebionetoonerelationshiptarget>();
                        });
                    }
                    return KomorebiOTOonRemoveResult::Ok(target_entity, index);
                }
            }
        KomorebiOTOonRemoveResult::Err
    } else {
        warn!(
            "{}Failed to get {} for {}.",
            caller
                .map(|location| format!("{location}: "))
                .unwrap_or_default(),
            core::any::type_name::<KomorebiChildOf>(),
            core::any::type_name::<Relationship::Komorebionetoonerelationshiptarget>()
        );
        KomorebiOTOonRemoveResult::Err
    }
}

pub fn komorebi_oto_relationship_target_on_replace<
    RelationshipTarget: KomorebiOneToOneRelationshipTarget,
>(
    mut world: DeferredWorld,
    HookContext {
        entity,
        caller,
        relationship_hook_mode,
        ..
    }: HookContext,
) {
    if let RelationshipHookMode::Skip = relationship_hook_mode {
        return;
    }
    if let Some(komorebi_oto_target) = world.entity(entity).get::<RelationshipTarget>() {
        let target_entity = komorebi_oto_target.get_entity();
        world
            .commands()
            .entity(target_entity)
            .remove::<RelationshipTarget::Komorebionetoonerelationship>();
    } else {
        warn!(
            "{}Failed to get {} for {}. for on_replace of the komorebi one to one relationship target.",
            caller
                .map(|location| format!("{location}: "))
                .unwrap_or_default(),
            core::any::type_name::<RelationshipTarget>(),
            core::any::type_name::<RelationshipTarget>()
        );
    }
}

pub fn get_komorebi_oto_parent<Relationship: KomorebiOneToOneRelationship>(
    komorebi_type: KomorebiType,
    mut world: DeferredWorld,
    caller: MaybeLocation,
    mut target_entity: Entity,
    entity: Entity,
    on_remove: bool,
) -> Option<Entity> {
    while let Some(parent_komorebi_type) = komorebi_type.parent() {
        if parent_komorebi_type == Relationship::PARENT
            || matches!(Relationship::PARENT, KomorebiType::NotDefined)
        {
            break;
        } else if matches!(parent_komorebi_type, KomorebiType::Monitor) {
            warn!(
                "{}Reached the top of the hierarchy, which is monitor. The child is maybe higher in the hierarchy.",
                caller
                    .map(|location| format!("{location}: "))
                    .unwrap_or_default(),
            );
            if on_remove {
                world.commands().entity(entity).remove::<Relationship>();
            }
            return None;
        }
        if let Some(parent_entity) = world.entity(target_entity).get::<KomorebiChildOf>() {
            target_entity = parent_entity.get();
        } else {
            warn!(
                "{}Reached the top of the hierarchy. for this relationship: {:?}",
                caller
                    .map(|location| format!("{location}: "))
                    .unwrap_or_default(),
                parent_komorebi_type
            );
            if on_remove {
                world.commands().entity(entity).remove::<Relationship>();
            }
            return None;
        }
    }
    Some(target_entity)
}
