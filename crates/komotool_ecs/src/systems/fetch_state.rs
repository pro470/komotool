use crate::resources::KomorebiState;
use bevy::prelude::*;
use komorebi_client::{send_query, SocketMessage};

pub fn fetch_komorebi_state(mut komorebi_state: ResMut<KomorebiState>) {
    if let Ok(response) = send_query(&SocketMessage::State) {
        if let Ok(new_state) = serde_json::from_str(&response) {
            komorebi_state.last = komorebi_state.current.take();
            komorebi_state.current = Some(new_state);
        }
    }
}
