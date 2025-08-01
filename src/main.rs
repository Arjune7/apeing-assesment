use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::{
    select, signal, spawn,
    sync::{broadcast, watch},
};
use tracing::{error, info, instrument};
use tracing_subscriber::EnvFilter;

mod broadcasts;
mod config;
mod errors;
mod models;
mod solana;
mod utils;
mod ws;

use crate::broadcasts::broadcasts::BroadcastMessage;
use crate::config::config::Settings;
use crate::solana::rpc_ws::start_solana_listener;
use crate::ws::server::start_ws_server;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load .env and initialize tracing
    dotenv::from_filename(".env.example").ok();
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // 2. Load and share configuration
    let settings =
        Arc::new(Settings::new().context("Failed to load configuration from environment")?);

    // 3. Set up broadcast and shutdown channels
    let (tx, _) = broadcast::channel::<BroadcastMessage>(100);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // 4. Spawn services with debugging
    info!("Starting WebSocket service...");
    let tx_ws = tx.clone();
    let settings_ws = settings.clone();
    let shutdown_ws = shutdown_rx.clone();
    let ws_handle = spawn(async move {
        info!("WebSocket service task started");
        let result = run_ws_service(tx_ws, settings_ws, shutdown_ws).await;
        error!("WebSocket service completed with result: {:?}", result);
        result
    });

    info!("Starting Solana service...");
    let tx_sol = tx.clone();
    let settings_sol = settings.clone();
    let shutdown_sol = shutdown_rx.clone();
    let sol_handle = spawn(async move {
        info!("Solana service task started");
        let result = run_solana_service(tx_sol, settings_sol, shutdown_sol).await;
        error!("Solana service completed with result: {:?}", result);
        result
    });

    info!("Services started successfully");

    // 5. Wait for Ctrl+C or any task error
    select! {
        _ = signal::ctrl_c() => {
            info!("Shutdown signal received");
        }
        res = ws_handle => {
            error!("WebSocket service task finished: {:?}", res);
        }
        res = sol_handle => {
            error!("Solana service task finished: {:?}", res);
        }
    }

    // 6. Notify tasks to shut down
    let _ = shutdown_tx.send(true);
    info!("Notified tasks to shut down. Exiting.");
    Ok(())
}

/// Run the WebSocket server with tracing and error context
#[instrument(skip(tx, settings, shutdown))]
async fn run_ws_service(
    tx: broadcast::Sender<BroadcastMessage>,
    settings: Arc<Settings>,
    shutdown: watch::Receiver<bool>,
) -> Result<()> {
    start_ws_server(tx, settings, shutdown)
        .await
        .context("WebSocket server encountered an error")
}

/// Run the Solana listener with tracing and error context
#[instrument(skip(tx, settings, shutdown))]
async fn run_solana_service(
    tx: broadcast::Sender<BroadcastMessage>,
    settings: Arc<Settings>,
    mut shutdown: watch::Receiver<bool>,
) -> Result<()> {
    start_solana_listener(tx, settings, &mut shutdown)
        .await
        .context("Solana listener encountered an error")
}
