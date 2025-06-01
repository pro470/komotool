use crate::components::{insert_container_marker_component, insert_monitor_marker_component, insert_workspace_marker_component};
use crate::relationships;
use crate::relationships::window_manager::{WindowManagerChildOf, WindowManagerChildren};
use crate::relationships::{GetIndex, MonitorChildOf, MonitorChildren, RelationshipIndexSet, apply_markers_to_monitor_hierarchy, apply_parent_markers_to_hierarchy, apply_markers_to_workspace_hierarchy, bevy_on_insert, bevy_on_remove, relationships_hook, apply_markers_to_container_hierarchy};
use crate::resources::{
    ContainerExtendedMarkerMap, MonitorExtendedMarkerMap, WorkspaceExtendedMarkerMap,
};
use bevy_ecs::component::{Component, HookContext};
use bevy_ecs::entity::Entity;
use bevy_ecs::relationship::{Relationship, RelationshipTarget};
use bevy_ecs::world::DeferredWorld;
use bevy_log::warn;
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

        // In WorkspaceChildOf::on_insert, `entity` ist die Container-Entität.
        // `target_relationship.get()` gibt die übergeordnete Workspace-Entität zurück.
        // `Self::RelationshipTarget` ist `WorkspaceChildren`.
        // `container_idx_in_workspace_list` ist der Index der `entity` (Container)
        // innerhalb der Kinderliste des Workspaces.
        if let Some(target_relationship) = world.entity(entity).get::<Self>() {
            let parent_workspace_entity = target_relationship.get();
            if let Some(workspace_children) = world
                .entity(parent_workspace_entity)
                .get::<Self::RelationshipTarget>()
            {
                if let Some(container_idx_in_workspace_list) =
                    workspace_children.0.get_index_of(&entity)
                {
                    // Klone die Ressourcen-Map, um die immutable Leihe von `world` aufzuheben.
                    let marker_map_clone =
                        world.get_resource::<ContainerExtendedMarkerMap>().cloned();

                    if let Some(cloned_map) = marker_map_clone {
                        // Rufe die neue Hilfsfunktion auf.
                        // `entity` (der Container) ist der Startpunkt dieser Hierarchie.
                        crate::relationships::apply_markers_to_container_hierarchy(
                            world.reborrow(),
                            entity,
                            container_idx_in_workspace_list,
                            &cloned_map,
                            insert_container_marker_component
                        );
                    } else {
                        warn!(
                            "Failed to get ContainerExtendedMarkerMap. Markers over the default threshold will not be applied."
                        );
                        apply_markers_to_container_hierarchy(
                            world.reborrow(),
                            entity,
                            container_idx_in_workspace_list,
                            &ContainerExtendedMarkerMap::default(),
                            insert_container_marker_component
                        );
                    }
                    if let Some(parent_monitor_entity) = apply_parent_markers_to_hierarchy::<
                        MonitorChildOf,
                        MonitorChildren,
                        WorkspaceExtendedMarkerMap,
                    >(
                        entity,
                        parent_workspace_entity,
                        world.reborrow(),
                        apply_markers_to_container_hierarchy,
                        insert_workspace_marker_component
                    ) {
                        apply_parent_markers_to_hierarchy::<
                            WindowManagerChildOf,
                            WindowManagerChildren,
                            MonitorExtendedMarkerMap,
                        >(
                            entity,
                            parent_monitor_entity,
                            world.reborrow(),
                            apply_markers_to_container_hierarchy,
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

impl GetIndex for WorkspaceChildren {
    fn get_index_of(&self, entity: &Entity) -> Option<usize> {
        self.0.get_index_of(entity)
    }
}
