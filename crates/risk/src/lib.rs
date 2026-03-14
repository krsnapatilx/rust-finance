use common::{Action, Result};
use tracing::{warn, info};

pub struct RiskManager {
    max_position_size: f64,
    min_confidence: f32,
    // Add state like current_drawdown, active_positions
}

impl RiskManager {
    pub fn new(max_position_size: f64, min_confidence: f32) -> Self {
        Self {
            max_position_size,
            min_confidence,
        }
    }

    pub fn check_action(&self, action: Action) -> Result<Action> {
        match action {
            Action::Buy { token, size, confidence } => {
                if confidence < self.min_confidence {
                    warn!("Risk: Buy rejected due to low confidence: {} < {}", confidence, self.min_confidence);
                    return Ok(Action::Hold);
                }

                let adjusted_size = size.min(self.max_position_size);
                if adjusted_size < size {
                    info!("Risk: Buy size capped: {} -> {}", size, adjusted_size);
                }

                Ok(Action::Buy { token, size: adjusted_size, confidence })
            }
            Action::Sell { .. } => Ok(action), // Sells usually allowed by risk unless closing at bad price
            Action::Hold => Ok(Action::Hold),
        }
    }
}
