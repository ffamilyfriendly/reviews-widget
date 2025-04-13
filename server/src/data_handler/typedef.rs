use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(serde::Deserialize, Serialize, Clone)]
pub struct ScoreDistribution {
    pub key: u8,
    pub value: u32,
}

#[derive(serde::Deserialize, Serialize, Clone)]
pub struct Score {
    #[serde(alias = "averageScore")]
    pub average_score: f32,
    #[serde(alias = "reviewCount")]
    pub review_count: u32,
    #[serde(alias = "scoreDistribution")]
    pub score_distribution: Vec<ScoreDistribution>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Author {
    pub username: String,
    #[serde(alias = "avatarUrl")]
    pub avatar_url: Option<String>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Review {
    pub content: String,
    pub score: u8,
    pub author: Author,
    pub timestamp: String
}

#[derive(Deserialize, Serialize)]
pub struct Reviews {
    pub reviews: Vec<Review>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ReturnData {
    pub reviews: Vec<Review>,
    pub name: String,
    pub id: String,
    #[serde(alias = "iconUrl")]
    pub icon_url: Option<String>,
    #[serde(alias = "reviewStats")]
    pub review_stats: Score
}

#[derive(Deserialize, Serialize)]
pub struct EntityExternal {
    #[serde(alias = "entityExternal")]
    pub entity_external: Option<ReturnData>
}

#[derive(Deserialize, Serialize)]
pub struct GraphQlError {
    pub message: String
}

#[derive(Deserialize, Serialize)]
pub struct ApiResponse {
    pub data: Option<EntityExternal>,
    pub errors: Option<GraphQlError>
}

impl ApiResponse {
    pub fn get_inner(self) -> Result<ReturnData, crate::error::Error> {
        if let Some(data) = self.data {
            match data.entity_external {
                Some(data) => Ok(data),
                None => Err(crate::error::Error::GraphQL(crate::error::GraphQlError::NoData))
            }
        } else if let Some(errors) = self.errors {
            Err(crate::error::Error::GraphQL(crate::error::GraphQlError::from(errors)))
        } else {
            Err(crate::error::Error::GraphQL(crate::error::GraphQlError::NoErrOrData))
        }
    }
}
