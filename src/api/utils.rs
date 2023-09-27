use std::collections::HashMap;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use log::error;

/// Represents an error that can be converted into a JSON API response.
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
    pub fn from_sqlx(error: sqlx::Error) -> Self {
        let (status_code, message) = match error {
            sqlx::Error::RowNotFound => {
                (StatusCode::NOT_FOUND, "Resource not found")
            }
            _ => {
                error!("Internal error during query: {error}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal database error")
            }
        };
        Self {
            status_code,
            message: message.to_owned(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(HashMap::from([("error_message", self.message)])),
        )
            .into_response()
    }
}
