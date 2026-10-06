use std::sync::Arc;

use zylotek_domain::woods::entities::Wood;

use crate::woods::create_wood::{CreateWood, CreateWoodCommand, CreateWoodError};

#[derive(Clone)]
pub struct WoodFacade {
    create_wood: Arc<CreateWood>,
}

impl WoodFacade {
    pub fn new(create_wood: CreateWood) -> Self {
        Self {
            create_wood: Arc::new(create_wood),
        }
    }

    pub async fn create_wood(&self, command: CreateWoodCommand) -> Result<Wood, CreateWoodError> {
        self.create_wood.execute(command).await
    }
}
