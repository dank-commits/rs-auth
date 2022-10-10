use std::convert::From;

use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use uuid::Error as ParseError;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),
    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError().json("Internal Server Error - Try again later."),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}