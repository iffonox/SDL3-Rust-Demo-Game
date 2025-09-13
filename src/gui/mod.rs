pub mod constraints;

use std::collections::HashMap;
use sdl3::pixels::Color;
use sdl3::render::{FPoint, FRect};
use serde::{Deserialize, Serialize};
use crate::actions::Action;
use crate::game_object::drawable::DrawLayer;
use crate::mouse::Mouse;
use crate::serialization::{AssetBounds, AssetColor, AssetId};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Handler {
	Action(Action),
	#[serde(with = "AssetColor")]
	SetBackgroundColor(Color),
	#[serde(with = "AssetColor")]
	SetTextColor(Color),
	SetText(String),
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Event {
	MouseDown,
	MouseUp
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Align {
	Start,
	Center,
	End
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct TextFormat {
	pub font_id: AssetId,
	#[serde(with = "AssetColor")]
	pub color: Color,
	pub justify: Align,
	pub align: Align,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BoxElement {
	pub id: AssetId,
	#[serde(with = "AssetBounds")]
	pub bounds: FRect,
	pub z: DrawLayer,
	#[serde(with = "AssetColor")]
	pub bg: Color,
	#[serde(default)]
	pub on_event: HashMap<Event, Vec<Handler>>,
	#[serde(default)]
	pub children: Vec<UiElement>,
}

pub type FormattedString = String;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LabelElement {
	pub id: AssetId,
	#[serde(with = "AssetBounds")]
	pub bounds: FRect,
	pub z: DrawLayer,
	#[serde(with = "AssetColor")]
	pub bg: Color,
	pub text: FormattedString,
	pub format: TextFormat,
	#[serde(default)]
	pub on_event: HashMap<Event, Vec<Handler>>,
	#[serde(default)]
	pub children: Vec<UiElement>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum UiElement {
	Box(BoxElement),
	Label(LabelElement),
}

impl UiElement {
	pub fn element_with_id(&self, id: AssetId) -> Option<&UiElement>
	{
		if let Some(self_id) = self.get_id() && id == self_id {
			return Some(self);
		}

		let children: &Vec<UiElement> = self.get_children();

		for i in 0..children.len() {
			let child = &children[i];

			if let Some(element) = child.element_with_id(id) {
				return Some(element);
			}
		}

		None
	}

	pub fn get_children(&self) -> &Vec<UiElement>
	{
		match self {
			UiElement::Box(r#box) => { &r#box.children },
			UiElement::Label(label) => { &label.children },
		}
	}

	pub fn get_id(&self) -> Option<AssetId> {
		match self {
			UiElement::Box(r#box) => { Some(r#box.id) },
			UiElement::Label(label) => { Some(label.id) },
		}
	}
	
	fn get_elements_at(&self, pos: FPoint) -> Vec<UiElement>
	{
		
		Vec::new()
	}
	
	pub fn handle_event(&mut self, mouse: Mouse) -> Option<Action>
	{
		
		
		None
	}
}