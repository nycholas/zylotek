#[derive(Debug, thiserror::Error)]
pub enum InfrastructureError {
    #[error("database error: {0}")]
    Database(#[from] anyhow::Error),

    #[error("fatal error: {0}")]
    Fatal(String),
}
