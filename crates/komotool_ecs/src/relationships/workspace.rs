use crate::components::insert_container_marker_component;
use crate::relationships::{
    RelationshipIndexSet, bevy_on_insert, bevy_on_remove, relationships_hook,
};
use crate::resources::ContainerExtendedMarkerMap;
use bevy_ecs::component::{Component, HookContext};
use bevy_ecs::entity::Entity;
use bevy_ecs::relationship::{Relationship, RelationshipTarget};
use bevy_ecs::world::DeferredWorld;
use bevy_reflect::Reflect;
use komorebi_client::{Container, Workspace};

#[derive(Component, Reflect)]
#[component(immutable)]
pub struct WorkspaceChildOf(pub Entity);

#[derive(Component, Reflect)]
pub struct WorkspaceChildren(pub(crate) RelationshipIndexSet);

impl Relationship for WorkspaceChildOf {
    type RelationshipTarget = WorkspaceChildren;

    fn get(&self) -> Entity {
        self.0
    }

    fn from(entity: Entity) -> Self {
        WorkspaceChildOf(entity)
    }

    fn on_insert(
        mut world: DeferredWorld,
        HookContext {
            entity,
            relationship_hook_mode,
            caller,
            component_id,
        }: HookContext,
    ) {
        if bevy_on_insert::<Self, Container, Workspace>(
            world.reborrow(),
            HookContext {
                entity,
                relationship_hook_mode,
                caller,
                component_id,
            },
        ) {
            return;
        }

        if let Some(target) = world.entity(entity).get::<Self>() {
            if let Some(children) = world.entity(target.get()).get::<Self::RelationshipTarget>() {
                if let Some(index) = children.0.get_index_of(&entity) {
                    if let Some(marker) = world.get_resource::<ContainerExtendedMarkerMap>() {
                        let marker = marker.clone();
                        insert_container_marker_component(index, entity, world.commands(), &marker);
                    }
                }
            }
        }
    }

    fn on_replace(
        mut world: DeferredWorld,
        HookContext {
            entity,
            relationship_hook_mode,
            caller,
            component_id,
        }: HookContext,
    ) {
        if !relationships_hook::<Self>(relationship_hook_mode) {
            return;
        }

        bevy_on_remove::<Self>(
            world.reborrow(),
            HookContext {
                entity,
                caller,
                relationship_hook_mode,
                component_id,
            },
        );
    }
}

impl RelationshipTarget for WorkspaceChildren {
    const LINKED_SPAWN: bool = false;

    type Relationship = WorkspaceChildOf;

    type Collection = RelationshipIndexSet;

    fn collection(&self) -> &Self::Collection {
        &self.0
    }

    fn collection_mut_risky(&mut self) -> &mut Self::Collection {
        &mut self.0
    }

    fn from_collection_risky(collection: Self::Collection) -> Self {
        WorkspaceChildren(collection)
    }
}
