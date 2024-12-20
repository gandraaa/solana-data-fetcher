#[macro_use]
extern crate rocket;

mod routes;
mod templates;
mod state;

use rocket::fs::{relative, FileServer};
use templates::init_templates;
use state::{AppState, LiveState, update_sol_to_usd, start_geyser_stream};
use std::sync::Arc;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Create shared states
    let app_state = Arc::new(AppState::new());
    let (geyser_tx, _) = rocket::tokio::sync::broadcast::channel(100); // Create broadcast channel
    let live_state = Arc::new(LiveState { geyser_tx });

    // Create the Rocket instance
    let rocket = rocket::build()
        .manage(app_state.clone()) // Attach AppState for USD conversion
        .manage(live_state.clone()) // Attach LiveState for Geyser stream
        .manage(init_templates())
        .mount("/", routes![
            routes::balance::balance_page,
            routes::about::about_page,
            routes::live::live_page,
            routes::live::live_stream // New route for live updates
        ])
        .mount("/static", FileServer::from(relative!("static")));

    // Spawn background tasks
    rocket::tokio::spawn(update_sol_to_usd(app_state)); // Update SOL to USD
    rocket::tokio::spawn(start_geyser_stream(live_state)); // Start Geyser stream

    // Launch Rocket
    rocket.launch().await?;

    Ok(())
}
