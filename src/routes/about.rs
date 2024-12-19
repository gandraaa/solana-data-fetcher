use handlebars::Handlebars;
use rocket::{get, State};
use rocket::response::content::RawHtml;
use std::sync::Arc;

#[get("/about")]
pub fn about_page(handlebars: &State<Arc<Handlebars<'_>>>) -> RawHtml<String> {
    let data = serde_json::Map::new(); // No dynamic data needed
    RawHtml(handlebars.render("about", &data).unwrap())
}
