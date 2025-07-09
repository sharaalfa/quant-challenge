// domain layer

pub mod backtest;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize)]
pub struct OrderbookSnapshot {
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
}
#[derive(Debug, Deserialize)]
pub struct BybitOrderBookResponse {
    pub result: BybitOrderbookResult,
}

#[derive(Debug, Deserialize)]
pub struct BybitOrderbookResult {
    pub b: Vec<(String, String)>,
    pub a: Vec<(String, String)>,
}
