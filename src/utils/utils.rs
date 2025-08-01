use crate::errors::errors::AppError;
use crate::models::models::{PumpData, TokenCreatedEvent, TokenInfo};
use chrono::Utc;

pub fn parse_token_created(raw: &serde_json::Value) -> Result<Option<TokenCreatedEvent>, AppError> {
    {
        // Only handle RPC notifications that include logs
        if let Some(params) = raw.get("params") {
            if let Some(result) = params.get("result") {
                if let Some(value) = result.get("value") {
                    if let Some(logs) = value.get("logs").and_then(serde_json::Value::as_array) {
                        // Look for an InitializeMint log entry
                        for line in logs.iter().filter_map(serde_json::Value::as_str) {
                            if line.contains("InitializeMint") {
                                // Extract the transaction signature if present
                                let signature = result
                                    .get("value")
                                    .and_then(|v| v.get("signature"))
                                    .and_then(serde_json::Value::as_str)
                                    .unwrap_or_default()
                                    .to_string();
                                // Build a minimal TokenCreatedEvent; adjust fields as needed
                                let event = TokenCreatedEvent {
                                    event_type: "token_created".to_string(),
                                    timestamp: Utc::now(),
                                    transaction_signature: signature,
                                    token: TokenInfo {
                                        mint_address: "".to_string(),
                                        name: "".to_string(),
                                        symbol: "".to_string(),
                                        creator: "".to_string(),
                                        supply: 0,
                                        decimals: 0,
                                    },
                                    pump_data: PumpData {
                                        bonding_curve: "".to_string(),
                                        virtual_sol_reserves: 0,
                                        virtual_token_reserves: 0,
                                    },
                                };
                                return Ok(Some(event));
                            }
                        }
                    }
                }
            }
            Ok(None)
        } else {
            Ok(None)
        }
    }
}
