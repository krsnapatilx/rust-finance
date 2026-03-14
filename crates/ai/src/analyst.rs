use anyhow::Result;
use common::events::{BotEvent, ControlCommand};
use tokio::sync::mpsc;
use tracing::{info, error};

pub struct DexterAnalyst {
    // API logic to come
}

impl DexterAnalyst {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self) -> Result<()> {
        info!("Dexter Analyst started - waiting for data...");
        // This will listen for MarketEvents and periodically ping Claude
        Ok(())
    }
}
