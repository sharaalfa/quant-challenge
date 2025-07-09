#[derive(Debug, Clone)]
pub struct BacktestResult {
    pub total_trades: usize,
    pub profitable_trades: usize,
    pub pnl: f64,
    pub win_rate: f64,
}

impl BacktestResult {
    pub fn new() -> Self {
        Self {
            total_trades: 0,
            profitable_trades: 0,
            pnl: 0.0,
            win_rate: 0.0,
        }
    }
    
    pub fn calculate_win_rate(&mut self) {
        if self.total_trades > 0 {
            self.win_rate = (self.profitable_trades as f64 / self.total_trades as f64) * 100.0;
        }
    }
}