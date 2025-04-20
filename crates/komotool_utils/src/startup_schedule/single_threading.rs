use bevy_app::{
    First, FixedFirst, FixedLast, FixedPostUpdate, FixedPreUpdate, FixedUpdate, Last, PostUpdate,
    PreUpdate, SpawnScene, Update,
};
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

    // SpawnScene schedule
    if let Some(schedule) = schedules.get_mut(SpawnScene) {
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

    // FixedFirst schedule
    if let Some(schedule) = schedules.get_mut(FixedFirst) {
        schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    }

    // FixedPreUpdate schedule
    if let Some(schedule) = schedules.get_mut(FixedPreUpdate) {
        schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    }

    // FixedUpdate schedule
    if let Some(schedule) = schedules.get_mut(FixedUpdate) {
        schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    }

    // FixedPostUpdate schedule
    if let Some(schedule) = schedules.get_mut(FixedPostUpdate) {
        schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    }

    // FixedLast schedule
    if let Some(schedule) = schedules.get_mut(FixedLast) {
        schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    }
}
