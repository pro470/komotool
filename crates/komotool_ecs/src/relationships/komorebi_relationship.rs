use crate::components::{
    WindowManager, despawn_container_marker_component, despawn_monitor_marker_component,
    despawn_window_marker_component, despawn_workspace_marker_component,
    insert_container_marker_component, insert_window_marker_component,
    insert_workspace_marker_component,
};
use crate::prelude::{
    MarkerFn, OldIndexInner, apply_markers_to_children, get_children,
    insert_monitor_marker_component, run_insert_marker,
};
use crate::relationships::{
    Check, ContainsParentChild, DespawnInsertMarker, GetIndex, InsertMarkerFn, KomorebiType,
    RelationshipIndexSet, bevy_on_insert, bevy_on_remove, get_old_index_inner, komotool_on_insert,
    parent_markers_to_hierarchy, relationships_hook, remove_all_markers, to_hierarchy_with_marker,
    update_markers_inner,
};
use crate::resources::{
    ContainerExtendedMarkerMap, MonitorExtendedMarkerMap, WindowExtendedMarkerMap,
    WorkspaceExtendedMarkerMap,
};
use bevy_ecs::component::{Component, HookContext};
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Resource;
use bevy_ecs::relationship::Relationship;
use bevy_ecs::system::{In, InMut, Query};
use bevy_ecs::world::DeferredWorld;
use bevy_log::{info, warn};
use bevy_reflect::Reflect;
use komorebi_client::{Container, Monitor, Window, Workspace};

#[derive(Reflect)]
pub struct KomorebiChildOf(pub Entity);

impl bevy_ecs::component::Component for KomorebiChildOf
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

#[derive(Reflect, Component)]
#[relationship_target(relationship = KomorebiChildOf)]
pub struct KomorebiChildren {
    #[relationship]
    collection: RelationshipIndexSet,
    komorebi_child_type: KomorebiType,
}

impl Relationship for KomorebiChildOf {
    type RelationshipTarget = KomorebiChildren;

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
        let mut komorebi_type = KomorebiType::default();
        if bevy_on_insert::<Self>(
            world.reborrow(),
            HookContext {
                entity,
                caller,
                relationship_hook_mode,
                component_id,
            },
            |world: DeferredWorld<'_>, entity, parent, _| {
                komorebi_relationship_check(world, entity, parent, &mut komorebi_type)
            },
        ) {
            return;
        }

        komotool_on_insert::<Self>(
            entity,
            world.reborrow(),
            |mut world: DeferredWorld<'_>, entity, parent, child_idx| {
                if matches!(komorebi_type, KomorebiType::NotDefined) {
                    warn!("Komorebi type not defined");
                    return;
                }
                if child_idx == 1 {
                    world.commands().run_system_cached_with(
                        set_komorebi_type_in_children,
                        (parent, komorebi_type),
                    );
                }

                insert_match_komorebi_type_to_hierarchy(
                    entity,
                    world.reborrow(),
                    child_idx,
                    komorebi_type,
                );

                insert_parent_markers_to_komorebi_hierarchy(
                    world.reborrow(),
                    entity,
                    parent,
                    komorebi_type,
                )
            },
        )
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

        let old_idx = get_old_index_and_komorebi_type(entity, world.reborrow());

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
            OldIndexInner::OldIndex((old_idx, komorebi_type)) => {
                if let Some(old_idx) = old_idx {
                    despawn_match_komorebi_type_to_hierarchy(
                        entity,
                        world.reborrow(),
                        old_idx + 1,
                        komorebi_type,
                    );

                    despawn_parent_markers_to_komorebi_hierarchy(
                        world.reborrow(),
                        entity,
                        komorebi_type,
                    );

                    update_markers_komorebi_relationship(
                        komorebi_type,
                        world.reborrow(),
                        entity,
                        old_idx,
                    );
                }
            }
            OldIndexInner::EntityDoesNotExist => {
                warn!("Entity does not exist");
            }
            OldIndexInner::ParentEntityDoesNotExist
            | OldIndexInner::ParentEntityHasNoChildren
            | OldIndexInner::EntityHasNoRelationship => {
                warn!("Entity has no relationship");
                remove_all_markers(world, entity);
            }
        }
    }
}

