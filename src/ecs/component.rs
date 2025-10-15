use std::collections::HashMap;
use crate::ecs::{EntityId, ValueId};

pub trait ComponentTrait {
	fn add_entity(&mut self, entity_id: EntityId);
	fn remove_entity(&mut self, entity_id: EntityId);
}

pub struct Component<T> {
	values: Vec<T>,
	active_indices: Vec<bool>,
	entity_to_value_map: HashMap<EntityId, ValueId>,
	value_to_entity_map: HashMap<ValueId, EntityId>,
}

impl<T> Component<T> {
	pub fn new() -> Self {
		Self {
			values: Default::default(),
			active_indices: Default::default(),
			entity_to_value_map: Default::default(),
			value_to_entity_map: Default::default(),
		}
	}
}

impl<T> ComponentTrait for Component<T> {
	fn add_entity(&mut self, entity_id: EntityId) {
		let len = self.active_indices.len();
		let mut value_id = 0;
		let mut found = false;

		for i in 0..len {
			if !self.active_indices[i] {
				self.active_indices[i] = true;
				value_id = i;
				found = true;
				break;
			}
		}

		if !found {
			self.active_indices.push(true);
		}

		self.entity_to_value_map.insert(entity_id, value_id);
		self.value_to_entity_map.insert(value_id, entity_id);
	}

	fn remove_entity(&mut self, entity_id: EntityId) {
		if !self.value_to_entity_map.contains_key(&entity_id) {
			return;
		}

		let value_id = self.entity_to_value_map[&entity_id];

		self.entity_to_value_map.remove(&entity_id);
		self.value_to_entity_map.remove(&value_id);
		self.active_indices[value_id] = false;
	}
}