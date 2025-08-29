use crate::components::{despawn_window_marker_component, insert_window_marker_component};
use crate::prelude::{
    OldIndex, get_old_index, remove_parent_markers_from_hierarchy, update_markers,
};
use crate::relationships::window_manager::WindowManagerChildOf;
use crate::relationships::{
    ContainsParentChild, GetIndex, HierarchyFnType, InsertMarkerFn, KomotoolRelationship, MarkerFn,
    MonitorChildOf, RelationshipIndexSet, WorkspaceChildOf, apply_parent_markers_to_hierarchy,
    bevy_on_insert, bevy_on_remove, komotool_on_insert, relationships_hook, remove_all_markers,
    to_hierarchy_with_marker,
};
use crate::resources::WindowExtendedMarkerMap;
use bevy_ecs::component::HookContext;
use bevy_ecs::entity::Entity;
use bevy_ecs::relationship::{Relationship, RelationshipTarget};
use bevy_ecs::world::DeferredWorld;
use bevy_log::warn;
use bevy_reflect::Reflect;
use komorebi_client::{Container, Window};

#[derive(Reflect)]
pub struct ContainerChildOf(pub Entity);

impl bevy_ecs::component::Component for ContainerChildOf
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
        bevy_ecs::component::DefaultCloneBehaviorSpecialization::<Self>::default()
            .default_clone_behavior()
    }
}

#[derive(Reflect)]
pub struct ContainerChildren(pub(crate) RelationshipIndexSet);

impl bevy_ecs::component::Component for ContainerChildren
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
        if bevy_on_insert::<Self>(
            world.reborrow(),
            HookContext {
                entity,
                caller,
                relationship_hook_mode,
                component_id,
            },
            ContainsParentChild::<Window, Container> {
                _phantom: std::marker::PhantomData,
            },
        ) {
            return;
        }

        komotool_on_insert::<Self>(
            entity,
            world.reborrow(),
            |mut world, entity, parent_container_entity, parent_idx| {
                to_hierarchy_with_marker(
                    entity,
                    world.reborrow(),
                    &Self::HIERARCHY,
                    Self::INSERT_MARKER,
                    parent_idx,
                );

                if let Some(parent_workspace_entity) =
                    apply_parent_markers_to_hierarchy::<WorkspaceChildOf>(
                        entity,
                        parent_container_entity,
                        world.reborrow(),
                        |mut world: DeferredWorld<'_>,
                         entity,
                         index,
                         marker: &_,
                         insert_marker: &dyn MarkerFn<
                            <WorkspaceChildOf as KomotoolRelationship>::Marker,
                        >| {
                            insert_marker.marker(index, entity, world.commands(), marker)
                        },
                    )
                    && let Some(parent_monitor_entity) =
                        apply_parent_markers_to_hierarchy::<MonitorChildOf>(
                            entity,
                            parent_workspace_entity,
                            world.reborrow(),
                            |mut world: DeferredWorld<'_>,
                             entity,
                             index,
                             marker: &_,
                             insert_marker: &dyn MarkerFn<
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
                             insert_marker: &dyn MarkerFn<
                                <WindowManagerChildOf as KomotoolRelationship>::Marker,
                            >| {
                                insert_marker.marker(index, entity, world.commands(), marker)
                            },
                        );
                    }
            },
        );
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

        match old_idx {
            OldIndex::OldIndex(old_idx) => {
                if let Some(old_idx) = old_idx {
                    let marker_map_optional =
                        world.get_resource::<WindowExtendedMarkerMap>().cloned();
                    despawn_window_marker_component(
                        old_idx + 1,
                        entity,
                        world.commands(),
                        marker_map_optional
                            .as_ref()
                            .unwrap_or(&WindowExtendedMarkerMap::default()),
                    );

                    let parent_workspace_entity =
                        remove_parent_markers_from_hierarchy::<WorkspaceChildOf>(
                            entity,
                            None,
                            world.reborrow(),
                            |mut world: DeferredWorld<'_>,
                             entity,
                             index,
                             marker: &_,
                             insert_marker: &dyn MarkerFn<
                                <WorkspaceChildOf as KomotoolRelationship>::Marker,
                            >| {
                                insert_marker.marker(index, entity, world.commands(), marker)
                            },
                        );

                    let parent_monitor_entity =
                        remove_parent_markers_from_hierarchy::<MonitorChildOf>(
                            entity,
                            parent_workspace_entity,
                            world.reborrow(),
                            |mut world: DeferredWorld<'_>,
                             entity,
                             index,
                             marker: &_,
                             insert_marker: &dyn MarkerFn<
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
                         insert_marker: &dyn MarkerFn<
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
                         insert_marker: &dyn MarkerFn<
                            <ContainerChildOf as KomotoolRelationship>::Marker,
                        >| {
                            insert_marker.marker(index, entity, world.commands(), marker)
                        },
                    )
                }
            }
            OldIndex::EntityDoesNotExist => {
                warn!("Entity does not exist");
            }
            OldIndex::ParentEntityDoesNotExist
            | OldIndex::ParentEntityHasNoChildren
            | OldIndex::EntityHasNoRelationship => {
                remove_all_markers(world, entity);
            }
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

    const HIERARCHY: HierarchyFnType<WindowExtendedMarkerMap> =
        |mut world: DeferredWorld<'_>,
         entity,
         index,
         marker: &_,
         insert_marker: &dyn MarkerFn<<Self as KomotoolRelationship>::Marker>| {
            insert_marker.marker(index, entity, world.commands(), marker);
        };

    type Komorebi = Window;

    type Child = ContainerChildOf;
}
