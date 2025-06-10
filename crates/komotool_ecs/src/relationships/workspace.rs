use crate::components::{despawn_container_marker_component, insert_container_marker_component};
use crate::prelude::{get_old_index, remove_parent_markers_from_hierarchy};
use crate::relationships::window_manager::WindowManagerChildOf;
use crate::relationships::{
    ContainerChildOf, ContainsParentChild, GetIndex, InsertMarkerFn, KomotoolRelationship,
    MonitorChildOf, RelationshipIndexSet, apply_markers_to_container_hierarchy,
    apply_parent_markers_to_hierarchy, bevy_on_insert, bevy_on_remove, relationships_hook,
    update_markers,
};
use crate::resources::ContainerExtendedMarkerMap;
use bevy_ecs::component::HookContext;
use bevy_ecs::entity::Entity;
use bevy_ecs::relationship::{Relationship, RelationshipTarget};
use bevy_ecs::world::DeferredWorld;
use bevy_log::warn;
use bevy_reflect::Reflect;
use komorebi_client::{Container, Workspace};

#[derive(Reflect)]
pub struct WorkspaceChildOf(pub Entity);

impl bevy_ecs::component::Component for WorkspaceChildOf
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
}

#[derive(Reflect)]
pub struct WorkspaceChildren(pub(crate) RelationshipIndexSet);

impl bevy_ecs::component::Component for WorkspaceChildren
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
            ContainsParentChild,
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
                            container_idx_in_workspace_list + 1,
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
            } else {
                warn!(
                    "Failed to get WorkspaceChildren. It has to be the first child of the workspace."
                );

                // Klone die Ressourcen-Map, um die immutable Leihe von `world` aufzuheben.
                let marker_map_clone = world.get_resource::<ContainerExtendedMarkerMap>().cloned();
                let mut default_map = None;

                crate::relationships::apply_markers_to_container_hierarchy(
                    world.reborrow(),
                    entity,
                    1,
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
                old_idx + 1,
                marker_map_optional.as_ref().unwrap_or_else(||{
                    warn!(
                        "Failed to get ContainerExtendedMarkerMap. Markers over the default threshold will not be applied."
                    );
                    default_map.get_or_insert_with(ContainerExtendedMarkerMap::default)
                }),
                despawn_container_marker_component,
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

            update_markers::<Self>(
                world.reborrow(),
                marker_map_optional,
                entity,
                old_idx,
                apply_markers_to_container_hierarchy,
            )
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

    type Child = ContainerChildOf;
}
