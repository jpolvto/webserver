use std::num::ParseIntError;
use actix_web::{error, HttpResponse, http::header::ContentType, http::StatusCode};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "not found")]
    NotFound,
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(format!("{{ \"status_code\":{}, \"message\": \"{}\" }}", self.status_code().as_u16(), self.to_string()))
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
