mod recipes;

use axum::Router;

pub fn create_router() -> Router {
    Router::new()
        .nest("/recipes", recipes::create_router())
}
