use bevy_ecs;
use bevy_ecs::schedule::ScheduleLabel;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PreUpdateStartup;
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UpdateStartup;
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PostUpdateStartup;
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct KomoToolStartUpFinished;
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct KomoToolStartUp;
