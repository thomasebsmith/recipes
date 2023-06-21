use serde::Serialize;

type RealQuantity = f64;

#[derive(Serialize)]
pub struct Ingredient {
    pub id: u64,
    pub name: String,
    pub energy_density: RealQuantity, // In J/kg.
}

#[derive(Serialize)]
pub enum MeasurementType {
    Mass = 0,
    Volume = 1,
}

#[derive(Serialize)]
pub struct Amount {
    pub quantity: RealQuantity, // In SI standard units.
    pub measurement: MeasurementType,
}

#[derive(Serialize)]
pub struct QuantifiedIngredient {
    pub ingredient: Ingredient,
    pub amount: Amount,
}
