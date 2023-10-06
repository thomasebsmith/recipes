use std::collections::HashMap;
use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use log::error;

use crate::database;

/// Represents an error that can be converted into a JSON API response.
#[derive(Debug)]
pub struct Error {
    status_code: StatusCode,
    message: String,
}

impl Error {
    /// Creates an API error from a SQLx error.
    ///
    /// `RowNotFound` errors are converted into 404 errors.
    ///
    /// All other errors are converted into 500 errors.
    pub fn from_db(error: database::Error) -> Self {
        let (status_code, message) = match error {
            database::Error::BadArguments(message) => {
                (StatusCode::BAD_REQUEST, message)
            }
            database::Error::Sql(sqlx::Error::RowNotFound) => {
                (StatusCode::NOT_FOUND, "Resource not found".to_owned())
            }
            _ => {
                error!("Internal error during query: {error}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal database error".to_owned(),
                )
            }
        };
        Self {
            status_code,
            message,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> Result<(), fmt::Error> {
        write!(formatter, "{}: {}", self.status_code, self.message)
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(HashMap::from([("error_message", self.message)])),
        )
            .into_response()
    }
}
