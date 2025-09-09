use sdl3::pixels::Color;
use sdl3::render::FRect;

#[derive(Debug, Clone)]
pub struct Label {
	pub text: String,
	pub bounds: FRect,
	pub z: i32,
	pub fg: Color,
	pub bg: Color,
}
