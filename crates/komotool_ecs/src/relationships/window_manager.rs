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

        if let Some(target_relationship) = world.entity(entity).get::<Self>() {
            let window_manager_entity = target_relationship.get();
            if let Some(window_manager_children) = world.entity(window_manager_entity).get::<Self::RelationshipTarget>() {
                if let Some(monitor_index_in_manager_list) = window_manager_children.0.get_index_of(&entity) {
                    if let Some(monitor_marker_map_resource) = world.get_resource::<MonitorExtendedMarkerMap>() {
                        apply_monitor_markers_to_hierarchy(
                            world.reborrow(),
                            entity,
                            monitor_index_in_manager_list,
                            monitor_marker_map_resource,
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

// Hilfsfunktion zum rekursiven/kaskadierenden Setzen der Monitor-Marker-Komponenten
fn apply_monitor_markers_to_hierarchy(
    mut deferred_world: DeferredWorld,
    monitor_entity: Entity,
    monitor_index: usize,
    marker_map: &MonitorExtendedMarkerMap,
) {
    // Marker für die Monitor-Entität selbst setzen
    insert_monitor_marker_component(monitor_index, monitor_entity, deferred_world.commands(), marker_map);

    // Kinder des Monitors durchgehen (Workspaces)
    if let Some(monitor_children) = deferred_world.entity(monitor_entity).get::<MonitorChildren>() {
        for &workspace_entity in monitor_children.0.iter() {
            insert_monitor_marker_component(monitor_index, workspace_entity, deferred_world.commands(), marker_map);

            // Kinder des Workspaces durchgehen (Container)
            if let Some(workspace_children) = deferred_world.entity(workspace_entity).get::<WorkspaceChildren>() {
                for &container_entity in workspace_children.0.iter() {
                    insert_monitor_marker_component(monitor_index, container_entity, deferred_world.commands(), marker_map);

                    // Kinder des Containers durchgehen (Windows)
                    if let Some(container_children) = deferred_world.entity(container_entity).get::<ContainerChildren>() {
                        for &window_entity in container_children.0.iter() {
                            insert_monitor_marker_component(monitor_index, window_entity, deferred_world.commands(), marker_map);
                        }
                    }
                }
            }
        }
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
