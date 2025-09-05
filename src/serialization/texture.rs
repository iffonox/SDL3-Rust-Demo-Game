use crate::serialization::AssetId;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TextureDefinition {
    pub id: AssetId,
    pub path: String,
}
