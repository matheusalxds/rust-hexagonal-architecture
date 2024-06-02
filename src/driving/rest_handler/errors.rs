use std::fmt::{Display, Formatter, Result};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(Debug, PartialEq)]
pub enum ApiError {
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    ValidationError(Vec<String>),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ApiError::BadRequest(err)
            | ApiError::InternalServerError(err)
            | ApiError::NotFound(err)
            | ApiError::InvalidData(err)
            | ApiError::Conflict(err)
            | ApiError::Unknown(err) => writeln!(f, "{},", err),
            ApiError::ValidationError(mex_vec) => mex_vec.iter().fold(Ok(()), |result, err| {
                result.and_then(|_| writeln!(f, "{}, ", err))
            }),
        }
    }
}

// automatically convert ApiErrors to external ResponseError
impl ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            ApiError::BadRequest(error) => HttpResponse::BadRequest().json(error),
            ApiError::NotFound(error) => HttpResponse::NotFound().json(error),
            ApiError::ValidationError(error) => HttpResponse::UnprocessableEntity().json(error),
            ApiError::InternalServerError(error) => HttpResponse::InternalServerError().json(error),
            ApiError::Conflict(error) => HttpResponse::Conflict().json(error),
            ApiError::InvalidData(error) => HttpResponse::BadRequest().json(error),
            ApiError::Unknown(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
