use crate::models::Recipe;

use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use log::debug;
use serde::Deserialize;
use std::collections::HashMap;

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
    debug!("Listing recipes: {}", name);

    Json::<Vec<Recipe>>(vec![])
}

async fn get_recipe(Path(recipe_id): Path<i64>) -> Json<Recipe> {
    Json(Recipe {
        id: recipe_id,
        name: format!("Not yet implemented: {}", recipe_id),
        versions: HashMap::from([]),
        categories: vec![],
    })
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(list_recipes))
        .route("/:recipe_id", get(get_recipe))
}
