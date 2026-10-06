use zylotek_domain::woods::entities as domain;

use crate::persistence::woods::models::Wood;

impl From<Wood> for domain::Wood {
    fn from(model: Wood) -> Self {
        Self {
            id: domain::WoodId::from_uuid(model.id),
            name: model.name,
            scientific_name: model.scientific_name,
        }
    }
}
