use std::fmt;
use actix_web::{HttpResponse, ResponseError};
use actix_web::error::{JsonPayloadError, QueryPayloadError};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum AppError {
    InternalError(String),
    BadRequest(String),
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}

impl From<&JsonPayloadError> for ErrorResponse {
    fn from(error: &JsonPayloadError) -> Self {
        return ErrorResponse {
            code: error.status_code().as_u16(),
            message: error.to_string(),
        };
    }
}

impl From<&QueryPayloadError> for ErrorResponse {
    fn from(error: &QueryPayloadError) -> Self {
        return ErrorResponse {
            code: error.status_code().as_u16(),
            message: error.to_string(),
        };
    }
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
                let error_code = self.status_code();
                let error_response = ErrorResponse {
                    code: error_code.as_u16(),
                    message: err.to_string(),
                };
                HttpResponse::NotFound().json(error_response)
            }
            AppError::InternalError(err) => {
                let error_code = self.status_code();
                let error_response = ErrorResponse {
                    code: error_code.as_u16(),
                    message: err.to_string(),
                };
                HttpResponse::InternalServerError().json(error_response)
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