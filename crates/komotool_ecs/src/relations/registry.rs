use bevy::prelude::Entity;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct RelationRegistry {
    // For filtering
    pub index: HashMap<String, HashSet<Entity>>,
    
    // For sorting
    pub monitor: HashMap<Entity, u32>,
    pub workspace: HashMap<Entity, u32>,
    pub container: HashMap<Entity, u32>,
    pub window: HashMap<Entity, u32>,
}

impl RelationRegistry {
    // Call this whenever tags change
    fn update_component(&mut self, entity: Entity, tag: &str) {
        if let Some((component, value)) = tag.split_once('=') {
            if let Ok(num) = value.parse::<u32>() {
                match component {
                    "Monitor" => self.monitor.insert(entity, num),
                    "Workspace" => self.workspace.insert(entity, num),
                    "Container" => self.container.insert(entity, num),
                    "Window" => self.window.insert(entity, num),
                    _ => None,
                };
            }
        }
    }
}



