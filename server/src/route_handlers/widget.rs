use actix_web::{get, http::header::{HeaderValue, CACHE_CONTROL}, web::{self}, HttpResponse, Responder};
use serde::Deserialize;
use crate::data_handler::{self};
use validator::{Validate, ValidationError};
use crate::error::Result;


#[derive(Deserialize, Validate)]
struct Path {
    #[validate(custom(function = "validate_discord_id"))]
    bot_id: String
}

fn validate_discord_id(id: &str) -> Result<(), ValidationError> {
    
    let range = 17..19;
    if !range.contains(&id.len()) {
        return Err(ValidationError::new("is not 17 - 19 characters"))
    }

    if !id.parse::<u64>().is_ok() {
        return Err(ValidationError::new("is not numeric"))
    }

    Ok(())
}


#[get("/widget/{bot_id}")]
async fn get_base_data(path: web::Path<Path>) -> Result<impl Responder> {
    path.validate()?;
    let data = web::Json(data_handler::fetch_data::get_data_json(path.bot_id.clone()).await?);

    let response = HttpResponse::Ok()
        .append_header((CACHE_CONTROL, HeaderValue::from_static("max-age=1800")))
        .json(data);

    Ok(response)
}

