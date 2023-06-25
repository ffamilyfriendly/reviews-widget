use std::path::Path;

use rocket::{State, fs::NamedFile};
use rocket_dyn_templates::{Template, context};
use crate::{AppState};

#[get("/embed/<id>?<css>")]
pub async fn generate_embed( id: String, css: Option<String>, state: &State<AppState> ) -> Template {
    let shared_client = state.inner();
    let css_file = css.unwrap_or("/standard.css".into());

    match shared_client.client.lock().await.get_entity(id).await {
        Ok(entity_listing) => Template::render("reviews-widget", context!{ avg_votes: entity_listing.ratings_avg, reviews_count: entity_listing.reviews_count, reviews: entity_listing.reviews, css_file } ),
        Err(_e) => Template::render("error", context! { message: "something went wrong" })
    }
}

#[get("/image/<id>")]
pub async fn generate_embed_png( id: String, _state: &State<AppState> ) -> String {
    format!("not implemented for {}", id).to_string()
}

#[get("/standard.css")]
pub async fn get_standard_css() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("static/standard.css")).await
}