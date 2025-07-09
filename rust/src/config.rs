#[derive(Clone)]
pub struct Config {
    pub symbol: String,
    pub interval_sec: u64,
    pub samples_count: usize,
    pub delta_threshold: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self{
            symbol: "BTCUSDT".to_string(),
            interval_sec: 5,
            samples_count: 100,
            delta_threshold: 5.0,
        }
    }
}
