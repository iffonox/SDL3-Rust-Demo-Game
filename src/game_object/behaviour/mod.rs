pub mod collision;
pub mod controllable;
pub mod dvd;
pub mod physics;

use crate::serialization::{Action, AssetId};
use sdl3::render::FRect;
use crate::game_object::PhysicsVector;

#[derive(Clone, Copy)]
pub struct BehaviourParameter<'a> {
    pub id: AssetId,
    pub bounds: FRect,
    pub actions: Action,
    pub other_bounds: &'a Vec<(i32, FRect)>,
	pub collisions: &'a Vec<(i32, FRect)>,
	pub force: Option<PhysicsVector>
}

pub struct BehaviourResult {
    pub bounds: Option<FRect>,
    pub collisions: Option<Vec<(i32, FRect)>>,
	pub force: Option<PhysicsVector>
}

pub trait Behaviour {
    fn tick(&mut self, params: BehaviourParameter, delta_t: f64) -> BehaviourResult;
}
