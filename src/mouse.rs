use crate::serialization::AssetPosition;
use bitmask_enum::bitmask;
use sdl3::render::FPoint;
use serde::{Deserialize, Serialize};

#[bitmask(usize)]
#[derive(Deserialize, Serialize)]
pub enum MouseButtonState {
	NONE = 0x0000,
	LEFT_BUTTON = 0x0001,
	RIGHT_BUTTON = 0x0002,
	MIDDLE_BUTTON = 0x0004,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Mouse {
	pub buttons: MouseButtonState,
	#[serde(with = "AssetPosition")]
	pub pos: FPoint
}

impl Default for Mouse {
	fn default() -> Self {
		Self {
			buttons: MouseButtonState::NONE,
			pos: FPoint { x: 0.0, y: 0.0 }
		}
	}
}
