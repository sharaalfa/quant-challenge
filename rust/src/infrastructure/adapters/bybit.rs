use anyhow::anyhow;
use async_trait::async_trait;
use reqwest::Client;
use crate::domain::BybitOrderBookResponse;
pub(crate) use crate::domain::OrderbookSnapshot;

#[async_trait]
pub trait BybitClient: Send + Sync {
    async fn fetch_orderbook_snapshot(&self, symbol: &str) -> anyhow::Result<OrderbookSnapshot>;
    async fn calculate_delta(
        &self,
        prev: &OrderbookSnapshot,
        curr: &OrderbookSnapshot,
    ) -> anyhow::Result<f64>;
}

pub struct HttpBybitClient{
    client: Client,
    base_url: String,
}


impl OrderbookSnapshot {
    pub fn get_top_bid(&self) -> Option<f64> {
        self.bids.first().map(|(price, _)| *price)
    }

    pub fn get_top_ask(&self) -> Option<f64> {
        self.asks.first().map(|(price, _)| *price)
    }

    pub fn mid_price(&self) -> Option<f64> {
        match (self.get_top_bid(), self.get_top_ask()) {
            (Some(bid), Some(ask)) => Some((bid + ask) / 2.0),
            _ => None,
        }
    }
}
impl HttpBybitClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api-testnet.bybit.com/v5/market/orderbook".to_string(),
        }
    }
}
#[async_trait]
impl BybitClient for HttpBybitClient {
    async fn fetch_orderbook_snapshot(&self, _symbol: &str) -> anyhow::Result<OrderbookSnapshot> {
        let url = format!("{}?category=spot&symbol={}", self.base_url, _symbol);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch orderbook: {}", response.status()));
        }

        let orderbook_data: BybitOrderBookResponse = response.json().await?;

        let parse_level = |levels: Vec<(String, String)>| -> anyhow::Result<Vec<(f64, f64)>>{
            levels.into_iter()
                .map(|(price, size)| {
                    Ok((price.parse()?, size.parse()?))
                })
                .collect()
        };

        Ok(OrderbookSnapshot{
            bids: parse_level(orderbook_data.result.b)?,
            asks: parse_level(orderbook_data.result.a)?,
        })
    }

    async fn calculate_delta(
        &self,
        _prev: &OrderbookSnapshot,
        _curr: &OrderbookSnapshot,
    ) -> anyhow::Result<f64> {
        let prev_bid_volume: f64 = _prev.bids.iter().map(|(_, size)| size).sum();
        let pred_ask_volume: f64 = _prev.asks.iter().map(|(_, size)| size).sum();
        let cur_bid_volume: f64 = _curr.bids.iter().map(|(_, size)| size).sum();
        let cur_ask_volume: f64 = _curr.asks.iter().map(|(_, size)| size).sum();

        let delta = (cur_bid_volume - prev_bid_volume) - (cur_ask_volume - pred_ask_volume);
        Ok(delta)
    }
}
