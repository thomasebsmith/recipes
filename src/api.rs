mod recipes;

use recipes::Recipe;

use axum::{
    extract::Path,
    routing::get,
    Json,
    Router,
};

async fn list_recipes() -> Json<Vec<Recipe>> {
    Json(vec![])
}

async fn get_recipe(Path(recipe_id): Path<u64>) -> Json<Recipe> {
    Json(Recipe {
        id: recipe_id,
        name: format!("Not yet implemented: {}", recipe_id),
        ingredients: vec![],
        instructions: vec![format!("First instruction for {}.", recipe_id)],
    })
}

pub fn create_router() -> Router {
    Router::new()
        .route("/recipes", get(list_recipes))
        .route("/recipes/:recipe_id", get(get_recipe))
}
