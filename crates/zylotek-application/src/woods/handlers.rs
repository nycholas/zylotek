use zylotek_domain::woods::events::WoodEvent;
use zylotek_infrastructure::event_bus::{EventHandler, EventHandlerError};

pub struct WoodEventHandler;

impl WoodEventHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for WoodEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl EventHandler<WoodEvent> for WoodEventHandler {
    async fn handle(&self, event: WoodEvent) -> Result<(), EventHandlerError> {
        match event {
            WoodEvent::WoodCreated { wood_id } => {
                let id = wood_id.as_uuid().to_string();
                log::info!("wood {id} was created");
                Ok(())
            }
        }
    }
}
