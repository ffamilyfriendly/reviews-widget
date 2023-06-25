use rocket::{State};
use rocket_dyn_templates::{Template, context};
use crate::{AppState};

#[get("/embed/<id>")]
pub async fn generate_embed( id: String, state: &State<AppState> ) -> Template {
    let shared_client = state.inner();

    match shared_client.client.lock().await.get_entity(id).await {
        Ok(entity_listing) => Template::render("reviews-widget", context!{ avg_votes: entity_listing.ratings_avg, reviews_count: entity_listing.reviews_count, reviews: entity_listing.reviews } ),
        Err(_e) => Template::render("error", context! { message: "something went wrong" })
    }
}