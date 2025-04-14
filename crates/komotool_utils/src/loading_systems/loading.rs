use bevy_ecs::change_detection::DetectChanges;
use crate::{PostUpdateStartup, PreUpdateStartup, UpdateStartup};
use bevy_ecs::schedule::Schedules;
use bevy_ecs::system::{Res, ResMut, Resource};
use bevy_reflect::Reflect;
use bevy_state::state::{NextState, State, States};

#[derive(Resource, Default, Reflect)]
pub struct LoadingCounter(pub usize);

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GlobalLoadingState {
    #[default]
    Loading,
    Loaded,
    CleanupDone,
    AllDone,
    Finished,
}

pub fn update_global_state(
    counter: Res<LoadingCounter>,
    current_state: Res<State<GlobalLoadingState>>,
    mut next_state: ResMut<NextState<GlobalLoadingState>>,
) {
    let target_state = if counter.0 > 0 {
        GlobalLoadingState::Loading
    } else {
        GlobalLoadingState::Loaded
    };

    if current_state.get() != &target_state {
        next_state.set(target_state);
    }
}

pub fn increment_loading_counter(mut counter: ResMut<LoadingCounter>) {
    counter.0 += 1;
}

pub fn decrement_loading_counter(mut counter: ResMut<LoadingCounter>) {
    counter.0 = counter.0.saturating_sub(1);
}

pub fn remove_startup_schedules(
    mut schedules: ResMut<Schedules>,
    mut state: ResMut<NextState<GlobalLoadingState>>,
) {
    if !state.is_changed() && !state.is_added() {
        return;
    }
    schedules.remove_entry(PreUpdateStartup);
    schedules.remove_entry(UpdateStartup);
    schedules.remove_entry(PostUpdateStartup);
    state.set(GlobalLoadingState::Finished);
}
