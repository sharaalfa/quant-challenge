use async_trait::async_trait;

#[derive(Debug, Default)]
pub struct OrderbookSnapshot {
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
}

#[async_trait]
pub trait BybitClient {
    async fn fetch_orderbook_snapshot(&self, symbol: &str) -> anyhow::Result<OrderbookSnapshot>;
    async fn calculate_delta(
        &self,
        prev: &OrderbookSnapshot,
        curr: &OrderbookSnapshot,
    ) -> anyhow::Result<f64>;
}

pub struct HttpBybitClient;

#[async_trait]
impl BybitClient for HttpBybitClient {
    async fn fetch_orderbook_snapshot(&self, _symbol: &str) -> anyhow::Result<OrderbookSnapshot> {
        // TODO: HTTP call
        Ok(OrderbookSnapshot::default())
    }

    async fn calculate_delta(
        &self,
        _prev: &OrderbookSnapshot,
        _curr: &OrderbookSnapshot,
    ) -> anyhow::Result<f64> {
        // TODO: compute delta
        Ok(0.0)
    }
}
