use anyhow::{Result, Context};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    signature::Signature,
    transaction::Transaction,
};
use std::sync::Arc;
use tracing::info;
use common::Action;
use signer::LocalSigner;

pub struct ExecutorService {
    selector: Arc<relay::NodeSelector>,
    signer: Option<Arc<LocalSigner>>,
}

impl ExecutorService {
    pub fn new(selector: Arc<relay::NodeSelector>, signer: Option<LocalSigner>) -> Self {
        Self { 
            selector,
            signer: signer.map(Arc::new),
        }
    }

    pub async fn execute_action(&self, action: Action) -> Result<Signature> {
        let signer = self.signer.as_ref().context("No signer configured for execution")?;
        
        match action {
            Action::Buy { token, size, confidence } => {
                info!("Executing BUY for {}: size={}, confidence={}", token, size, confidence);
                let instructions = self.build_buy_instructions(&token, size)?;
                self.send_and_confirm(instructions, signer).await
            }
            Action::Sell { token, size, confidence } => {
                info!("Executing SELL for {}: size={}, confidence={}", token, size, confidence);
                let instructions = self.build_sell_instructions(&token, size)?;
                self.send_and_confirm(instructions, signer).await
            }
            Action::Hold => Ok(Signature::default()),
        }
    }

    fn build_buy_instructions(&self, _token: &str, _size: f64) -> Result<Vec<Instruction>> {
        let mut ixs = Vec::new();
        // 1. Priority Fees (Compute Budget)
        ixs.push(solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(200_000));
        ixs.push(solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_price(100_000)); // 100k microlamports
        
        // 2. Real implementation would include Swap instructions
        Ok(ixs) 
    }

    fn build_sell_instructions(&self, _token: &str, _size: f64) -> Result<Vec<Instruction>> {
        let mut ixs = Vec::new();
        ixs.push(solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(200_000));
        ixs.push(solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_price(100_000));
        Ok(ixs)
    }

    async fn send_and_confirm(&self, instructions: Vec<Instruction>, signer: &LocalSigner) -> Result<Signature> {
        if instructions.is_empty() {
             return Ok(Signature::default());
        }

        if std::env::var("USE_MOCK").is_ok() {
            info!("MOCK mode: Skipping actual transaction send. Returning dummy signature.");
            return Ok(Signature::new_unique());
        }

        // 1. Fetch blockhash (In production, subscribe to slot updates for zero-latency hash)
        let rpc_url = self.selector.get_best().await;
        let rpc_client = RpcClient::new(rpc_url);
        let recent_blockhash = rpc_client.get_latest_blockhash().await?;
        
        // 2. Build & Sign
        let mut tx = Transaction::new_with_payer(&instructions, Some(&signer.pubkey()));
        tx.message.recent_blockhash = recent_blockhash;
        signer.sign_transaction(&mut tx);
        
        // 3. Send (Use send_transaction for signed transactions)
        let signature = rpc_client.send_transaction(&tx).await
            .context("Failed to send transaction")?;
        
        info!("Transaction sent: {}", signature);
        
        // 4. Async confirmation (Don't block the executor task if possible)
        // For now we just return the signature. Confirmation can be monitored by a separate service.
        
        Ok(signature)
    }
}
