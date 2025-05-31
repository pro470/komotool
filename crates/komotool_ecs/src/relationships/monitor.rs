use crate::components::insert_workspace_marker_component;
use crate::prelude::relationships_hook;
use crate::relationships::{RelationshipIndexSet, bevy_on_insert, bevy_on_remove};
use crate::resources::WorkspaceExtendedMarkerMap;
use bevy_ecs::component::{Component, HookContext};
use bevy_ecs::entity::Entity;
use bevy_ecs::relationship::{Relationship, RelationshipTarget};
use bevy_ecs::world::DeferredWorld;
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
            if let Some(monitor_children) = world.entity(parent_monitor_entity).get::<Self::RelationshipTarget>() {
                if let Some(workspace_idx_in_monitor_list) = monitor_children.0.get_index_of(&entity) {
                    // Klone die Ressourcen-Map, um die immutable Leihe von `world` aufzuheben.
                    let marker_map_clone = world.get_resource::<WorkspaceExtendedMarkerMap>().cloned();

                    if let Some(cloned_map) = marker_map_clone {
                        // Rufe die neue Hilfsfunktion auf.
                        // `entity` (der Workspace) ist der Startpunkt dieser Hierarchie.
                        crate::relationships::apply_workspace_markers_to_hierarchy(
                            world.reborrow(),
                            entity,
                            workspace_idx_in_monitor_list,
                            &cloned_map,
                        );
                    } else {
                        warn!("Failed to get WorkspaceExtendedMarkerMap. Markers over the default threshold will not be applied.");
                        crate::relationships::apply_workspace_markers_to_hierarchy(
                            world.reborrow(),
                            entity,
                            workspace_idx_in_monitor_list,
                            &WorkspaceExtendedMarkerMap::default(),
                        );
                    }
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
