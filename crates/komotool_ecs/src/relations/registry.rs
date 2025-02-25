use bevy_ecs::entity::Entity;
use bevy_reflect::Reflect;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::Range;

pub trait RangeExt {
    /// Returns the intersection of two ranges, or None if they do not overlap.
    fn intersect(&self, other: &Self) -> Option<Range<usize>>;
    /// Returns the union of two ranges if they overlap or are adjacent.
    fn union(&self, other: &Self) -> Option<Range<usize>>;
}

impl RangeExt for Range<usize> {
    fn intersect(&self, other: &Self) -> Option<Range<usize>> {
        let start = max(self.start, other.start);
        let end = min(self.end, other.end);
        if start < end {
            Some(start..end)
        } else {
            None
        }
    }

    fn union(&self, other: &Self) -> Option<Range<usize>> {
        if self.end >= other.start && other.end >= self.start {
            Some(self.start.min(other.start)..self.end.max(other.end))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct EntityRecord {
    pub entity: Entity,
    pub monitor: u32,
    pub workspace: u32,
    pub container: u32,
    pub window: u32,
}

impl EntityRecord {
    /// The canonical key used for ordering.
    pub fn key(&self) -> (u32, u32, u32, u32) {
        (self.monitor, self.workspace, self.container, self.window)
    }
}

#[derive(Default, Debug, Clone, Reflect)]
pub struct RelationRegistry {
    /// Our master list of records, maintained in sorted order.
    pub records: Vec<EntityRecord>,
    /// Mapping from an Entity to its index in `records`.
    pub entity_to_index: HashMap<Entity, usize>,
    /// Secondary index mapping tag (e.g. "Monitor=1") to one or more ranges in `records`.
    pub index: HashMap<String, Vec<Range<usize>>>,
}

impl RelationRegistry {
    /// Insert a new record and then re-sort and rebuild indexes.
    pub fn insert(
        &mut self,
        entity: Entity,
        monitor: u32,
        workspace: u32,
        container: u32,
        window: u32,
    ) {
        self.records.push(EntityRecord {
            entity,
            monitor,
            workspace,
            container,
            window,
        });
        self.resort_and_rebuild();
    }

    /// Resort the Vec and rebuild the auxiliary indexes.
    pub fn resort_and_rebuild(&mut self) {
        self.records.sort_unstable_by(|a, b| a.key().cmp(&b.key()));

        self.entity_to_index.clear();
        for (i, record) in self.records.iter().enumerate() {
            self.entity_to_index.insert(record.entity, i);
        }

        self.rebuild_range_index();
    }

    /// Rebuild the secondary index mapping each tag to contiguous ranges.
    pub fn rebuild_range_index(&mut self) {
        self.index.clear();
        self.build_range_index_for_component("Monitor", |r: &EntityRecord| r.monitor);
        self.build_range_index_for_component("Workspace", |r: &EntityRecord| r.workspace);
        self.build_range_index_for_component("Container", |r: &EntityRecord| r.container);
        self.build_range_index_for_component("Window", |r: &EntityRecord| r.window);
    }

    fn build_range_index_for_component<F>(&mut self, comp: &str, getter: F)
    where
        F: Fn(&EntityRecord) -> u32,
    {
        let mut current_tag: Option<String> = None;
        let mut range_start: Option<usize> = None;

        for (i, record) in self.records.iter().enumerate() {
            let tag = format!("{}={}", comp, getter(record));
            if current_tag.as_ref() != Some(&tag) {
                if let (Some(prev_tag), Some(start)) = (current_tag.take(), range_start) {
                    self.index.entry(prev_tag).or_default().push(start..i);
                }
                current_tag = Some(tag);
                range_start = Some(i);
            }
        }

        if let (Some(tag_val), Some(start)) = (current_tag, range_start) {
            self.index
                .entry(tag_val)
                .or_default()
                .push(start..self.records.len());
        }
    }

    /// Update a component and then re-sort/rebuild.
    pub fn update_component(&mut self, entity: Entity, tag: &str) {
        if let Some((component, value_str)) = tag.split_once('=') {
            if let Ok(new_val) = value_str.parse::<u32>() {
                if let Some(&idx) = self.entity_to_index.get(&entity) {
                    let record = &mut self.records[idx];
                    match component {
                        "Monitor" => record.monitor = new_val,
                        "Workspace" => record.workspace = new_val,
                        "Container" => record.container = new_val,
                        "Window" => record.window = new_val,
                        _ => {}
                    }
                    self.resort_and_rebuild();
                }
            }
        }
    }
}
