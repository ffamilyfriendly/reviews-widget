use actix_web::http::StatusCode;
use validator::ValidationErrors;

impl From<crate::data_handler::typedef::GraphQlError> for GraphQlError  {
    fn from(value: crate::data_handler::typedef::GraphQlError) -> Self {
        println!("ERR: {}", value.message);
        match value.message.as_str() {
            "hello" => return GraphQlError::NoErrOrData,
            any => return GraphQlError::UnknownError(any.to_owned())
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GraphQlError {
    #[error("did not find bot with id `{0}`")]
    NotFound(String),

    #[error("entityExternal was null. No data was passed")]
    NoData,

    #[error("ApiResponse had neither data or error messages")]
    NoErrOrData,

    #[error("Unknown error: {0}")]
    UnknownError(String)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error(transparent)]
    GraphQL(#[from] GraphQlError),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    ValidationError(#[from] ValidationErrors)
}


impl actix_web::error::ResponseError for Error {

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::GraphQL(e) => match e {
                GraphQlError::NoData => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR
            },
            Error::Reqwest(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ValidationError(_) => StatusCode::BAD_REQUEST
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;