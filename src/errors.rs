use std::num::ParseIntError;
use actix_web::{error, HttpResponse, http::header::ContentType, http::StatusCode};
use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "not found")]
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_code = self.status_code();
        let error_response = ErrorResponse {
            code: error_code.as_u16(),
            message: self.to_string(),
        };
        HttpResponse::build(error_code).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadClientData => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}


impl From<mongodb::error::Error> for AppError {
    fn from(_error: mongodb::error::Error) -> Self {
        return AppError::InternalError;
    }
}

impl From<ParseIntError> for AppError {
    fn from(_error: ParseIntError) -> Self {
        return AppError::BadClientData;
    }
}
