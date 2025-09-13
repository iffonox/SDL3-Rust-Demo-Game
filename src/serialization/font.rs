use crate::serialization::AssetId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FontDefinition {
    pub id: AssetId,
    pub path: String,
    pub size: f32,
}
