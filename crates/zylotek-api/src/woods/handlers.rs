use axum::{Json, extract::State, http::StatusCode};
use zylotek_application::woods::create_wood::{CreateWoodCommand, CreateWoodError};
use zylotek_domain::woods::{errors::WoodError, services::WoodCreationError};

use crate::{
    errors::ApiError,
    state::AppState,
    woods::schemas::{CreateWoodRequest, WoodResponse},
};

pub async fn create_wood_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateWoodRequest>,
) -> Result<(StatusCode, Json<WoodResponse>), ApiError> {
    let command = CreateWoodCommand {
        name: request.name,
        scientific_name: request.scientific_name,
    };

    let wood = state
        .woods
        .create_wood(command)
        .await
        .map_err(ApiError::from)?;

    Ok((StatusCode::CREATED, Json(WoodResponse::from(&wood))))
}

impl From<CreateWoodError> for ApiError {
    fn from(error: CreateWoodError) -> Self {
        match error {
            CreateWoodError::Domain(WoodCreationError::Domain(
                WoodError::ScientificNameAlreadyExists { scientific_name: _ },
            )) => ApiError::UnprocessableEntity,

            CreateWoodError::Persistence(error) => ApiError::Internal(anyhow::Error::new(error)),

            CreateWoodError::Domain(WoodCreationError::Repository(error)) => {
                ApiError::Internal(anyhow::Error::new(error))
            }

            _ => ApiError::Internal(anyhow::Error::new(error)),
        }
    }
}
