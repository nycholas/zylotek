use std::sync::Arc;

use zylotek_domain::woods::{
    entities::Wood,
    events::WoodEvent,
    repositories::WoodRepository,
    services::WoodCreationService,
    value_objects::{ScientificName, WoodDetails, WoodName},
};
use zylotek_infrastructure::event_bus::EventPublisher;

use crate::woods::errors::WoodUseCaseError;

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

    pub async fn execute(&self, command: CreateWoodCommand) -> Result<Wood, WoodUseCaseError> {
        let name = WoodName::new(command.name)?;
        let scientific_name = ScientificName::new(command.scientific_name)?;

        let (wood, event) = self
            .wood_creation_service
            .create(name, scientific_name, command.details)
            .await?;

        let wood = self
            .wood_repository
            .save(&wood)
            .await
            .map_err(WoodUseCaseError::Persistence)?;

        self.event_publisher
            .publish(event)
            .await
            .map_err(WoodUseCaseError::Event)?;

        Ok(wood)
    }
}

pub struct CreateWoodCommand {
    pub name: String,
    pub scientific_name: String,
    pub details: WoodDetails,
}
