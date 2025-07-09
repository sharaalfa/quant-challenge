mod domain;

mod application;
mod config;
mod infrastructure;

use crate::application::orderbook_analyzer::OrderbookAnalyzer;
use crate::application::reporting::report::generate_backtest_report;
use crate::application::reporting::{Saver, generate_report};
use crate::config::Config;
use crate::infrastructure::adapters::HttpBybitClient;
use std::fmt::write;
use std::path::Path;
use tracing::{error, info};
use tracing_subscriber::fmt::init;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let file_appender = tracing_appender::rolling::daily("./logs", "quant_research.log");

    // Configure formatting
    let fmt_layer = fmt::layer().with_target(false).with_ansi(false).compact();

    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt_layer
                    .with_writer(file_appender)
               // .with_writer(std::io::stdout),
        )
        .init();
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //tracing_subscriber::fmt::init();
    init_logging();
    info!("Hello, Quant!");

    let bybit_client = HttpBybitClient::new();

    let analyser = OrderbookAnalyzer::new(bybit_client);

    let cfg = Config::default();
    let cfg_clone = cfg.clone();
    
    info!(
        "Starting data collection for {} ({} samples, interval {}s)",
        cfg_clone.symbol,
        cfg_clone.samples_count,
        cfg_clone.samples_count
    );
    
    info!("Collecting {} snapshots..", cfg.samples_count);
    let snapshots = analyser
        .collect_snapshots(&cfg.symbol, cfg.interval_sec, cfg.samples_count)
        .await?;

    if let Err(e) = analyser.analyze_orderbook(cfg, &snapshots).await {
        error!("Analysis failed: {}", e);
        return Err(e);
    }

    Saver::save_snapshots(&snapshots, Path::new("data/snapshots.json"))?;

    info!("Running backtest with threshold {:.2}", cfg_clone.delta_threshold);
    let backtest_result = analyser.backtest(&cfg_clone, &snapshots).await?;

    generate_backtest_report(
        &backtest_result,
        &cfg_clone.symbol,
        cfg_clone.delta_threshold,
        "reports/backtest.md",
    )?;
    info!("Finished analysing!");
    Ok(())
}
