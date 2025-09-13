use serde::{Deserialize, Serialize};

pub type Pixels = u16;
pub type Fps = u16;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Settings {
	pub width: Pixels,
	pub height: Pixels,
	pub frame_limit_active: bool,
	pub frame_limit: Fps,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			width: 800,
			height: 600,
			frame_limit_active: true,
			frame_limit: 60,
		}
	}
}
