use crate::serialization::AssetId;
use crate::serialization::font::FontDefinition;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AssetDefinition {
	pub id: AssetId,
	pub path: String,
}

pub type TextureDefinition = AssetDefinition;
pub type LevelDefinition = AssetDefinition;
pub type GuiDefinition = AssetDefinition;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameData {
    pub fonts: Vec<FontDefinition>,
    pub textures: Vec<TextureDefinition>,
    pub levels: Vec<LevelDefinition>,
	pub guis: Vec<GuiDefinition>,
    pub debug_font_id: AssetId,
}
