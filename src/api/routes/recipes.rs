use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use log::debug;
use serde::Deserialize;

use crate::api::constants::DEFAULT_PAGE_SIZE;
use crate::api::utils::Error;
use crate::database::Database;
use crate::models::{Category, Model, Recipe, Ref};

mod versions;

fn default_filter_limit() -> u64 {
    DEFAULT_PAGE_SIZE
}

/// A filter to determine which recipes to list.
///
/// The `text` filters results to only recipes containing certain keywords.
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

/// Lists all recipes in the database matching `filter`.
async fn list_recipes(Query(filter): Query<RecipeFilter>) -> Json<Vec<Recipe>> {
    debug!(
        "Listing recipes: {} (not yet implemented)",
        filter.summary()
    );

    Json::<Vec<Recipe>>(vec![])
}

/// Retrieves the recipe with ID `recipe_id`.
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
            .map_err(Error::from_db)?,
    ))
}

#[derive(Deserialize)]
struct CreateRecipeData {
    name: String,
    categories: Vec<Category>,
}

/// Creates a new recipe. Returns the recipe JSON, including the new recipe's
/// ID.
async fn create_recipe(
    State(database): State<Arc<Database>>,
    Json(data): Json<CreateRecipeData>,
) -> Result<Json<Recipe>, Error> {
    debug!("Creating recipe with name {}", data.name);

    let name = data.name.clone();
    let categories = data
        .categories
        .iter()
        .map(|category| Ref::new(category.id))
        .collect::<Vec<_>>();
    let id = database
        .with_transaction(move |transaction| {
            Box::pin(async move {
                Recipe::store_new(transaction, &data.name, data.categories)
                    .await
            })
        })
        .await
        .map_err(Error::from_db)?;

    Ok(Json(Recipe {
        id,
        name,
        versions: HashMap::new(),
        categories,
    }))
}

/// Creates a router that handles routes for getting and creating recipes and
/// their versions.
pub fn create_router<S>(database: Arc<Database>) -> Router<S> {
    Router::new()
        .route("/", get(list_recipes))
        .route("/", post(create_recipe))
        .route("/:recipe_id", get(get_recipe))
        .nest(
            "/:recipe_id/versions",
            versions::create_router(database.clone()),
        )
        .with_state(database)
}
