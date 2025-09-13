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

pub struct Mouse {
	pub buttons: MouseButtonState,
	pub pos: FPoint
}
