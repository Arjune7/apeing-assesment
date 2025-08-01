use crate::{
    broadcasts::broadcasts::BroadcastMessage,
    utils::utils::parse_token_created,
};
use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio::sync::broadcast::Sender;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

pub async fn start_solana_listener(
    tx: Sender<BroadcastMessage>,
    settings: std::sync::Arc<crate::config::config::Settings>,
    shutdown: &mut tokio::sync::watch::Receiver<bool>,
) -> anyhow::Result<()> {
    loop {
        // 1) Connect & subscribe
        let (mut ws_stream, _) = connect_async(&settings.solana_ws_url).await?;
        let sub = json!({
            "jsonrpc":"2.0",
            "id":1,
            "method":"accountSubscribe",
            "params":[ settings.pump_program_id, { "encoding":"json", "commitment":"final" } ]
        });
        ws_stream.send(Message::Text(sub.to_string())).await?;

        // 2) Listen & parse
        while let Some(msg) = ws_stream.next().await {
            if *shutdown.borrow() {
                return Ok(());
            }
            let msg = msg?;
            if let Message::Text(txt) = msg {
                let raw: serde_json::Value = serde_json::from_str(&txt)?;
                if let Some(event) = parse_token_created(&raw)? {
                    tx.send(BroadcastMessage::TokenCreated(event))?;
                }
            }
        }

        // 3) Reconnect delay
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}
