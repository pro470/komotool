use crate::components::{
    MonocleContainer, despawn_container_marker_component, insert_container_marker_component,
};
use crate::prelude::{ContainerExtendedMarkerMap, KomorebiType};
use crate::relationships::komorebi_one_to_one_relationship::{
    KomorebiOTOonInsertResult, KomorebiOTOonRemoveResult, KomorebiOneToOneRelationship,
    KomorebiOneToOneRelationshipTarget, komorebi_oto_relationship_on_insert,
    komorebi_oto_relationship_on_remove, komorebi_oto_relationship_target_on_replace,
};
use crate::relationships::{
    KomorebiChildOf, KomorebiChildren, KomorebiObserver, apply_markers_to_container_hierarchy,
};
use bevy_ecs::component::HookContext;
use bevy_ecs::entity::Entity;
use bevy_ecs::observer::Trigger;
use bevy_ecs::prelude::Component;
use bevy_ecs::relationship::RelationshipTarget;
use bevy_ecs::world::DeferredWorld;
use bevy_log::warn;
use bevy_reflect::Reflect;

impl KomorebiOneToOneRelationship for MonocleContainer {
    type Komorebionetoonerelationshiptarget = MonocleContainerTarget;

    const CHILD: KomorebiType = KomorebiType::Container;

    const PARENT: KomorebiType = KomorebiType::Workspace;

    fn on_insert(mut world: DeferredWorld, hookcontext: HookContext) {
        if let KomorebiOTOonInsertResult::Ok(target_entity, idx) =
            komorebi_oto_relationship_on_insert::<Self>(world.reborrow(), hookcontext)
        {
            if let Some(mut komorebi_children) = world
                .entity_mut(target_entity)
                .get_mut::<KomorebiChildren>()
            {
                komorebi_children
                    .collection_mut_risky()
                    .shift_remove(&hookcontext.entity);

                let marker_map_optional =
                    world.get_resource::<ContainerExtendedMarkerMap>().cloned();
                let mut default_map = None;
                apply_markers_to_container_hierarchy(
                    world.reborrow(),
                    hookcontext.entity,
                    idx,
                    marker_map_optional.as_ref().unwrap_or_else(|| {
                        warn!(
                        "Failed to get ContainerExtendedMarkerMap. Markers over the default threshold will not be applied."
                    );
                        default_map.get_or_insert_with(ContainerExtendedMarkerMap::default)
                    }),
                    &despawn_container_marker_component,
                )
            }
        };
    }

    fn on_remove(mut world: DeferredWorld, hookcontext: HookContext) {
        if let KomorebiOTOonRemoveResult::Ok(target_entity, idx) =
            komorebi_oto_relationship_on_remove::<Self>(world.reborrow(), hookcontext)
        {
            if let Some(mut komorebi_children) = world
                .entity_mut(target_entity)
                .get_mut::<KomorebiChildren>()
            {
                if !komorebi_children
                    .collection_mut_risky()
                    .contains(&hookcontext.entity)
                {
                    komorebi_children
                        .collection_mut_risky()
                        .shift_insert(idx, hookcontext.entity);

                    let marker_map_optional =
                        world.get_resource::<ContainerExtendedMarkerMap>().cloned();
                    let mut default_map = None;
                    apply_markers_to_container_hierarchy(
                        world.reborrow(),
                        hookcontext.entity,
                        idx,
                        marker_map_optional.as_ref().unwrap_or_else(|| {
                            warn!(
                            "Failed to get ContainerExtendedMarkerMap. Markers over the default threshold will not be applied."
                        );
                            default_map.get_or_insert_with(ContainerExtendedMarkerMap::default)
                        }),
                        &insert_container_marker_component,
                    )
                }
            }
        }
    }
}

#[derive(Component, Reflect)]
pub struct MonocleContainerTarget {
    entity: Entity,
    idx: usize,
}

impl KomorebiOneToOneRelationshipTarget for MonocleContainerTarget {
    type Komorebionetoonerelationship = MonocleContainer;

    fn set_entity(&mut self, entity: Entity) {
        self.entity = entity
    }

    fn get_entity(&self) -> Entity {
        self.entity
    }

    fn set_index(&mut self, index: usize) {
        self.idx = index
    }

    fn get_index(&self) -> usize {
        self.idx
    }

    fn from(entity: Entity, index: usize) -> Self {
        Self { entity, idx: index }
    }

    fn on_replace(world: DeferredWorld, hookcontext: HookContext) {
        komorebi_oto_relationship_target_on_replace::<Self>(world, hookcontext);
    }
}

pub fn remove_on_new_insert_komorebi_child_of<EventTrigger: KomorebiObserver>(
    trigger: Trigger<EventTrigger>,
    mut world: DeferredWorld,
) {
    let entity = trigger.event().hook_context().entity;
    world.commands().entity(entity).remove::<MonocleContainer>();
}
