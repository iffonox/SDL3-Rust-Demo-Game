use crate::game_object::behaviour::{BehaviourParameter, BehaviourResult};
use crate::math::bounds::Bounds;
use serde::{Deserialize, Serialize};
use crate::game_object::{BoundInfo, ObjectMask};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct CollisionBehaviour {
	#[serde(default)]
	pub mask: ObjectMask,
}

impl CollisionBehaviour {
    pub fn new() -> Self {
        Self { 
			mask: ObjectMask::default()
		}
    }
}

impl CollisionBehaviour {
    pub fn tick(&mut self, params: BehaviourParameter, _: f64) -> BehaviourResult {
        let mut collisions: Vec<BoundInfo> = Vec::new();

        let bounds = params.bounds;

        for i in 0..params.other_bounds.len() {
            let (id, rect, mask) = params.other_bounds[i];

            if id == params.id {
                continue;
            }

			if mask != 0 && self.mask != 0 && mask & self.mask == 0 {
				continue;
			}
			
            if rect.intersects(bounds) {
                collisions.push((id, rect.intersection(bounds), mask))
            }
        }

        BehaviourResult {
            bounds: None,
            collisions: Some(collisions),
            force: None,
            impulse: None,
        }
    }
}
