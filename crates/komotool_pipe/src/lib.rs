use anyhow::Result;
use bevy_app::{App, First, Plugin};
use bevy_ecs::event::{Event, EventWriter};
use bevy_ecs::system::NonSend;
use bevy_reflect::Reflect;
use crossbeam_channel::{unbounded, Receiver, Sender};
use komorebi_client::{
    send_query, subscribe_with_options, Notification, SocketMessage, SubscribeOptions,
};
use std::io::{BufReader, Read};
use std::thread;
use std::time::Duration;

pub struct KomoToolPipePlugin;

#[derive(Event, Reflect)]
pub struct PipeNotificationEvent {
    pub notification: Notification,
}

impl Plugin for KomoToolPipePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PipeNotificationEvent>();
        let (sender, receiver) = unbounded();

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

fn run_pipe_listener(sender: &Sender<Notification>) -> Result<()> {
    const NAME: &str = "komotool";

    log::info!("Connecting to named pipe: {}", NAME);

    // Attempt to subscribe
    let socket = match subscribe_with_options(
        NAME,
        SubscribeOptions {
            filter_state_changes: true,
        },
    ) {
        Ok(socket) => {
            log::info!("Connected to named pipe successfully");
            socket
        }
        Err(e) => {
            log::warn!(
                "Failed to connect to the named pipe: {}. Retrying in 2s...",
                e
            );
            return Ok(()); // Retry connecting
        }
    };

    for incoming in socket.incoming() {
        match incoming {
            Ok(subscription) => {
                let mut buffer = Vec::new();
                let mut reader = BufReader::new(subscription);

                // Detect disconnections
                if matches!(reader.read_to_end(&mut buffer), Ok(0)) {
                    log::warn!("Disconnected from komorebi. Attempting to reconnect...");

                    // Keep retrying until it successfully reconnects
                    while send_query(&SocketMessage::AddSubscriberSocket(NAME.to_string())).is_err()
                    {
                        log::warn!("Reconnection attempt failed. Retrying in 1s...");
                        thread::sleep(Duration::from_secs(1));
                    }

                    log::info!("Reconnected to komorebi!");
                    continue; // Restart pipe listening
                }

                // Process incoming notifications
                match String::from_utf8(buffer) {
                    Ok(notification_string) => {
                        match serde_json::from_str::<Notification>(&notification_string) {
                            Ok(notification) => {
                                if sender.send(notification).is_err() {
                                    log::warn!("Failed to send notification to channel");
                                }
                            }
                            Err(e) => log::debug!("Malformed notification: {}", e),
                        }
                    }
                    Err(e) => {
                        log::error!("Notification string was invalid UTF-8: {}", e);
                    }
                }
            }
            Err(e) => {
                log::warn!("Socket error: {}. Reconnecting...", e);
                return Ok(()); // Exit to trigger reconnection
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
