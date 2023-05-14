use axum::{
    routing::get,
    Router,
};

async fn list_recipes() -> &'static str {
    "List recipes not yet implemented"
}

async fn get_recipe() -> &'static str {
    "Get recipe not yet implemented"
}

pub fn create_router() -> Router {
    Router::new()
        .route("/recipes", get(list_recipes))
        .route("/recipes/:recipe_id", get(get_recipe))
}
