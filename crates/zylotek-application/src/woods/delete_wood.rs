use std::sync::Arc;

use zylotek_domain::woods::{entities::WoodId, events::WoodEvent, repositories::WoodRepository};
use zylotek_infrastructure::event_bus::EventPublisher;

use crate::woods::errors::WoodUseCaseError;

pub struct DeleteWood {
    event_publisher: Arc<dyn EventPublisher<WoodEvent>>,
    wood_repository: Arc<dyn WoodRepository>,
}

impl DeleteWood {
    pub fn new(
        event_publisher: Arc<dyn EventPublisher<WoodEvent>>,
        wood_repository: Arc<dyn WoodRepository>,
    ) -> Self {
        Self {
            event_publisher,
            wood_repository,
        }
    }

    pub async fn execute(&self, id: WoodId) -> Result<(), WoodUseCaseError> {
        let wood = self
            .wood_repository
            .find_by_id(id)
            .await
            .map_err(WoodUseCaseError::Persistence)?
            .ok_or(WoodUseCaseError::NotFound(id))?;

        let event = wood.delete();

        self.wood_repository
            .delete(id)
            .await
            .map_err(WoodUseCaseError::Persistence)?;

        self.event_publisher
            .publish(event)
            .await
            .map_err(WoodUseCaseError::Event)?;

        Ok(())
    }
}
