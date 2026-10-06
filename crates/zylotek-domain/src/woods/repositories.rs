use crate::{errors::RepositoryError, woods::entities::Wood};

#[async_trait::async_trait]
pub trait WoodRepository: Send + Sync {
    async fn exists_by_scientific_name(
        &self,
        scientific_name: &str,
    ) -> Result<bool, RepositoryError>;

    async fn save(&self, wood: &Wood) -> Result<Wood, RepositoryError>;
}
