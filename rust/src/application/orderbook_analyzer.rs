use crate::application::EmptyQuery;
use crate::application::reporting::{Saver, generate_report};
use crate::config::Config;
use crate::infrastructure::adapters::BybitClient;
use anyhow::anyhow;
use std::path::Path;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tracing::{info, warn};
use crate::domain::backtest::BacktestResult;
use crate::domain::OrderbookSnapshot;

pub struct OrderbookAnalyzer<C: BybitClient> {
    client: Arc<C>,
}

impl<C: BybitClient> OrderbookAnalyzer<C> {
    pub fn new(client: C) -> Self {
        Self {
            client: Arc::new(client),
        }
    }

    pub async fn analyze_orderbook(
        &self, 
        cfg: Config,
        snapshots: &[OrderbookSnapshot],
    ) -> anyhow::Result<()> {
        let symbol: &str = cfg.symbol.as_str();
        let interval_sec: u64 = cfg.interval_sec;
        let samples_count: usize = cfg.samples_count;
        let threshold = cfg.delta_threshold;
        let mut deltas = Vec::with_capacity(samples_count - 1);
        
        for i in 1..snapshots.len() {
            let delta = self
                .client
                .calculate_delta(&snapshots[i - 1], &snapshots[i])
                .await?;
            deltas.push(delta);
            info!("Delta between snapshots {} and {}: {:.4}", i, i + 1, delta);
        }

        for (i, &delta) in deltas.iter().enumerate() {
            if delta > threshold {
                info!("STRATEGY SIGNAL #{}: BUY (delta = {:.4})", i + 1, delta);
            } else if delta < -threshold {
                info!("STRATEGY SIGNAL #{}: SELL (delta = {:.4})", i + 1, delta);
            }
        }
        
        Saver::save_deltas(&deltas, Path::new("data/deltas.json"))?;

        generate_report(symbol, &deltas, samples_count, "reports/latest.md")?;
        Ok(())
    }
    
    pub async fn collect_snapshots(
        &self,
    symbol: &str,
    interval_sec: u64,
    samples_count: usize) -> anyhow::Result<Vec<OrderbookSnapshot>> {
        let mut snapshots = Vec::with_capacity(samples_count);
        
        for i in 0..samples_count {
            match self.client.fetch_orderbook_snapshot(symbol).await {
                Ok(snapshot) => {
                    info!("Collected snapshot {}/{}", i + 1, samples_count);
                    snapshots.push(snapshot);
                }
                Err(e) => {
                    warn!("Failed to collect snapshot {}: {}", i + 1, e);
                }
            }
            
            if i < samples_count - 1 {
                sleep(Duration::from_secs(interval_sec));
            }
        }
        Ok(snapshots)
    }
    pub async fn backtest(
        &self,
        cfg: &Config,
        snapshots: &[OrderbookSnapshot],
    ) -> anyhow::Result<BacktestResult> {
        let mut result = BacktestResult::new();
        let threshold = cfg.delta_threshold;
        
        for i in 1..snapshots.len() {
            let delta = self.client.calculate_delta(&snapshots[i], &snapshots[i - 1]).await?;
            
            if let (Some(pred_min), Some(curr_mid)) = (
                snapshots[i - 1].mid_price(),
                snapshots[i].mid_price(),
                ) {
                if delta > threshold {
                    result.pnl += curr_mid - pred_min;
                    result.total_trades += 1;
                    if curr_mid > pred_min {
                        result.profitable_trades += 1;
                    }
                } else if delta < -threshold {
                    result.pnl -= curr_mid - pred_min;
                    result.total_trades += 1;
                    if pred_min > curr_mid {
                        result.profitable_trades += 1;
                    }
                }
            }
        }
        result.calculate_win_rate();
        Ok(result)
    }
}
