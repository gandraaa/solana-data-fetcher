use handlebars::Handlebars;
use rocket::{get, State};
use rocket::response::content::RawHtml;
use serde_json::{Map, Value};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;
use std::str::FromStr;
use crate::state::AppState;

#[get("/?<public_key>")]
pub fn balance_page(
    public_key: Option<String>,
    state: &State<Arc<AppState>>, // Access the shared state
    handlebars: &State<Arc<Handlebars<'_>>>,
) -> RawHtml<String> {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let mut data = Map::new();
    let sol_to_usd = *state.sol_to_usd.lock().unwrap(); // Get the cached conversion rate

    if let Some(pk) = public_key.clone() {
        match Pubkey::from_str(&pk) {
            Ok(pubkey) => match client.get_balance(&pubkey) {
                Ok(balance) => {
                    let sol_balance = balance as f64 / 1_000_000_000.0;
                    let usd_balance = sol_balance * sol_to_usd;

                    data.insert("balance".to_string(), Value::from(sol_balance));
                    data.insert("usd_balance".to_string(), Value::from(format!("{:.2}", usd_balance))); // Format USD to 2 decimals
                }
                Err(_) => {
                    data.insert("error".to_string(), Value::from("Failed to fetch balance from Solana RPC."));
                }
            },
            Err(_) => {
                data.insert("error".to_string(), Value::from("Invalid public key format."));
            }
        }
        data.insert("public_key".to_string(), Value::from(pk));
    } else {
        data.insert("error".to_string(), Value::from("Please enter a public key to check the balance."));
    }

    data.insert("css_file".to_string(), Value::from("balance.css"));
    RawHtml(handlebars.render("balance", &data).unwrap())
}