pub fn set_komorebi_type_in_children(
    (In(entity), In(komorebi_type)): (In<Entity>, In<KomorebiType>),
    mut komorebi_children: Query<&mut KomorebiChildren>,
) {
    let Ok(mut komorebi_children) = komorebi_children.get_mut(entity) else {
        return;
    };

    if !matches!(komorebi_type, KomorebiType::NotDefined) {
        komorebi_children.set_komorebi_type(komorebi_type);
    }
}
pub fn update_markers_komorebi_relationship(
    komorebi_type: KomorebiType,
    mut world: DeferredWorld,
    entity: Entity,
    old_index: usize,
) {
    if let Some(relationship) = world.entity(entity).get::<KomorebiChildOf>() {
        let child_entity = relationship.get();
        let children = get_children::<KomorebiChildren>(world.reborrow(), child_entity);
        match komorebi_type {
            KomorebiType::Monitor => {
                let marker_map = world.get_resource::<MonitorExtendedMarkerMap>().cloned();
                let marker_func = DespawnInsertMarker {
                    despawn: despawn_monitor_marker_component,
                    insert: insert_monitor_marker_component,
                };

                update_markers_inner(
                    world.reborrow(),
                    marker_map,
                    children,
                    old_index,
                    apply_markers_to_komorebi_hierarchy,
                    marker_func,
                )
            }
            KomorebiType::Workspace => {
                let marker_map = world.get_resource::<WorkspaceExtendedMarkerMap>().cloned();
                let marker_func = DespawnInsertMarker {
                    despawn: despawn_workspace_marker_component,
                    insert: insert_workspace_marker_component,
                };

                update_markers_inner(
                    world.reborrow(),
                    marker_map,
                    children,
                    old_index,
                    apply_markers_to_komorebi_hierarchy,
                    marker_func,
                )
            }
            KomorebiType::Container => {
                let marker_map = world.get_resource::<ContainerExtendedMarkerMap>().cloned();
                let marker_func = DespawnInsertMarker {
                    despawn: despawn_container_marker_component,
                    insert: insert_container_marker_component,
                };

                update_markers_inner(
                    world.reborrow(),
                    marker_map,
                    children,
                    old_index,
                    apply_markers_to_komorebi_hierarchy,
                    marker_func,
                )
            }
            KomorebiType::Window => {
                let marker_map = world.get_resource::<WindowExtendedMarkerMap>().cloned();
                let marker_func = DespawnInsertMarker {
                    despawn: despawn_window_marker_component,
                    insert: insert_window_marker_component,
                };

                update_markers_inner(
                    world.reborrow(),
                    marker_map,
                    children,
                    old_index,
                    apply_markers_to_komorebi_hierarchy,
                    marker_func,
                )
            }
            KomorebiType::NotDefined => {}
        };
    }
}

pub fn get_old_index_and_komorebi_type(
    entity: Entity,
    mut world: DeferredWorld,
) -> OldIndexInner<(Option<usize>, KomorebiType)> {
    get_old_index_inner::<KomorebiChildOf, (Option<usize>, KomorebiType)>(
        entity,
        world.reborrow(),
        |children: &KomorebiChildren| {
            (children.get_index_of(&entity), children.get_komorebi_type())
        },
    )
}

pub fn insert_match_komorebi_type_to_hierarchy(
    entity: Entity,
    mut world: DeferredWorld,
    child_idx: usize,
    komorebi_type: KomorebiType,
) {
    match_komorebi_type_to_hierarchy(
        entity,
        world.reborrow(),
        child_idx,
        komorebi_type,
        KomorebiInsertMarkerFns {
            monitor: insert_monitor_marker_component,
            workspace: insert_workspace_marker_component,
            container: insert_container_marker_component,
            window: insert_window_marker_component,
        },
    );
}

pub fn despawn_match_komorebi_type_to_hierarchy(
    entity: Entity,
    mut world: DeferredWorld,
    child_idx: usize,
    komorebi_type: KomorebiType,
) {
    match_komorebi_type_to_hierarchy(
        entity,
        world.reborrow(),
        child_idx,
        komorebi_type,
        KomorebiInsertMarkerFns {
            monitor: despawn_monitor_marker_component,
            workspace: despawn_workspace_marker_component,
            container: despawn_container_marker_component,
            window: despawn_window_marker_component,
        },
    );
}

