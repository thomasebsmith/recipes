use chrono::{offset::Utc, DateTime, Duration, NaiveDateTime};
use log::warn;
use serde::{Serialize, Serializer};
use sqlx::{Any, Transaction};

use super::{Ingredient, Model, Ref};
use crate::database::{self, to_internal_db_error, DBResult};

/// The kind of quantity of a recipe ingredient measurement.
#[derive(Clone, Copy, Serialize)]
#[repr(i64)]
pub enum MeasurementType {
    Mass = 0,
    Volume = 1,
    Count = 2,
}

impl TryFrom<i64> for MeasurementType {
    type Error = ();

    /// Attempts to convert from a database value (integer) to a
    /// `MeasurementType`.
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Mass),
            1 => Ok(Self::Volume),
            2 => Ok(Self::Count),
            _ => Err(()),
        }
    }
}

/// A recipe ingredient paired with its quantity.
#[derive(Serialize)]
pub struct QuantifiedIngredient {
    pub ingredient: Ref<Ingredient>,
    pub quantity: f64, // In SI standard units.
    pub measurement: MeasurementType,
}

/// A step that should be performed as part of a recipe.
#[derive(Serialize)]
pub struct Instruction {
    pub text: String,
}

/// A specific version of a recipe, with certain ingredients and instructions.
#[derive(Serialize)]
pub struct RecipeVersion {
    pub id: i64,
    pub created: DateTime<Utc>,
    pub ingredients: Vec<QuantifiedIngredient>,
    pub instructions: Vec<Instruction>,

    #[serde(serialize_with = "duration_to_seconds")]
    pub duration: Duration,
}

#[derive(Clone, Copy, Serialize)]
pub struct RecipeVersionID {
    pub recipe_id: i64,
    pub version_id: i64,
}

async fn ensure_recipe_visible(
    transaction: &mut Transaction<'_, Any>,
    recipe_id: i64,
) -> DBResult<()> {
    let matching_recipe_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(id) FROM recipes \
         WHERE id = $1 AND NOT hidden",
    )
    .bind(recipe_id)
    .fetch_one(&mut **transaction)
    .await?;

    if matching_recipe_count == 1 {
        Ok(())
    } else {
        Err(database::Error::BadArguments("Invalid recipe".to_owned()))
    }
}

impl RecipeVersion {
    #[allow(dead_code)]
    pub async fn store_new(
        transaction: &mut Transaction<'_, Any>,
        recipe_id: i64,
        created: DateTime<Utc>,
        ingredients: Vec<QuantifiedIngredient>,
        instructions: Vec<Instruction>,
        duration: Duration,
    ) -> DBResult<RecipeVersionID> {
        ensure_recipe_visible(transaction, recipe_id).await?;

        let last_version_id: Option<i64> = sqlx::query_scalar(
            "SELECT MAX(version_id) FROM recipes_versions \
            WHERE recipe_id = $1",
        )
        .bind(recipe_id)
        .fetch_optional(&mut **transaction)
        .await?;

        let version_id = last_version_id.map_or(0, |old_id| old_id + 1);

        // Store the overall version information.
        sqlx::query(
            "INSERT INTO recipes_versions \
            (recipe_id, version_id, created, duration) \
            VALUES ($1, $2, $3, $4)",
        )
        .bind(recipe_id)
        .bind(version_id)
        .bind(created.timestamp())
        .bind(duration.num_seconds())
        .execute(&mut **transaction)
        .await?;

        // Store the ingredient list (with measurements).
        for (list_order, ingredient) in ingredients.into_iter().enumerate() {
            sqlx::query(
                "INSERT INTO recipes_ingredients \
                (recipe_id, version_id, ingredient_id, \
                list_order, quantity, measurement) \
                VALUES ($1, $2, $3, $4, $5, $6)",
            )
            .bind(recipe_id)
            .bind(version_id)
            .bind(ingredient.ingredient.id)
            .bind(try_into::<i64, _>(list_order)?)
            .bind(ingredient.quantity)
            .bind(ingredient.measurement as i64)
            .execute(&mut **transaction)
            .await?;
        }

        // Store the instructions.
        for (step_number, instruction) in instructions.into_iter().enumerate() {
            sqlx::query(
                "INSERT INTO recipes_instructions \
                (recipe_id, version_id, step_number, step_text) \
                VALUES ($1, $2, $3, $4)",
            )
            .bind(recipe_id)
            .bind(version_id)
            .bind(try_into::<i64, _>(step_number)?)
            .bind(instruction.text)
            .execute(&mut **transaction)
            .await?;
        }

        Ok(RecipeVersionID {
            recipe_id,
            version_id,
        })
    }
}

impl Model for RecipeVersion {
    type ID = RecipeVersionID;

    async fn get(
        transaction: &mut Transaction<'_, Any>,
        id: Self::ID,
    ) -> DBResult<Self> {
        ensure_recipe_visible(transaction, id.recipe_id).await?;

        // Retrieve everything needed from the recipes_versions table.
        let (created_secs_since_epoch, duration_secs): (i64, i64) =
            sqlx::query_as(
                "SELECT created, duration FROM recipes_versions \
                WHERE recipe_id = $1 AND version_id = $2",
            )
            .bind(id.recipe_id)
            .bind(id.version_id)
            .fetch_one(&mut **transaction)
            .await?;

        // Retrieve everything needed from the recipes_ingredients table.
        let ingredients_raw: Vec<(i64, f64, i64)> = sqlx::query_as(
            "SELECT ingredient_id, quantity, measurement FROM \
              recipes_ingredients \
                WHERE recipe_id = $1 AND version_id = $2 \
                ORDER BY list_order",
        )
        .bind(id.recipe_id)
        .bind(id.version_id)
        .fetch_all(&mut **transaction)
        .await?;

        // Retrieve everything needed from the recipes_instructions table.
        let instructions_raw: Vec<String> = sqlx::query_scalar(
            "SELECT step_text FROM recipes_instructions \
                WHERE recipe_id = $1 AND version_id = $2 \
                ORDER BY step_number",
        )
        .bind(id.recipe_id)
        .bind(id.version_id)
        .fetch_all(&mut **transaction)
        .await?;

        // Parse and validate what was retrieved from the tables.
        let Some(created_naive) =
            NaiveDateTime::from_timestamp_opt(created_secs_since_epoch, 0)
        else {
            return Err(database::Error::Internal(
                "Internal error: timestamp out-of-range".into(),
            ));
        };
        let created = created_naive.and_utc();

        let duration = Duration::seconds(duration_secs);

        let ingredients = ingredients_raw
            .into_iter()
            .map(|(ingredient_id, quantity, measurement)| {
                QuantifiedIngredient {
                    ingredient: Ref::new(ingredient_id),
                    quantity,
                    measurement: measurement.try_into().unwrap_or_else(|()| {
                        warn!(
                            "Invalid measurement {} while retrieving \
                        recipe={} version={}, ingredient={}",
                            measurement,
                            id.recipe_id,
                            id.version_id,
                            ingredient_id,
                        );
                        MeasurementType::Count
                    }),
                }
            })
            .collect::<Vec<_>>();
        let instructions = instructions_raw
            .into_iter()
            .map(|text| Instruction { text })
            .collect::<Vec<_>>();

        Ok(Self {
            id: id.version_id,
            created,
            ingredients,
            instructions,
            duration,
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

fn duration_to_seconds<S: Serializer>(
    value: &Duration,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_i64(value.num_seconds())
}

fn try_into<T, F: TryInto<T>>(value: F) -> DBResult<T>
where
    F::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    TryInto::<T>::try_into(value).map_err(to_internal_db_error)
}
