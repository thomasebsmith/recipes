use super::QuantifiedIngredient;

use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct RecipeVersion {
    pub id: u64,
    pub ingredients: Vec<QuantifiedIngredient>,
    pub instructions: Vec<String>,
}

#[derive(Serialize)]
pub struct Recipe {
    pub id: u64,
    pub name: String,
    pub versions: HashMap<u64, RecipeVersion>,
}
