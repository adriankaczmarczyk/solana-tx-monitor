mod config;
mod monitor;

use config::Config;

// Main application entry point.
#[tokio::main]
async fn main() -> Result<(), solana_client::client_error::ClientError> {
    // 1. Parse configuration
    let config = Config::parse();

    // 2. Display startup info
    config.display_info();

    // 3. Start the core monitoring loop
    // Note: monitor::start_monitoring handles the actual business logic
    monitor::start_monitoring(config).await
}