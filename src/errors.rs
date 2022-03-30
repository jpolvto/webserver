use std::fmt;
use actix_web::{error, HttpResponse, http::StatusCode, ResponseError};
use actix_web::error::{JsonPayloadError, PathError};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum AppError {
    InternalError,
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

impl From<&PathError> for ErrorResponse {
    fn from(error: &PathError) -> Self {
        return ErrorResponse {
            code: error.status_code().as_u16(),
            message: error.to_string(),
        };
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::InternalError => write!(f, "internal error"),
            AppError::NotFound => write!(f, "not found"),
        }
    }
}

impl error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_code = self.status_code();
        let error_response = ErrorResponse {
            code: error_code.as_u16(),
            message: self.to_string(),
        };
        HttpResponse::build(error_code).json(error_response)
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(_error: mongodb::error::Error) -> Self {
        return AppError::InternalError;
    }
}
