#[macro_use]
extern crate rocket;

mod routes;
mod templates;
mod state;

use rocket::fs::{relative, FileServer};
use templates::init_templates;
use state::{AppState, update_sol_to_usd};
use std::sync::Arc;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let state = Arc::new(AppState::new());

    // Create the Rocket instance
    let rocket = rocket::build()
        .manage(state.clone()) // Attach the shared state
        .manage(init_templates())
        .mount("/", routes![routes::balance::balance_page, routes::about::about_page])
        .mount("/static", FileServer::from(relative!("static")));

    // Spawn the background task using Rocket's runtime
    rocket::tokio::spawn(update_sol_to_usd(state));

    // Launch Rocket
    rocket.launch().await?;

    Ok(())
}
