use rocket::Route;

pub mod server;

pub fn routes() -> Vec<Route> {
    routes![
        server::generate_embed
    ]
}