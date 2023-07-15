mod ingredients;
mod recipes;

use crate::database::Database;

use axum::Router;
use std::sync::Arc;

pub fn create_router(database: Arc<Database>) -> Router {
    Router::new()
        .nest("/ingredients", ingredients::create_router(database.clone()))
        .nest("/recipes", recipes::create_router(database))
}
