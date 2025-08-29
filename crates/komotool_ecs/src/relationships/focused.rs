use crate::components::Focused;
use crate::prelude::{KomorebiChildOf, KomorebiType};
use crate::relationships::KomorebiObserver;
use crate::relationships::komorebi_one_to_one_relationship::{
    KomorebiOneToOneRelationship, KomorebiOneToOneRelationshipTarget,
    komorebi_oto_relationship_on_insert, komorebi_oto_relationship_on_remove,
    komorebi_oto_relationship_target_on_replace,
};
use bevy_ecs::component::{Component, HookContext};
use bevy_ecs::entity::Entity;
use bevy_ecs::event::Event;
use bevy_ecs::observer::Trigger;
use bevy_ecs::relationship::Relationship;
use bevy_ecs::resource::Resource;
use bevy_ecs::system::{Commands, Query};
use bevy_ecs::world::DeferredWorld;
use bevy_log::{info, warn};
use bevy_reflect::Reflect;
use std::ops::{Deref, DerefMut};

impl KomorebiOneToOneRelationship for Focused {
    type Komorebionetoonerelationshiptarget = FocusTarget;

    const CHILD: KomorebiType = KomorebiType::NotDefined;

    const PARENT: KomorebiType = KomorebiType::NotDefined;

    fn on_insert(
        mut world: DeferredWorld,
        HookContext {
            entity,
            caller,
            relationship_hook_mode,
            component_id,
        }: HookContext,
    ) {
        world.commands().trigger(FocusOnInsert {
            hook_context: HookContext {
                entity,
                caller,
                relationship_hook_mode,
                component_id,
            },
        });
    }

    fn on_remove(
        mut world: DeferredWorld,
        HookContext {
            entity,
            caller,
            relationship_hook_mode,
            component_id,
        }: HookContext,
    ) {
        world.commands().trigger(FocusOnRemove {
            hook_context: HookContext {
                entity,
                caller,
                relationship_hook_mode,
                component_id,
            },
        });
    }
}

#[derive(Component, Reflect)]
pub struct FocusTarget {
    entity: Entity,
    idx: usize,
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
        self.idx = index;
    }

    fn get_index(&self) -> usize {
        self.idx
    }

    fn from(entity: Entity, index: usize) -> Self {
        Self { entity, idx: index }
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

#[derive(Component)]
pub struct OldFocusTarget(pub(crate) Entity);

impl OldFocusTarget {
    pub fn get_entity(&self) -> Entity {
        self.0
    }
}

pub fn focus_target_on_insert<EventTrigger: KomorebiObserver>(
    trigger: Trigger<EventTrigger>,
    focused: Query<&Focused>,
    old_focus_target: Query<&OldFocusTarget>,
    child_of: Query<&EventTrigger::ChildOf>,
    mut commands: Commands,
) {
    let entity = trigger.event().hook_context().entity;
    if let Ok(old_focus_target) = old_focus_target.get(entity) {
        if focused.contains(entity) {
            let old_focus_target = old_focus_target.get_entity();
            if let Ok(komorebi_child_of) = child_of.get(entity) {
                let target = komorebi_child_of.get();
                if target != old_focus_target {
                    commands.entity(old_focus_target).remove::<FocusTarget>();
                } else {
                    commands.entity(entity).insert(Focused);
                }
            }
        }
        commands.entity(entity).remove::<OldFocusTarget>();
    }
}
pub fn focus_target_on_replace<EventTrigger: KomorebiObserver>(
    trigger: Trigger<EventTrigger>,
    focused: Query<&Focused>,
    child_of: Query<&EventTrigger::ChildOf>,
    mut commands: Commands,
) {
    let entity = trigger.event().hook_context().entity;

    if focused.contains(entity) {
        if let Ok(komorebi_child_of) = child_of.get(entity) {
            let old_focus_entity = komorebi_child_of.get();
            commands
                .entity(entity)
                .insert(OldFocusTarget(old_focus_entity));
        } else {
            warn!("Entity has no KomorebiChildOf. Focused will be removed.");
            commands.entity(entity).remove::<Focused>();
        }
    }
}

#[derive(Resource, Reflect)]
pub struct AutoManagedFocus(bool);

impl Default for AutoManagedFocus {
    fn default() -> Self {
        Self(true)
    }
}

impl Deref for AutoManagedFocus {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AutoManagedFocus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Event)]
pub struct FocusOnInsert {
    pub hook_context: HookContext,
}

#[derive(Event)]
pub struct FocusOnRemove {
    pub hook_context: HookContext,
}

pub fn komorebi_oto_relationship_on_insert_event(
    trigger: Trigger<FocusOnInsert>,
    world: DeferredWorld,
) {
    komorebi_oto_relationship_on_insert::<Focused>(world, trigger.event().hook_context);
}

pub fn komorebi_oto_relationship_on_remove_event(
    trigger: Trigger<FocusOnRemove>,
    world: DeferredWorld,
) {
    komorebi_oto_relationship_on_remove::<Focused>(world, trigger.event().hook_context);
}
