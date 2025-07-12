use crate::components::Focused;
use crate::prelude::KomorebiType;
use crate::relationships::komorebi_one_to_one_relationship::{
    KomorebiOneToOneRelationship, KomorebiOneToOneRelationshipTarget,
    komorebi_oto_relationship_on_insert, komorebi_oto_relationship_on_remove,
    komorebi_oto_relationship_target_on_replace,
};
use bevy_ecs::component::{Component, HookContext};
use bevy_ecs::entity::Entity;
use bevy_ecs::world::DeferredWorld;
use bevy_log::info;
use bevy_reflect::Reflect;

impl KomorebiOneToOneRelationship for Focused {
    type Komorebionetoonerelationshiptarget = FocusTarget;

    const CHILD: KomorebiType = KomorebiType::NotDefined;

    const PARENT: KomorebiType = KomorebiType::NotDefined;

    fn on_insert(
        world: DeferredWorld,
        HookContext {
            entity,
            caller,
            relationship_hook_mode,
            component_id,
        }: HookContext,
    ) {
        komorebi_oto_relationship_on_insert::<Self>(
            world,
            HookContext {
                entity,
                caller,
                relationship_hook_mode,
                component_id,
            },
        );
    }

    fn on_remove(
        world: DeferredWorld,
        HookContext {
            entity,
            caller,
            relationship_hook_mode,
            component_id,
        }: HookContext,
    ) {
        komorebi_oto_relationship_on_remove::<Self>(
            world,
            HookContext {
                entity,
                caller,
                relationship_hook_mode,
                component_id,
            },
        );
    }
}

#[derive(Component, Reflect)]
pub struct FocusTarget {
    entity: Entity,
    index: usize,
}

impl KomorebiOneToOneRelationshipTarget for FocusTarget {
    type Komorebionetoonerelationship = Focused;

    fn set_entity(&mut self, entity: Entity) {
        self.entity = entity;
    }

    fn get_entity(&self) -> Entity {
        self.entity
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn from(entity: Entity, index: usize) -> Self {
        Self { entity, index }
    }

    fn on_replace(
        world: DeferredWorld,
        HookContext {
            entity,
            caller,
            relationship_hook_mode,
            component_id,
        }: HookContext,
    ) {
        komorebi_oto_relationship_target_on_replace::<Self>(
            world,
            HookContext {
                entity,
                caller,
                relationship_hook_mode,
                component_id,
            },
        );
    }
}
