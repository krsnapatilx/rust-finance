use common::{Action, SwapEvent};

pub trait Strategy: Send {
    fn on_event(&mut self, event: &SwapEvent) -> Action;
}

pub struct SimpleStrategy {
    threshold: u128,
}

impl SimpleStrategy {
    pub fn new(threshold: u128) -> Self {
        Self { threshold }
    }
}

impl Strategy for SimpleStrategy {
    fn on_event(&mut self, event: &SwapEvent) -> Action {
        // Zero-allocation path preferred
        // Simple logic: if anyone swaps > threshold into our target token, we follow
        if event.amount_in > self.threshold {
            Action::Buy {
                token: event.token_out.clone(),
                size: 0.1,
                confidence: 0.9,
            }
        } else {
            Action::Hold
        }
    }
}
