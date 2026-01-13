mod config;
mod rest;
mod websocket;

use anyhow::Result;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "goaties_backend=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Arc::new(config::Config::load()?);
    info!("Configuration loaded successfully");
    info!("REST API will be available on port {} at {}",
        config.server.rest_port,
        config.server.rest_path
    );
    info!("WebSocket will be available on port {} at {}",
        config.server.ws_port,
        config.server.ws_path
    );

    // Run both servers concurrently
    tokio::try_join!(
        rest::serve(config.clone()),
        websocket::serve(config.clone()),
    )?;

    Ok(())
}