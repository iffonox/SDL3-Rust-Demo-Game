use crate::serialization::{AssetBounds, AssetColor, AssetId, AssetPosition, AssetSize};
use sdl3::pixels::Color;
use sdl3::render::FRect;
use serde::Deserialize;
use crate::game_object::behaviour::BehaviourType;

#[derive(Deserialize, Debug)]
pub struct Player {
    pub texture_id: AssetId,
    pub size: AssetSize,
}

#[derive(Deserialize, Debug)]
pub struct LevelObject {
    pub id: AssetId,
    pub tint_texture: Option<bool>,
    pub texture_id: Option<AssetId>,
    #[serde(default, with = "AssetColor")]
    pub color: Option<Color>,
    #[serde(with = "AssetBounds")]
    pub bounds: FRect,
    pub behaviours: Vec<BehaviourType>,
}

#[derive(Deserialize, Debug)]
pub struct LevelData {
    pub name: String,
    pub start: AssetPosition,
    #[serde(default, with = "AssetBounds")]
    pub bounds: Option<FRect>,
    pub player: Player,
    pub objects: Vec<LevelObject>,
}
