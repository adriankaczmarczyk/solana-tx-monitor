use clap::Parser;

// Defines the command-line arguments structure.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Public key of the wallet to monitor (e.g., SomeE8M1GgBwD7... - example)
    #[arg(short, long)]
    pub pubkey: String,

    /// RPC URL for the Solana node (e.g., https://api.devnet.solana.com)
    #[arg(short, long, default_value = "https://api.devnet.solana.com")]
    pub rpc_url: String,

    /// Refresh interval in seconds for the balance check
    #[arg(short, long, default_value_t = 10)]
    pub interval_sec: u64,

    /// Number of recent transactions to fetch and display (max 5)
    #[arg(short, long, default_value_t = 3)]
    pub tx_limit: u8,
}

impl Config {
    /// Prints the current configuration settings.
    pub fn display_info(&self) {
        println!("\n=======================================================");
        println!("🚀 Starting Solana Transaction Monitor");
        println!("=======================================================");
        println!("✅ Monitoring Address: {}", self.pubkey);
        println!("🔗 RPC Endpoint: {}", self.rpc_url);
        println!("⏱️ Balance Check Interval: {} seconds", self.interval_sec);
        println!("#️⃣ Max Transactions to Fetch: {}", self.tx_limit);
        println!("-------------------------------------------------------");
    }
}