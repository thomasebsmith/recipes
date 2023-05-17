use super::Ingredient;

use serde::Serialize;

#[derive(Serialize)]
pub struct Recipe {
    pub id: u64,
    pub name: String,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<String>,
}
