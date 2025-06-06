use super::{
    GetIndex, KomotoolRelationship, RelationshipIndexSet, apply_markers_to_monitor_hierarchy,
    bevy_on_insert, bevy_on_remove, get_old_index, relationships_hook, update_markers,
};
use crate::components::{
    WindowManager, despawn_monitor_marker_component, insert_monitor_marker_component,
};
use crate::prelude::InsertMarkerFn;
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
                    let mut default_map = None;
                    relationships::apply_markers_to_monitor_hierarchy(
                            world.reborrow(),
                            entity,
                            monitor_index_in_manager_list,
                            marker_map_clone.as_ref().unwrap_or_else(|| {
                                warn!(
                            "Failed to get MonitorExtendedMarkerMap. Markers over the default threeshold will not be applied."
                        );

                                default_map.get_or_insert_with(MonitorExtendedMarkerMap::default)

                            }),
                            insert_monitor_marker_component,
                        );
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
            let marker_map_optional = world.get_resource::<MonitorExtendedMarkerMap>().cloned();
            let mut default_map = None;
            apply_markers_to_monitor_hierarchy(
                world.reborrow(),
                entity,
                old_idx,
                marker_map_optional.as_ref().unwrap_or_else(||{
                    warn!("Failed to get MonitorExtendedMarkerMap. Markers over the default threshold will not be applied.");
                    default_map.get_or_insert_with(MonitorExtendedMarkerMap::default)
                }),
                despawn_monitor_marker_component,
            );

            update_markers::<Self>(
                world.reborrow(),
                marker_map_optional,
                entity,
                old_idx,
                apply_markers_to_monitor_hierarchy,
            );
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

impl GetIndex for WindowManagerChildren {
    fn get_index_of(&self, entity: &Entity) -> Option<usize> {
        self.0.get_index_of(entity)
    }
}

impl KomotoolRelationship for WindowManagerChildOf {
    type Marker = MonitorExtendedMarkerMap;

    const INSERT_MARKER: InsertMarkerFn<MonitorExtendedMarkerMap> = insert_monitor_marker_component;

    const DESPAWN_MARKER: InsertMarkerFn<MonitorExtendedMarkerMap> =
        despawn_monitor_marker_component;

    type Komorebi = Monitor;
}
