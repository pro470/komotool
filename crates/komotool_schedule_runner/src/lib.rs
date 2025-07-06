pub mod fixed_time;

use crate::fixed_time::{did_fixed_time_change, send_fixed_time};
use bevy_app::{App, AppExit, First, Last, MainScheduleOrder, Plugin, PluginsState, PreStartup};
use bevy_ecs::resource::Resource;
use bevy_ecs::schedule::{IntoScheduleConfigs, ScheduleLabel};
use bevy_ecs::system::ResMut;
use bevy_log::{error, info, warn};
use bevy_time::{Time, TimeSystem, Virtual, time_system};
use crossbeam_channel::{Receiver, RecvTimeoutError, Sender};
use komorebi_client::Notification;
use komotool_framepace::framerate_limiter;
use komotool_pipe::PipeNotificationEvent;
use komotool_utils::startup_schedule::KomoToolStartUp;
use std::thread;
use std::time::Duration;

pub struct KomotoolScheduleRunnerPlugin;

impl Plugin for KomotoolScheduleRunnerPlugin {
    fn build(&self, app: &mut App) {
        let (event_tx, event_rx) = crossbeam_channel::unbounded::<AppEvent>();

        app.world_mut().insert_resource(KomotoolChannal {
            sender: event_tx.clone(),
            reciver: event_rx.clone(),
        });

        app.set_runner(move |mut app: App| -> AppExit {
            let plugins_state = app.plugins_state();
            if plugins_state != PluginsState::Cleaned {
                while app.plugins_state() == PluginsState::Adding {
                    #[cfg(not(all(target_arch = "wasm32")))]
                    bevy_tasks::tick_global_task_pools_on_main_thread();
                }
                app.finish();
                app.cleanup();
            }

            if let Some(reciver) = app
                .world()
                .get_non_send_resource::<Receiver<Notification>>()
            {
                info!("Receiver found");
                println!("Receiver found");
                let reciver = reciver.clone();

                let (event_timer_tx, event_timer_rx) = crossbeam_channel::unbounded::<Duration>();
                app.world_mut()
                    .insert_resource(KomotoolFixedTimeSender(event_timer_tx.clone()));

                loop {
                    app.update();

                    if let Some(exit) = app.should_exit() {
                        return exit;
                    };

                    if let Some(has_run) = app
                        .world()
                        .get_resource::<komotool_ecs::resources::HasRunStartUp>()
                    {
                        if **has_run {
                            break;
                        }
                    }
                }

                if let Some(mut mainscheduleorder) =
                    app.world_mut().get_resource_mut::<MainScheduleOrder>()
                {
                    mainscheduleorder
                        .labels
                        .retain(|&l| l != KomoToolStartUp.intern());
                }

                let thread_event_tx = event_tx.clone();

                // Spawne einen Thread für den Pipe-Listener
                thread::spawn(move || {
                    while let Ok(notification) = reciver.recv() {
                        if thread_event_tx
                            .send(AppEvent::PipeNotification(Box::new(notification)))
                            .is_err()
                        {
                            // Empfänger wurde gedroppt, Thread beenden
                            break;
                        }
                    }
                    info!("Pipe listener thread finished.");
                });

                let thread_event_timer_rx = event_timer_rx.clone();
                let thread_event_timer_tx = event_tx.clone();

                thread::spawn(move || {
                    let f = move |e| {
                        warn!("Failed to recv fixed time: {}", e);
                        Duration::ZERO
                    };

                    let mut timer = thread_event_timer_rx.recv().unwrap_or_else(f);

                    loop {
                        timer = match thread_event_timer_rx.recv_timeout(timer) {
                            Ok(sleep) => {
                                if sleep.is_zero() {
                                    thread_event_timer_tx
                                        .send(AppEvent::FixedTime)
                                        .unwrap_or_else(|e| {
                                            warn!("Failed to send fixed time: {}", e);
                                        });
                                    println!("it was zero");
                                    thread_event_timer_rx.recv().unwrap_or_else(f)
                                } else {
                                    sleep
                                }
                            }
                            Err(RecvTimeoutError::Timeout) => {
                                thread_event_timer_tx
                                    .send(AppEvent::FixedTime)
                                    .unwrap_or_else(|e| {
                                        warn!("Failed to send fixed time: {}", e);
                                    });

                                thread_event_timer_rx.recv().unwrap_or_else(f)
                            }
                            Err(RecvTimeoutError::Disconnected) => {
                                warn!("Fixed time Pipe listener thread disconnected.");
                                break;
                            }
                        }
                    }
                });

                let tick = move |app: &mut App| -> Result<(), AppExit> {
                    app.update();

                    if let Some(exit) = app.should_exit() {
                        return Err(exit);
                    };

                    Ok(())
                };

                loop {
                    match event_rx.recv() {
                        Ok(app_event) => match app_event {
                            AppEvent::PipeNotification(notification) => {
                                println!("Tick");
                                app.world_mut().send_event(PipeNotificationEvent {
                                    notification: *notification,
                                });
                                if let Err(exit) = tick(&mut app) {
                                    return exit;
                                }
                            }
                            AppEvent::FixedTime => {
                                println!("Fixed time tick");

                                if let Err(exit) = tick(&mut app) {
                                    return exit;
                                }
                            }
                            AppEvent::AssetEvent => {
                                println!("Asset event tick");
                                if let Err(exit) = tick(&mut app) {
                                    return exit;
                                }
                            }
                        },
                        Err(e) => {
                            println!("Error: {e}");
                            return AppExit::Success;
                        }
                    }
                }
            } else {
                warn!("No receiver found");
            };
            AppExit::Success
        })
        .add_systems(
            First,
            send_fixed_time
                .after_ignore_deferred(time_system)
                .in_set(TimeSystem),
        )
        .add_systems(
            Last,
            did_fixed_time_change.after_ignore_deferred(framerate_limiter),
        )
        .add_systems(PreStartup, set_max_delta_virtual_time);
    }
}

pub enum AppEvent {
    PipeNotification(Box<Notification>),
    FixedTime,
    AssetEvent,
}

#[derive(Resource)]
pub struct KomotoolFixedTimeSender(pub Sender<Duration>);

pub fn set_max_delta_virtual_time(mut fixed_time: ResMut<Time<Virtual>>) {
    fixed_time.set_max_delta(Duration::MAX);
}

#[derive(Resource)]
pub struct KomotoolChannal {
    pub sender: Sender<AppEvent>,
    pub reciver: Receiver<AppEvent>,
}
