use std::{error::Error, collections::{HashMap, HashSet}};
use serde::{Deserialize, Serialize};
use reqwest;
use chrono;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Review {
    pub rating: u8,
    pub timestamp: String,
    pub votes: u16,
    pub content: String,
    pub author_display_name: String,
    pub author_profile_picture: String,
    pub score_factor: u8,
    pub score_breakdown: String
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

#[repr(u8)]
#[derive(Clone)]
enum Score {
    Bad = 0,
    Minimal = 1,
    Good = 2,
    Great = 3,
}

fn calculate_score_factor( rev_data: &RawApiResponse ) -> (u8, String) {
    // Create a HashSet with all the "words" (segments of text terminated by ' ')
    let words: HashSet<&str> = rev_data.content.split(" ").collect();

    // Decode the timestamp to an Arr [year, month, day]
    let timestamp_arr: Vec<u32> = rev_data.timestamp[..10].split("-").map(|x| x.parse::<u32>().unwrap_or(2016)).collect();

    // Get the number of days ago the review was submitted
    let date_score: Score = (|| {
        let current_date: chrono::NaiveDate = chrono::UTC::now().naive_utc().date();
        let review_date = chrono::NaiveDate::from_ymd_opt(timestamp_arr[0] as i32, timestamp_arr[1], timestamp_arr[2]).unwrap();
        let delta_days = (current_date - review_date).num_days();

        match delta_days {
            0..=31 => Score::Great,
            32..=182 => Score::Good,
            183..=365 => Score::Minimal,
            _ => Score::Bad
        }
    })();

    // divide the len of unique words by 2 and return that OR 12 (whichever is smallest) as score
    let word_count_score: Score = match words.len() {
        2..=7 => Score::Minimal,
        8..=15 => Score::Good,
        16..=100 => Score::Great,
        _ => Score::Bad
    };

    let votes_relevant_score: Score = match rev_data.votes {
        1..=5 => Score::Minimal,
        6..=20 => Score::Good,
        21.. => Score::Great,
        _ => Score::Bad
    };

    let good_vote_score = (rev_data.score == 100) as u8;

    (good_vote_score + (date_score.clone() as u8) + (word_count_score.clone() as u8) + (votes_relevant_score.clone() as u8), format!("w: {}, t: {}, v: {}, r: {}", word_count_score as u8, date_score as u8, votes_relevant_score as u8, good_vote_score))
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

                let temp_score_factor = calculate_score_factor(&x);

                let avatar: String = match x.poster.avatar {
                    Some(img_link) => {
                        if img_link.starts_with("https://") {
                            img_link
                        } else {
                            "https://notused.example.com".to_string()
                        }
                    },
                    None => "https://notused.example.com".to_string()
                };

                Review { score_factor: temp_score_factor.0, score_breakdown: temp_score_factor.1, rating: ((x.score as f32 / 100.0) * 5.0) as u8, timestamp: x.timestamp[..10].to_string(), votes: x.votes, content: x.content, author_display_name: x.poster.username, author_profile_picture: avatar }
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


