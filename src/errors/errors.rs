use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Broadcast channel error: {0}")]
    Broadcast(
        #[from]
        tokio::sync::broadcast::error::SendError<crate::broadcasts::broadcasts::BroadcastMessage>,
    ),
}
