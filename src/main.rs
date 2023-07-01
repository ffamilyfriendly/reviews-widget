#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::tokio::sync::Mutex;
use rocket_dyn_templates::Template;
use rocket::shield::Shield;
use rocket::shield::{Frame};
use rocket_governor;


pub mod fetcher;
pub mod http;

pub struct AppState {
    client: Mutex<fetcher::fetch::TopClient>
}

fn get_shield() -> Shield {
    Shield::default()
        .disable::<Frame>()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage( AppState { client: Mutex::new(fetcher::fetch::TopClient::new()) } )
        .attach(Template::fairing())
        .attach(get_shield())
        .mount("/", http::routes())
        .register("/", catchers![rocket_governor::rocket_governor_catcher])
}