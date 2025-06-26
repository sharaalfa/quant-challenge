mod domain;

mod application;
mod infrastructure;

use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Hello, Quant!");
}
