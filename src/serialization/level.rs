use crate::serialization::{AssetBounds, AssetId, AssetPosition, AssetSize};
use sdl3::render::FRect;
use serde::{Deserialize, Serialize};
use crate::game_object::GameObject;

#[derive(Deserialize, Serialize, Debug)]
pub struct Player {
    pub texture_id: AssetId,
    pub size: AssetSize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LevelData {
    pub name: String,
    pub start: AssetPosition,
    #[serde(with = "AssetBounds")]
    pub bounds: FRect,
    pub player: Player,
    pub objects: Vec<GameObject>,
}
