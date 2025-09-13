use crate::game_object::behaviour::_ser_optional_rect;
use crate::game_object::behaviour::_de_optional_rect;
use crate::game_object::PhysicsVector;
use crate::game_object::behaviour::{Behaviour, BehaviourParameter, BehaviourResult, BehaviourSpeed};
use crate::math::bounds::Bounds;
use sdl3::render::FRect;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct DvdBehaviour {
	#[serde(default, deserialize_with = "_de_optional_rect", serialize_with = "_ser_optional_rect")]
    pub bounds: Option<FRect>,
	#[serde(deserialize_with = "_de_behavior_speed")]
    pub speed: PhysicsVector,
}

fn _de_behavior_speed<'de, D>(deserializer: D) -> Result<PhysicsVector, D::Error> where D: Deserializer<'de>
{
	let res = BehaviourSpeed::deserialize(deserializer);

	match res {
		Ok(speed) => Ok(PhysicsVector::from(speed)),
		Err(err) => Err(err)
	}
}

impl DvdBehaviour {
    pub fn new(bounds: FRect, speed: PhysicsVector) -> Self {
        Self { bounds: Some(bounds), speed }
    }
}

impl Behaviour for DvdBehaviour {
    fn tick(&mut self, params: BehaviourParameter, delta_t: f64) -> BehaviourResult {
        let center = params.bounds.center();
        let mut position = PhysicsVector::from(center);
		let clamp_bounds = self.bounds.unwrap_or(params.world_bounds);

		position += self.speed * delta_t as f32;

        if position.x < clamp_bounds.left() {
            self.speed.x = -self.speed.x;
        } else if position.x > clamp_bounds.right() {
            self.speed.x = -self.speed.x;
        }

        if position.y < clamp_bounds.top() {
            self.speed.y = -self.speed.y;
        } else if position.y > clamp_bounds.bottom() {
            self.speed.y = -self.speed.y;
        }

		position.x = position.x.clamp(clamp_bounds.left(), clamp_bounds.right());
		position.y = position.y.clamp(clamp_bounds.top(), clamp_bounds.bottom());

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
