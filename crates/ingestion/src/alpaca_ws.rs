use anyhow::Result;
use common::events::BotEvent;
use tokio::sync::mpsc;
use tracing::info;

pub struct AlpacaWs {
    api_key: String,
    secret_key: String,
}

impl AlpacaWs {
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self { api_key, secret_key }
    }

    pub async fn run(&self, _tx: mpsc::UnboundedSender<BotEvent>) -> Result<()> {
        info!("Alpaca WebSocket pending implementation...");
        // Placeholder for alpaca-market-data-client-rs or custom ws
        // Can be fleshed out similar to Finnhub
        
        // Let's keep it simple and just do a sleep loop for mock execution
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }
}
