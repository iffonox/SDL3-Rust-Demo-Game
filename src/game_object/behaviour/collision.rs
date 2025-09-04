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
	fn tick(&mut self, params: BehaviourParameter, _: f64) -> BehaviourResult {
		let mut collisions: Vec<(i32, FRect)> = Vec::new();

		self.bounds = params.bounds;

		for i in 0..params.other_bounds.len() {
			let (id, rect) = params.other_bounds[i];

			if id == params.id {
				continue
			}

			if rect.intersects(self.bounds) {
				collisions.push((id, rect))
			}
		}

		BehaviourResult {
			bounds: None,
			collisions: Some(collisions)
		}
	}
}
