pub mod font;
pub mod game;
pub mod level;

use sdl3::render::FPoint;
use sdl3::pixels::Color;
use sdl3::render::FRect;
use serde::{Deserialize, Serialize};

pub type AssetId = i32;

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
#[serde(remote = "FPoint")]
pub struct AssetPosition {
	pub x: f32,
	pub y: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
pub struct AssetSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
#[serde(remote = "FRect")]
pub struct AssetBounds {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
#[serde(remote = "Color")]
pub struct AssetColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<AssetColor> for Color {
    fn from(value: AssetColor) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

impl From<Color> for AssetColor {
	fn from(value: Color) -> Self {
		Self {
			r: value.r,
			g: value.g,
			b: value.b,
			a: value.a,
		}
	}
}
