use crate::components::{
    despawn_window_marker_component, insert_container_marker_component,
    insert_monitor_marker_component, insert_window_marker_component,
    insert_workspace_marker_component,
};
use crate::prelude::{
    DespawnInsertMarker, get_old_index, remove_parent_markers_from_hierarchy, update_markers,
};
use crate::relationships;
use crate::relationships::window_manager::{WindowManagerChildOf, WindowManagerChildren};
use crate::relationships::{
    GetIndex, InsertMarkerFn, KomotoolRelationship, MarkerFn, MonitorChildOf, MonitorChildren,
    RelationshipIndexSet, WorkspaceChildOf, WorkspaceChildren, apply_parent_markers_to_hierarchy,
    bevy_on_insert, bevy_on_remove, relationships_hook,
};
use crate::resources::{
    ContainerExtendedMarkerMap, MonitorExtendedMarkerMap, WindowExtendedMarkerMap,
    WorkspaceExtendedMarkerMap,
};
use bevy_ecs::component::{Component, HookContext};
use bevy_ecs::entity::Entity;
use bevy_ecs::relationship::{Relationship, RelationshipTarget};
use bevy_ecs::system::Commands;
use bevy_ecs::world::DeferredWorld;
use bevy_reflect::Reflect;
use komorebi_client::{Container, Window};

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
                    if let Some(parent_workspace_entity) =
                        apply_parent_markers_to_hierarchy::<WorkspaceChildOf>(
                            entity,
                            parent_container_entity,
                            world.reborrow(),
                            |mut world: DeferredWorld<'_>,
                             entity,
                             index,
                             marker: &_,
                             insert_marker: InsertMarkerFn<
                                <WorkspaceChildOf as KomotoolRelationship>::Marker,
                            >| {
                                insert_marker.marker(index, entity, world.commands(), marker)
                            },
                        )
                    {
                        if let Some(parent_monitor_entity) =
                            apply_parent_markers_to_hierarchy::<MonitorChildOf>(
                                entity,
                                parent_workspace_entity,
                                world.reborrow(),
                                |mut world: DeferredWorld<'_>,
                                 entity,
                                 index,
                                 marker: &_,
                                 insert_marker: InsertMarkerFn<
                                    <MonitorChildOf as KomotoolRelationship>::Marker,
                                >| {
                                    insert_marker.marker(index, entity, world.commands(), marker)
                                },
                            )
                        {
                            apply_parent_markers_to_hierarchy::<WindowManagerChildOf>(
                                entity,
                                parent_monitor_entity,
                                world.reborrow(),
                                |mut world: DeferredWorld<'_>,
                                 entity,
                                 index,
                                 marker: &_,
                                 insert_marker: InsertMarkerFn<
                                    <WindowManagerChildOf as KomotoolRelationship>::Marker,
                                >| {
                                    insert_marker.marker(index, entity, world.commands(), marker)
                                },
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
            let marker_map_optional = world.get_resource::<WindowExtendedMarkerMap>().cloned();
            despawn_window_marker_component(
                old_idx,
                entity,
                world.commands(),
                marker_map_optional
                    .as_ref()
                    .unwrap_or(&WindowExtendedMarkerMap::default()),
            );

            let parent_workspace_entity = remove_parent_markers_from_hierarchy::<WorkspaceChildOf>(
                entity,
                None,
                world.reborrow(),
                |mut world: DeferredWorld<'_>,
                 entity,
                 index,
                 marker: &_,
                 insert_marker: InsertMarkerFn<
                    <WorkspaceChildOf as KomotoolRelationship>::Marker,
                >| {
                    insert_marker.marker(index, entity, world.commands(), marker)
                },
            );

            let parent_monitor_entity = remove_parent_markers_from_hierarchy::<MonitorChildOf>(
                entity,
                parent_workspace_entity,
                world.reborrow(),
                |mut world: DeferredWorld<'_>,
                 entity,
                 index,
                 marker: &_,
                 insert_marker: InsertMarkerFn<
                    <MonitorChildOf as KomotoolRelationship>::Marker,
                >| {
                    insert_marker.marker(index, entity, world.commands(), marker)
                },
            );
            remove_parent_markers_from_hierarchy::<WindowManagerChildOf>(
                entity,
                parent_monitor_entity,
                world.reborrow(),
                |mut world: DeferredWorld<'_>,
                 entity,
                 index,
                 marker: &_,
                 insert_marker: InsertMarkerFn<
                    <WindowManagerChildOf as KomotoolRelationship>::Marker,
                >| {
                    insert_marker.marker(index, entity, world.commands(), marker)
                },
            );

            update_markers::<Self>(
                world.reborrow(),
                marker_map_optional,
                entity,
                old_idx,
                |mut world: DeferredWorld<'_>,
                 entity,
                 index,
                 marker: &_,
                 insert_marker: DespawnInsertMarker<
                    <ContainerChildOf as KomotoolRelationship>::Marker,
                >| {
                    insert_marker.marker(index, entity, world.commands(), marker)
                },
            )
        }
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

impl KomotoolRelationship for ContainerChildOf {
    type Marker = WindowExtendedMarkerMap;

    const INSERT_MARKER: InsertMarkerFn<WindowExtendedMarkerMap> = insert_window_marker_component;

    const DESPAWN_MARKER: InsertMarkerFn<WindowExtendedMarkerMap> = despawn_window_marker_component;

    type Komorebi = Window;
}
