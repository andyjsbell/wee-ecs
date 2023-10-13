mod math;
mod prelude;
mod tests;

use lazy_static::lazy_static;
use math::*;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Mutex;

// Bit index, covers up to 128 bits(0-127) or 128 components
type BitIndex = i8;
type Component = dyn Any;

pub struct Entity<T, U: BitSet> {
    id: T,
    bitmap: U,
    components: Vec<Box<Component>>,
}

trait ComponentOps {
    fn add<R: Register>(self, component: Box<Component>) -> Self;
    fn add_many<R: Register>(self, components: Vec<Box<Component>>) -> Self;
}

impl<T, U: BitSet> Entity<T, U> {
    fn new(id: T) -> Self {
        let this = Self {
            id,
            bitmap: U::initialise(),
            components: Default::default(),
        };
        this
        // this.add_many::<R>(components)
    }
}

impl<T, U: BitSet> ComponentOps for Entity<T, U> {
    fn add<R: Register>(mut self, component: Box<Component>) -> Self {
        if let Some(id) = R::get_id(component.deref().type_id()) {
            if let Ok(_) = self.bitmap.set(id as u8) {
                self.components.push(component);
            }
        }

        self
    }

    fn add_many<R: Register>(mut self, components: Vec<Box<Component>>) -> Self {
        for component in components {
            self = self.add::<R>(component);
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

    fn spawn(self, components: Vec<Box<Component>>) -> Self;

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

    fn spawn(mut self, components: Vec<Box<Component>>) -> Self {
        let new_entity_id = self.entity_count.increment();
        let mut entity = Entity::new(new_entity_id);
        entity = entity.add_many::<Self>(components);
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

pub type ComponentSet8 = u8;
pub type ComponentSet16 = u16;
pub type ComponentSet32 = u32;
pub type ComponentSet64 = u64;
pub type ComponentSet128 = u128;

lazy_static! {
    static ref WORLD_STATE: Mutex<HashMap<TypeId, BitIndex>> = Mutex::new(HashMap::new());
    static ref INDEX: Mutex<BitIndex> = Mutex::new(0);
}

// A world where we have 8 components
pub type PrimitiveWorld = GenericWorld<u8, ComponentSet8>;
// A world where we have 128 components
pub type World = GenericWorld<u128, ComponentSet128>;

pub trait Register {
    fn register<A: Any>();
    fn get_id(type_id: TypeId) -> Option<BitIndex>;
}

fn register_state<A: Any>(state: &mut HashMap<TypeId, BitIndex>) {
    if !(*state).contains_key(&TypeId::of::<A>()) {
        let mut index = INDEX.lock().unwrap();
        state.insert(TypeId::of::<A>(), (*index).into());
        *index += 1;
    }
}

impl<T, U: BitSet> Register for GenericWorld<T, U> {
    fn register<A: Any>() {
        register_state::<A>(&mut (*WORLD_STATE.lock().unwrap()));
    }

    fn get_id(type_id: TypeId) -> Option<BitIndex> {
        WORLD_STATE.lock().unwrap().get(&type_id).map(|id| *id)
    }
}
