pub mod constraints;

use std::collections::HashMap;
use sdl3::pixels::Color;
use sdl3::render::{FPoint, FRect};
use serde::{Deserialize, Serialize};
use crate::actions::Action;
use crate::game_object::drawable::DrawLayer;
use crate::math::bounds::Bounds;
use crate::mouse::{Mouse, MouseButtonState};
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
	MouseUp,
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

pub type FormattedString = String;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ElementType {
	Box,
	Label {
		text: FormattedString,
		format: TextFormat,
	}
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UiElement {
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
	#[serde(default)]
	pub mouse: Mouse,

	pub element_type: ElementType
}

impl UiElement {
	pub fn update_element(&mut self, element: UiElement) {
		if self.id == element.id {
			self.mouse = element.mouse;
			self.bg = element.bg;
			self.element_type = element.element_type;

			return;
		}

		for i in 0..self.children.len() {
			let child = &mut self.children[i];

			if child.id == element.id || child.has_child_with_id(element.id) {
				child.update_element(element);
				return;
			}
		}
	}

	pub fn has_child_with_id(&self, id: AssetId) -> bool
	{
		for i in 0..self.children.len() {
			let child = &self.children[i];

			if child.id == id && child.has_child_with_id(id) {
				return true;
			}
		}

		false
	}

	pub fn element_with_id(&self, id: AssetId) -> Option<&UiElement>
	{
		if id == self.id {
			return Some(self);
		}

		let children: &Vec<UiElement> = &self.children;

		for i in 0..children.len() {
			let child = &children[i];

			if let Some(element) = child.element_with_id(id) {
				return Some(element);
			}
		}

		None
	}
	
	fn get_elements_at(&self, pos: FPoint) -> Vec<&UiElement>
	{
		let mut elements = Vec::new();

		let bounds = self.bounds;

		if !bounds.contains(pos) {
			return elements;
		}

		elements.push(self);

		let children = &self.children;

		let child_pos = FPoint {
			x: pos.x - bounds.x,
			y: pos.y - bounds.y,
		};

		for i in 0..children.len() {
			let child = &children[i];
			let child_bounds = child.bounds;

			if !child_bounds.contains(child_pos) {
				continue
			}

			let matches = &mut child.get_elements_at(child_pos);

			elements.append(matches);
		}

		elements
	}

	fn get_element_at(&self, pos: FPoint) -> Option<&UiElement>
	{
		let bounds = self.bounds;

		if !bounds.contains(pos) {
			return None;
		}

		let children = &self.children;

		let child_pos = FPoint {
			x: pos.x - bounds.x,
			y: pos.y - bounds.y,
		};
		let mut elements = Vec::new();

		for i in 0..children.len() {
			let child = &children[i];
			let child_bounds = child.bounds;

			if !child_bounds.contains(child_pos) {
				continue
			}

			let matches = &mut child.get_elements_at(child_pos);

			elements.append(matches);
		}

		if elements.is_empty() {
			return Some(self);
		}

		elements.sort_by_key(|e| e.z);

		elements.last().map(|e| *e)
	}
	
	pub fn handle_event(&mut self, mouse: Mouse) -> Option<Action>
	{
		let mut e;

		if let Some(element) = self.get_element_at(mouse.pos) {
			e = element.clone();
		} else {
			return None;
		}

		let prev_state = e.mouse;

		e.mouse = mouse;

		let Some(event) = Self::get_event_type(&prev_state, &mouse) else {
			self.update_element(e);
			return None;
		};

		let on_event = &e.on_event;

		let Some(handlers) = on_event.get(&event) else {
			return None;
		};

		let mut res_action = None;

		for i in 0..handlers.len() {
			let handler = &handlers[i];

			match (handler, &mut e.element_type) {
				(Handler::Action(action), _) => { res_action = Some(*action) }
				(Handler::SetBackgroundColor(color), _) => { e.bg = *color }
				(Handler::SetTextColor(color), ElementType::Label { format, .. }) => {
					format.color = *color;
				}
				(Handler::SetText(new_text),  ElementType::Label { text, .. }) => {
					text.clear();
					text.push_str(new_text);
				}
				(_, _) => {}
			}
		}
		
		self.update_element(e);

		res_action
	}

	fn get_event_type(old_state: &Mouse, new_state: &Mouse) -> Option<Event> {
		let buttons_changes = old_state.buttons ^ new_state.buttons;

		if buttons_changes == MouseButtonState::NONE {
			return None;
		}

		if buttons_changes & new_state.buttons == MouseButtonState::NONE {
			Some(Event::MouseUp)
		} else {
			Some(Event::MouseDown)
		}
	}
}
