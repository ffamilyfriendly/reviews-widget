use std::{error::Error, collections::HashMap};
use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Review {
    pub rating: u8,
    pub timestamp: String,
    pub votes: u16,
    pub content: String,
    pub author_display_name: String,
    pub author_profile_picture: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReviewsListing {
    pub id: String,
    pub ratings_avg: f32,
    pub reviews: Vec<Review>,
    pub reviews_count: u16
}

#[derive(Deserialize, Serialize)]
struct RawApiPoster {
    id: String,
    username: String,
    avatar: Option<String>
}

#[derive(Deserialize, Serialize)]
struct RawApiResponse {
    id: String,
    score: u8,
    content: String,
    timestamp: String,
    votes: u16,
    poster: RawApiPoster
}


async fn fetch_entity_reviews( id: String ) -> Result<ReviewsListing, Box<dyn Error>> {
    
    let mut reviews_score_total: u16 = 0;
    let mut reviews_count: u16 = 0;
    let mut reviews_list: Vec<Review> = Vec::new();

    for page in 1..5 {
        println!("page {} of entity {}", page, &id);
        let topgg_reviews_url = format!("https://top.gg/api/client/entities/{}/reviews?page={}", &id, page);
        let mut result: Vec<Review> = reqwest::get(topgg_reviews_url).await?.json::<Vec<RawApiResponse>>().await?
            .into_iter().map(|x| {
                reviews_count += 1;
                reviews_score_total += x.score as u16;

                Review { rating: ((x.score as f32 / 100.0) * 5.0) as u8, timestamp: x.timestamp[..10].to_string(), votes: x.votes, content: x.content, author_display_name: x.poster.username, author_profile_picture: x.poster.avatar.unwrap_or("".to_string()) }
            }).collect();
        if result.len() == 0 {
            break;
        }
        reviews_list.append(&mut result)
    }

    reviews_list.sort_by(| b, a | a.timestamp.cmp(&b.timestamp).then(a.rating.cmp(&b.rating)) );

    Ok(ReviewsListing {
        id: id,
        ratings_avg: (reviews_score_total / reviews_list.len() as u16) as f32,
        reviews: reviews_list,
        reviews_count: reviews_count
    })
}

pub struct TopClient {
    cache: HashMap<String, ReviewsListing>
}

// raw reviews string:
// https://top.gg/api/client/entities/870715447136366662/reviews

impl TopClient {
    pub fn new() -> TopClient {
        TopClient { cache:  HashMap::new() }
    }

    pub async fn get_entity(&mut self, id: String) -> Result<ReviewsListing, Box<dyn Error>> {
        if self.cache.contains_key(&id) {
            return Ok(self.cache.get(&id).unwrap().clone())
        }

        match fetch_entity_reviews(id.to_string()).await {
            Ok(entity_reviews) => {
                self.cache.insert(id, entity_reviews.clone());
                Ok(entity_reviews)
            },
            Err(e) => Err(e)
        }
    }
}


