#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("database error")]
    Database,

    #[error("unique constraint violation")]
    Duplicate,

    #[error("repository unavailable")]
    Unavailable,

    #[error("repository operation failed")]
    OperationFailed,
}
