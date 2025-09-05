use crate::serialization::AssetId;
use crate::serialization::font::FontDefinition;
use crate::serialization::texture::TextureDefinition;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LevelDefinition {
    pub id: AssetId,
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct GameData {
    pub fonts: Vec<FontDefinition>,
    pub textures: Vec<TextureDefinition>,
    pub levels: Vec<LevelDefinition>,
    pub debug_font_id: AssetId,
}
