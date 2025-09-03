pub mod dvd;
pub mod controllable;
pub mod collision;

use sdl3::render::FRect;
use crate::game::Action;
use crate::game_data::AssetId;

#[derive(Clone, Copy)]
pub struct BehaviourParameter<'a> {
	pub id: AssetId,
	pub bounds: FRect,
	pub actions: Action,
	pub other_bounds: &'a Vec<(i32, FRect)>,
}

pub struct BehaviourResult {
	pub bounds: Option<FRect>,
	pub collisions: Option<Vec<(i32, FRect)>>
}

pub trait Behaviour {
	fn tick(&mut self, params: BehaviourParameter, delta_t: u64) -> BehaviourResult;
}
