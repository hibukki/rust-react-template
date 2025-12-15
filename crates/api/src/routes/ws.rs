use crate::state::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use shared::types::WsEvent;

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/ws", get(ws_handler))
}

async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to events BEFORE fetching initial state to avoid race conditions
    let mut events_rx = state.subscribe_events();

    // 1. Send ALL current profiles immediately (initial state)
    let profiles = state.profile_service.list_profiles().await.unwrap_or_default();
    for profile in profiles {
        let event = WsEvent::Profile(profile);
        let json = match serde_json::to_string(&event) {
            Ok(j) => j,
            Err(e) => {
                tracing::error!("Failed to serialize profile: {}", e);
                continue;
            }
        };
        if sender.send(Message::Text(json.into())).await.is_err() {
            tracing::debug!("WebSocket closed during initial state");
            return;
        }
    }

    // 2. Forward future updates (same message type as initial state)
    let send_task = tokio::spawn(async move {
        while let Ok(event) = events_rx.recv().await {
            let json = match serde_json::to_string(&event) {
                Ok(j) => j,
                Err(e) => {
                    tracing::error!("Failed to serialize event: {}", e);
                    continue;
                }
            };

            if sender.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages (for future use, e.g., ping/pong)
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Close(_)) => break,
                Ok(Message::Ping(data)) => {
                    // Ping is handled automatically by axum
                    tracing::debug!("Received ping: {:?}", data);
                }
                Err(e) => {
                    tracing::error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    tracing::debug!("WebSocket connection closed");
}
