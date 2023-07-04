use super::{Ingredient, Model, Ref};
use crate::database::DBResult;
use log::warn;
use serde::Serialize;
use sqlx::{Any, Transaction};
use std::convert::TryFrom;

#[derive(Clone, Copy, Serialize)]
#[repr(i64)]
pub enum MeasurementType {
    Mass = 0,
    Volume = 1,
    Count = 2,
}

impl TryFrom<i64> for MeasurementType {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Mass),
            1 => Ok(Self::Volume),
            2 => Ok(Self::Count),
            _ => Err(()),
        }
    }
}

#[derive(Serialize)]
pub struct QuantifiedIngredient {
    pub ingredient: Ref<Ingredient>,
    pub quantity: f64, // In SI standard units.
    pub measurement: MeasurementType,
}

#[derive(Serialize)]
pub struct Instruction {
    pub text: String,
}

#[derive(Serialize)]
pub struct RecipeVersion {
    pub id: i64,
    pub ingredients: Vec<QuantifiedIngredient>,
    pub instructions: Vec<Instruction>,
}

#[derive(Clone, Copy, Serialize)]
pub struct RecipeVersionID {
    pub recipe_id: i64,
    pub version_id: i64,
}

impl Model for RecipeVersion {
    type ID = RecipeVersionID;

    async fn get(
        transaction: &mut Transaction<'_, Any>,
        id: Self::ID,
    ) -> DBResult<Self> {
        let ingredients_raw: Vec<(i64, f64, i64)> = sqlx::query_as(
            "SELECT ingredient_id, quantity, measurement FROM recipes_ingredients WHERE recipe_id = $1 AND version_id = $2 ORDER BY list_order"
        )
            .bind(id.recipe_id)
            .bind(id.version_id)
            .fetch_all(&mut *transaction)
            .await?;

        let instructions_raw: Vec<String> = sqlx::query_scalar(
            "SELECT step_text FROM recipes_instructions WHERE recipe_id = $1 AND version_id = $2 ORDER BY step_number"
        ).bind(id.recipe_id).bind(id.version_id).fetch_all(transaction).await?;

        let ingredients = ingredients_raw.into_iter().map(|(ingredient_id, quantity, measurement)| {
            QuantifiedIngredient {
                ingredient: Ref::new(ingredient_id),
                quantity,
                measurement: measurement.try_into().unwrap_or_else(|_| {
                    warn!(
                        "Invalid measurement {} while retrieving recipe={} version={}, ingredient={}",
                        measurement,
                        id.recipe_id,
                        id.version_id,
                        ingredient_id,
                    );
                    MeasurementType::Count
                }),
            }
        }).collect::<Vec<_>>();
        let instructions = instructions_raw
            .into_iter()
            .map(|text| Instruction { text })
            .collect::<Vec<_>>();

        Ok(Self {
            id: id.version_id,
            ingredients,
            instructions,
        })
    }

    async fn fill_refs(
        &mut self,
        transaction: &mut Transaction<'_, Any>,
    ) -> DBResult<()> {
        for quantified_ingredient in &mut self.ingredients {
            quantified_ingredient.ingredient.fill(transaction).await?;
        }
        Ok(())
    }
}
