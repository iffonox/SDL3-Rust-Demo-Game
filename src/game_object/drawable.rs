use std::cmp::Ordering;
use sdl3::pixels::Color;
use serde::{Deserialize, Deserializer};
use crate::serialization::{AssetColor, AssetId};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawLayer {
	Background(i32),
	Foreground(i32),
}

impl PartialOrd for DrawLayer {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(DrawLayer::Background(..), DrawLayer::Foreground(..)) => Some(Ordering::Less),
			(DrawLayer::Foreground(..), DrawLayer::Background(..)) => Some(Ordering::Greater),
			(DrawLayer::Background(i), DrawLayer::Background(j)) => i.partial_cmp(j),
			(DrawLayer::Foreground(i), DrawLayer::Foreground(j)) => i.partial_cmp(j),
		}
	}
}

impl Ord for DrawLayer {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Drawable {
	pub z: DrawLayer,
	#[serde(default, deserialize_with = "_de_optional_color")]
	pub color: Option<Color>,
	pub texture_id: Option<AssetId>, // index of the texture
	#[serde(default)]
	pub tint_texture: bool,       // if the texture should be tinted by the color
}

impl Default for Drawable {
	fn default() -> Self {
		Self {
			z: DrawLayer::Background(0),
			color: None,
			texture_id: None,
			tint_texture: bool::default(),
		}
	}
}

fn _de_optional_color<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error> where D: Deserializer<'de>
{
	Ok(AssetColor::deserialize(deserializer).ok())
}
