use anyhow::Result;
use clap::Parser;
use ingestion::{IngestionArgs, IngestionService};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = IngestionArgs::parse();
    
    // For standalone run
    let (tx, _rx) = crossbeam_channel::bounded(10000);
    let service = IngestionService::new(args, tx);
    
    service.run().await
}
