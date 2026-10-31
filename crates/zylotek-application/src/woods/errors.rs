use zylotek_domain::{
    errors::RepositoryError,
    woods::{entities::WoodId, errors::WoodError, services::WoodServiceError},
};
use zylotek_infrastructure::event_bus::EventPublishError;

#[derive(Debug, thiserror::Error)]
pub enum WoodUseCaseError {
    #[error("wood {0} not found")]
    NotFound(WoodId),

    #[error(transparent)]
    Domain(#[from] WoodError),

    #[error("failed to persist wood")]
    Persistence(#[source] RepositoryError),

    #[error("failed to publish wood event")]
    Event(#[source] EventPublishError),
}

impl From<WoodServiceError> for WoodUseCaseError {
    fn from(error: WoodServiceError) -> Self {
        match error {
            WoodServiceError::Domain(error) => Self::Domain(error),
            WoodServiceError::Repository(error) => Self::Persistence(error),
        }
    }
}
