use solana_client::{
    rpc_client::RpcClient,
    client_error::ClientError,
    rpc_response::{Response as RpcResponse, UiTransactionEncoding, ConfirmedBlock},
};
use solana_sdk::pubkey::Pubkey;
use std::{str::FromStr, time::Duration};
use tokio::time::interval;
use crate::config::Config; // Use the Config struct from the parent module

// The core monitoring function
pub async fn start_monitoring(config: Config) -> Result<(), ClientError> {
    
    // Parse the Public Key from the string configuration
    let pubkey = match Pubkey::from_str(&config.pubkey) {
        Ok(pk) => pk,
        Err(_) => {
            eprintln!("🛑 Error: Invalid Public Key format: {}", config.pubkey);
            return Ok(());
        }
    };
    
    // Initialize the RPC client
    let client = RpcClient::new(config.rpc_url.clone());
    
    // Setup the async interval ticker
    let mut ticker = interval(Duration::from_secs(config.interval_sec));

    // Main monitoring loop
    loop {
        // Wait for the next tick
        ticker.tick().await;

        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        println!("\n\n[{}] --- Starting New Cycle ---", now);

        // 1. Check Wallet Balance
        if let Err(e) = check_balance(&client, &pubkey).await {
            eprintln!("❌ Error during Balance Check: {}", e);
        }

        // 2. Check Recent Transactions
        if let Err(e) = check_recent_transactions(&client, &pubkey, config.tx_limit as usize).await {
            eprintln!("❌ Error during Transaction Check: {}", e);
        }
        
        println!("[{}] --- Cycle Finished ---", now);
    }
}

// Function to fetch and display the current wallet balance
async fn check_balance(client: &RpcClient, pubkey: &Pubkey) -> Result<(), ClientError> {
    match client.get_balance(pubkey) {
        Ok(lamports) => {
            // Convert Lamports to SOL (1 SOL = 1,000,000,000 Lamports)
            let sol_balance = lamports as f64 / 1_000_000_000.0;
            println!("💰 Current SOL Balance: {:.9} SOL (Lamports: {})", sol_balance, lamports);
        }
        Err(e) => {
            return Err(e);
        }
    }
    Ok(())
}

// Function to fetch and display recent transactions
async fn check_recent_transactions(client: &RpcClient, pubkey: &Pubkey, limit: usize) -> Result<(), ClientError> {
    println!("📋 Fetching last {} transactions...", limit);

    // Get the signatures of the latest transactions for the account
    match client.get_signatures_for_address_with_limit(pubkey, limit) {
        Ok(signatures) => {
            if signatures.is_empty() {
                println!("    No recent transactions found.");
                return Ok(());
            }

            println!("    Found {} recent transaction signatures.", signatures.len());
            
            // Iterate over signatures and fetch transaction details
            for (index, sig_info) in signatures.iter().enumerate() {
                // Fetch the confirmed transaction details (full info, not just metadata)
                match client.get_transaction(&sig_info.signature, solana_client::rpc_request::RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::Json),
                    commitment: Some(solana_sdk::commitment_config::CommitmentConfig::confirmed()),
                    max_supported_transaction_version: Some(0),
                }) {
                    Ok(tx_response) => {
                        // Display minimal transaction info
                        let tx_hash = sig_info.signature.to_string();
                        let block_time = tx_response.block_time.map_or("N/A".to_string(), |t| {
                            chrono::DateTime::from_timestamp(t, 0)
                                .map(|dt| dt.with_timezone(&chrono::Local).format("%H:%M:%S").to_string())
                                .unwrap_or("N/A".to_string())
                        });

                        println!("    Tx {}: Hash: {} (Time: {})", index + 1, &tx_hash[..12], block_time);

                        // Note: Parsing the full transaction details (who sent what) is complex
                        // and would require much more code to handle different instruction types (System, Token, etc.).
                        // We keep it simple here by just displaying the hash and time.
                    },
                    Err(e) => {
                        eprintln!("    Warning: Could not fetch details for transaction {}: {}", sig_info.signature, e);
                    }
                }
            }
        },
        Err(e) => {
            return Err(e);
        }
    }
    Ok(())
}