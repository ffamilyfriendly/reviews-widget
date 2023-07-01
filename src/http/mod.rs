use rocket::Route;

pub mod server;
pub mod ratelimits;
pub mod headers;

pub fn routes() -> Vec<Route> {
    routes![
        server::generate_embed,
        server::generate_embed_png,
        server::get_standard_css
    ]
}