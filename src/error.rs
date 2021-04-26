use actix_web::{
    error::ResponseError, 
    http::StatusCode, 
    HttpResponse
};
use actix::MailboxError;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DieselError},
};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use libreauth::pass::ErrorCode as PassErrorCode;
use serde_json::{Map as JsonMap, Value as JsonValue, json};
use std::convert::From;
use validator::ValidationErrors;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // 401
    #[error("Unauthorized: {}", .0)]
    Unauthorized(JsonValue),

    // 403
    #[error("Forbidden: {}", .0)]
    Forbidden(JsonValue),

    // 404
    #[error("Not Found: {}", .0)]
    NotFound(JsonValue),

    // 422
    #[error("Unprocessable Entity: {}", .0)]
    UnprocessableEntity(JsonValue),

    // 400
    #[error("Validation failed: {}", .0)]
    ValidationFailed(JsonValue),

    // 500
    #[error("Internal Server Error")]
    InternalServerError,
}

// the ResponseError trait lets us convert errors to http responses with appropriate data
// https://actix.rs/docs/errors/
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::Unauthorized(ref message) => HttpResponse::Unauthorized().json(message),
            Error::Forbidden(ref message) => HttpResponse::Forbidden().json(message),
            Error::NotFound(ref message) => HttpResponse::NotFound().json(message),
            Error::UnprocessableEntity(ref message) => {
                HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(message)
            },
            Error::ValidationFailed(ref message) => {
                HttpResponse::build(StatusCode::BAD_REQUEST).json(message)
            }
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
        }
    }
}

impl From<MailboxError> for Error {
    fn from(_error: MailboxError) -> Self {
        Error::InternalServerError
    }
}

impl From<JwtError> for Error {
    fn from(error: JwtError) -> Self {
        match error.kind() {
            JwtErrorKind::InvalidToken => Error::Unauthorized(json!({
                "error": "Token is invalid",
            })),
            JwtErrorKind::InvalidIssuer => Error::Unauthorized(json!({
                "error": "Issuer is invalid",
            })),
            _ => Error::Unauthorized(json!({
                "error": "An issue was found with the token provided",
            })),
        }
    }
}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::UnprocessableEntity(json!({ "error": message }));
                }
                Error::InternalServerError
            }
            DieselError::NotFound => {
                Error::NotFound(json!({ "error": "requested record was not found" }))
            }
            _ => Error::InternalServerError,
        }
    }
}

impl From<PoolError> for Error {
    fn from(_error: PoolError) -> Self {
        Error::InternalServerError
    }
}

impl From<PassErrorCode> for Error {
    fn from(_error: PassErrorCode) -> Self {
        Error::InternalServerError
    }
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();

        // transforms errors into objects that err_map can take
        for (field, errors) in errors.field_errors().iter() {
            let errors: Vec<JsonValue> = errors
                .iter()
                .map(|error| {
                    // dbg!(error) // <- Uncomment this if you want to see what error looks like
                    json!(error.message)
                })
                .collect();
            err_map.insert(field.to_string(), json!(errors));
        }

        Error::ValidationFailed(json!({
            "errors": err_map,
        }))
    }
}
