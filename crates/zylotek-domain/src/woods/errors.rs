use thiserror::Error;

use crate::errors::RepositoryError;

#[derive(Debug, Error)]
pub enum WoodError {
    #[error("wood scientific name '{scientific_name}' already exists")]
    ScientificNameAlreadyExists { scientific_name: String },

    #[error("repository error")]
    Repository(#[from] RepositoryError),
}
