use super::{
    Amount, Ingredient, MeasurementType, QuantifiedIngredient, Recipe,
    RecipeVersion,
};

use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
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

    Json(vec![Recipe {
        id: 0,
        name,
        versions: HashMap::from([(
            0,
            RecipeVersion {
                id: 0,
                ingredients: vec![],
                instructions: vec![],
            },
        )]),
    }])
}

async fn get_recipe(Path(recipe_id): Path<u64>) -> Json<Recipe> {
    Json(Recipe {
        id: recipe_id,
        name: format!("Not yet implemented: {}", recipe_id),
        versions: HashMap::from([(
            0,
            RecipeVersion {
                id: 0,
                ingredients: vec![
                    QuantifiedIngredient {
                        ingredient: Ingredient {
                            id: 0,
                            name: "First ingredient".to_string(),
                            energy_density: 3.15,
                        },
                        amount: Amount {
                            quantity: 2.719,
                            measurement: MeasurementType::Mass,
                        },
                    },
                    QuantifiedIngredient {
                        ingredient: Ingredient {
                            id: 1,
                            name: "Second ingredient".to_string(),
                            energy_density: 29.9,
                        },
                        amount: Amount {
                            quantity: 7.92,
                            measurement: MeasurementType::Volume,
                        },
                    },
                    QuantifiedIngredient {
                        ingredient: Ingredient {
                            id: 2,
                            name: "Third ingredient".to_string(),
                            energy_density: 458.0,
                        },
                        amount: Amount {
                            quantity: 2.0,
                            measurement: MeasurementType::Count,
                        },
                    },
                ],
                instructions: vec![format!(
                    "First instruction for {}.",
                    recipe_id
                )],
            },
        )]),
    })
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(list_recipes))
        .route("/:recipe_id", get(get_recipe))
}
