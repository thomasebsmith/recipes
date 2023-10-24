use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use log::debug;

use crate::api::utils::Error;
use crate::database::Database;
use crate::models::{Model, RecipeVersion, RecipeVersionID};

async fn get_version(
    State(database): State<Arc<Database>>,
    Path(recipe_id): Path<i64>,
    Path(version_id): Path<i64>,
) -> Result<Json<RecipeVersion>, Error> {
    debug!("Getting recipe {recipe_id} version {version_id}");

    let id = RecipeVersionID {
        recipe_id,
        version_id,
    };

    Ok(Json(
        database
            .with_transaction(move |transaction| {
                Box::pin(async move {
                    RecipeVersion::get_filled(transaction, id).await
                })
            })
            .await
            .map_err(Error::from_db)?,
    ))
}

pub fn create_router() -> Router<Arc<Database>> {
    Router::new().route("/:version_id", get(get_version))
}
