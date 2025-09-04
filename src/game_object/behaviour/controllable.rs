use sdl3::render::FRect;
use crate::game_data::Action;
use crate::game_object::behaviour::{Behaviour, BehaviourParameter, BehaviourResult};
use crate::game_object::PhysicsVector;
use crate::math::bounds::Bounds;

pub struct ControllableBehaviour {
	bounds: FRect,
	speed: f32,
	run_speed: f32,
}

impl ControllableBehaviour {
	pub fn new(bounds: FRect, speed: f32, run_speed: f32) -> Self {
		Self {
			bounds,
			speed,
			run_speed
		}
	}
}

impl Behaviour for ControllableBehaviour {
	fn tick(&mut self, params: BehaviourParameter, delta_t: f64) -> BehaviourResult {
		let sec = delta_t as f32;
		let actions = params.actions;
		let center = params.bounds.center();
		let mut position = PhysicsVector::from(center);
		let speed = if actions.contains(Action::Sprint) { self.run_speed } else { self.speed };

		if actions.contains(Action::MoveUp) {
			position.y -= speed * sec;
		} else if actions.contains(Action::MoveDown) {
			position.y += speed * sec;
		}

		if actions.contains(Action::MoveLeft) {
			position.x -= speed * sec;
		} else if actions.contains(Action::MoveRight) {
			position.x += speed * sec;
		}

		position.x = position.x.clamp(self.bounds.left(), self.bounds.right());
		position.y = position.y.clamp(self.bounds.top(), self.bounds.bottom());

		let mut bounds = params.bounds;

		bounds.set_center(position);

		BehaviourResult {
			bounds: Some(bounds),
			collisions: None,
		}
	}
}