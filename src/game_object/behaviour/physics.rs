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

static PIXELS_PER_METER: f32 = 32.0;
static METERS_PER_PIXEL: f32 = 1.0/PIXELS_PER_METER;

static GRAVITY: PhysicsVector = PhysicsVector {
	x: 0.0,
	y: 10.0,
};

static AIR_RESISTANCE_COEF: f32 = 0.01;

impl Behaviour for PhysicsBehaviour {
    fn tick(&mut self, params: BehaviourParameter, delta_t: f64) -> BehaviourResult {
        let sec = delta_t as f32;
		let bounds = params.bounds;
        let center = bounds.center();
        let mut position = PhysicsVector::from(center) * METERS_PER_PIXEL;
		let speed_magnitude = self.speed.len();
		let speed_anti_normal = -self.speed.normal();

		let impulse = if let Some(impulse) = params.impulse { impulse / self.mass } else { PhysicsVector::default() };
		let force_accel = if let Some(force) = params.force { force / self.mass } else { PhysicsVector::default() };
		let air_resistance_accel = if speed_magnitude != 0.0 { speed_anti_normal * (AIR_RESISTANCE_COEF * speed_magnitude * speed_magnitude) } else { PhysicsVector::default() };

		let mut acceleration = GRAVITY;
		acceleration += air_resistance_accel;
		acceleration += force_accel;

		assert!(!position.x.is_nan());

		self.speed += impulse;
		self.speed += acceleration * sec;

		for i in 0..params.collisions.len() {
			let (_, collision) = params.collisions[i];
			let col_center = collision.center();

			if collision.w > collision.h {
				let pos_sign = (col_center.y - center.y).signum();
				let speed_sign = self.speed.y.signum();

				if pos_sign == speed_sign {
					// cancel movement in the direction of the collision
					self.speed.y = 0.0;
				}
			} else {
				let pos_sign = (col_center.x - center.x).signum();
				let speed_sign = self.speed.x.signum();

				if pos_sign == speed_sign {
					// cancel movement in the direction of the collision
					self.speed.x = 0.0;
				}
			}
		}

		position += self.speed * sec;

		position *= PIXELS_PER_METER;
		position.x = position.x.clamp(self.bounds.left(), self.bounds.right());
		position.y = position.y.clamp(self.bounds.top(), self.bounds.bottom());

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
