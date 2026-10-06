use std::sync::Arc;

use thiserror::Error;

use crate::{
    errors::RepositoryError,
    woods::{entities::Wood, errors::WoodError, events::WoodEvent, repositories::WoodRepository},
};

pub struct WoodCreationService {
    wood_repository: Arc<dyn WoodRepository>,
}

impl WoodCreationService {
    pub fn new(wood_repository: Arc<dyn WoodRepository>) -> Self {
        Self { wood_repository }
    }

    pub async fn create(
        &self,
        name: String,
        scientific_name: String,
    ) -> Result<(Wood, WoodEvent), WoodCreationError> {
        let exists = self
            .wood_repository
            .exists_by_scientific_name(&scientific_name)
            .await
            .map_err(WoodCreationError::Repository)?;

        if exists {
            return Err(WoodCreationError::Domain(
                WoodError::ScientificNameAlreadyExists { scientific_name },
            ));
        }

        Wood::create(name, scientific_name).map_err(WoodCreationError::Domain)
    }
}

#[derive(Error, Debug)]
pub enum WoodCreationError {
    #[error(transparent)]
    Domain(#[from] WoodError),

    #[error(transparent)]
    Repository(#[from] RepositoryError),
}