pub fn match_komorebi_type_to_hierarchy(
    entity: Entity,
    mut world: DeferredWorld,
    child_idx: usize,
    komorebi_type: KomorebiType,
    funcs: KomorebiInsertMarkerFns,
) {
    match komorebi_type {
        KomorebiType::Monitor => to_hierarchy_with_marker(
            entity,
            world.reborrow(),
            &apply_markers_to_komorebi_hierarchy,
            funcs.monitor,
            child_idx,
        ),
        KomorebiType::Workspace => to_hierarchy_with_marker(
            entity,
            world.reborrow(),
            &apply_markers_to_komorebi_hierarchy,
            funcs.workspace,
            child_idx,
        ),
        KomorebiType::Container => to_hierarchy_with_marker(
            entity,
            world.reborrow(),
            &apply_markers_to_komorebi_hierarchy,
            funcs.container,
            child_idx,
        ),
        KomorebiType::Window => to_hierarchy_with_marker(
            entity,
            world.reborrow(),
            &apply_markers_to_komorebi_hierarchy,
            funcs.window,
            child_idx,
        ),
        _ => {}
    };
}

pub fn apply_markers_to_komorebi_hierarchy<Marker: Resource + Clone + Default>(
    mut deferred_world: DeferredWorld,
    workspace_entity: Entity,
    child_index: usize,
    marker_map: &Marker,
    insert_marker: &dyn MarkerFn<Marker>,
) {
    run_insert_marker(
        child_index,
        workspace_entity,
        deferred_world.reborrow(),
        marker_map,
        insert_marker,
    );

    let children = get_children::<KomorebiChildren>(deferred_world.reborrow(), workspace_entity);

    apply_markers_to_children(
        deferred_world.reborrow(),
        child_index,
        marker_map,
        insert_marker,
        apply_markers_to_komorebi_hierarchy,
        children,
    );
}

pub enum KomotoolDirection {
    Up,
    Down,
}

pub fn insert_parent_markers_to_komorebi_hierarchy(
    mut deferred_world: DeferredWorld,
    entity: Entity,
    parent: Entity,
    komorebi_type: KomorebiType,
) {
    parent_markers_to_komorebi_hierarchy(
        deferred_world.reborrow(),
        entity,
        parent,
        komorebi_type,
        KomorebiInsertMarkerFns {
            monitor: insert_monitor_marker_component,
            workspace: insert_workspace_marker_component,
            container: insert_container_marker_component,
            window: insert_window_marker_component,
        },
    );
}

pub fn despawn_parent_markers_to_komorebi_hierarchy(
    mut deferred_world: DeferredWorld,
    entity: Entity,
    komorebi_type: KomorebiType,
) {
    let parent = deferred_world
        .entity(entity)
        .get::<KomorebiChildOf>()
        .map_or(Entity::PLACEHOLDER, |childof| childof.get());
    parent_markers_to_komorebi_hierarchy(
        deferred_world.reborrow(),
        entity,
        parent,
        komorebi_type,
        KomorebiInsertMarkerFns {
            monitor: despawn_monitor_marker_component,
            workspace: despawn_workspace_marker_component,
            container: despawn_container_marker_component,
            window: despawn_window_marker_component,
        },
    );
}
pub fn parent_markers_to_komorebi_hierarchy(
    mut deferred_world: DeferredWorld,
    entity: Entity,
    mut parent: Entity,
    komorebi_type: KomorebiType,
    funcs: KomorebiInsertMarkerFns,
) {
    let mut parent_komorebi_type = komorebi_type;

    while let Some(parent_komorebi_type_inner) = parent_komorebi_type.parent() {
        if let Some(parent_entity) = run_komorebi_type_apply_markers(
            parent_komorebi_type_inner,
            entity,
            parent,
            deferred_world.reborrow(),
            &funcs,
        ) {
            parent = parent_entity;
            parent_komorebi_type = parent_komorebi_type_inner;
        } else {
            break;
        }
    }
}

pub struct KomorebiInsertMarkerFns {
    pub monitor: InsertMarkerFn<MonitorExtendedMarkerMap>,
    pub workspace: InsertMarkerFn<WorkspaceExtendedMarkerMap>,
    pub container: InsertMarkerFn<ContainerExtendedMarkerMap>,
    pub window: InsertMarkerFn<WindowExtendedMarkerMap>,
}

