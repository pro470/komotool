use crate::prelude::focused::{focus_target_on_insert, focus_target_on_replace};
use crate::prelude::{KomorebiRelationshipOnInsert, KomorebiRelationshipOnReplace};
use crate::relationships::monocle_container::remove_on_new_insert_komorebi_child_of;
use bevy_ecs::world::World;

pub fn add_relationships_observers_komorebi_relationship(world: &mut World) {
    world.add_observer(focus_target_on_insert::<KomorebiRelationshipOnInsert>);
    world.add_observer(focus_target_on_replace::<KomorebiRelationshipOnReplace>);
    world.add_observer(remove_on_new_insert_komorebi_child_of::<KomorebiRelationshipOnInsert>);
}
