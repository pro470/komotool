use super::registry::RelationRegistry; 
use bevy::prelude::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub struct RelationQueryBuilder<'a> {
    registry: &'a RelationRegistry,
    current_set: HashSet<Entity>,
    ordering: Vec<&'static str>,
}

impl<'a> RelationQueryBuilder<'a> {
    fn new(registry: &'a RelationRegistry) -> Self {
        Self {
            registry,
            current_set: HashSet::new(),
            ordering: Vec::new(),
        }
    }

    // --------- Filtering Methods ---------
    fn or(mut self, tags: &[&str]) -> Self {
        let mut result = HashSet::new();
        for tag in tags {
            if let Some(entities) = self.registry.index.get(*tag) {
                result.extend(entities);
            }
        }
        self.current_set = result;
        self
    }

    fn and(mut self, tags: &[&str]) -> Self {
        let mut result = self.current_set.clone();
        for tag in tags {
            if let Some(entities) = self.registry.index.get(*tag) {
                result.retain(|e| entities.contains(e));
            } else {
                result.clear();
            }
        }
        self.current_set = result;
        self
    }

    fn with(mut self, tag: &str) -> Self {
        if let Some(entities) = self.registry.index.get(tag) {
            if self.current_set.is_empty() {
                self.current_set = entities.clone();
            } else {
                self.current_set.retain(|e| entities.contains(e));
            }
        }
        self
    }

    fn without(mut self, tag: &str) -> Self {
        if let Some(entities) = self.registry.index.get(tag) {
            self.current_set.retain(|e| !entities.contains(e));
        }
        self
    }

    // --------- Sorting Methods ---------
    fn order_by(mut self, components: &[&'static str]) -> Self {
        self.ordering = components.to_vec();
        self
    }

    // --------- Execution ---------
    fn execute(self) -> Vec<Entity> {
        // Destructure self to move components separately
        let Self {
            registry,
            current_set,
            ordering,
        } = self;

        let mut entities: Vec<Entity> = current_set.into_iter().collect();
        
        if !ordering.is_empty() {
            // Create a closure that captures registry and ordering
            entities.sort_unstable_by(|a, b| {
                for component in &ordering {
                    let a_val = get_component_value(registry, *a, component);
                    let b_val = get_component_value(registry, *b, component);
                    
                    match a_val.cmp(&b_val) {
                        Ordering::Equal => continue,
                        ord => return ord,
                    }
                }
                Ordering::Equal
            });
        }
        
        entities
    }
}

fn get_component_value(registry: &RelationRegistry, entity: Entity, component: &str) -> u32 {
    match component {
        "Monitor" => registry.monitor.get(&entity).copied().unwrap_or(0),
        "Workspace" => registry.workspace.get(&entity).copied().unwrap_or(0),
        "Container" => registry.container.get(&entity).copied().unwrap_or(0),
        "Window" => registry.window.get(&entity).copied().unwrap_or(0),
        _ => 0,
    }
}
