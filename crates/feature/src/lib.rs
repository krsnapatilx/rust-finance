use common::SwapEvent;
use dashmap::DashMap;
use std::sync::Arc;

pub struct FeatureEngine {
    // Thread-safe store for per-token metrics
    metrics: Arc<DashMap<String, TokenMetrics>>,
}

#[derive(Default)]
pub struct TokenMetrics {
    pub last_price: f64,
    pub volume_24h: u128,
    pub buy_count_5m: u32,
}

impl FeatureEngine {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(DashMap::new()),
        }
    }

    pub fn process_event(&self, event: &SwapEvent) {
        let mut entry = self.metrics.entry(event.token_out.clone()).or_default();
        entry.buy_count_5m += 1;
        entry.volume_24h += event.amount_in;
        // ... more complex feature engineering
    }

    pub fn get_features(&self, token: &str) -> Option<TokenMetrics> {
        self.metrics.get(token).map(|v| TokenMetrics {
            last_price: v.last_price,
            volume_24h: v.volume_24h,
            buy_count_5m: v.buy_count_5m,
        })
    }
}
