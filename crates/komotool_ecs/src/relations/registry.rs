use bevy::prelude::Entity;
use std::collections::{HashMap, HashSet};

/// Each entity is stored exactly once here.
#[derive(Debug, Clone)]
pub struct EntityRecord {
    pub entity: Entity,
    pub monitor: u32,
    pub workspace: u32,
    pub container: u32,
    pub window: u32,
}

#[derive(Default)]
pub struct RelationRegistry {
    /// Our “database” of entities.
    pub records: Vec<EntityRecord>,
    /// Maps an Entity to its index in `records`.
    pub entity_to_index: HashMap<Entity, usize>,
    /// Our “secondary index”: for each tag string, we store the list of indices in `records`
    /// that have that tag. (For example, "Monitor=1" might map to [0, 4, 7] if those records have monitor 1.)
    pub index: HashMap<String, Vec<usize>>,
}

impl RelationRegistry {
    /// Insert a new entity with its components.
    pub fn insert(&mut self, entity: Entity, monitor: u32, workspace: u32, container: u32, window: u32) {
        let record = EntityRecord { entity, monitor, workspace, container, window };
        self.records.push(record);
        let idx = self.records.len() - 1;
        self.entity_to_index.insert(entity, idx);

        // Update our secondary index for each tag.
        // In this design, a "tag" is a string like "Monitor=1".
        self.add_tag(idx, format!("Monitor={}", monitor));
        self.add_tag(idx, format!("Workspace={}", workspace));
        self.add_tag(idx, format!("Container={}", container));
        self.add_tag(idx, format!("Window={}", window));
    }

    fn add_tag(&mut self, idx: usize, tag: String) {
        self.index.entry(tag).or_insert_with(Vec::new).push(idx);
    }

    pub fn update_component(&mut self, entity: Entity, tag: &str) {
        if let Some((component, value_str)) = tag.split_once('=') {
            if let Ok(new_val) = value_str.parse::<u32>() {
                if let Some(&idx) = self.entity_to_index.get(&entity) {
                    let record = &mut self.records[idx];
                    // Remove the old tag for this component
                    let old_tag = match component {
                        "Monitor" => {
                            let old = record.monitor;
                            record.monitor = new_val;
                            format!("Monitor={}", old)
                        },
                        "Workspace" => {
                            let old = record.workspace;
                            record.workspace = new_val;
                            format!("Workspace={}", old)
                        },
                        "Container" => {
                            let old = record.container;
                            record.container = new_val;
                            format!("Container={}", old)
                        },
                        "Window" => {
                            let old = record.window;
                            record.window = new_val;
                            format!("Window={}", old)
                        },
                        _ => return,
                    };
                    // Remove the index from the old tag vector.
                    if let Some(vec) = self.index.get_mut(&old_tag) {
                        vec.retain(|&i| i != idx);
                    }
                    // Add the new tag.
                    self.add_tag(idx, tag.to_string());
                }
            }
        }
        self.rebuild_index();
    }

    pub fn rebuild_index(&mut self) {
        self.index.clear();
        for (idx, record) in self.records.iter().enumerate() {
            self.add_tag(idx, format!("Monitor={}", record.monitor));
            self.add_tag(idx, format!("Workspace={}", record.workspace));
            self.add_tag(idx, format!("Container={}", record.container));
            self.add_tag(idx, format!("Window={}", record.window));
        }
    }
}

