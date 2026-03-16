use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmConfig {
    pub agent_count: usize,
    pub retail_fraction: f64,
    pub hedge_fund_fraction: f64,
    pub market_maker_fraction: f64,
    pub arbitrage_fraction: f64,
    pub momentum_fraction: f64,
    pub news_trader_fraction: f64,
    pub rounds_per_day: u32,
    pub round_delay_ms: u64,
    pub price_impact_lambda: f64,
    pub base_spread_frac: f64,
    pub annualized_vol: f64,
    pub activation_prob: f64,
    pub peak_hour_multiplier: f64,
    pub max_position_usd: f64,
    pub max_parallel_ai: usize,
    pub db_path: String,
    pub db_batch_size: usize,
    pub signal_emit_interval: u32,
}

impl Default for SwarmConfig {
    fn default() -> Self {
        Self {
            agent_count: 5_000,
            retail_fraction: 0.60,
            hedge_fund_fraction: 0.10,
            market_maker_fraction: 0.15,
            arbitrage_fraction: 0.08,
            momentum_fraction: 0.05,
            news_trader_fraction: 0.02,
            rounds_per_day: 390,
            round_delay_ms: 100,
            price_impact_lambda: 0.0001,
            base_spread_frac: 0.0005,
            annualized_vol: 0.20,
            activation_prob: 0.30,
            peak_hour_multiplier: 2.5,
            max_position_usd: 50_000.0,
            max_parallel_ai: 30,
            db_path: "swarm_simulation.db".to_string(),
            db_batch_size: 500,
            signal_emit_interval: 5,
        }
    }
}

impl SwarmConfig {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str::<Self>(&content)?)
    }

    pub fn daily_vol(&self) -> f64 {
        self.annualized_vol / (252_f64).sqrt()
    }

    pub fn round_vol(&self) -> f64 {
        self.daily_vol() / (self.rounds_per_day as f64).sqrt()
    }

    pub fn validate(&self) -> Result<(), String> {
        let sum = self.retail_fraction
            + self.hedge_fund_fraction
            + self.market_maker_fraction
            + self.arbitrage_fraction
            + self.momentum_fraction
            + self.news_trader_fraction;

        if (sum - 1.0).abs() > 0.001 {
            return Err(format!("Trader type fractions sum to {:.4}, must equal 1.0", sum));
        }
        Ok(())
    }
}
