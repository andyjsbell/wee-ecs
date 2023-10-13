mod math;
mod prelude;
mod tests;

use math::*;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::Hash;

pub trait Component {
    fn id(&self) -> u8;
    fn mask(&self) -> u8 {
        1 << self.id()
    }
}

pub struct Entity<T, U: BitSet> {
    id: T,
    bitmap: U,
    components: Vec<Box<dyn Component>>,
}

trait ComponentOps {
    fn add(self, component: Box<dyn Component>) -> Self;
    fn add_many(self, components: Vec<Box<dyn Component>>) -> Self;
}

impl<T, U: BitSet> Entity<T, U> {
    fn new(id: T, components: Vec<Box<dyn Component>>) -> Self {
        let this = Self {
            id,
            bitmap: U::initialise(),
            components: Default::default(),
        };

        this.add_many(components)
    }
}

impl<T, U: BitSet> ComponentOps for Entity<T, U> {
    fn add(mut self, component: Box<dyn Component>) -> Self {
        if let Ok(_) = self.bitmap.set(component.id()) {
            self.components.push(component);
        }

        self
    }

    fn add_many(mut self, components: Vec<Box<dyn Component>>) -> Self {
        for component in components {
            if let Ok(_) = self.bitmap.set(component.id()) {
                self.components.push(component);
            }
        }
        self
    }
}
#[derive(Default)]
pub struct GenericWorld<T, U: BitSet> {
    name: &'static str,
    entity_count: T,
    entities: HashMap<T, Entity<T, U>>,
    systems: Vec<(U, fn(U, Vec<&Entity<T, U>>))>,
    next_system: usize,
}

pub trait EntityOps<T, U: BitSet> {
    fn get_entity(&self, id: T) -> Option<&Entity<T, U>>;

    fn spawn(self, components: Vec<Box<dyn Component>>) -> Self;

    fn spawn_empty(self) -> Self;

    fn despawn(self, entity: &Entity<T, U>) -> Self;
}

impl<T, U: BitSet> EntityOps<T, U> for GenericWorld<T, U>
where
    T: Increment + Default + Eq + Hash,
{
    fn get_entity(&self, id: T) -> Option<&Entity<T, U>> {
        self.entities.get(&id)
    }

    fn spawn(mut self, components: Vec<Box<dyn Component>>) -> Self {
        let new_entity_id = self.entity_count.increment();
        let entity = Entity::new(new_entity_id, components);
        self.entities.insert(new_entity_id, entity);
        self
    }

    fn spawn_empty(self) -> Self {
        Self::spawn(self, vec![])
    }

    fn despawn(mut self, entity: &Entity<T, U>) -> Self {
        self.entities.remove(&entity.id);
        self
    }
}

pub trait WorldOps<T, U: BitSet> {
    fn new(name: &'static str) -> Self;
    fn query(&self, mask: U) -> Vec<&Entity<T, U>>;
    fn name(&self) -> &'static str;
}

impl<T, U> WorldOps<T, U> for GenericWorld<T, U>
where
    T: Increment + Default + Eq + Hash,
    U: BitSet + Default,
{
    fn new(name: &'static str) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    fn query(&self, mask: U) -> Vec<&Entity<T, U>> {
        self.entities
            .iter()
            .filter_map(|(_, entity)| entity.bitmap.contains(mask).then(|| entity))
            .collect()
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

pub trait System<T, U: BitSet> {
    fn add_system(&mut self, mask: U, f: fn(mask: U, Vec<&Entity<T, U>>));
    fn next(&self) -> Option<(U, fn(U, Vec<&Entity<T, U>>))>;
}

impl<T, U: BitSet> System<T, U> for GenericWorld<T, U> {
    fn add_system(&mut self, mask: U, f: fn(U, Vec<&Entity<T, U>>)) {
        self.systems.push((mask, f));
    }

    fn next(&self) -> Option<(U, fn(U, Vec<&Entity<T, U>>))> {
        if self.systems.len() > self.next_system {
            let system = self.systems[self.next_system];
            return Some(system);
        }
        None
    }
}

pub trait Runnable<T, U: BitSet>
where
    Self: System<T, U> + WorldOps<T, U>,
{
    fn run(&self) {
        if let Some((mask, f)) = self.next() {
            let entities = self.query(mask);
            if entities.len() > 0 {
                f(mask, entities);
            }
        }
    }
}

impl<T, U: BitSet + Default> Runnable<T, U> for GenericWorld<T, U> where
    T: Default + Eq + Hash + Increment
{
}

pub trait Register<T> {
    fn register<A: Any>();
    fn get_id(type_id: TypeId) -> Option<T>;
}

type PrimitiveComponentSet = u8;
type WorldComponentSet = u128;

pub type PrimitiveWorld = GenericWorld<u8, PrimitiveComponentSet>;
pub type World = GenericWorld<u128, WorldComponentSet>;
