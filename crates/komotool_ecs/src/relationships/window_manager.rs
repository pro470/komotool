use super::{
    ContainerChildren, GetIndex, MonitorChildren, RelationshipIndexSet, bevy_on_insert,
    bevy_on_remove, relationships_hook,
};
use crate::components::{WindowManager, insert_monitor_marker_component};
use crate::prelude::WorkspaceChildren;
use crate::relationships;
use crate::resources::MonitorExtendedMarkerMap;
use bevy_ecs::component::{Component, HookContext};
use bevy_ecs::entity::Entity;
use bevy_ecs::relationship::{Relationship, RelationshipTarget};
use bevy_ecs::world::DeferredWorld;
use bevy_log::warn;
use bevy_reflect::Reflect;
use komorebi_client::Monitor;

#[derive(Component)]
#[component(immutable)]
pub struct WindowManagerChildOf(pub Entity);

#[derive(Component, Reflect)]
pub struct WindowManagerChildren(pub(crate) RelationshipIndexSet);

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

        if let Some(target_relationship) = world.entity(entity).get::<Self>() {
            let window_manager_entity = target_relationship.get();
            if let Some(window_manager_children) = world
                .entity(window_manager_entity)
                .get::<Self::RelationshipTarget>()
            {
                if let Some(monitor_index_in_manager_list) =
                    window_manager_children.0.get_index_of(&entity)
                {
                    let marker_map_clone =
                        world.get_resource::<MonitorExtendedMarkerMap>().cloned();
                    if let Some(cloned_map) = marker_map_clone {
                        relationships::apply_markers_to_monitor_hierarchy(
                            world.reborrow(),
                            entity,
                            monitor_index_in_manager_list,
                            &cloned_map,
                            insert_monitor_marker_component
                        );
                    } else {
                        warn!(
                            "Failed to get MonitorExtendedMarkerMap. Markers over the default threeshold will not be applied."
                        );
                        relationships::apply_markers_to_monitor_hierarchy(
                            world.reborrow(),
                            entity,
                            monitor_index_in_manager_list,
                            &MonitorExtendedMarkerMap::default(),
                            insert_monitor_marker_component
                        );
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

impl GetIndex for WindowManagerChildren {
    fn get_index_of(&self, entity: &Entity) -> Option<usize> {
        self.0.get_index_of(entity)
    }
}
