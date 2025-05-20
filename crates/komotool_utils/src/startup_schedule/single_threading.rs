use crate::komotool_schedule::{KomoToolPostUpdate, KomoToolPreUpdate, KomoToolUpdate};
use bevy_app::{
    First, FixedFirst, FixedLast, FixedPostUpdate, FixedPreUpdate, FixedUpdate, Last, PostUpdate,
    PreUpdate, SpawnScene, Update,
};
use bevy_ecs::change_detection::Mut;
use bevy_ecs::schedule::{Schedule, ScheduleLabel, Schedules};
use bevy_ecs::system::ResMut;

// This is a startup system that configures all main schedules to be single-threaded
pub fn configure_single_threaded_schedules(mut schedules: ResMut<Schedules>) {
    // Get a mutable reference to each schedule and set it to single-threaded

    // First schedule
    setup_single_threaded_schedules(First, schedules.reborrow());

    // PreUpdate schedule
    setup_single_threaded_schedules(PreUpdate, schedules.reborrow());

    // Update schedule
    setup_single_threaded_schedules(Update, schedules.reborrow());

    // SpawnScene schedule
    setup_single_threaded_schedules(SpawnScene, schedules.reborrow());

    // PostUpdate schedule
    setup_single_threaded_schedules(PostUpdate, schedules.reborrow());

    // Last schedule
    setup_single_threaded_schedules(Last, schedules.reborrow());

    // FixedFirst schedule
    setup_single_threaded_schedules(FixedFirst, schedules.reborrow());

    // FixedPreUpdate schedule
    setup_single_threaded_schedules(FixedPreUpdate, schedules.reborrow());

    // FixedUpdate schedule
    setup_single_threaded_schedules(FixedUpdate, schedules.reborrow());

    // FixedPostUpdate schedule
    setup_single_threaded_schedules(FixedPostUpdate, schedules.reborrow());

    // FixedLast schedule
    setup_single_threaded_schedules(FixedLast, schedules.reborrow());

    setup_single_threaded_schedules(KomoToolPreUpdate, schedules.reborrow());

    setup_single_threaded_schedules(KomoToolUpdate, schedules.reborrow());

    setup_single_threaded_schedules(KomoToolPostUpdate, schedules.reborrow());
}

pub fn setup_single_threaded_schedules(label: impl ScheduleLabel, mut schedules: Mut<Schedules>) {
    if let Some(schedule) = schedules.get_mut(label.intern()) {
        schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    } else {
        println!("{:#?} schedule not found, initializing it", label);

        if schedules.insert(Schedule::new(label.intern())).is_some() {
            println!(
                "This should not happen schedule already exists but could not be found by get_mut method call in configure_single_threaded_schedules: {:#?}",
                label
            );
        };

        if let Some(schedule) = schedules.get_mut(label) {
            schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
        }
    }
}
