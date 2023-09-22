use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use log::debug;

use crate::api::utils::Error;
use crate::database::Database;
use crate::models::{Category, Model};

async fn list_categories(
    State(database): State<Arc<Database>>,
) -> Result<Json<Vec<Category>>, Error> {
    debug!("Listing all categories");

    Ok(Json(
        database
            .with_transaction(move |transaction| {
                Box::pin(async move {
                    // TODO: Move this elsewhere.
                    const LISTING_LIMIT: i64 = 1024;

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
            .map_err(Error::from_sqlx)?,
    ))
}

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
            .map_err(Error::from_sqlx)?,
    ))
}

pub fn create_router(database: Arc<Database>) -> Router {
    Router::new()
        .route("/", get(list_categories))
        .route("/:category_id", get(get_category))
        .with_state(database)
}
