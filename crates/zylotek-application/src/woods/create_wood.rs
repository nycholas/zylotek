use std::sync::Arc;

use zylotek_domain::{
    errors::RepositoryError,
    woods::{
        entities::Wood,
        events::WoodEvent,
        repositories::WoodRepository,
        services::{WoodCreationError, WoodCreationService},
    },
};
use zylotek_infrastructure::event_bus::{EventPublishError, EventPublisher};

pub struct CreateWood {
    event_publisher: Arc<dyn EventPublisher<WoodEvent>>,
    wood_repository: Arc<dyn WoodRepository>,
    wood_creation_service: WoodCreationService,
}

impl CreateWood {
    pub fn new(
        event_publisher: Arc<dyn EventPublisher<WoodEvent>>,
        wood_repository: Arc<dyn WoodRepository>,
        wood_creation_service: WoodCreationService,
    ) -> Self {
        Self {
            event_publisher,
            wood_repository,
            wood_creation_service,
        }
    }

    pub async fn execute(&self, command: CreateWoodCommand) -> Result<Wood, CreateWoodError> {
        let (wood, event) = self
            .wood_creation_service
            .create(command.name, command.scientific_name)
            .await
            .map_err(CreateWoodError::Domain)?;

        self.wood_repository
            .save(&wood)
            .await
            .map_err(CreateWoodError::Persistence)?;

        self.event_publisher
            .publish(event)
            .await
            .map_err(CreateWoodError::Event)?;

        Ok(wood)
    }
}

pub struct CreateWoodCommand {
    pub name: String,
    pub scientific_name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateWoodError {
    #[error(transparent)]
    Domain(#[from] WoodCreationError),

    #[error("failed to persist wood")]
    Persistence(#[source] RepositoryError),

    #[error("failed to publish wood event")]
    Event(#[source] EventPublishError),
}
