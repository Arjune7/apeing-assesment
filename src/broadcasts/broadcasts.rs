use crate::models::models::TokenCreatedEvent;
use serde::Serialize;

/// The message enum we broadcast internally
#[derive(Debug, Clone, Serialize)]
pub enum BroadcastMessage {
    TokenCreated(TokenCreatedEvent),
    // (extendable for other event types)
}
