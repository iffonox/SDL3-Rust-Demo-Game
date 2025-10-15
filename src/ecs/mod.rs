pub mod component;
pub mod system;

use crate::ecs::component::{Component, ComponentTrait};
use crate::ecs::system::SystemTrait;
use std::collections::HashMap;

pub type EntityId = usize;
pub type ValueId = usize;
pub type ComponentId = usize;
pub type SystemId = usize;

pub struct ECS {
    entities: Vec<bool>,
    components: Vec<Box<dyn ComponentTrait>>,
    systems: Vec<Box<dyn SystemTrait>>,
    connections: HashMap<SystemId, Vec<ComponentId>>,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            entities: Default::default(),
            components: Default::default(),
            systems: Default::default(),
            connections: Default::default(),
        }
    }

    pub fn register_component<T>(&mut self) -> ComponentId
    where
        T: 'static,
    {
        self.components.push(Box::new(Component::<T>::new()));

        self.components.len() - 1
    }

    pub fn register_system<T>(&mut self, system: T) -> SystemId
    where
        T: 'static + SystemTrait,
    {
        self.systems.push(Box::new(system));

        self.systems.len() - 1
    }

    pub fn connect(&mut self, system_id: SystemId, component_id: ComponentId) {
        if self.connections.contains_key(&system_id) {
            let vec = &self.connections[&system_id];

            if !vec.contains(&component_id) {
                let mut new_vec = vec.clone();
                new_vec.push(component_id);
                self.connections.insert(system_id, new_vec);
            }
        } else {
            self.connections.insert(system_id, vec![component_id]);
        }
    }

    pub fn disconnect(&mut self, system_id: SystemId, component_id: ComponentId) {
        if self.connections.contains_key(&system_id) {
            let vec = &self.connections[&system_id];

            if let Some(index) = vec.iter().position(|v| *v == component_id) {
                let mut new_vec = vec.clone();
                new_vec.remove(index);
                self.connections.insert(system_id, new_vec);
            }
        }
    }

    pub fn create_entity(&mut self, component_ids: &[ComponentId]) -> EntityId {
        let len = self.entities.len();
        let mut entity_id = 0;
        let mut found = false;

        for i in 0..len {
            if !self.entities[i] {
                self.entities[i] = true;
                entity_id = i;
                found = true;
                break;
            }
        }

        if !found {
            self.entities.push(true);
        }

        for i in component_ids {
            self.components[*i].add_entity(entity_id);
        }

        entity_id
    }

    pub fn destroy_entity(&mut self, entity_id: EntityId) {
        if entity_id >= self.entities.len() {
            return;
        }
		
		self.entities[entity_id] = false;

        for i in 0..self.components.len() {
            self.components[i].remove_entity(entity_id);
        }
    }
}
