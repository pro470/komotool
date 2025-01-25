use bevy::prelude::*;
use komorebi_client::{send_message, OperationDirection, SocketMessage};
use bevy_mod_scripting::core::bindings::function::namespace::NamespaceBuilder;

#[derive(Reflect)]
struct KomorebiMessageWrapper;

pub struct KomoToolKomorebicPlugin;

impl Plugin for KomoToolKomorebicPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();

        NamespaceBuilder::<KomorebiMessageWrapper>::new(world)
            .register("retile", || {
                let message = SocketMessage::Retile;
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send retile message: {}", e);
                        false
                    }
                }
            })
            .register("focus_window", |operationdirection: String| {
                let param: OperationDirection = match operationdirection.to_lowercase().as_str() {
                    "left" => OperationDirection::Left,
                    "right" => OperationDirection::Right,
                    "up" => OperationDirection::Up,
                    "down" => OperationDirection::Down,
                    _ => {
                        log::error!("Invalid direction: {}", operationdirection);
                        return false
                    },
                };
                let message = SocketMessage::FocusWindow(param);
                match send_message(&message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send retile message: {}", e);
                        false
                    }
                }
                
            });
            
    }
}
