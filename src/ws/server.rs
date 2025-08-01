use crate::broadcasts::broadcasts::BroadcastMessage;
use crate::config::config::Settings;
use futures::{SinkExt, StreamExt};
use prometheus::{Encoder, IntCounter, TextEncoder, register_int_counter};
use serde_json;
use std::convert::Infallible;
use std::sync::LazyLock;
use tokio::sync::{broadcast, watch};
use warp::{Filter, Rejection, Reply};

static WS_CONN_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "ws_connections_total",
        "Total number of WebSocket connections"
    )
    .unwrap()
});

pub async fn start_ws_server(
    tx: broadcast::Sender<BroadcastMessage>,
    settings: std::sync::Arc<Settings>,
    mut shutdown: tokio::sync::watch::Receiver<bool>,
) -> anyhow::Result<()> {
    // Clone Arc's before moving into closures
    let settings_clone = settings.clone();
    let shutdown_clone = shutdown.clone();

    // 1) WebSocket route: for each upgrade, create a receiver from the sender
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || tx.subscribe())) // Create receiver here
        .and(warp::any().map(move || settings_clone.clone()))
        .and(warp::any().map(move || shutdown_clone.clone()))
        .and_then(ws_handler);

    // 2) Metrics route
    let metrics_route = warp::path("metrics").and_then(metrics_handler);

    // 3) Combine routes and start serving
    let routes = ws_route.or(metrics_route);

    let (_addr, server) = warp::serve(routes).bind_with_graceful_shutdown(
        ([0, 0, 0, 0], settings.port),
        async move {
            // Wait for shutdown signal
            let _ = shutdown.changed().await;
        },
    );

    server.await;

    Ok(())
}

async fn ws_handler(
    ws: warp::ws::Ws,
    rx: broadcast::Receiver<BroadcastMessage>,
    settings: std::sync::Arc<crate::config::config::Settings>,
    shutdown: watch::Receiver<bool>,
) -> Result<impl Reply, Rejection> {
    WS_CONN_COUNTER.inc();
    Ok(ws.on_upgrade(move |socket| client_loop(socket, rx, settings, shutdown)))
}

async fn client_loop(
    ws: warp::ws::WebSocket,
    mut rx: broadcast::Receiver<BroadcastMessage>,
    _settings: std::sync::Arc<crate::config::config::Settings>,
    mut shutdown: watch::Receiver<bool>,
) {
    let (mut tx_ws, mut rx_ws) = ws.split();
    // Rate limiting placeholder
    // let mut limiter = RateLimiter::new(...);

    loop {
        tokio::select! {
            // incoming messages from client (e.g. filter commands)
            msg = rx_ws.next() => {
                if msg.is_none() { break; }
                // TODO: handle client messages (rate-limiting, dynamic filters)
            }
            // broadcast events to client
            Ok(event) = rx.recv() => {
                let json = serde_json::to_string(&event).unwrap();
                if tx_ws.send(warp::ws::Message::text(json)).await.is_err() {
                    break;
                }
            }
            // shutdown
            _ = shutdown.changed() => {
                break;
            }
        }
    }
}

async fn metrics_handler() -> Result<impl Reply, Infallible> {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let mf = prometheus::default_registry().gather();
    encoder.encode(&mf, &mut buffer).unwrap();
    let body = String::from_utf8(buffer).unwrap();
    Ok(warp::reply::with_header(
        body,
        "Content-Type",
        "text/plain; version=0.0.4",
    ))
}
