use crate::database::Database;
use crate::models::{Model, Recipe};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use log::{debug, error};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

fn default_filter_limit() -> u64 {
    100
}

struct Error {
    status_code: StatusCode,
    message: String,
}

impl Error {
    fn from_sqlx(error: sqlx::Error) -> Self {
        let (status_code, message) = match error {
            sqlx::Error::RowNotFound => {
                (StatusCode::NOT_FOUND, "Resource not found")
            }
            _ => {
                error!("Internal error during query: {}", error);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal database error")
            }
        };
        Self {
            status_code,
            message: message.to_owned(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(HashMap::from([("error_message", self.message)])),
        )
            .into_response()
    }
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

async fn get_recipe(
    State(database): State<Arc<Database>>,
    Path(recipe_id): Path<i64>,
) -> Result<Json<Recipe>, Error> {
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
