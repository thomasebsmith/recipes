use serde::Serialize;

#[derive(Serialize)]
pub struct Ingredient {
    pub id: u64,
    pub name: String,
    pub energy_density: f64, // In J/kg. TODO: use a better type
}

#[derive(Serialize)]
pub struct Amount {
    pub quantity: f64, // In SI standard units. TODO: use a better type.
    pub measurement: String,
}


#[derive(Serialize)]
pub struct QuantifiedIngredient {
    pub ingredient: Ingredient,
    pub amount: Amount,
}
