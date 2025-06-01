use crate::components::{insert_monitor_marker_component, insert_workspace_marker_component};
use crate::prelude::relationships_hook;
use crate::relationships;
use crate::relationships::window_manager::{WindowManagerChildOf, WindowManagerChildren};
use crate::relationships::{GetIndex, RelationshipIndexSet, apply_markers_to_monitor_hierarchy, apply_parent_markers_to_hierarchy, bevy_on_insert, bevy_on_remove, apply_markers_to_workspace_hierarchy};
use crate::resources::{MonitorExtendedMarkerMap, WorkspaceExtendedMarkerMap};
use bevy_ecs::component::{Component, HookContext};
use bevy_ecs::entity::Entity;
use bevy_ecs::relationship::{Relationship, RelationshipTarget};
use bevy_ecs::world::DeferredWorld;
use bevy_log::warn;
use bevy_reflect::Reflect;
use komorebi_client::{Monitor, Workspace};

#[derive(Component, Reflect)]
#[component(immutable)]
pub struct MonitorChildOf(pub Entity);

#[derive(Component, Reflect)]
pub struct MonitorChildren(pub(crate) RelationshipIndexSet);

impl Relationship for MonitorChildOf {
    type RelationshipTarget = MonitorChildren;

    fn get(&self) -> Entity {
        self.0
    }

    fn from(entity: Entity) -> Self {
        MonitorChildOf(entity)
    }

    /// The `on_insert` component hook that maintains the [`Relationship`] / [`RelationshipTarget`] connection.
    fn on_insert(
        mut world: DeferredWorld,
        HookContext {
            entity,
            caller,
            relationship_hook_mode,
            component_id,
        }: HookContext,
    ) {
        if bevy_on_insert::<Self, Workspace, Monitor>(
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

        // In MonitorChildOf::on_insert, `entity` ist die Workspace-Entität.
        // `target_relationship.get()` gibt die übergeordnete Monitor-Entität zurück.
        // `Self::RelationshipTarget` ist `MonitorChildren`.
        // `workspace_idx_in_monitor_list` ist der Index der `entity` (Workspace)
        // innerhalb der Kinderliste des Monitors.
        if let Some(target_relationship) = world.entity(entity).get::<Self>() {
            let parent_monitor_entity = target_relationship.get();
            if let Some(monitor_children) = world
                .entity(parent_monitor_entity)
                .get::<Self::RelationshipTarget>()
            {
                if let Some(workspace_idx_in_monitor_list) =
                    monitor_children.0.get_index_of(&entity)
                {
                    // Klone die Ressourcen-Map, um die immutable Leihe von `world` aufzuheben.
                    let marker_map_clone =
                        world.get_resource::<WorkspaceExtendedMarkerMap>().cloned();

                    if let Some(cloned_map) = marker_map_clone {
                        // Rufe die neue Hilfsfunktion auf.
                        // `entity` (der Workspace) ist der Startpunkt dieser Hierarchie.
                        apply_markers_to_workspace_hierarchy(
                            world.reborrow(),
                            entity,
                            workspace_idx_in_monitor_list,
                            &cloned_map,
                            insert_workspace_marker_component
                        );
                    } else {
                        warn!(
                            "Failed to get WorkspaceExtendedMarkerMap. Markers over the default threshold will not be applied."
                        );
                        apply_markers_to_workspace_hierarchy(
                            world.reborrow(),
                            entity,
                            workspace_idx_in_monitor_list,
                            &WorkspaceExtendedMarkerMap::default(),
                            insert_workspace_marker_component
                        );
                    }

                    apply_parent_markers_to_hierarchy::<
                        WindowManagerChildOf,
                        WindowManagerChildren,
                        MonitorExtendedMarkerMap,
                    >(
                        entity,
                        parent_monitor_entity,
                        world.reborrow(),
                        apply_markers_to_workspace_hierarchy,
                        insert_monitor_marker_component
                    );
                }
            }
        }
    }

    /// The `on_replace` component hook that maintains the [`Relationship`] / [`RelationshipTarget`] connection.
    // note: think of this as "on_drop"
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

impl RelationshipTarget for MonitorChildren {
    const LINKED_SPAWN: bool = false;
    type Relationship = MonitorChildOf;

    type Collection = RelationshipIndexSet;

    fn collection(&self) -> &Self::Collection {
        &self.0
    }

    fn collection_mut_risky(&mut self) -> &mut Self::Collection {
        &mut self.0
    }

    fn from_collection_risky(collection: Self::Collection) -> Self {
        MonitorChildren(collection)
    }
}

impl GetIndex for MonitorChildren {
    fn get_index_of(&self, entity: &Entity) -> Option<usize> {
        self.0.get_index_of(entity)
    }
}
