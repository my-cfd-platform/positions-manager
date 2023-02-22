use chrono::{DateTime, Utc};
use trading_sdk::ExecutionBidAsk;

#[derive(Debug, Clone)]
pub struct EngineBidAsk {
    pub asset_pair: String,
    pub bid: f64,
    pub ask: f64,
    pub datetime: DateTime<Utc>,
}

impl ExecutionBidAsk for EngineBidAsk {
    fn get_asset_pair(&self) -> &str {
        &self.asset_pair
    }

    fn get_bid(&self) -> f64 {
        self.bid
    }

    fn get_ask(&self) -> f64 {
        self.ask
    }
}