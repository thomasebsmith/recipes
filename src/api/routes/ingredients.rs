use crate::api::utils::Error;
use crate::database::Database;
use crate::models::{Ingredient, Model};

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use log::debug;
use std::sync::Arc;

async fn list_ingredients(
    State(database): State<Arc<Database>>,
) -> Result<Json<Vec<Ingredient>>, Error> {
    debug!("Listing all ingredients");

    Ok(Json(
        database
            .with_transaction(move |transaction| {
                Box::pin(async move {
                    const LISTING_LIMIT: i64 = 1024;

                    let result: Vec<(i64, String, f64)> = sqlx::query_as(
                    "SELECT id, name, energy_density FROM ingredients LIMIT $1",
                )
                .bind(LISTING_LIMIT)
                .fetch_all(transaction)
                .await?;

                    Ok(result
                        .into_iter()
                        .map(|(id, name, energy_density)| Ingredient {
                            id,
                            name,
                            energy_density,
                        })
                        .collect())
                })
            })
            .await
            .map_err(Error::from_sqlx)?,
    ))
}

async fn get_ingredient(
    State(database): State<Arc<Database>>,
    Path(ingredient_id): Path<i64>,
) -> Result<Json<Ingredient>, Error> {
    debug!("Getting ingredient {}", ingredient_id);

    Ok(Json(
        database
            .with_transaction(move |transaction| {
                Box::pin(async move {
                    Ingredient::get_filled(transaction, ingredient_id).await
                })
            })
            .await
            .map_err(Error::from_sqlx)?,
    ))
}

pub fn create_router(database: Arc<Database>) -> Router {
    Router::new()
        .route("/", get(list_ingredients))
        .route("/:ingredient_id", get(get_ingredient))
        .with_state(database)
}
