use axum::{
    extract::Path,
    routing::get,
    Router,
};

async fn list_recipes() -> &'static str {
    "List recipes not yet implemented"
}

async fn get_recipe(Path(recipe_id): Path<u64>) -> String {
    format!("Get recipe (id = {}) not yet implemented", recipe_id)
}

pub fn create_router() -> Router {
    Router::new()
        .route("/recipes", get(list_recipes))
        .route("/recipes/:recipe_id", get(get_recipe))
}
