use crate::KomotoolFixedTimeSender;
use bevy_ecs::system::{Local, Res, ResMut};
use bevy_ecs::world::Mut;
use bevy_log::warn;
use bevy_time::{Fixed, Time, Virtual};
use std::time::Duration;

pub fn send_fixed_time(
    mut fixed_time_sender: ResMut<KomotoolFixedTimeSender>,
    fixed_time: Res<Time<Fixed>>,
    virtual_time: Res<Time<Virtual>>,
) {
    let timestep = fixed_time.timestep();
    let delta = virtual_time.delta();
    let overstep = fixed_time.overstep();
    calculate_overstep(fixed_time_sender.reborrow(), timestep, delta + overstep);
}

pub fn did_fixed_time_change(
    mut fixed_time_sender: ResMut<KomotoolFixedTimeSender>,
    fixed_time: Res<Time<Fixed>>,
    mut local_duration: Local<Duration>,
) {
    if *local_duration != fixed_time.timestep() {
        calculate_overstep(
            fixed_time_sender.reborrow(),
            fixed_time.timestep(),
            fixed_time.overstep(),
        );
        *local_duration = fixed_time.timestep();
    }
}

pub fn calculate_overstep(
    fixed_time_sender: Mut<KomotoolFixedTimeSender>,
    timestep: Duration,
    overstep: Duration,
) {
    if let Some(real_timestep) = timestep.checked_sub(overstep) {
        fixed_time_sender.0.send(real_timestep).unwrap_or_else(|e| {
            warn!("Failed to send fixed time: {}", e);
        });
    } else {
        fixed_time_sender
            .0
            .send(Duration::ZERO)
            .unwrap_or_else(|e| {
                warn!("Failed to send fixed time: {}", e);
            });
    }
}
