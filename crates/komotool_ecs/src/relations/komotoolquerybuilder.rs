use super::registry::{RelationRegistry, EntityRecord};
use bevy::prelude::*;
use std::cmp::Ordering;
use std::collections::HashSet;

pub struct RelationQueryBuilder<'a> {
    registry: &'a RelationRegistry,
    /// Instead of storing Entity we store indices into the `records` Vec.
    current_set: HashSet<usize>,
    /// Which components to sort by. (For simplicity, we sort on the record field names.)
    ordering: Vec<&'static str>,
}

impl<'a> RelationQueryBuilder<'a> {
    pub fn new(registry: &'a RelationRegistry) -> Self {
        Self {
            registry,
            current_set: HashSet::new(),
            ordering: Vec::new(),
        }
    }

    fn or(mut self, tags: &[&str]) -> Self {
        let mut result = self.current_set;
        for tag in tags {
            if let Some(entities) = self.registry.index.get(*tag) {
                result.extend(entities);
            }
        }
        self.current_set = result;
        self
    }

    /// AND: intersection of the current set with all indices matching all the given tags.
    pub fn and(mut self, tags: &[&str]) -> Self {
        let mut result = self.current_set.clone();
        for tag in tags {
            if let Some(indices) = self.registry.index.get(*tag) {
                // Convert the indices list into a set for fast lookup.
                let tag_set: HashSet<_> = indices.iter().copied().collect();
                result.retain(|i| tag_set.contains(i));
            } else {
                result.clear();
            }
        }
        self.current_set = result;
        self
    }

    /// WITH: same as AND, but handles the “current set is empty” case.
    pub fn with(mut self, tag: &str) -> Self {
        if let Some(indices) = self.registry.index.get(tag) {
            if self.current_set.is_empty() {
                self.current_set = indices.iter().copied().collect();
            } else {
                let tag_set: HashSet<_> = indices.iter().copied().collect();
                self.current_set.retain(|i| tag_set.contains(i));
            }
        }
        self
    }

    /// WITHOUT: remove all indices that match the tag.
    pub fn without(mut self, tag: &str) -> Self {
        if let Some(indices) = self.registry.index.get(tag) {
            let tag_set: HashSet<_> = indices.iter().copied().collect();
            self.current_set.retain(|i| !tag_set.contains(i));
        }
        self
    }

    /// Order by one or more components.
    pub fn order_by(mut self, components: &[&'static str]) -> Self {
        self.ordering = components.to_vec();
        self
    }

    /// Returns the raw indices as a HashSet.
    /// This is useful if the caller wishes to iterate over the indices and
    /// look up records directly in the registry.
    pub fn execute_indices(self) -> HashSet<usize> {
        // When no ordering is requested, there's no need to convert to a Vec.
        self.current_set
    }

    /// Execute the query returning the matching entities.
    pub fn execute(self) -> Vec<Entity> {
        // Collect the indices into a vector.
        let mut indices: Vec<usize> = self.current_set.into_iter().collect();

        if !self.ordering.is_empty() {
            // Sort the indices by the specified ordering.
            indices.sort_unstable_by(|&a, &b| {
                let rec_a = &self.registry.records[a];
                let rec_b = &self.registry.records[b];
                for component in &self.ordering {
                    let a_val = get_component_value(rec_a, component);
                    let b_val = get_component_value(rec_b, component);
                    match a_val.cmp(&b_val) {
                        Ordering::Equal => continue,
                        non_eq => return non_eq,
                    }
                }
                Ordering::Equal
            });
        }

        // Map the indices back to the actual Entity.
        indices.into_iter().map(|i| self.registry.records[i].entity).collect()
    }
}

/// Helper to extract a component value from an EntityRecord.
fn get_component_value(record: &EntityRecord, component: &str) -> u32 {
    match component {
        "Monitor" => record.monitor,
        "Workspace" => record.workspace,
        "Container" => record.container,
        "Window" => record.window,
        _ => 0,
    }
}
