use crate::handler::komotool_event_handler::komotool_event_handler_all;
use crate::{OnPostUpdate, OnPreUpdate, OnUpdate};
use bevy_app::{PostUpdate, PreUpdate, Update};
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::prelude::Schedules;

pub fn insert_komotool_handlers(mut schedule: ResMut<Schedules>) {
    schedule.add_systems(PreUpdate, komotool_event_handler_all::<OnPreUpdate>);
    schedule.add_systems(Update, komotool_event_handler_all::<OnUpdate>);
    schedule.add_systems(PostUpdate, komotool_event_handler_all::<OnPostUpdate>);
}
