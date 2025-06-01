use crate::components::{insert_container_marker_component, insert_monitor_marker_component, insert_window_marker_component, insert_workspace_marker_component};
use crate::relationships::{GetIndex, RelationshipIndexSet, bevy_on_insert, bevy_on_remove, relationships_hook, apply_parent_markers_to_hierarchy, WorkspaceChildOf, WorkspaceChildren, MonitorChildOf, MonitorChildren};
use crate::resources::{ContainerExtendedMarkerMap, MonitorExtendedMarkerMap, WindowExtendedMarkerMap, WorkspaceExtendedMarkerMap};
use bevy_ecs::component::{Component, HookContext};
use bevy_ecs::entity::Entity;
use bevy_ecs::relationship::{Relationship, RelationshipTarget};
use bevy_ecs::world::DeferredWorld;
use bevy_reflect::Reflect;
use komorebi_client::{Container, Window};
use crate::relationships::window_manager::{WindowManagerChildOf, WindowManagerChildren};

#[derive(Component, Reflect)]
#[component(immutable)]
pub struct ContainerChildOf(pub Entity);

#[derive(Component, Reflect)]
pub struct ContainerChildren(pub(crate) RelationshipIndexSet);

impl Relationship for ContainerChildOf {
    type RelationshipTarget = ContainerChildren;

    fn get(&self) -> Entity {
        self.0
    }

    fn from(entity: Entity) -> Self {
        Self(entity)
    }

    fn on_insert(
        mut world: DeferredWorld,
        HookContext {
            entity,
            caller,
            relationship_hook_mode,
            component_id,
        }: HookContext,
    ) {
        if bevy_on_insert::<Self, Window, Container>(
            world.reborrow(),
            HookContext {
                entity,
                caller,
                relationship_hook_mode,
                component_id,
            },
        ) {
            return;
        }

        if let Some(target) = world.entity(entity).get::<Self>() {
            let parent_container_entity = target.get();
            if let Some(children) = world.entity(target.get()).get::<Self::RelationshipTarget>() {
                if let Some(index) = children.0.get_index_of(&entity) {
                    if let Some(marker) = world.get_resource::<WindowExtendedMarkerMap>() {
                        let marker = marker.clone();
                        insert_window_marker_component(index, entity, world.commands(), &marker);
                    } else {
                        insert_window_marker_component(
                            index,
                            entity,
                            world.commands(),
                            &WindowExtendedMarkerMap::default(),
                        );
                    }
                    if let Some(parent_workspace_entity) = apply_parent_markers_to_hierarchy::<WorkspaceChildOf, WorkspaceChildren, ContainerExtendedMarkerMap>(entity, parent_container_entity, world.reborrow(), |mut world, entity, index, marker, insert_marker| {

                        insert_marker(
                            index,
                            entity,
                            world.commands(),
                            marker,
                        )

                    }, insert_container_marker_component) {

                        if let Some(parent_monitor_entity) = apply_parent_markers_to_hierarchy::<MonitorChildOf, MonitorChildren, WorkspaceExtendedMarkerMap>(
                            entity,
                            parent_workspace_entity,
                            world.reborrow(),
                            |mut world, entity, index, marker, insert_marker| {

                                insert_marker(
                                    index,
                                    entity,
                                    world.commands(),
                                    marker,
                                )

                            }, insert_workspace_marker_component
                        ) {

                            apply_parent_markers_to_hierarchy::<WindowManagerChildOf, WindowManagerChildren, MonitorExtendedMarkerMap>(
                                entity,
                                parent_monitor_entity,
                                world.reborrow(),
                                |mut world, entity, index, marker, insert_marker| {

                                    insert_marker(
                                        index,
                                        entity,
                                        world.commands(),
                                        marker,
                                    )

                                }, insert_monitor_marker_component
                            );


                        }

                    }
                }
            }
        }
    }

    fn on_replace(
        mut world: DeferredWorld,
        HookContext {
            entity,
            caller,
            relationship_hook_mode,
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

impl RelationshipTarget for ContainerChildren {
    const LINKED_SPAWN: bool = false;

    type Relationship = ContainerChildOf;

    type Collection = RelationshipIndexSet;

    fn collection(&self) -> &Self::Collection {
        &self.0
    }

    fn collection_mut_risky(&mut self) -> &mut Self::Collection {
        &mut self.0
    }

    fn from_collection_risky(collection: Self::Collection) -> Self {
        Self(collection)
    }
}

impl GetIndex for ContainerChildren {
    fn get_index_of(&self, entity: &Entity) -> Option<usize> {
        self.0.get_index_of(entity)
    }
}
