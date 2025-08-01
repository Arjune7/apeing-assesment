use clap::Parser;
use dotenv::dotenv;

/// Shared application settings
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Settings {
    /// WebSocket port to serve
    #[arg(long, env = "SERVER_PORT", default_value_t = 8080)]
    pub port: u16,

    /// Solana WebSocket RPC URL
    #[arg(long, env = "SOLANA_WS_URL")]
    pub solana_ws_url: String,

    /// Pump.fun program ID
    #[arg(long, env = "PUMP_FUN_PROGRAM_ID")]
    pub pump_program_id: String,

    /// Optional filter symbols (comma-separated)
    #[arg(long, env = "FILTER_SYMBOLS", default_value = "")]
    pub filter_symbols: String,

    /// Optional rate limit (requests per minute)
    #[arg(long, env = "RATE_LIMIT_RPM", default_value_t = 0)]
    pub rate_limit_rpm: u32,
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        dotenv().ok();
        let s = Settings::parse();
        // split filter_symbols into Vec<String> if needed
        Ok(s)
    }
}
