use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use log::debug;

use crate::api::constants::LISTING_LIMIT;
use crate::api::utils::Error;
use crate::database::{self, Database};
use crate::models::{Model, RecipeVersion, RecipeVersionID};

/// Lists all versions of the recipe with the id `recipe_id`, using `database`
/// to retrieve the recipes.
///
/// Returns an error if `recipe_id` does not refer to a visible recipe.
async fn list_versions(
    State(database): State<Arc<Database>>,
    Path(recipe_id): Path<i64>,
) -> Result<Json<Vec<RecipeVersion>>, Error> {
    debug!("Listing all versions of recipe {recipe_id}");

    Ok(Json(
        database
            .with_transaction(move |transaction| {
                Box::pin(async move {
                    let matching_recipe_count: i64 = sqlx::query_scalar(
                        "SELECT COUNT(id) FROM recipes \
                         WHERE id = $1 AND NOT hidden",
                    )
                    .bind(recipe_id)
                    .fetch_one(&mut *transaction)
                    .await?;

                    if matching_recipe_count != 1 {
                        return Err(database::Error::BadArguments(
                            "Invalid recipe".to_owned(),
                        ));
                    }

                    // TODO: Make this more efficient by merging queries
                    let version_ids: Vec<i64> = sqlx::query_scalar(
                        "SELECT version_id FROM recipes_versions \
                         WHERE recipe_id = $1 ORDER BY version_id LIMIT $2",
                    )
                    .bind(recipe_id)
                    .bind(LISTING_LIMIT)
                    .fetch_all(&mut *transaction)
                    .await?;

                    let mut versions: Vec<RecipeVersion> = vec![];

                    for version_id in version_ids {
                        let version = RecipeVersion::get_filled(
                            &mut *transaction,
                            RecipeVersionID {
                                recipe_id,
                                version_id,
                            },
                        )
                        .await?;
                        versions.push(version);
                    }

                    Ok(versions)
                })
            })
            .await
            .map_err(Error::from_db)?,
    ))
}

/// Gets the version with ID `version_id` of the recipe with ID `recipe_id`.
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

/// Creates a router that serves version-specific routes.
///
/// This router must be nested under a path that provides `:recipe_id`.
pub fn create_router<S>(database: Arc<Database>) -> Router<S> {
    Router::new()
        .route("/", get(list_versions))
        .route("/:version_id", get(get_version))
        .with_state(database)
}
