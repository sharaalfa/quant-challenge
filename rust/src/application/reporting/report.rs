use std::fs::File;
use std::path::Path;
use std::io::Write;
use crate::domain::backtest::BacktestResult;

pub fn generate_backtest_report(
    result: &BacktestResult,
    symbol: &str,
    threshold: f64,
    output_path: &str,
) -> anyhow::Result<()> {
    let mut file = File::create(output_path)?;

    writeln!(file, "# Backtest Report")?;
    writeln!(file, "## Symbol: {}", symbol)?;
    writeln!(file, "## Threshold: {:.2}", threshold)?;
    writeln!(file, "### Total Trades: {}", result.total_trades)?;
    writeln!(file, "### Profitable Trades: {} ({:.2}%)",
             result.profitable_trades, result.win_rate)?;
    writeln!(file, "### Total PnL: {:.4}", result.pnl)?;

    Ok(())
}

pub fn generate_report(
    symbol: &str,
    deltas: &[f64],
    signals: usize,
    output_path: &str,
) -> Result<(), anyhow::Error> {
    let mut file = File::create(output_path)?;
    
    writeln!(file, "# Orderbook Delta Strategy Report")?;
    writeln!(file, "## Symbol{}", symbol)?;
    writeln!(file,  "## Total samples: {}", deltas.len() + 1)?;
    writeln!(file, "## Signals generated: {}", signals)?;
    
    writeln!(file,  "\n## Orderbook Delta Statistics:")?;
    writeln!(file, "- Average delta: {:.4}", average(deltas))?;
    writeln!(file, "- Max delta: {:.4}", deltas.iter().fold(f64::MIN, |a, &v| a.max(v)))?;
    writeln!(file, "- Min delta: {:.4}", deltas.iter().fold(f64::MAX, |a, &v| a.min(v)))?;
    writeln!(file, "\n## Conclusion")?;
    writeln!(file, "The preliminary analysis shows that orderbook delta can be used as a potential indicator for market movements. Further backtesting is recommended with different thresholds and time intervals.")?;
    Ok(())
}

fn average(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() as f64 / data.len() as f64
}