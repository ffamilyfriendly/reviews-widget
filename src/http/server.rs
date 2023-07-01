use std::path::Path;

use rocket::{State, fs::NamedFile};
use rocket_dyn_templates::{Template, context};
use rocket_governor::RocketGovernor;
use crate::{AppState};

use super::ratelimits::RateLimitGuard;

#[get("/embed/<id>?<css>&<limit>")]
pub async fn generate_embed( id: String, css: Option<String>, limit: Option<usize>, state: &State<AppState>, _ratelimits: RocketGovernor<'_, RateLimitGuard> ) -> Template {
    let shared_client = state.inner();
    let css_file = css.unwrap_or("/standard.css".into());
    let reviews_limit = limit.unwrap_or(10);

    match shared_client.client.lock().await.get_entity(id).await {
        Ok(mut entity_listing) => {
            entity_listing.reviews.truncate(reviews_limit);
            Template::render("reviews-widget", context!{ avg_votes: entity_listing.ratings_avg, reviews_count: entity_listing.reviews_count, reviews: entity_listing.reviews, css_file } )
        },
        Err(e) => {
            println!("something went dookie: {}", e);
            Template::render("error", context! { message: e.to_string() })
        }
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