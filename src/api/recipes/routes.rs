use super::Recipe;

use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use serde::Deserialize;

fn default_filter_limit() -> u64 {
    100
}

#[derive(Deserialize)]
struct RecipeFilter {
    text: Option<String>,

    #[serde(default = "default_filter_limit")]
    limit: u64,
}

async fn list_recipes(Query(filter): Query<RecipeFilter>) -> Json<Vec<Recipe>> {
    let name = if let Some(text) = filter.text {
        format!("Recipes matching \"{}\" with limit {}", text, filter.limit)
    } else {
        format!("All recipes with limit {}", filter.limit)
    };

    Json(vec![Recipe {
        id: 0,
        name: name,
        ingredients: vec![],
        instructions: vec![],
    }])
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
        .route("/", get(list_recipes))
        .route("/:recipe_id", get(get_recipe))
}
