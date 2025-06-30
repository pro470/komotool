use bevy_app::{App, AppExit, MainScheduleOrder, Plugin, PluginsState};
use bevy_ecs::schedule::ScheduleLabel;
use bevy_log::{info, warn};
use crossbeam_channel::Receiver;
use komorebi_client::Notification;
use komotool_pipe::PipeNotificationEvent;
use komotool_utils::startup_schedule::KomoToolStartUp;
use std::thread;

pub struct KomotoolScheduleRunnerPlugin;

impl Plugin for KomotoolScheduleRunnerPlugin {
    fn build(&self, app: &mut App) {
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

                loop {
                    app.update();

                    if let Some(exit) = app.should_exit() {
                        return exit;
                    };

                    if let Some(has_run) =
                        app.world().get_resource::<komotool_assets::HasRunStartUp>()
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

                let (event_tx, event_rx) = crossbeam_channel::unbounded::<AppEvent>();
                let thread_event_tx = event_tx.clone();

                // Spawne einen Thread für den Pipe-Listener
                thread::spawn(move || {
                    while let Ok(notification) = reciver.recv() {
                        if thread_event_tx
                            .send(AppEvent::PipeNotification(notification))
                            .is_err()
                        {
                            // Empfänger wurde gedroppt, Thread beenden
                            break;
                        }
                    }
                    info!("Pipe listener thread finished.");
                });

                let tick =
                    move |app: &mut App, notification: Notification| -> Result<(), AppExit> {
                        app.world_mut()
                            .send_event(PipeNotificationEvent { notification });
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
                                if let Err(exit) = tick(&mut app, notification) {
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
        });
    }
}

pub enum AppEvent {
    PipeNotification(Notification),
}
