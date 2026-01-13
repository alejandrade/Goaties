use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use std::sync::Arc;
use tracing::{error, info};

use crate::config::Config;

#[derive(Clone)]
pub struct WsState {
    pub config: Arc<Config>,
}

pub fn create_router(config: Arc<Config>) -> Router {
    let state = WsState { config: config.clone() };

    Router::new()
        .route(&config.server.ws_path, get(ws_handler))
        .with_state(state)
}

async fn ws_handler(
    State(_state): State<WsState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    info!("WebSocket client connected");

    // Send welcome message
    if socket
        .send(Message::Text("Welcome to Goaties WebSocket!".to_string().into()))
        .await
        .is_err()
    {
        error!("Failed to send welcome message");
        return;
    }

    // Handle incoming messages
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                info!("Received text message: {}", text);

                // Echo back
                if socket
                    .send(Message::Text(format!("Echo: {}", text).into()))
                    .await
                    .is_err()
                {
                    error!("Failed to send echo message");
                    break;
                }
            }
            Ok(Message::Close(_)) => {
                info!("WebSocket client disconnected");
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }

    info!("WebSocket connection closed");
}

pub async fn serve(config: Arc<Config>) -> anyhow::Result<()> {
    let app = create_router(config.clone());
    let addr = format!("0.0.0.0:{}", config.server.ws_port);

    info!("WebSocket server listening on {}{}", addr, config.server.ws_path);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}