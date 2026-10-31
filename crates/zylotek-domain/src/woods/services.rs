use std::sync::Arc;

use thiserror::Error;

use crate::{
    errors::RepositoryError,
    woods::{
        entities::Wood,
        errors::WoodError,
        events::WoodEvent,
        repositories::WoodRepository,
        value_objects::{ScientificName, WoodDetails, WoodName},
    },
};

/// Creates woods, enforcing that a scientific name is registered only once.
pub struct WoodCreationService {
    wood_repository: Arc<dyn WoodRepository>,
}

impl WoodCreationService {
    pub fn new(wood_repository: Arc<dyn WoodRepository>) -> Self {
        Self { wood_repository }
    }

    pub async fn create(
        &self,
        name: WoodName,
        scientific_name: ScientificName,
        details: WoodDetails,
    ) -> Result<(Wood, WoodEvent), WoodServiceError> {
        ensure_scientific_name_available(self.wood_repository.as_ref(), &scientific_name).await?;

        Ok(Wood::create(name, scientific_name, details))
    }
}

/// Updates woods, enforcing that a renamed scientific name is not taken.
pub struct WoodUpdateService {
    wood_repository: Arc<dyn WoodRepository>,
}

impl WoodUpdateService {
    pub fn new(wood_repository: Arc<dyn WoodRepository>) -> Self {
        Self { wood_repository }
    }

    pub async fn update(
        &self,
        wood: &mut Wood,
        name: WoodName,
        scientific_name: ScientificName,
        details: WoodDetails,
    ) -> Result<WoodEvent, WoodServiceError> {
        if wood.scientific_name() != &scientific_name {
            ensure_scientific_name_available(self.wood_repository.as_ref(), &scientific_name)
                .await?;
        }

        Ok(wood.update(name, scientific_name, details))
    }
}

async fn ensure_scientific_name_available(
    wood_repository: &dyn WoodRepository,
    scientific_name: &ScientificName,
) -> Result<(), WoodServiceError> {
    let exists = wood_repository
        .exists_by_scientific_name(scientific_name.as_str())
        .await
        .map_err(WoodServiceError::Repository)?;

    if exists {
        return Err(WoodServiceError::Domain(
            WoodError::ScientificNameAlreadyExists {
                scientific_name: scientific_name.to_string(),
            },
        ));
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum WoodServiceError {
    #[error(transparent)]
    Domain(#[from] WoodError),

    #[error(transparent)]
    Repository(#[from] RepositoryError),
}
