use crate::game_object::PhysicsVector;
use crate::game_object::behaviour::{Behaviour, BehaviourParameter, BehaviourResult};
use crate::math::bounds::Bounds;
use sdl3::render::FRect;

pub struct DvdBehaviour {
    bounds: FRect,
    speed: PhysicsVector,
}

impl DvdBehaviour {
    pub fn new(bounds: FRect, speed: PhysicsVector) -> Self {
        Self { bounds, speed }
    }
}

impl Behaviour for DvdBehaviour {
    fn tick(&mut self, params: BehaviourParameter, delta_t: f64) -> BehaviourResult {
        let sec = delta_t as f32;
        let center = params.bounds.center();
        let mut position = PhysicsVector::from(center);

        position = position + self.speed * sec;

        if position.x < self.bounds.left() {
            self.speed.x = -self.speed.x;
            position.x = self.bounds.left();
        } else if position.x > self.bounds.right() {
            self.speed.x = -self.speed.x;
            position.x = self.bounds.right();
        }

        if position.y < self.bounds.top() {
            self.speed.y = -self.speed.y;
            position.y = self.bounds.top();
        } else if position.y > self.bounds.bottom() {
            self.speed.y = -self.speed.y;
            position.y = self.bounds.bottom();
        }

        let mut bounds = params.bounds;

        bounds.set_center(position);

        BehaviourResult {
            bounds: Some(bounds),
            collisions: None,
			force: None,
			impulse: None,
        }
    }
}
