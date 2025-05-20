use bevy_ecs::schedule::ScheduleLabel;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct KomoToolPreUpdate;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct KomoToolUpdate;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct KomoToolPostUpdate;
