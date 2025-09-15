use serde::{Deserialize, Serialize};

pub type Pixels = u16;
pub type Fps = u16;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
	pub width: Pixels,
	pub height: Pixels,
	pub frame_limit_active: bool,
	pub frame_limit: Fps,
	pub asset_file: String
}
