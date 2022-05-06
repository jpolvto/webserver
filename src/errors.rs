use std::fmt;
use actix_web::{HttpResponse, ResponseError};
use actix_web::error::{JsonPayloadError, QueryPayloadError};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    InternalError(String),
    BadRequest(String),
    NotFound,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::InternalError(ref cause) => write!(f, "internal error: {}", cause),
            AppError::BadRequest(ref cause) => write!(f, "bad request: {}", cause),
            AppError::NotFound => write!(f, "not found"),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::BadRequest(err) => {
                HttpResponse::BadRequest().json(json!({
                    "code": self.status_code().as_u16(),
                    "message": err.to_string() 
                }))
            }
            AppError::InternalError(err) => {
                HttpResponse::InternalServerError().json(json!({
                    "code": self.status_code().as_u16(),
                    "message": err.to_string() 
                }))
            }
            AppError::NotFound => HttpResponse::NotFound().finish(),
        }
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(error: mongodb::error::Error) -> Self {
        return AppError::InternalError(error.to_string());
    }
}

impl From<JsonPayloadError> for AppError {
    fn from(error: JsonPayloadError) -> Self {
        return AppError::InternalError(error.to_string());
    }
}

impl From<QueryPayloadError> for AppError {
    fn from(error: QueryPayloadError) -> Self {
        return AppError::InternalError(error.to_string());
    }
}

impl From<bson::ser::Error> for AppError {
    fn from(error: bson::ser::Error) -> Self {
        return AppError::BadRequest(error.to_string());
    }
}

impl From<bson::de::Error> for AppError {
    fn from(error: bson::de::Error) -> Self {
        return AppError::BadRequest(error.to_string());
    }
}
