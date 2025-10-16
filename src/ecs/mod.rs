pub mod component;
pub mod system;

use crate::ecs::component::{Component, ComponentTrait};
use crate::ecs::system::SystemTrait;
use std::any::{TypeId};
use std::collections::HashMap;

pub type EntityId = usize;
pub type ValueId = usize;
pub type ComponentId = TypeId;
pub type SystemId = usize;

pub struct ECS {
    entities: Vec<bool>,
    components: HashMap<ComponentId, Box<dyn ComponentTrait>>,
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
        T: 'static + Sized,
    {
        let component_id = TypeId::of::<T>();

        self.components
            .insert(component_id.clone(), Box::new(Component::<T>::new()));

        component_id
    }

    pub fn register_system<T>(&mut self, system: T) -> SystemId
    where
        T: 'static + SystemTrait,
    {
        self.systems.push(Box::new(system));

        self.systems.len() - 1
    }

    pub fn connect_system<T>(&mut self, system_id: SystemId)
    where
        T: 'static + Sized,
    {
        if system_id >= self.systems.len() {
            return;
        }

        let component_id = TypeId::of::<T>();

        if self.connections.contains_key(&system_id) {
            self.connections
                .iter_mut()
                .filter(|connection| connection.0 == &system_id)
                .for_each(|connection| {
                    if !connection.1.contains(&component_id) {
                        connection.1.push(component_id);
                    }
                });
        } else {
            self.connections.insert(system_id, vec![component_id]);
        }
    }

    pub fn disconnect_system<T>(&mut self, system_id: SystemId)
    where
        T: 'static + Sized,
    {
        if system_id >= self.systems.len() {
            return;
        }

        let component_id = TypeId::of::<T>();

        self.connections
            .iter_mut()
            .filter(|connection| connection.0 == &system_id)
            .for_each(|connection| {
                if let Some(index) = connection.1.iter().position(|v| v == &component_id) {
                    connection.1.remove(index);
                }
            });
    }

    pub fn create_entity(&mut self) -> EntityId {
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

        entity_id
    }

    pub fn destroy_entity(&mut self, entity_id: EntityId) {
        if entity_id >= self.entities.len() {
            return;
        }

        self.entities[entity_id] = false;

        self.components
            .iter_mut()
            .for_each(|v| v.1.remove_entity(entity_id));
    }

    pub fn connect_entity<T>(&mut self, entity_id: EntityId)
    where
        T: 'static + Sized,
    {
        if entity_id >= self.entities.len() {
            return;
        }

        let component_id = TypeId::of::<T>();

        self.components
            .iter_mut()
            .filter(|v| &component_id == v.0)
            .for_each(|v| v.1.add_entity(entity_id));
    }

    pub fn disconnect_entity<T>(&mut self, entity_id: EntityId)
    where
        T: 'static + Sized,
    {
        if entity_id >= self.entities.len() {
            return;
        }

        let component_id = TypeId::of::<T>();

        self.components
            .iter_mut()
            .filter(|v| &component_id == v.0)
            .for_each(|v| v.1.remove_entity(entity_id));
    }

    pub fn tick(&mut self, delta_t: u64) {
        self.systems.iter_mut().for_each(|system| {});
    }
}

#[cfg(test)]
mod tests {
    use crate::ecs::ECS;
    use crate::ecs::system::SystemTrait;
    use sdl3::render::FRect;
    use std::any::TypeId;

    struct TestSystem {
        last_tick: u64,
    }

    impl TestSystem {
        fn new() -> Self {
            Self {
                last_tick: Default::default(),
            }
        }
    }

    impl SystemTrait for TestSystem {
        fn tick(&mut self, delta_t: u64) {
            self.last_tick = delta_t;
        }
    }

    #[test]
    fn test_register_component() {
        let mut ecs = ECS::new();

        let id = ecs.register_component::<i32>();
        let _ = ecs.register_component::<(i32, f32)>();
        let _ = ecs.register_component::<FRect>();

        let tuple_key = TypeId::of::<(i32, f32)>();
        let struct_key = TypeId::of::<FRect>();

        assert_eq!(ecs.components.len(), 3);
        assert!(ecs.components.contains_key(&tuple_key));
        assert!(ecs.components.contains_key(&struct_key));
        assert!(ecs.components.contains_key(&id));
    }

    #[test]
    fn test_register_system() {
        let mut ecs = ECS::new();

        let id = ecs.register_system(TestSystem::new());
        let _ = ecs.register_system(TestSystem::new());
        let _ = ecs.register_system(TestSystem::new());

        assert_eq!(ecs.systems.len(), 3);
    }

    #[test]
    fn test_connect_system() {
        let mut ecs = ECS::new();

        let system1_id = ecs.register_system(TestSystem::new());
        let system2_id = ecs.register_system(TestSystem::new());
        let system3_id = ecs.register_system(TestSystem::new());

        let i32_component_id = ecs.register_component::<i32>();
        let frect_component_id = ecs.register_component::<FRect>();

        ecs.connect_system::<i32>(system1_id);

        ecs.connect_system::<FRect>(system2_id);

        ecs.connect_system::<i32>(system3_id);
        ecs.connect_system::<FRect>(system3_id);

        assert_eq!(ecs.connections.len(), 3);

        assert!(ecs.connections.contains_key(&system1_id));
        assert!(ecs.connections.contains_key(&system2_id));
        assert!(ecs.connections.contains_key(&system3_id));

        assert_eq!(ecs.connections[&system1_id].len(), 1);
        assert_eq!(ecs.connections[&system2_id].len(), 1);
        assert_eq!(ecs.connections[&system3_id].len(), 2);

        assert!(ecs.connections[&system1_id].contains(&i32_component_id));
        assert!(ecs.connections[&system2_id].contains(&frect_component_id));
        assert!(ecs.connections[&system3_id].contains(&i32_component_id));
        assert!(ecs.connections[&system3_id].contains(&frect_component_id));
    }

    #[test]
    fn test_disconnect_system() {
        let mut ecs = ECS::new();

        let system1_id = ecs.register_system(TestSystem::new());

        let i32_component_id = ecs.register_component::<i32>();
        let frect_component_id = ecs.register_component::<FRect>();

        ecs.connect_system::<i32>(system1_id);
        ecs.connect_system::<FRect>(system1_id);

        ecs.disconnect_system::<i32>(system1_id);

        assert_eq!(ecs.connections.len(), 1);
        assert!(ecs.connections.contains_key(&system1_id));
        assert_eq!(ecs.connections[&system1_id].len(), 1);
        assert!(ecs.connections[&system1_id].contains(&frect_component_id));
        assert!(!ecs.connections[&system1_id].contains(&i32_component_id));
    }

    #[test]
    fn test_create_entity() {}

    #[test]
    fn test_destroy_entity() {}

    #[test]
    fn test_connect_entity() {}

    #[test]
    fn test_disconnect_entity() {}

    #[test]
    fn test_tick() {}
}
