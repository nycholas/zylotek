use std::sync::Arc;

use tokio::{net::TcpListener, signal};
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

use crate::{errors::ApiError, router::router, state::AppState};

pub mod errors;
pub mod middleware;
pub mod router;
pub mod state;
pub mod woods;

async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .json()
        .init();

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

    print_banner();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

pub fn cli(args: Vec<&str>) -> Result<(), ApiError> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| ApiError::Internal(anyhow::Error::new(e)))?;

    let mut args: Vec<String> = args.into_iter().map(|str| str.to_string()).collect();
    let _ = args.insert_mut(0, String::from("--"));

    rt.block_on(run()).map_err(ApiError::Internal)
}

fn print_banner() {
    let banner = r#"
            _       _       _                  _
           | |     | |     | |                (_)
  _____   _| | ___ | |_ ___| | __   __ _ _ __  _
 |_  / | | | |/ _ \| __/ _ \ |/ /  / _` | '_ \| |
  / /| |_| | | (_) | ||  __/   <  | (_| | |_) | |
 /___|\__, |_|\___/ \__\___|_|\_\  \__,_| .__/|_|
       __/ |                            | |
      |___/                             |_|
    "#;
    println!("{}", banner);
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
