use std::sync::Arc;

use zylotek_domain::woods::{
    entities::{Wood, WoodId},
    events::WoodEvent,
    repositories::WoodRepository,
    services::WoodUpdateService,
    value_objects::{ScientificName, WoodDetails, WoodName},
};
use zylotek_infrastructure::event_bus::EventPublisher;

use crate::woods::errors::WoodUseCaseError;

pub struct UpdateWood {
    event_publisher: Arc<dyn EventPublisher<WoodEvent>>,
    wood_repository: Arc<dyn WoodRepository>,
    wood_update_service: WoodUpdateService,
}

impl UpdateWood {
    pub fn new(
        event_publisher: Arc<dyn EventPublisher<WoodEvent>>,
        wood_repository: Arc<dyn WoodRepository>,
        wood_update_service: WoodUpdateService,
    ) -> Self {
        Self {
            event_publisher,
            wood_repository,
            wood_update_service,
        }
    }

    /// Replaces every attribute of the wood with the command's values.
    pub async fn execute(&self, command: UpdateWoodCommand) -> Result<Wood, WoodUseCaseError> {
        let name = WoodName::new(command.name)?;
        let scientific_name = ScientificName::new(command.scientific_name)?;

        let mut wood = self
            .wood_repository
            .find_by_id(command.id)
            .await
            .map_err(WoodUseCaseError::Persistence)?
            .ok_or(WoodUseCaseError::NotFound(command.id))?;

        let event = self
            .wood_update_service
            .update(&mut wood, name, scientific_name, command.details)
            .await?;

        let wood = self
            .wood_repository
            .update(&wood)
            .await
            .map_err(WoodUseCaseError::Persistence)?;

        self.event_publisher
            .publish(event)
            .await
            .map_err(WoodUseCaseError::Event)?;

        Ok(wood)
    }
}

pub struct UpdateWoodCommand {
    pub id: WoodId,
    pub name: String,
    pub scientific_name: String,
    pub details: WoodDetails,
}
