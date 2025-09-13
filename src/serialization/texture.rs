use crate::serialization::AssetId;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct TextureDefinition {
    pub id: AssetId,
    pub path: String,
}
