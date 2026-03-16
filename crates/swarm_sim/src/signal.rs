use serde::{Deserialize, Serialize};
use crate::market::MarketState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmSignal {
    pub round: u64,
    pub symbol: String,
    pub direction: SignalDirection,
    pub conviction: Conviction,
    pub bullish_prob: f64,
    pub bearish_prob: f64,
    pub neutral_prob: f64,
    pub net_flow_usd: f64,
    pub regime: MarketRegime,
    pub confidence: f64,
    pub price: f64,
    pub volatility: f64,
    pub rsi: f64,
    pub momentum_1h: f64,
    pub momentum_1d: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SignalDirection {
    Long,
    Short,
    Neutral,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Conviction {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketRegime {
    Trending,
    MeanReverting,
    HighVolatility,
    LowVolatility,
    OrderImbalance,
}

impl SwarmSignal {
    pub fn from_round(round: u64, market: &MarketState, buy_fraction: f64, sell_fraction: f64, net_flow_usd: f64) -> Self {
        let neutral_prob = (1.0 - buy_fraction - sell_fraction).max(0.0);

        let direction = if buy_fraction > sell_fraction + 0.15 {
            SignalDirection::Long
        } else if sell_fraction > buy_fraction + 0.15 {
            SignalDirection::Short
        } else {
            SignalDirection::Neutral
        };

        let margin = (buy_fraction - sell_fraction).abs();
        let conviction = if margin > 0.30 { Conviction::High } else if margin > 0.15 { Conviction::Medium } else { Conviction::Low };

        let regime = detect_regime(market, net_flow_usd);

        let momentum_aligned = match &direction {
            SignalDirection::Long => market.momentum_1h > 0.0,
            SignalDirection::Short => market.momentum_1h < 0.0,
            SignalDirection::Neutral => true,
        };
        let regime_aligned = match (&direction, &regime) {
            (SignalDirection::Long, MarketRegime::Trending) | (SignalDirection::Short, MarketRegime::Trending) => true,
            (_, MarketRegime::HighVolatility) => false,
            _ => true,
        };

        let mut confidence = margin * 2.0;
        if momentum_aligned { confidence += 0.15; }
        if regime_aligned { confidence += 0.10; }
        confidence = confidence.min(1.0);

        SwarmSignal {
            round,
            symbol: market.symbol.clone(),
            direction,
            conviction,
            bullish_prob: buy_fraction,
            bearish_prob: sell_fraction,
            neutral_prob,
            net_flow_usd,
            regime,
            confidence,
            price: market.mid_price,
            volatility: market.volatility_realized,
            rsi: market.rsi_14(),
            momentum_1h: market.momentum_1h,
            momentum_1d: market.momentum_1d,
        }
    }

    pub fn to_prompt_context(&self) -> String {
        format!(
            "[SwarmSim Round {}] {} | Direction: {:?} ({:?}) | Bulls: {:.0}% Bears: {:.0}% | Net flow: ${:.0}K | Regime: {:?} | Confidence: {:.0}% | RSI: {:.1} | Mom1H: {:.2}% | Vol: {:.2}%",
            self.round, self.symbol, self.direction, self.conviction, self.bullish_prob * 100.0, self.bearish_prob * 100.0, self.net_flow_usd / 1000.0, self.regime, self.confidence * 100.0, self.rsi, self.momentum_1h * 100.0, self.volatility * 100.0,
        )
    }

    pub fn is_actionable(&self) -> bool {
        self.conviction != Conviction::Low && !matches!(self.regime, MarketRegime::HighVolatility) && self.confidence > 0.40
    }
}

fn detect_regime(market: &MarketState, net_flow: f64) -> MarketRegime {
    let flow_fraction = (net_flow.abs() / 1_000_000.0).min(1.0);

    if market.is_high_vol() { MarketRegime::HighVolatility }
    else if flow_fraction > 0.7 { MarketRegime::OrderImbalance }
    else if market.momentum_1h.abs() > 0.01 { MarketRegime::Trending }
    else if market.volatility_realized < 0.003 { MarketRegime::LowVolatility }
    else { MarketRegime::MeanReverting }
}
