use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketState {
    pub symbol: String,
    pub mid_price: f64,
    pub bid: f64,
    pub ask: f64,
    pub spread: f64,
    pub volume_24h: f64,
    pub volatility_realized: f64,
    pub momentum_1h: f64,
    pub momentum_1d: f64,
    pub vwap: f64,
    pub order_imbalance: f64,
    pub round: u64,
    pub timestamp_ms: i64,
    #[serde(skip)]
    pub price_history: VecDeque<f64>,
    #[serde(skip)]
    pub volume_history: VecDeque<f64>,
}

impl MarketState {
    pub fn new(symbol: impl Into<String>, initial_price: f64) -> Self {
        let spread = initial_price * 0.0005;
        Self {
            symbol: symbol.into(),
            mid_price: initial_price,
            bid: initial_price - spread / 2.0,
            ask: initial_price + spread / 2.0,
            spread,
            volume_24h: 0.0,
            volatility_realized: 0.20 / (252_f64).sqrt(),
            momentum_1h: 0.0,
            momentum_1d: 0.0,
            vwap: initial_price,
            order_imbalance: 0.0,
            round: 0,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
            price_history: {
                let mut dq = VecDeque::with_capacity(200);
                dq.push_back(initial_price);
                dq
            },
            volume_history: VecDeque::with_capacity(200),
        }
    }

    pub fn advance(&mut self, net_flow: f64, lambda: f64, round_vol: f64, rng: &mut impl Rng) {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let epsilon = normal.sample(rng);
        let diffusion = self.mid_price * round_vol * epsilon;
        let impact = lambda * net_flow;
        let new_mid = (self.mid_price + diffusion + impact).max(0.01);

        let vol_scalar = (self.volatility_realized / 0.01).max(1.0);
        let imbalance_scalar = (1.0 + self.order_imbalance.abs() * 0.5).min(3.0);
        let new_spread = self.mid_price * 0.0005 * vol_scalar * imbalance_scalar;

        let round_vol_usd = net_flow.abs().max(1.0);
        self.vwap = (self.vwap * self.volume_24h + new_mid * round_vol_usd) / (self.volume_24h + round_vol_usd);
        self.volume_24h += round_vol_usd;

        if self.price_history.len() >= 60 {
            let old_1h = self.price_history[self.price_history.len() - 60];
            self.momentum_1h = (new_mid - old_1h) / old_1h;
        }
        if self.price_history.len() >= 390 {
            let old_1d = self.price_history[self.price_history.len() - 390];
            self.momentum_1d = (new_mid - old_1d) / old_1d;
        }

        if self.price_history.len() >= 2 {
            let returns: Vec<f64> = self.price_history.iter().rev().take(20).collect::<Vec<_>>().windows(2).map(|w| (w[0] / w[1]).ln()).collect();
            if !returns.is_empty() {
                let mean = returns.iter().sum::<f64>() / returns.len() as f64;
                let variance = returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / returns.len() as f64;
                self.volatility_realized = variance.sqrt();
            }
        }

        self.mid_price = new_mid;
        self.bid = new_mid - new_spread / 2.0;
        self.ask = new_mid + new_spread / 2.0;
        self.spread = new_spread;
        self.order_imbalance = net_flow / (round_vol_usd + 1.0);
        self.round += 1;
        self.timestamp_ms = chrono::Utc::now().timestamp_millis();

        self.price_history.push_back(new_mid);
        if self.price_history.len() > 500 { self.price_history.pop_front(); }
        self.volume_history.push_back(round_vol_usd);
        if self.volume_history.len() > 500 { self.volume_history.pop_front(); }
    }

    pub fn rsi_14(&self) -> f64 {
        if self.price_history.len() < 15 { return 50.0; }
        let prices: Vec<f64> = self.price_history.iter().rev().take(15).cloned().collect();
        let mut gains = 0.0_f64;
        let mut losses = 0.0_f64;

        for i in 0..14 {
            let change = prices[i] - prices[i + 1];
            if change > 0.0 { gains += change; } else { losses += change.abs(); }
        }

        if losses == 0.0 { return 100.0; }
        let rs = (gains / 14.0) / (losses / 14.0);
        100.0 - (100.0 / (1.0 + rs))
    }

    pub fn is_high_vol(&self) -> bool {
        self.volatility_realized > 0.02 / (390_f64).sqrt()
    }
}

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLevel {
    pub price: f64,
    pub size: f64,
}

impl OrderBook {
    pub fn imbalance(&self, depth: usize) -> f64 {
        let bid_vol: f64 = self.bids.iter().take(depth).map(|l| l.size).sum();
        let ask_vol: f64 = self.asks.iter().take(depth).map(|l| l.size).sum();
        let total = bid_vol + ask_vol;
        if total == 0.0 { return 0.0; }
        (bid_vol - ask_vol) / total
    }

    pub fn cost_to_buy(&self, notional: f64) -> f64 {
        let mut remaining = notional;
        let mut total_cost = 0.0;
        for level in &self.asks {
            if remaining <= 0.0 { break; }
            let fill = remaining.min(level.size);
            total_cost += fill * level.price;
            remaining -= fill;
        }
        total_cost
    }
}
