mod recipes;

use crate::database::Database;

use axum::Router;
use std::sync::Arc;

pub fn create_router(database: Arc<Database>) -> Router {
    Router::new().nest("/recipes", recipes::create_router(database))
}
