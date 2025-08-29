use crate::components::{Focused, MonocleContainer};
use crate::relationships::KomorebiChildOf;
use crate::relationships::focused::OldFocusTarget;
use crate::relationships::monocle_container::MonocleContainerTarget;
use bevy_ecs::component::HookContext;
use bevy_ecs::relationship::RelationshipHookMode;
use bevy_ecs::world::{DeferredWorld, World};

pub fn register_relationships_hooks_komorebi_relationship(wrold: &mut World) {
    let komorebi_relationship_hook = wrold.register_component_hooks::<KomorebiChildOf>();

    komorebi_relationship_hook.on_remove(
        |mut world: DeferredWorld,
         HookContext {
             entity,
             relationship_hook_mode,
             ..
         }| {
            if let RelationshipHookMode::Skip = relationship_hook_mode {
                return;
            }

            world.commands().entity(entity).remove::<Focused>();

            world.commands().entity(entity).remove::<OldFocusTarget>();

            world.commands().entity(entity).remove::<MonocleContainer>();

            world
                .commands()
                .entity(entity)
                .remove::<MonocleContainerTarget>();
        },
    );
}
