use std::sync::Arc;

use zylotek_domain::woods::{entities::Wood, repositories::WoodRepository};

use crate::woods::errors::WoodUseCaseError;

pub const DEFAULT_LIMIT: usize = 20;
pub const MAX_LIMIT: usize = 100;

pub struct ListWoods {
    wood_repository: Arc<dyn WoodRepository>,
}

impl ListWoods {
    pub fn new(wood_repository: Arc<dyn WoodRepository>) -> Self {
        Self { wood_repository }
    }

    pub async fn execute(&self, query: ListWoodsQuery) -> Result<WoodPage, WoodUseCaseError> {
        let offset = query.offset.unwrap_or(0);
        let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);

        let items = self
            .wood_repository
            .list(offset, limit)
            .await
            .map_err(WoodUseCaseError::Persistence)?;

        let total = self
            .wood_repository
            .count()
            .await
            .map_err(WoodUseCaseError::Persistence)?;

        Ok(WoodPage {
            items,
            total,
            offset,
            limit,
        })
    }
}

#[derive(Debug, Default)]
pub struct ListWoodsQuery {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

pub struct WoodPage {
    pub items: Vec<Wood>,
    pub total: u64,
    pub offset: usize,
    pub limit: usize,
}
