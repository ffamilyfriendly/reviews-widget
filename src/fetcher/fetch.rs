use std::{error::Error, cmp::{max, min}, collections::{HashMap, HashSet}};
use serde::{Deserialize, Serialize};
use reqwest;
use chrono::{DateTime, Datelike};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Review {
    pub rating: u8,
    pub timestamp: String,
    pub votes: u16,
    pub content: String,
    pub author_display_name: String,
    pub author_profile_picture: String,
    score_factor: u16
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

fn calculate_score_factor( rev_data: &RawApiResponse ) -> u16 {

    /*
        TODO: I kinda wanna refractor this until the result is a float where a 1.0 means highest relevance
        and 0.0 means zero relevance. I'm no good at these type of algos but I'll take a peep around the internet for inspiration

        works decently for now
    */

    let mut score: u16 = 0;

    // Create a HashSet with all the "words" (segments of text terminated by ' ')
    let words: HashSet<&str> = rev_data.content.split(" ").collect();

    // Decode the timestamp to an Arr [year, month, day]
    let timestamp_arr: Vec<u32> = rev_data.timestamp[..10].split("-").map(|x| x.parse::<u32>().unwrap_or(2016)).collect();

    // Get the number of days ago the review was submitted
    let days_since: u16 = (|| {
        let current_date: chrono::NaiveDate = chrono::UTC::now().naive_utc().date();
        let review_date = chrono::NaiveDate::from_ymd_opt(timestamp_arr[0] as i32, timestamp_arr[1], timestamp_arr[2]).unwrap();
        let delta = current_date - review_date;

        return delta.num_days() as u16;
    })();

    // divide the len of unique words by 2 and return that OR 12 (whichever is smallest) as score
    score += min(( words.len() as f32 / 2.0) as u16, 12);
    score += min(max( 0,  365 - days_since as i32), 100) as u16;
    score += rev_data.votes / 2;

    // Dunno if I actually want to rank based on the rating
    // as I want the widget to be fair but relevant
    score += rev_data.score as u16;

    score
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

                Review { score_factor: calculate_score_factor(&x), rating: ((x.score as f32 / 100.0) * 5.0) as u8, timestamp: x.timestamp[..10].to_string(), votes: x.votes, content: x.content, author_display_name: x.poster.username, author_profile_picture: x.poster.avatar.unwrap_or("".to_string()) }
            }).collect();
        if result.len() == 0 {
            break;
        }
        reviews_list.append(&mut result)
    }

    reviews_list.sort_by(| b, a | a.score_factor.cmp(&b.score_factor) );

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


