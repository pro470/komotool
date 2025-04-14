use bevy_app::{First, Last, PostUpdate, PreUpdate, Update};
use bevy_ecs::schedule::Schedules;
use bevy_ecs::system::ResMut;

// This is a startup system that configures all main schedules to be single-threaded
pub fn configure_single_threaded_schedules(mut schedules: ResMut<Schedules>) {
    // Get a mutable reference to each schedule and set it to single-threaded

    // First schedule
    if let Some(schedule) = schedules.get_mut(First) {
        schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    }

    // PreUpdate schedule
    if let Some(schedule) = schedules.get_mut(PreUpdate) {
        schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    }

    // Update schedule
    if let Some(schedule) = schedules.get_mut(Update) {
        schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    }

    // PostUpdate schedule
    if let Some(schedule) = schedules.get_mut(PostUpdate) {
        schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    }

    // Last schedule
    if let Some(schedule) = schedules.get_mut(Last) {
        schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    }
}