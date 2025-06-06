use crate::components::{
    despawn_container_marker_component, insert_container_marker_component,
};
use crate::prelude::{get_old_index, remove_parent_markers_from_hierarchy};
use crate::relationships::window_manager::WindowManagerChildOf;
use crate::relationships::{
    GetIndex, InsertMarkerFn, KomotoolRelationship, MonitorChildOf, 
    RelationshipIndexSet, apply_markers_to_container_hierarchy, apply_parent_markers_to_hierarchy,
    bevy_on_insert, bevy_on_remove, relationships_hook,
};
use crate::resources::ContainerExtendedMarkerMap;
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
                    let mut default_map = None;

                    crate::relationships::apply_markers_to_container_hierarchy(
                            world.reborrow(),
                            entity,
                            container_idx_in_workspace_list,
                            marker_map_clone.as_ref().unwrap_or_else(||{
                                warn!(
                            "Failed to get ContainerExtendedMarkerMap. Markers over the default threshold will not be applied."
                        );
                                default_map.get_or_insert_with(ContainerExtendedMarkerMap::default)
                            }),
                            insert_container_marker_component,
                        );
                    if let Some(parent_monitor_entity) =
                        apply_parent_markers_to_hierarchy::<MonitorChildOf>(
                            entity,
                            parent_workspace_entity,
                            world.reborrow(),
                            apply_markers_to_container_hierarchy,
                        )
                    {
                        apply_parent_markers_to_hierarchy::<WindowManagerChildOf>(
                            entity,
                            parent_monitor_entity,
                            world.reborrow(),
                            apply_markers_to_container_hierarchy,
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

        let old_idx = get_old_index::<Self>(entity, world.reborrow());

        bevy_on_remove::<Self>(
            world.reborrow(),
            HookContext {
                entity,
                caller,
                relationship_hook_mode,
                component_id,
            },
        );

        if let Some(old_idx) = old_idx {
            let marker_map_optional = world.get_resource::<ContainerExtendedMarkerMap>().cloned();
            let mut default_map = None;
            apply_markers_to_container_hierarchy(
                world.reborrow(),
                entity,
                old_idx,
                marker_map_optional.as_ref().unwrap_or_else(||{
                    warn!(
                        "Failed to get ContainerExtendedMarkerMap. Markers over the default threshold will not be applied."
                    );
                    default_map.get_or_insert_with(ContainerExtendedMarkerMap::default)
                }),
                insert_container_marker_component,
            );

            let parent_monitor_entity = remove_parent_markers_from_hierarchy::<MonitorChildOf>(
                entity,
                None,
                world.reborrow(),
                apply_markers_to_container_hierarchy,
            );
            remove_parent_markers_from_hierarchy::<WindowManagerChildOf>(
                entity,
                parent_monitor_entity,
                world.reborrow(),
                apply_markers_to_container_hierarchy,
            );
        }
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

impl KomotoolRelationship for WorkspaceChildOf {
    type Marker = ContainerExtendedMarkerMap;

    const INSERT_MARKER: InsertMarkerFn<ContainerExtendedMarkerMap> =
        insert_container_marker_component;

    const DESPAWN_MARKER: InsertMarkerFn<ContainerExtendedMarkerMap> =
        despawn_container_marker_component;
    type Komorebi = Container;
}
