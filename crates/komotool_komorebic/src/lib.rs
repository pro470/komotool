use bevy::prelude::*;
use komorebi_client::{send_message, SocketMessage};
use bevy_mod_scripting::core::bindings::function::namespace::NamespaceBuilder;

pub struct KomoToolKomorebicPlugin;

impl Plugin for KomoToolKomorebicPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        
        NamespaceBuilder::<SocketMessage>::new(world)
            .register("retile", |window_id: u32, container_id: u32| {
                let message = SocketMessage::Retile { window_id, container_id };
                match send_message(message) {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send retile message: {}", e);
                        false
                    }
                }
            });
    }
}
