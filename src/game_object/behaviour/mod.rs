pub mod collision;
pub mod controllable;
pub mod dvd;
pub mod physics;

use crate::serialization::{Action, AssetId};
use sdl3::render::FRect;
use serde::Deserialize;
use crate::game_object::behaviour::collision::CollisionBehaviour;
use crate::game_object::behaviour::controllable::ControllableBehaviour;
use crate::game_object::behaviour::dvd::DvdBehaviour;
use crate::game_object::PhysicsVector;
use crate::util::random;

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(tag = "type")]
pub enum BehaviourSpeed {
	Fixed(PhysicsVector),
	Random {
		min: PhysicsVector,
		max: PhysicsVector,
	},
}

impl From<BehaviourSpeed> for PhysicsVector {
	fn from(value: BehaviourSpeed) -> Self {
		match value {
			BehaviourSpeed::Fixed(v) => v,
			BehaviourSpeed::Random { min, max } => Self {
				x: random(min.x, max.x),
				y: random(min.y, max.y),
			},
		}
	}
}

impl From<PhysicsVector> for BehaviourSpeed {
	fn from(value: PhysicsVector) -> Self {
		BehaviourSpeed::Fixed(value)
	}
}

#[derive(Clone, Copy)]
pub struct BehaviourParameter<'a> {
    pub id: AssetId,
    pub bounds: FRect,
    pub actions: Action,
    pub other_bounds: &'a Vec<(i32, FRect)>,
	pub collisions: &'a Vec<(i32, FRect)>,
	pub force: Option<PhysicsVector>,
	pub impulse: Option<PhysicsVector>,
}

pub struct BehaviourResult {
    pub bounds: Option<FRect>,
    pub collisions: Option<Vec<(i32, FRect)>>,
	pub force: Option<PhysicsVector>,
	pub impulse: Option<PhysicsVector>,
}

pub trait Behaviour {
    fn tick(&mut self, params: BehaviourParameter, delta_t: f64) -> BehaviourResult;
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum BehaviourType {
	Dvd(DvdBehaviour),
	Controllable(ControllableBehaviour),
	Collision(CollisionBehaviour),
}
