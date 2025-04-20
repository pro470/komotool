use crate::handler::komotool_event_handler::komotool_event_handler_all;
use crate::{OnPostUpdate, OnPreUpdate, OnUpdate};
use bevy_app::{FixedPostUpdate, FixedPreUpdate, FixedUpdate};
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::prelude::Schedules;

pub fn insert_komotool_handlers(mut schedule: ResMut<Schedules>) {
    schedule.add_systems(FixedPreUpdate, komotool_event_handler_all::<OnPreUpdate>);
    schedule.add_systems(FixedUpdate, komotool_event_handler_all::<OnUpdate>);
    schedule.add_systems(FixedPostUpdate, komotool_event_handler_all::<OnPostUpdate>);
}
