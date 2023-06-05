use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    InvalidToken(String),
    WrongCredentials(String),
    InternalServerError,
    ValidationError(String),
    BadRequestError(String),
    UnAuthorized(String),
    NotFound(String),
    Forbidden
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_message) = match self {
            Self::InvalidToken(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::WrongCredentials(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "something went wrong".to_owned(),
            ),
            Self::BadRequestError(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::UnAuthorized(msg)=>(StatusCode::UNAUTHORIZED,msg),
            Self::NotFound(msg)=>(StatusCode::NOT_FOUND,msg),
            Self::Forbidden=>(StatusCode::NOT_FOUND,"Unauthorized to acess the content".to_owned()),
        };
        (status, Json(json!({ "error": err_message }))).into_response()
    }
}
