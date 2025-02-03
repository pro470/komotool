use super::registry::*;
use bevy::prelude::*;
use std::ops::Range;

pub struct RelationQueryBuilder<'a> {
    registry: &'a RelationRegistry,
    /// Instead of storing a HashSet of indices, we maintain a list of ranges.
    current_ranges: Vec<Range<usize>>,
    /// When true, the final execution expands ranges into a Vec<Entity>.
    require_ordering: bool,
}

impl<'a> RelationQueryBuilder<'a> {
    pub fn new(registry: &'a RelationRegistry) -> Self {
        Self {
            registry,
            current_ranges: vec![],
            require_ordering: false,
        }
    }

    /// Helper: Add all ranges corresponding to a tag.
    fn add_ranges_from_tag(&mut self, tag: &str) {
        if let Some(ranges) = self.registry.index.get(tag) {
            self.current_ranges.extend(ranges.iter().cloned());
            self.normalize_ranges();
        }
    }

    /// Merge overlapping or adjacent ranges.
    fn normalize_ranges(&mut self) {
        if self.current_ranges.is_empty() {
            return;
        }
        self.current_ranges.sort_by_key(|r| r.start);
        let mut merged = Vec::with_capacity(self.current_ranges.len());
        let mut current = self.current_ranges[0].clone();
        for next in &self.current_ranges[1..] {
            if let Some(unioned) = current.union(next) {
                current = unioned;
            } else {
                merged.push(current);
                current = next.clone();
            }
        }
        merged.push(current);
        self.current_ranges = merged;
    }

    /// OR: add ranges from the given tags.
    pub fn or(mut self, tags: &[&str]) -> Self {
        for tag in tags {
            self.add_ranges_from_tag(tag);
        }
        self
    }

    /// AND: intersect the current ranges with ranges corresponding to the given tags.
    pub fn and(mut self, tags: &[&str]) -> Self {
        for tag in tags {
            if let Some(ranges) = self.registry.index.get(*tag) {
                let mut new_ranges = Vec::new();
                for r in ranges {
                    for cur in &self.current_ranges {
                        if let Some(intersection) = cur.intersect(r) {
                            new_ranges.push(intersection);
                        }
                    }
                }
                self.current_ranges = new_ranges;
                self.normalize_ranges();
            } else {
                self.current_ranges.clear();
            }
        }
        self
    }

    /// WITH: if no current ranges exist, start with the tag's ranges; otherwise intersect.
    pub fn with(mut self, tag: &str) -> Self {
        if self.current_ranges.is_empty() {
            self.add_ranges_from_tag(tag);
        } else {
            self = self.and(&[tag]);
        }
        self
    }

    /// WITHOUT: subtract the ranges for the tag from the current ranges.
    pub fn without(mut self, tag: &str) -> Self {
        if let Some(tag_ranges) = self.registry.index.get(tag) {
            let tag_ranges: Vec<Range<usize>> = tag_ranges.clone();
            let mut new_ranges = Vec::new();
            for cur in self.current_ranges {
                let mut remaining = vec![cur];
                for tr in &tag_ranges {
                    remaining = remaining.into_iter().flat_map(|r| subtract_range(r, tr.clone())).collect();
                }
                new_ranges.extend(remaining);
            }
            self.current_ranges = new_ranges;
            self.normalize_ranges();
        }
        self
    }

    /// Set whether we require ordered output.
    pub fn require_ordering(mut self, ordering: bool) -> Self {
        self.require_ordering = ordering;
        self
    }

    /// Expand the current ranges into a Vec<Entity>.
    pub fn execute_entities(self) -> Vec<Entity> {
        let mut out = Vec::new();
        for range in self.current_ranges {
            for i in range {
                out.push(self.registry.records[i].entity);
            }
        }
        out
    }

    /// Optionally, return the current ranges directly.
    pub fn execute_ranges(self) -> Vec<Range<usize>> {
        self.current_ranges
    }
}

/// Subtract one range (subtrahend) from another (minuend), returning the remaining parts.
fn subtract_range(minuend: Range<usize>, subtrahend: Range<usize>) -> Vec<Range<usize>> {
    let mut result = Vec::new();
    if minuend.start < subtrahend.start && subtrahend.start < minuend.end {
        result.push(minuend.start..subtrahend.start);
    }
    if subtrahend.end < minuend.end && subtrahend.end > minuend.start {
        result.push(subtrahend.end..minuend.end);
    }
    if subtrahend.end <= minuend.start || subtrahend.start >= minuend.end {
        result.push(minuend);
    }
    result
}
