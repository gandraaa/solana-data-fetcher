use std::sync::Mutex;
use rocket::tokio::{time::{interval, Duration}};
use std::sync::Arc;
use reqwest;

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
