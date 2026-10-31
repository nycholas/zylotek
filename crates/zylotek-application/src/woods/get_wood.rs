use std::sync::Arc;

use zylotek_domain::woods::{
    entities::{Wood, WoodId},
    repositories::WoodRepository,
};

use crate::woods::errors::WoodUseCaseError;

pub struct GetWood {
    wood_repository: Arc<dyn WoodRepository>,
}

impl GetWood {
    pub fn new(wood_repository: Arc<dyn WoodRepository>) -> Self {
        Self { wood_repository }
    }

    pub async fn execute(&self, id: WoodId) -> Result<Wood, WoodUseCaseError> {
        self.wood_repository
            .find_by_id(id)
            .await
            .map_err(WoodUseCaseError::Persistence)?
            .ok_or(WoodUseCaseError::NotFound(id))
    }
}
