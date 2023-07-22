use crate::api::utils::Error;
use crate::database::Database;
use crate::models::{Model, Recipe};

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use log::debug;
use serde::Deserialize;
use std::sync::Arc;

fn default_filter_limit() -> u64 {
    100
}

#[derive(Deserialize)]
struct RecipeFilter {
    text: Option<String>,

    #[serde(default = "default_filter_limit")]
    limit: u64,
}

impl RecipeFilter {
    fn summary(&self) -> String {
        if let Some(ref text) = self.text {
            format!("Recipes matching \"{}\" with limit {}", text, self.limit)
        } else {
            format!("All recipes with limit {}", self.limit)
        }
    }
}

async fn list_recipes(Query(filter): Query<RecipeFilter>) -> Json<Vec<Recipe>> {
    debug!(
        "Listing recipes: {} (not yet implemented)",
        filter.summary()
    );

    Json::<Vec<Recipe>>(vec![])
}

async fn get_recipe(
    State(database): State<Arc<Database>>,
    Path(recipe_id): Path<i64>,
) -> Result<Json<Recipe>, Error> {
    debug!("Getting recipe {recipe_id}");

    Ok(Json(
        database
            .with_transaction(move |transaction| {
                Box::pin(async move {
                    Recipe::get_filled(transaction, recipe_id).await
                })
            })
            .await
            .map_err(Error::from_sqlx)?,
    ))
}

pub fn create_router(database: Arc<Database>) -> Router {
    Router::new()
        .route("/", get(list_recipes))
        .route("/:recipe_id", get(get_recipe))
        .with_state(database)
}
