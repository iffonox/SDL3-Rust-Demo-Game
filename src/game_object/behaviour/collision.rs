use sdl3::render::FRect;
use crate::game_object::behaviour::{Behaviour, BehaviourParameter, BehaviourResult};
use crate::math::bounds::Bounds;

pub struct CollisionBehaviour {
	bounds: FRect,
}

impl CollisionBehaviour {
	pub fn new(bounds: FRect) -> Self {
		Self {
			bounds
		}
	}
}

impl Behaviour for CollisionBehaviour {
	fn tick(&mut self, params: BehaviourParameter, delta_t: u64) -> BehaviourResult {
		let mut collisions: Vec<(i32, FRect)> = Vec::new();
		let rect_count = params.other_bounds.len();

		for i in 0..rect_count {
			let (id1, rect1) = params.other_bounds[i];

			if !rect1.intersects(self.bounds) {
				continue
			}

			for j in 0..i {
				let (id2, rect2) = params.other_bounds[i];

				if id1 == id2 {
					continue
				}

				if rect1.intersects(rect2) {
					collisions.push((id1, rect1))
				}
			}
		}

		BehaviourResult {
			bounds: None,
			collisions: Some(collisions)
		}
	}
}
