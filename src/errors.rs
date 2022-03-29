use std::fmt;
use actix_web::{error, HttpResponse, http::StatusCode};
use derive_more::{Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, Error)]
pub enum AppError {
    InternalError,
    BadClientData,
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::BadClientData => write!(f, "bad request"),
            AppError::InternalError => write!(f, "internal error"),
            AppError::NotFound => write!(f, "not found"),
        }
    }
}

impl error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadClientData => StatusCode::BAD_REQUEST,
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