pub fn run_komorebi_type_apply_markers(
    komorebi_type: KomorebiType,
    entity: Entity,
    parent: Entity,
    mut deferred_world: DeferredWorld,
    funcs: &KomorebiInsertMarkerFns,
) -> Option<Entity> {
    match komorebi_type {
        KomorebiType::Monitor => parent_markers_to_hierarchy::<KomorebiChildOf>(
            entity,
            parent,
            deferred_world.reborrow(),
            |entity, mut world, idx| {
                to_hierarchy_with_marker(
                    entity,
                    world.reborrow(),
                    &apply_markers_to_komorebi_hierarchy,
                    funcs.monitor,
                    idx,
                )
            },
        ),
        KomorebiType::Workspace => parent_markers_to_hierarchy::<KomorebiChildOf>(
            entity,
            parent,
            deferred_world.reborrow(),
            |entity, mut world, idx| {
                to_hierarchy_with_marker(
                    entity,
                    world.reborrow(),
                    &apply_markers_to_komorebi_hierarchy,
                    funcs.workspace,
                    idx,
                )
            },
        ),
        KomorebiType::Container => parent_markers_to_hierarchy::<KomorebiChildOf>(
            entity,
            parent,
            deferred_world.reborrow(),
            |entity, mut world, idx| {
                to_hierarchy_with_marker(
                    entity,
                    world.reborrow(),
                    &apply_markers_to_komorebi_hierarchy,
                    funcs.container,
                    idx,
                )
            },
        ),
        KomorebiType::Window => parent_markers_to_hierarchy::<KomorebiChildOf>(
            entity,
            parent,
            deferred_world.reborrow(),
            |entity, mut world, idx| {
                to_hierarchy_with_marker(
                    entity,
                    world.reborrow(),
                    &apply_markers_to_komorebi_hierarchy,
                    funcs.window,
                    idx,
                )
            },
        ),
        KomorebiType::NotDefined => None,
    }
}
impl GetIndex for KomorebiChildren {
    fn get_index_of(&self, entity: &Entity) -> Option<usize> {
        self.collection.get_index_of(entity)
    }
}

impl KomorebiChildren {
    pub fn get_komorebi_type(&self) -> KomorebiType {
        self.komorebi_child_type
    }

    fn set_komorebi_type(&mut self, komorebi_child_type: KomorebiType) {
        self.komorebi_child_type = komorebi_child_type;
    }
}

pub fn komorebi_relationship_check(
    mut world: DeferredWorld,
    entity: Entity,
    parent: Entity,
    komorebi_type: &mut KomorebiType,
) -> bool {
    if let Some(children) = world.entity(parent).get::<KomorebiChildren>() {
        let children_komorebi_type = children.get_komorebi_type();
        let komorebi_type_inner = find_komorebi_relationship(entity, parent, world.reborrow());
        if children_komorebi_type == komorebi_type_inner {
            *komorebi_type = children_komorebi_type;
            return false;
        } else if children_komorebi_type == KomorebiType::NotDefined {
            world.commands().entity(parent).remove::<KomorebiChildOf>();
            world.commands().entity(entity).remove::<KomorebiChildOf>();
        }
        true
    } else {
        let komorebi_type_inner = find_komorebi_relationship(entity, parent, world.reborrow());
        *komorebi_type = komorebi_type_inner;
        if komorebi_type_inner == KomorebiType::NotDefined {
            return true;
        }
        false
    }
}

pub fn find_komorebi_relationship(
    entity: Entity,
    parent: Entity,
    mut world: DeferredWorld,
) -> KomorebiType {
    let mut workspacecontainer = ContainsParentChild::<Container, Workspace> {
        _phantom: std::marker::PhantomData,
    };

    if !workspacecontainer.check::<KomorebiChildOf>(world.reborrow(), entity, parent, false) {
        return KomorebiType::Container;
    }

    let mut containerwindow = ContainsParentChild::<Window, Container> {
        _phantom: std::marker::PhantomData,
    };

    if !containerwindow.check::<KomorebiChildOf>(world.reborrow(), entity, parent, false) {
        return KomorebiType::Window;
    }

    let mut monitorworkspace = ContainsParentChild::<Workspace, Monitor> {
        _phantom: std::marker::PhantomData,
    };

    if !monitorworkspace.check::<KomorebiChildOf>(world.reborrow(), entity, parent, false) {
        return KomorebiType::Workspace;
    }

    let mut windowmangermonitor = ContainsParentChild::<Monitor, WindowManager> {
        _phantom: std::marker::PhantomData,
    };

    if !windowmangermonitor.check::<KomorebiChildOf>(world.reborrow(), entity, parent, false) {
        return KomorebiType::Monitor;
    }

    world.commands().entity(entity).remove::<KomorebiChildOf>();

    KomorebiType::NotDefined
}
