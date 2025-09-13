pub mod font;
pub mod game;
pub mod level;
pub mod texture;

use crate::math::vector2::Vector2;
use bitmask_enum::bitmask;
use sdl3::pixels::Color;
use sdl3::render::FRect;
use serde::Deserialize;

pub type AssetId = i32;

pub type AssetPosition = Vector2<f32>;

#[derive(Deserialize, Debug, Clone, Copy, Default)]
pub struct AssetSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Deserialize, Debug, Clone, Copy, Default)]
#[serde(remote = "FRect")]
pub struct AssetBounds {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Deserialize, Debug, Clone, Copy, Default)]
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

#[bitmask(u32)]
pub enum Action {
    None = 0x0000,
    Quit = 0x0001,
    Debug = 0x0002,
    FpsLimit = 0x0004,
    MoveLeft = 0x0008,
    MoveRight = 0x0010,
    MoveUp = 0x0020,
    MoveDown = 0x0040,
    Duck = 0x0080,
    Jump = 0x0100,
    Sprint = 0x0200,
    Attack = 0x0400,
    Menu = 0x0800,
    Click = 0x1000,
}
