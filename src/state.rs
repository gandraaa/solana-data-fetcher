use std::sync::Mutex;
use rocket::tokio::{time::{interval, Duration}};
use std::sync::Arc;
use reqwest;
use tokio::sync::broadcast;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr; // Import this for `Pubkey::from_str`

pub struct AppState {
    pub sol_to_usd: Mutex<f64>, // Cached conversion rate
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            sol_to_usd: Mutex::new(0.0),
        }
    }
}

pub struct LiveState {
    pub geyser_tx: broadcast::Sender<f64>, // Sender for Geyser balance updates
}

pub async fn update_sol_to_usd(state: Arc<AppState>) {
    let mut interval = interval(Duration::from_secs(60)); // Update every 60 seconds

    loop {
        interval.tick().await;

        match fetch_sol_to_usd().await {
            Ok(price) => {
                let mut sol_to_usd = state.sol_to_usd.lock().unwrap();
                *sol_to_usd = price;
                println!("Updated SOL to USD: {}", price);
            }
            Err(err) => {
                eprintln!("Failed to fetch SOL to USD: {}", err);
            }
        }
    }
}

async fn fetch_sol_to_usd() -> Result<f64, reqwest::Error> {
    // Use a public API like CoinGecko to get the conversion rate
    let response = reqwest::get("https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd")
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response["solana"]["usd"].as_f64().unwrap_or(0.0))
}

pub async fn start_geyser_stream(state: Arc<LiveState>) {
    let mut interval = interval(Duration::from_secs(10)); // Simulate updates every 10 seconds
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url.to_string());
    let public_key = Pubkey::from_str("5ZDgE7dyb6524PpMQfUPWMNn2Cz3zgVxt7jpPFbVEq7E").unwrap();

    loop {
        interval.tick().await;

        if let Ok(balance) = client.get_balance(&public_key) {
            let sol_balance = balance as f64 / 1_000_000_000.0;
            if let Err(err) = state.geyser_tx.send(sol_balance) {
                eprintln!("Failed to send Geyser update: {}", err);
            }
        } else {
            eprintln!("Failed to fetch balance for {}", public_key);
        }
    }
}
