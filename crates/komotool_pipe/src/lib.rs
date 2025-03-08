use anyhow::Result;
use bevy_app::{App, First, Plugin};
use bevy_ecs::event::{Event, EventWriter};
use bevy_ecs::system::NonSend;
use komorebi_client::{send_query, subscribe_with_options, Notification, SocketMessage, SubscribeOptions};
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

pub struct KomoToolPipePlugin;

#[derive(Event)]
pub struct PipeNotificationEvent {
    pub notification: Notification,
}

impl Plugin for KomoToolPipePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PipeNotificationEvent>();
        let (sender, receiver) = std::sync::mpsc::channel();

        // Spawn listener in a separate thread
        thread::spawn(move || loop {
            match run_pipe_listener(&sender) {
                Ok(_) => log::info!("Pipe listener finished, attempting to reconnect..."),
                Err(e) => log::warn!("Pipe listener error: {}. Retrying...", e),
            }

            // Wait before retrying to prevent overwhelming the system
            thread::sleep(Duration::from_secs(2));
        });

        // Add system to process received messages
        app.insert_non_send_resource(receiver)
            .add_systems(First, handle_pipe_notifications);
    }
}


// Helper function to check if the komorebi service is alive
fn check_connection_alive() -> bool {
    match send_query(&SocketMessage::State) {
        Ok(_) => true,
        Err(e) => {
            log::warn!("Connection check failed: {}", e);
            false
        }
    }
}

fn run_pipe_listener(sender: &Sender<Notification>) -> Result<()> {
    const NAME: &str = "komotool";
    const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(1);

    log::info!("Connecting to named pipe: {}", NAME);

    // Attempt to subscribe
    let socket = subscribe_with_options(
        NAME,
        SubscribeOptions {
            filter_state_changes: true,
        },
    )?;

    log::info!("Connected to named pipe successfully");

    // Keep track of when we last sent a heartbeat
    let mut last_heartbeat = Instant::now();

    // Process incoming data
    for incoming in socket.incoming() {
        // Periodically check if connection is still alive
        if last_heartbeat.elapsed() >= HEARTBEAT_INTERVAL {
            if !check_connection_alive() {
                log::warn!("Heartbeat failed - komorebi appears to be restarted. Reconnecting...");
                return Ok(());  // Exit to reconnect
            }
            last_heartbeat = Instant::now();
            log::debug!("Heartbeat successful - connection is alive");
        }

        match incoming {
            Ok(data) => {
                let reader = BufReader::new(data.try_clone()?);
                for line in reader.lines().map_while(Result::ok) {
                    match serde_json::from_str(&line) {
                        Ok(notification) => {
                            if sender.send(notification).is_err() {
                                log::warn!("Failed to send notification to channel");
                                return Ok(());
                            }
                        }
                        Err(e) => log::debug!("Malformed notification: {}", e),
                    }
                }
            }
            Err(e) => {
                log::warn!("Socket error: {}. Reconnecting...", e);
                return Ok(());  // Exit to trigger reconnection
            }
        }
    }

    Ok(())
}

pub fn handle_pipe_notifications(
    receiver: NonSend<Receiver<Notification>>,
    mut events: EventWriter<PipeNotificationEvent>,
) {
    for notification in receiver.try_iter() {
        events.send(PipeNotificationEvent { notification });
    }
}
