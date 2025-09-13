use crate::serialization::AssetId;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct FontDefinition {
    pub id: AssetId,
    pub path: String,
    pub size: f32,
}
