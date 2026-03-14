use common::events::BotEvent;

pub struct Normalizer;

impl Normalizer {
    pub fn new() -> Self {
        Self
    }
    
    // In a real system the individual WS modules could yield raw Strings 
    // and normalizer turns them into BotEvent::MarketEvent.
    // We embedded basic parsing in finnhub_ws.rs for brevity.
}
