#[derive(Debug, thiserror::Error)]
pub enum PostgresRepositoryError {
    #[error("database query failed")]
    Query(#[source] toasty::Error),

    #[error("database connection failed")]
    Connection(#[source] toasty::Error),
}
