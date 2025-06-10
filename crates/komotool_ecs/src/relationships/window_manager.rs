use super::{
    ContainsParentChild, GetIndex, KomotoolRelationship, MonitorChildOf, RelationshipIndexSet,
    apply_markers_to_monitor_hierarchy, bevy_on_insert, bevy_on_remove, get_old_index,
    relationships_hook, update_markers,
};
use crate::components::{
    WindowManager, despawn_monitor_marker_component, insert_monitor_marker_component,
};
use crate::prelude::InsertMarkerFn;
use crate::relationships;
use crate::resources::MonitorExtendedMarkerMap;
use bevy_ecs::component::HookContext;
use bevy_ecs::entity::Entity;
use bevy_ecs::relationship::{Relationship, RelationshipTarget};
use bevy_ecs::world::DeferredWorld;
use bevy_log::warn;
use bevy_reflect::Reflect;
use komorebi_client::Monitor;

pub struct WindowManagerChildOf(pub Entity);

impl bevy_ecs::component::Component for WindowManagerChildOf
where
    Self: Send + Sync + 'static,
{
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
    type Mutability = bevy_ecs::component::Immutable;
    fn on_insert() -> ::core::option::Option<bevy_ecs::component::ComponentHook> {
        ::core::option::Option::Some(<Self as bevy_ecs::relationship::Relationship>::on_insert)
    }
    fn on_replace() -> ::core::option::Option<bevy_ecs::component::ComponentHook> {
        ::core::option::Option::Some(<Self as bevy_ecs::relationship::Relationship>::on_replace)
    }

    fn register_required_components(
        requiree: bevy_ecs::component::ComponentId,
        components: &mut bevy_ecs::component::ComponentsRegistrator,
        required_components: &mut bevy_ecs::component::RequiredComponents,
        inheritance_depth: u16,
        recursion_check_stack: &mut bevy_ecs::__macro_exports::Vec<
            bevy_ecs::component::ComponentId,
        >,
    ) {
        bevy_ecs::component::enforce_no_required_components_recursion(
            components,
            recursion_check_stack,
        );
        let self_id = components.register_component::<Self>();
        recursion_check_stack.push(self_id);
        recursion_check_stack.pop();
    }
    fn clone_behavior() -> bevy_ecs::component::ComponentCloneBehavior {
        use bevy_ecs::component::{DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone};
        (&&&bevy_ecs::component::DefaultCloneBehaviorSpecialization::<Self>::default())
            .default_clone_behavior()
    }
    fn map_entities<M: bevy_ecs::entity::EntityMapper>(this: &mut Self, mapper: &mut M) {
        use bevy_ecs::entity::MapEntities;
        this.0.map_entities(mapper);
    }
}
#[derive(Reflect)]
pub struct WindowManagerChildren(pub(crate) RelationshipIndexSet);

impl bevy_ecs::component::Component for WindowManagerChildren
where
    Self: Send + Sync + 'static,
{
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
    type Mutability = bevy_ecs::component::Mutable;
    fn register_required_components(
        requiree: bevy_ecs::component::ComponentId,
        components: &mut bevy_ecs::component::ComponentsRegistrator,
        required_components: &mut bevy_ecs::component::RequiredComponents,
        inheritance_depth: u16,
        recursion_check_stack: &mut bevy_ecs::__macro_exports::Vec<
            bevy_ecs::component::ComponentId,
        >,
    ) {
        bevy_ecs::component::enforce_no_required_components_recursion(
            components,
            recursion_check_stack,
        );
        let self_id = components.register_component::<Self>();
        recursion_check_stack.push(self_id);
        recursion_check_stack.pop();
    }

    fn on_replace() -> ::core::option::Option<bevy_ecs::component::ComponentHook> {
        ::core::option::Option::Some(
            <Self as bevy_ecs::relationship::RelationshipTarget>::on_replace,
        )
    }
    fn clone_behavior() -> bevy_ecs::component::ComponentCloneBehavior {
        bevy_ecs::component::ComponentCloneBehavior::Custom(
            bevy_ecs::relationship::clone_relationship_target::<Self>,
        )
    }
}

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
            ContainsParentChild,
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
                            monitor_index_in_manager_list + 1,
                            marker_map_clone.as_ref().unwrap_or_else(|| {
                                warn!(
                            "Failed to get MonitorExtendedMarkerMap. Markers over the default threeshold will not be applied."
                        );

                                default_map.get_or_insert_with(MonitorExtendedMarkerMap::default)

                            }),
                            insert_monitor_marker_component,
                        );
                } else {
                    warn!("Failed to get monitor index in manager list");
                }
            } else {
                warn!(
                    "Failed to get window manager children. It has to be the first child of the window manager."
                );

                let marker_map_clone = world.get_resource::<MonitorExtendedMarkerMap>().cloned();
                let mut default_map = None;
                relationships::apply_markers_to_monitor_hierarchy(
                    world.reborrow(),
                    entity,
                    1,
                    marker_map_clone.as_ref().unwrap_or_else(|| {
                        warn!(
                            "Failed to get MonitorExtendedMarkerMap. Markers over the default threeshold will not be applied."
                        );

                        default_map.get_or_insert_with(MonitorExtendedMarkerMap::default)

                    }),
                    insert_monitor_marker_component,
                );
            }
        } else {
            warn!("Failed to get target relationship");
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
                old_idx + 1,
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

    type Child = MonitorChildOf;
}
