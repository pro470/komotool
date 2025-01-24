use bevy::prelude::*;
use komorebi_client::{send_message, SocketMessage};
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
            });
    }
}
