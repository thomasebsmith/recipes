use serde::Serialize;
use sqlx::{Any, Transaction};

use super::Model;
use crate::database::DBResult;

/// Represents a general ingredient that can be used in recipes. This can be
/// any edible recipe ingredient, from water to a spice to a baked good.
#[derive(Serialize)]
pub struct Ingredient {
    /// The ingredient's internal ID.
    pub id: i64,

    /// The human-readable name of the ingredient. This should be unique.
    pub name: String,

    /// The typical energy density of the ingredient (in J/kg).
    pub energy_density: f64,
}

impl Model for Ingredient {
    type ID = i64;

    async fn get(
        transaction: &mut Transaction<'_, Any>,
        id: Self::ID,
    ) -> DBResult<Self> {
        let result: (String, f64) = sqlx::query_as(
            "SELECT name, energy_density FROM ingredients WHERE id = $1",
        )
        .bind(id)
        .fetch_one(transaction)
        .await?;
        Ok(Self {
            id,
            name: result.0,
            energy_density: result.1,
        })
    }

    async fn fill_refs(
        &mut self,
        _transaction: &mut Transaction<'_, Any>,
    ) -> DBResult<()> {
        Ok(())
    }
}
