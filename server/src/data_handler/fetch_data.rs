use std::collections::HashMap;
use cached::proc_macro::cached;

use chrono::{DateTime, Utc};
use reqwest::header;
use crate::error::Result;

use super::typedef::{ApiResponse, ReturnData};

const QUERY_URL: &str = "https://api.top.gg/graphql";

fn get_graphql_query(bot_id: &str) -> String {
    format!("query {{
        entityExternal(externalId: \"{}\", platform: DISCORD, type: BOT) {{
            name
            id
            iconUrl
            reviewStats {{
                averageScore
                reviewCount
                scoreDistribution {{
                    key
                    value
                }}
            }}
            reviews(limit: 20) {{
                content
                score
                timestamp
                author {{
                    username
                    avatarUrl
                }}
            }}
        }}
    }}", bot_id)
}

fn generate_headers() -> reqwest::header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert("accept", header::HeaderValue::from_static("application/json"));
    headers.insert("Authorization", header::HeaderValue::from_static("Bearer whatever"));
    
    return headers
}

fn generate_client() -> reqwest::Client {
    let headers = generate_headers();
    reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build().expect("could not generate HTTP client")
}

#[cached(time = 600, result=true)]
pub async fn get_data_json(bot_id: String) -> Result<ReturnData> {
    
    let client = generate_client();

    let mut map = HashMap::new();
    map.insert("query", get_graphql_query(&bot_id));

    let res = client.post(QUERY_URL)
        .json(&map)
        .send()
        .await?;

    let mut data = res.json::<ApiResponse>().await?.get_inner()?;

    data.reviews.sort_by(|a, b| {
        let a_timestamp = DateTime::parse_from_rfc3339(&a.timestamp).unwrap_or(Utc::now().into());
        let b_timestamp = DateTime::parse_from_rfc3339(&b.timestamp).unwrap_or(Utc::now().into());
        b_timestamp.cmp(&a_timestamp)
    } );

    Ok(data)
}

pub async fn get_data_text(bot_id: &str) -> Result<String> {
    
    let client = generate_client();

    let mut map = HashMap::new();
    map.insert("query", get_graphql_query(bot_id));

    let res = client.post(QUERY_URL)
        .json(&map)
        .send()
        .await?;


    Ok(res.text().await?)
}


