use anyhow::Result;
use crossbeam_channel::Sender;
use serde_json::json;
use tracing::{info, warn, debug};

pub struct MockIngestionService {
    tx: Sender<String>,
}

impl MockIngestionService {
    pub fn new(tx: Sender<String>) -> Self {
        Self { tx }
    }

    pub async fn run(&self) -> Result<()> {
        info!("Mock Ingestion Service starting... (Simulating Solana Logs)");
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(1000));
        
        loop {
            interval.tick().await;
            
            let mock_log = json!({
                "jsonrpc": "2.0",
                "method": "logsNotification",
                "params": {
                    "result": {
                        "context": { "slot": 12345678 },
                        "value": {
                            "signature": format!("mock_sig_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()),
                            "err": null,
                            "logs": [
                                "Program log: Instruction: Swap",
                                "Program log: Parsed amount_in: 5000000",
                                "Program log: Parsed amount_out: 12000000"
                            ]
                        }
                    },
                    "subscription": 1
                }
            }).to_string();

            if let Err(e) = self.tx.try_send(mock_log) {
                warn!("Mock tx channel error: {:?}", e);
            } else {
                debug!("Mock log injected");
            }
        }
    }
}
