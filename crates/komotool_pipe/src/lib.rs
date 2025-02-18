use anyhow::Result;
use bevy::prelude::*;
use komorebi_client::{subscribe_with_options, Notification, SubscribeOptions};
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{Receiver, Sender};

pub struct KomoToolPipePlugin;

#[derive(Event)]
pub struct PipeNotificationEvent {
    pub notification: Notification,
}

impl Plugin for KomoToolPipePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PipeNotificationEvent>();
        let (sender, receiver) = std::sync::mpsc::channel();

        // Spawn listener thread
        std::thread::spawn(move || {
            if let Err(e) = run_pipe_listener(sender) {
                eprintln!("Pipe listener error: {}", e);
            }
        });

        // Add system to process received messages
        app.insert_non_send_resource(receiver)
            .add_systems(Update, handle_pipe_notifications);
    }
}

fn run_pipe_listener(sender: Sender<Notification>) -> Result<()> {
    const NAME: &str = "komotool";
    let socket = subscribe_with_options(NAME, SubscribeOptions {
        filter_state_changes: true
    })?;

    for incoming in socket.incoming() {
        match incoming {
            Ok(data) => {
                let reader = BufReader::new(data.try_clone()?);
                for line in reader.lines().map_while(Result::ok) {
                    match serde_json::from_str(&line) {
                        Ok(notification) => sender.send(notification)?,
                        Err(e) => log::debug!("Malformed notification: {}", e),
                    }
                }
            }
            Err(e) => log::debug!("Connection error: {}", e),
        }
    }

    Ok(())
}

fn handle_pipe_notifications(
    receiver: NonSend<Receiver<Notification>>,
    mut events: EventWriter<PipeNotificationEvent>,
) {
    for notification in receiver.try_iter() {
        events.send(PipeNotificationEvent { notification });
    }
}
