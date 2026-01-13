use axum::{
    extract::State,
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tracing::info;

use crate::config::Config;
use goaties_common::{CreateFileRequest, FileMetadata, FilesResponse, HealthResponse};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
}

pub fn create_router(config: Arc<Config>) -> Router {
    let state = AppState { config: config.clone() };

    Router::new()
        .route(&format!("{}/health", config.server.rest_path), get(health))
        .route(&format!("{}/files", config.server.rest_path), get(list_files))
        .route(&format!("{}/files", config.server.rest_path), post(create_file))
        .with_state(state)
}

async fn health() -> Json<HealthResponse> {
    info!("Health check requested");
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "Goaties backend is running".to_string(),
    })
}

async fn list_files(State(_state): State<AppState>) -> Json<FilesResponse> {
    info!("List files requested");
    // TODO: Implement actual database query
    Json(FilesResponse {
        files: vec![],
        total: 0,
    })
}

async fn create_file(
    State(_state): State<AppState>,
    Json(req): Json<CreateFileRequest>,
) -> Json<FileMetadata> {
    info!("Create file requested: {:?}", req.name);
    // TODO: Implement actual database insert
    let mut file = FileMetadata::new(req.name, req.path, req.size);
    file.mime_type = req.mime_type;
    Json(file)
}

pub async fn serve(config: Arc<Config>) -> anyhow::Result<()> {
    let app = create_router(config.clone());
    let addr = format!("0.0.0.0:{}", config.server.rest_port);

    info!("REST server listening on {}{}", addr, config.server.rest_path);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}