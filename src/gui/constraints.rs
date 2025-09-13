use sdl3::render::FRect;
use serde::Deserialize;
use crate::gui::UiElement;
use crate::serialization::{AssetBounds, AssetId};

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Anchor {
	Top,
	Bottom,
	Leading,
	Trailing,
	Width,
	Height,
	CenterVertically,
	CenterHorizontally,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
	Equal,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,
	None,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Constraint {
	pub own_anchor: Anchor,
	pub other_anchor: Option<Anchor>,
	pub other_id: Option<AssetId>,
	pub relation: Relation,
	pub factor: f32,
	pub constant: f32,
}

impl Constraint {
	pub fn to_asset_bounds(&self, root_view: &UiElement) -> Result<AssetBounds, String> {
		todo!();

		if self.relation != Relation::None {
			if self.other_anchor.is_none() || self.other_id.is_none() {
				return Err(String::from("missing anchor"))
			}
		}

		let mut x = 0.0;
		let mut y = 0.0;
		let mut w = 0.0;
		let mut h = 0.0;

		Ok(AssetBounds {
			x,
			y,
			w,
			h,
		})
	}
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ConstraintSet {
	pub constraint: [Constraint; 4]
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(tag = "type")]
pub enum Frame {
	#[serde(with = "AssetBounds")]
	Rect(FRect),
	Constraint(ConstraintSet)
}
