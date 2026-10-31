use std::sync::Arc;

use tokio::net::TcpListener;
use zylotek_application::woods::{
    create_wood::CreateWood, delete_wood::DeleteWood, facade::WoodFacade, get_wood::GetWood,
    handlers::WoodEventHandler, list_woods::ListWoods, update_wood::UpdateWood,
};
use zylotek_domain::woods::{
    events::WoodEvent,
    services::{WoodCreationService, WoodUpdateService},
};
use zylotek_infrastructure::{
    event_bus::tokio::{TokioEventPublisher, run_event_handler},
    persistence::{build_db, woods::repositories::PostgresWoodRepository},
};

use crate::{router::router, state::AppState};

pub mod errors;
pub mod router;
pub mod state;
pub mod woods;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let db = build_db("sqlite::memory:").await?;

    let wood_event_publisher = Arc::new(TokioEventPublisher::<WoodEvent>::new(1024));
    let wood_event_handler = WoodEventHandler::new();
    let wood_repository = Arc::new(PostgresWoodRepository::new(db));

    let create_wood = CreateWood::new(
        wood_event_publisher.clone(),
        wood_repository.clone(),
        WoodCreationService::new(wood_repository.clone()),
    );
    let get_wood = GetWood::new(wood_repository.clone());
    let list_woods = ListWoods::new(wood_repository.clone());
    let update_wood = UpdateWood::new(
        wood_event_publisher.clone(),
        wood_repository.clone(),
        WoodUpdateService::new(wood_repository.clone()),
    );
    let delete_wood = DeleteWood::new(wood_event_publisher.clone(), wood_repository.clone());
    let wood_facade = WoodFacade::new(create_wood, get_wood, list_woods, update_wood, delete_wood);

    tokio::spawn(run_event_handler(
        wood_event_publisher.subscribe(),
        wood_event_handler,
    ));

    let state = AppState {
        woods: Arc::new(wood_facade),
    };

    let app = router(state);
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    log::info!("zylotek-api running...");
    axum::serve(listener, app).await?;

    Ok(())
}
