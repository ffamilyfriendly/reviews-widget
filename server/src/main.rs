use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{App, HttpServer};
use actix_files::Files;
pub mod data_handler;
pub mod route_handlers;
pub mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let governor_conf = GovernorConfigBuilder::default()
        .seconds_per_request(3)
        .burst_size(20)
        .finish()
        .expect("could not configure ratelimit governor.");

    HttpServer::new(move || {
        App::new()
            .service(route_handlers::widget::get_base_data)
            .service(Files::new("/static", "./static").prefer_utf8(true))
            .wrap(Governor::new(&governor_conf))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}