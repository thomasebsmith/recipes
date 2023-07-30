use std::sync::Arc;

use axum::{routing::get, Router};

use crate::database::Database;

pub fn create_router(_database: Arc<Database>) -> Router {
    Router::new().route(
        "/",
        get(|| async {
            "Welcome to recipes. The front end is not yet available."
        }),
    )
}
