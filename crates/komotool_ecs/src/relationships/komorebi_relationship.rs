use crate::relationships::{GetIndex, KomorebiType, RelationshipIndexSet};
use bevy_ecs::component::{Component, HookContext};
use bevy_ecs::entity::Entity;
use bevy_ecs::relationship::{Relationship, RelationshipTarget};
use bevy_ecs::world::DeferredWorld;
use bevy_reflect::Reflect;

#[derive(Reflect)]
pub struct KomorebiChildOf(pub Entity);

impl bevy_ecs::component::Component for KomorebiChildOf
where
    Self: Send + Sync + 'static,
{
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
    type Mutability = bevy_ecs::component::Immutable;
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
        world: DeferredWorld,
        HookContext {
            entity,
            caller,
            relationship_hook_mode,
            component_id,
        }: HookContext,
    ) {
        todo!()
    }

    fn on_replace(
        world: DeferredWorld,
        HookContext {
            entity,
            caller,
            relationship_hook_mode,
            component_id,
        }: HookContext,
    ) {
        todo!()
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
