use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use zylotek_application::woods::{
    create_wood::CreateWoodCommand, errors::WoodUseCaseError, list_woods::ListWoodsQuery,
    update_wood::UpdateWoodCommand,
};
use zylotek_domain::woods::{entities::WoodId, errors::WoodError};

use crate::{
    errors::ApiError,
    state::AppState,
    woods::schemas::{ListWoodsParams, WoodPageResponse, WoodRequest, WoodResponse},
};

pub async fn create_wood_handler(
    State(state): State<AppState>,
    Json(request): Json<WoodRequest>,
) -> Result<(StatusCode, Json<WoodResponse>), ApiError> {
    let command = CreateWoodCommand {
        name: request.name,
        scientific_name: request.scientific_name,
        details: request.details.try_into().map_err(WoodUseCaseError::from)?,
    };

    let wood = state.woods.create_wood(command).await?;

    Ok((StatusCode::CREATED, Json(WoodResponse::from(&wood))))
}

pub async fn list_woods_handler(
    State(state): State<AppState>,
    Query(params): Query<ListWoodsParams>,
) -> Result<Json<WoodPageResponse>, ApiError> {
    let query = ListWoodsQuery {
        offset: params.offset,
        limit: params.limit,
    };

    let page = state.woods.list_woods(query).await?;

    Ok(Json(page.into()))
}

pub async fn get_wood_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<WoodResponse>, ApiError> {
    let wood = state.woods.get_wood(parse_id(&id)?).await?;

    Ok(Json(WoodResponse::from(&wood)))
}

pub async fn update_wood_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<WoodRequest>,
) -> Result<Json<WoodResponse>, ApiError> {
    let command = UpdateWoodCommand {
        id: parse_id(&id)?,
        name: request.name,
        scientific_name: request.scientific_name,
        details: request.details.try_into().map_err(WoodUseCaseError::from)?,
    };

    let wood = state.woods.update_wood(command).await?;

    Ok(Json(WoodResponse::from(&wood)))
}

pub async fn delete_wood_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    state.woods.delete_wood(parse_id(&id)?).await?;

    Ok(StatusCode::NO_CONTENT)
}

fn parse_id(id: &str) -> Result<WoodId, ApiError> {
    id.parse::<WoodId>()
        .map_err(|error| ApiError::BadRequest(error.to_string()))
}

impl From<WoodUseCaseError> for ApiError {
    fn from(error: WoodUseCaseError) -> Self {
        match error {
            WoodUseCaseError::NotFound(_) => ApiError::NotFound(error.to_string()),

            WoodUseCaseError::Domain(WoodError::Repository(error)) => {
                ApiError::Internal(anyhow::Error::new(error))
            }

            WoodUseCaseError::Domain(WoodError::InvalidId(_)) => {
                ApiError::BadRequest(error.to_string())
            }

            WoodUseCaseError::Domain(_) => ApiError::UnprocessableEntity(error.to_string()),

            WoodUseCaseError::Persistence(_) | WoodUseCaseError::Event(_) => {
                ApiError::Internal(anyhow::Error::new(error))
            }
        }
    }
}
