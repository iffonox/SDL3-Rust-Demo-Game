use crate::serialization::AssetId;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FontDefinition {
    pub id: AssetId,
    pub path: String,
    pub size: f32,
}
