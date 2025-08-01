use chrono::{DateTime, Utc};
use serde::Serialize;

/// The JSON schema for a token creation event
#[derive(Debug, Clone, Serialize)]
pub struct TokenCreatedEvent {
    pub event_type: String, // "token_created"
    pub timestamp: DateTime<Utc>,
    pub transaction_signature: String,
    pub token: TokenInfo,
    pub pump_data: PumpData,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenInfo {
    pub mint_address: String,
    pub name: String,
    pub symbol: String,
    pub creator: String,
    pub supply: u64,
    pub decimals: u8,
}

#[derive(Debug, Clone, Serialize)]
pub struct PumpData {
    pub bonding_curve: String,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
}
