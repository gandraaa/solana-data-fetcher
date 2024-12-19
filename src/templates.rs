use handlebars::Handlebars;
use std::sync::Arc;

pub fn init_templates() -> Arc<Handlebars<'static>> {
    let mut handlebars = Handlebars::new();

    // DELETE THIS
    // Automatically reload templates in development mode
    if std::env::var("ROCKET_ENV").unwrap_or_default() == "development" {
        handlebars.set_dev_mode(true); // Enable development mode for Handlebars
    }
    // DELETE THIS

    handlebars
        .register_template_file("layout", "./templates/partials/layout.hbs")
        .expect("Failed to register layout template");

    handlebars
        .register_template_file("balance", "./templates/balance.hbs")
        .expect("Failed to register balance template");

    handlebars
        .register_template_file("about", "./templates/about.hbs")
        .expect("Failed to register about template");

    Arc::new(handlebars)
}
