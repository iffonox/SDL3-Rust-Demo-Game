use sdl3::mouse::SystemCursor::No;
use serde::Deserialize;
use crate::serialization::{AssetBounds, AssetColor, AssetId};

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(tag = "type")]
pub enum Anchor {
	Top,
	Bottom,
	Leading,
	Trailing,
	Width,
	Height,
	VerticalCenter,
	HorizontalCenter,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(tag = "type")]
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
	Rect(AssetBounds),
	Constraint(ConstraintSet)
}

#[derive(Debug, Clone, Deserialize)]
pub struct BoxElement {
	pub id: AssetId,
	pub bounds: AssetBounds,
	pub z: i32,
	pub bg: AssetColor,
	pub children: Vec<UiElement>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LabelElement {
	pub id: AssetId,
	pub bounds: AssetBounds,
	pub z: i32,
	pub fg: AssetColor,
	pub bg: AssetColor,
	pub text: String,
	pub children: Vec<UiElement>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum UiElement {
	Box(BoxElement),
	Label(LabelElement),
}


impl UiElement {
	pub fn element_with_id(&self, id: AssetId) -> Option<UiElement>
	{
		let children: &Vec<UiElement> = self.get_children();

		None
	}

	pub fn get_children(&self) -> &Vec<UiElement>
	{
		match self {
			UiElement::Box(r#box) => { &r#box.children },
			UiElement::Label(label) => { &label.children },
		}
	}
}
