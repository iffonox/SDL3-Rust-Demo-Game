use crate::game_object::PhysicsVector;
use crate::math::vector2::Vector2;
use sdl3::pixels::Color;
use sdl3::render::FRect;
use serde::Deserialize;

pub type AssetId = i32;

pub type AssetPosition = Vector2<f32>;

#[derive(Deserialize, Debug)]
pub struct FontDefinition {
    pub id: AssetId,
    pub path: String,
    pub size: f32,
}

#[derive(Deserialize, Debug)]
pub struct TextureDefinition {
    pub id: AssetId,
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct LevelDefinition {
    pub id: AssetId,
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct AssetSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct Player {
    pub texture_id: AssetId,
    pub size: AssetSize,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum BehaviourSpeed {
    Fixed(PhysicsVector),
    Random {
        min: PhysicsVector,
        max: PhysicsVector,
    },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum BehaviourType {
    Dvd {
        bounds: Option<AssetBounds>,
        use_world_bounds: Option<bool>,
        speed: BehaviourSpeed,
    },
    Controllable {
        bounds: Option<AssetBounds>,
        use_world_bounds: Option<bool>,
        speed: f32,
        run_speed: f32,
    },
    Collision {
        bounds: Option<AssetBounds>,
        use_world_bounds: Option<bool>,
    },
}

#[derive(Deserialize, Debug)]
pub struct LevelObject {
    pub id: AssetId,
    pub tint_texture: Option<bool>,
    pub texture_id: Option<AssetId>,
    pub color: Option<AssetColor>,
    pub bounds: AssetBounds,
    pub behaviours: Vec<BehaviourType>,
}

#[derive(Deserialize, Debug)]
pub struct GameData {
    pub fonts: Vec<FontDefinition>,
    pub textures: Vec<TextureDefinition>,
    pub levels: Vec<LevelDefinition>,
    pub player: Player,
    pub debug_font_id: AssetId,
}

#[derive(Deserialize, Debug)]
pub struct LevelData {
    pub name: String,
    pub start: AssetPosition,
    pub bounds: AssetBounds,
    pub objects: Vec<LevelObject>,
}
