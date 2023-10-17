use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use log::debug;
use serde::Deserialize;

use crate::api::constants::LISTING_LIMIT;
use crate::api::utils::Error;
use crate::database::Database;
use crate::models::{Category, Model};

/// Lists all the categories that a recipe can be contained in.
async fn list_categories(
    State(database): State<Arc<Database>>,
) -> Result<Json<Vec<Category>>, Error> {
    debug!("Listing all categories");

    Ok(Json(
        database
            .with_transaction(move |transaction| {
                Box::pin(async move {
                    let results: Vec<(i64, String)> = sqlx::query_as(
                        "SELECT id, name FROM categories ORDER BY id LIMIT $1",
                    )
                    .bind(LISTING_LIMIT)
                    .fetch_all(transaction)
                    .await?;

                    Ok(results
                        .into_iter()
                        .map(|(id, name)| Category { id, name })
                        .collect())
                })
            })
            .await
            .map_err(Error::from_db)?,
    ))
}

/// Retrieves a category by ID.
async fn get_category(
    State(database): State<Arc<Database>>,
    Path(category_id): Path<i64>,
) -> Result<Json<Category>, Error> {
    debug!("Getting category {category_id}");

    Ok(Json(
        database
            .with_transaction(move |transaction| {
                Box::pin(async move {
                    Category::get_filled(transaction, category_id).await
                })
            })
            .await
            .map_err(Error::from_db)?,
    ))
}

/// Represents the data needed to create a new category.
#[derive(Deserialize)]
struct CreateCategoryData {
    name: String,
}

/// Creates a category and returns the new category's JSON, including its ID.
async fn create_category(
    State(database): State<Arc<Database>>,
    Json(data): Json<CreateCategoryData>,
) -> Result<Json<Category>, Error> {
    debug!("Creating category with name {}", data.name);

    let name = data.name.clone();
    let id = database
        .with_transaction(move |transaction| {
            Box::pin(async move {
                Category::store_new(transaction, &data.name).await
            })
        })
        .await
        .map_err(Error::from_db)?;

    Ok(Json(Category { id, name }))
}

/// Creates a router that serves all category routes.
pub fn create_router(database: Arc<Database>) -> Router {
    Router::new()
        .route("/", get(list_categories))
        .route("/", post(create_category))
        .route("/:category_id", get(get_category))
        .with_state(database)
}
