use crate::serialization::behaviour::BehaviourType;
use crate::serialization::{AssetBounds, AssetColor, AssetId, AssetPosition, AssetSize};
use serde::Deserialize;

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
    pub color: Option<AssetColor>,
    pub bounds: AssetBounds,
    pub behaviours: Vec<BehaviourType>,
}

#[derive(Deserialize, Debug)]
pub struct LevelData {
    pub name: String,
    pub start: AssetPosition,
    pub bounds: Option<AssetBounds>,
    pub player: Player,
    pub objects: Vec<LevelObject>,
}
