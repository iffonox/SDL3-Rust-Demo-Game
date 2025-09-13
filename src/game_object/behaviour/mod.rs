pub mod collision;
pub mod controllable;
pub mod dvd;
pub mod physics;

use crate::serialization::{Action, AssetBounds, AssetId};
use sdl3::render::FRect;
use serde::{Deserialize, Deserializer};
use crate::game_object::behaviour::collision::CollisionBehaviour;
use crate::game_object::behaviour::controllable::ControllableBehaviour;
use crate::game_object::behaviour::dvd::DvdBehaviour;
use crate::game_object::behaviour::physics::PhysicsBehaviour;
use crate::game_object::{BoundInfo, CollisionInfo, PhysicsVector};
use crate::util::random;

fn _de_optional_rect<'de, D>(deserializer: D) -> Result<Option<FRect>, D::Error> where D: Deserializer<'de>
{
	Ok(AssetBounds::deserialize(deserializer).ok())
}

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
	pub world_bounds: FRect,
    pub other_bounds: &'a Vec<BoundInfo>,
	pub collisions: &'a Vec<CollisionInfo>,
	pub force: Option<PhysicsVector>,
	pub impulse: Option<PhysicsVector>,
}

pub struct BehaviourResult {
    pub bounds: Option<FRect>,
    pub collisions: Option<Vec<CollisionInfo>>,
	pub force: Option<PhysicsVector>,
	pub impulse: Option<PhysicsVector>,
}

pub trait Behaviour {
    fn tick(&mut self, params: BehaviourParameter, delta_t: f64) -> BehaviourResult;
}

pub(crate) const fn _default_rect() -> FRect {
	FRect {
		x: 0.0,
		y: 0.0,
		w: 0.0,
		h: 0.0,
	}
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(tag = "type")]
pub enum BehaviourType {
	Dvd(DvdBehaviour),
	Controllable(ControllableBehaviour),
	Collision(CollisionBehaviour),
	Physics(PhysicsBehaviour)
}


impl BehaviourType {
	pub fn tick(&mut self, params: BehaviourParameter, delta_t: f64) -> BehaviourResult
	{
		match self {
			Self::Physics(behavior) => {
				behavior.tick(params, delta_t)
			}
			BehaviourType::Dvd(behavior) => {
				behavior.tick(params, delta_t)
			}
			BehaviourType::Controllable(behavior) => {
			behavior.tick(params, delta_t)
			}
			BehaviourType::Collision(behavior) => {
				behavior.tick(params, delta_t)
			}
		}
	}
}
