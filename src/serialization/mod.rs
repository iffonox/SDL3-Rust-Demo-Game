pub mod behaviour;
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

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct AssetSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct AssetBounds {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl From<FRect> for AssetBounds {
    fn from(value: FRect) -> Self {
        Self {
            x: value.x,
            y: value.y,
            w: value.w,
            h: value.h,
        }
    }
}

impl From<AssetBounds> for FRect {
    fn from(value: AssetBounds) -> Self {
        Self {
            x: value.x,
            y: value.y,
            w: value.w,
            h: value.h,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct AssetColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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
    None = 0,
    Quit = 1,
    Debug = 2,
    FpsLimit = 4,
    MoveLeft = 8,
    MoveRight = 16,
    MoveUp = 32,
    MoveDown = 64,
    Duck = 128,
    Jump = 256,
    Sprint = 512,
    Attack = 1024,
	Menu = 2048,
	Click = 4096,
}
