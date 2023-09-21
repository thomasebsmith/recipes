mod categories;
mod ingredients;
mod recipes;

use std::sync::Arc;

use axum::Router;

use crate::database::Database;

pub fn create_router(database: Arc<Database>) -> Router {
    Router::new()
        .nest("/categories", categories::create_router(database.clone()))
        .nest("/ingredients", ingredients::create_router(database.clone()))
        .nest("/recipes", recipes::create_router(database))
}
