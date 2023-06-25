#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::tokio::sync::Mutex;
use rocket_dyn_templates::Template;


pub mod fetcher;
pub mod http;

pub struct AppState {
    client: Mutex<fetcher::fetch::TopClient>
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage( AppState { client: Mutex::new(fetcher::fetch::TopClient::new()) } )
        .attach(Template::fairing())
        .mount("/", http::routes())
}