# 👁️ Solana Transaction Monitor

A robust and modular application written in Rust to continuously monitor a specific Solana wallet address. It provides real-time updates on the wallet's SOL balance and fetches the latest transaction history.

This project demonstrates a multi-file, production-ready structure for interacting with the Solana RPC API.

## 🚀 Getting Started

### Prerequisites

Before you begin, ensure you have the following tools installed:

* **Rust and Cargo:** Use `rustup` to install the latest stable version of Rust.
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
    ```
* **A Solana Wallet Address:** You will need a valid Solana Public Key, preferably one with some test SOL on **Devnet** for easy testing.

### Building and Running

1.  **Clone the repository:** (If using Git)
    ```bash
    git clone [https://github.com/YourUsername/solana-tx-monitor.git](https://github.com/YourUsername/solana-tx-monitor.git)
    cd solana-tx-monitor
    ```
    (Or create the files manually in a new directory)

2.  **Build the project:**
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/release/solana-tx-monitor`.

3.  **Run the monitor:**
    You must provide the public key using the `-p` or `--pubkey` flag.

    ```bash
    # Example using Devnet and default settings (check every 10s, fetch 3 transactions)
    cargo run --release -- -p <YOUR_SOLANA_PUBLIC_KEY>
    ```

### Command Line Arguments

The application is configured using command-line flags (handled by `clap`):

| Flag | Full Name | Description | Default Value |
| :--- | :--- | :--- | :--- |
| `-p` | `--pubkey` | **(Required)** The target wallet address to monitor. | *None* |
| `-r` | `--rpc-url` | The URL of the Solana RPC node to connect to. | `https://api.devnet.solana.com` |
| `-i` | `--interval-sec` | The frequency of the monitoring cycle (in seconds). | `10` |
| `-t` | `--tx-limit` | The maximum number of recent transactions to fetch per cycle. | `3` |

#### Example: Monitoring Mainnet with Custom Interval

To monitor a Mainnet address every 5 seconds and fetch the 5 latest transactions:

```bash
cargo run --release -- \
    -p <YOUR_MAINNET_PUBLIC_KEY> \
    -r [https://api.mainnet-beta.solana.com](https://api.mainnet-beta.solana.com) \
    -i 5 \
    -t 5