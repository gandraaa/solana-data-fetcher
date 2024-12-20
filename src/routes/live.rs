use rocket::get;
use rocket::response::stream::{Event, EventStream};
use rocket::response::content::RawHtml;
use rocket::State;
use std::sync::Arc;
use crate::state; // Import the state module

#[get("/live")]
pub fn live_page() -> RawHtml<&'static str> {
    RawHtml(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Live Updates</title>
            <script src="https://unpkg.com/htmx.org"></script>
        </head>
        <body>
            <h1>Live Balance Updates</h1>
            <div id="balance">Waiting for updates...</div>
            <script>
                const eventSource = new EventSource('/live/stream');
                eventSource.onmessage = (event) => {
                    document.getElementById('balance').textContent = `Balance: ${event.data} SOL`;
                };
                eventSource.onerror = (error) => {
                    console.error("Error with event stream:", error);
                };
            </script>
        </body>
        </html>
    "#)
}

#[get("/live/stream")]
pub async fn live_stream(state: &State<Arc<state::LiveState>>) -> EventStream![] {
    let mut rx = state.geyser_tx.subscribe();

    EventStream! {
        loop {
            match rx.recv().await {
                Ok(balance) => {
                    yield Event::data(balance.to_string());
                }
                Err(_) => break,
            }
        }
    }
}
