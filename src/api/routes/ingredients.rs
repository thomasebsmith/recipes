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
use crate::models::{Ingredient, Model};

/// Lists all the ingredients that are in the database.
async fn list_ingredients(
    State(database): State<Arc<Database>>,
) -> Result<Json<Vec<Ingredient>>, Error> {
    debug!("Listing all ingredients");

    Ok(Json(
        database
            .with_transaction(move |transaction| {
                Box::pin(async move {
                    let result: Vec<(i64, String, f64)> = sqlx::query_as(
                        "SELECT id, name, energy_density \
                         FROM ingredients ORDER BY id LIMIT $1",
                    )
                    .bind(LISTING_LIMIT)
                    .fetch_all(&mut **transaction)
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
            .map_err(Error::from_db)?,
    ))
}

/// Retrieves an ingredient by ID.
async fn get_ingredient(
    State(database): State<Arc<Database>>,
    Path(ingredient_id): Path<i64>,
) -> Result<Json<Ingredient>, Error> {
    debug!("Getting ingredient {ingredient_id}");

    Ok(Json(
        database
            .with_transaction(move |transaction| {
                Box::pin(async move {
                    Ingredient::get_filled(transaction, ingredient_id).await
                })
            })
            .await
            .map_err(Error::from_db)?,
    ))
}

/// Represents the data needed to create a new ingredient.
#[derive(Deserialize)]
struct CreateIngredientData {
    name: String,
    energy_density: f64,
}

/// Creates an ingredient and returns the ingredient's JSON, including its ID.
async fn create_ingredient(
    State(database): State<Arc<Database>>,
    Json(data): Json<CreateIngredientData>,
) -> Result<Json<Ingredient>, Error> {
    debug!("Creating ingredient with name {}", data.name);

    let name = data.name.clone();
    let id = database
        .with_transaction(move |transaction| {
            Box::pin(async move {
                Ingredient::store_new(
                    transaction,
                    &data.name,
                    data.energy_density,
                )
                .await
            })
        })
        .await
        .map_err(Error::from_db)?;

    Ok(Json(Ingredient {
        id,
        name,
        energy_density: data.energy_density,
    }))
}

/// Creates a router that serves all ingredient routes.
pub fn create_router<S>(database: Arc<Database>) -> Router<S> {
    Router::new()
        .route("/", get(list_ingredients))
        .route("/", post(create_ingredient))
        .route("/:ingredient_id", get(get_ingredient))
        .with_state(database)
}
