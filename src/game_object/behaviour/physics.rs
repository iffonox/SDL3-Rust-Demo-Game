use crate::game_object::PhysicsVector;
use crate::game_object::behaviour::{Behaviour, BehaviourParameter, BehaviourResult};
use crate::math::bounds::Bounds;
use sdl3::render::FRect;
use crate::math::VectorOps;

pub struct PhysicsBehaviour {
    bounds: FRect,
    speed: PhysicsVector,
	mass: f32,
}

impl PhysicsBehaviour {
    pub fn new(bounds: FRect, speed: PhysicsVector, mass: f32) -> Self {
        Self { bounds, speed, mass }
    }
}

static GRAVITY: PhysicsVector = PhysicsVector {
	x: 0.0,
	y: 100.0,
};

static AIR_RESISTANCE_COEF: f32 = 0.001;

impl Behaviour for PhysicsBehaviour {
    fn tick(&mut self, params: BehaviourParameter, delta_t: f64) -> BehaviourResult {
        let sec = delta_t as f32;
        let center = params.bounds.center();
        let mut position = PhysicsVector::from(center);
		let speed_magnitude = self.speed.len();
		let speed_anti_normal = -self.speed.normal();

		let force_accel = if let Some(force) = params.force { force / self.mass } else { PhysicsVector::default() };
		let air_resistance_accel = if speed_magnitude != 0.0 { speed_anti_normal * (AIR_RESISTANCE_COEF * speed_magnitude * speed_magnitude) } else { PhysicsVector::default() };

		dbg!(air_resistance_accel);

		let mut acceleration = GRAVITY;
		acceleration += air_resistance_accel;
		acceleration += force_accel;

		assert!(!position.x.is_nan());

		self.speed += acceleration * sec;

        position += self.speed * sec;

		position.x = position.x.clamp(self.bounds.left(), self.bounds.right());
		position.y = position.y.clamp(self.bounds.top(), self.bounds.bottom());

        let mut bounds = params.bounds;

        bounds.set_center(position);

        BehaviourResult {
            bounds: Some(bounds),
            collisions: None,
			force: None,
        }
    }
}
