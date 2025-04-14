use bevy_ecs::entity::Entity;
use bevy_ecs::system::Resource;
use bevy_reflect::Reflect;
use indexmap::IndexSet;
use std::borrow::Borrow;
use std::cmp::{Eq, Ordering};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Reflect, Copy)]
pub struct EntityRecord {
    pub entity: Entity,
    pub monitor: usize,
    pub workspace: usize,
    pub container: usize,
    pub window: usize,
}


// Hash implementation - only hashes the entity field
impl Hash for EntityRecord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entity.hash(state);
    }
}

// PartialEq implementation - equality based only on entity
impl PartialEq for EntityRecord {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

// Eq marker trait (required for Hash)
impl Eq for EntityRecord {}

// Borrow implementation - allows direct lookup with &Entity
impl Borrow<Entity> for EntityRecord {
    fn borrow(&self) -> &Entity {
        &self.entity
    }
}

impl PartialOrd for EntityRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EntityRecord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.monitor
            .cmp(&other.monitor)
            .then(self.workspace.cmp(&other.workspace))
            .then(self.container.cmp(&other.container))
            .then(self.window.cmp(&other.window))
    }
}

impl EntityRecord {
    /// The canonical key used for ordering.
    pub fn key(&self) -> (usize, usize, usize, usize) {
        (self.monitor, self.workspace, self.container, self.window)
    }
}

#[derive(Default, Debug, Clone, Reflect, PartialEq, Eq, Resource)]
pub struct RelationRegistry {
    /// The set of all records.
    #[reflect(ignore)]
    pub records: IndexSet<EntityRecord>,
}

impl RelationRegistry {
    pub fn insert(
        &mut self,
        entity: Entity,
        monitor: usize,
        workspace: usize,
        container: usize,
        window: usize,
    ) {
        let record = EntityRecord {
            entity,
            monitor,
            workspace,
            container,
            window,
        };

        // Maintain sorted order using IndexSet's insertion order
        self.records.insert(record);
        self.records.sort_unstable();
    }
}
