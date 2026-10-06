use serde::{Deserialize, Serialize};
use zylotek_domain::woods::entities::Wood;

#[derive(Debug, Deserialize)]
pub struct CreateWoodRequest {
    pub name: String,
    pub scientific_name: String,
}

#[derive(Debug, Serialize)]
pub struct WoodResponse {
    pub id: String,
    pub name: String,
    pub scientific_name: String,
}

impl From<&Wood> for WoodResponse {
    fn from(wood: &Wood) -> Self {
        Self {
            id: wood.id().as_uuid().to_string(),
            name: wood.name().to_owned(),
            scientific_name: wood.scientific_name().to_string(),
        }
    }
}
