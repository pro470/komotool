use super::{
    ContainerChildren, MonitorChildren, RelationshipIndexSet, bevy_on_insert, bevy_on_remove,
    relationships_hook,
};
use crate::components::{WindowManager, insert_monitor_marker_component};
use crate::prelude::WorkspaceChildren;
use crate::resources::MonitorExtendedMarkerMap;
use bevy_ecs::component::{Component, HookContext};
use bevy_ecs::entity::Entity;
use bevy_ecs::relationship::{Relationship, RelationshipTarget};
use bevy_ecs::world::DeferredWorld;
use bevy_reflect::Reflect;
use komorebi_client::Monitor;

#[derive(Component)]
#[component(immutable)]
pub struct WindowManagerChildOf(pub Entity);

#[derive(Component, Reflect)]
pub struct WindowManagerChildren(RelationshipIndexSet);

impl Relationship for WindowManagerChildOf {
    type RelationshipTarget = WindowManagerChildren;

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
        if bevy_on_insert::<Self, Monitor, WindowManager>(
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
            if let Some(children) = world.entity(target.get()).get::<Self::RelationshipTarget>() {
                if let Some(index) = children.0.get_index_of(&entity) {
                    if let Some(marker) = world.get_resource::<MonitorExtendedMarkerMap>() {
                        let marker = marker.clone();
                        insert_monitor_marker_component(index, entity, world.commands(), &marker);
                        if let Some(monitor_children) =
                            world.entity(entity).get::<MonitorChildren>()
                        {
                            for monitor_child in monitor_children.0.clone().iter() {
                                insert_monitor_marker_component(
                                    index,
                                    *monitor_child,
                                    world.commands(),
                                    &marker,
                                );
                                if let Some(workspace_children) =
                                    world.entity(*monitor_child).get::<WorkspaceChildren>()
                                {
                                    for workspace_child in workspace_children.0.clone().iter() {
                                        insert_monitor_marker_component(
                                            index,
                                            *workspace_child,
                                            world.commands(),
                                            &marker,
                                        );
                                        if let Some(container_children) = world
                                            .entity(*workspace_child)
                                            .get::<ContainerChildren>()
                                        {
                                            for container_child in
                                                container_children.0.clone().iter()
                                            {
                                                insert_monitor_marker_component(
                                                    index,
                                                    *container_child,
                                                    world.commands(),
                                                    &marker,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
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

impl RelationshipTarget for WindowManagerChildren {
    const LINKED_SPAWN: bool = false;

    type Relationship = WindowManagerChildOf;

    type Collection = RelationshipIndexSet;

    fn collection(&self) -> &Self::Collection {
        &self.0
    }

    fn collection_mut_risky(&mut self) -> &mut Self::Collection {
        &mut self.0
    }

    fn from_collection_risky(collection: Self::Collection) -> Self {
        WindowManagerChildren(collection)
    }
}
