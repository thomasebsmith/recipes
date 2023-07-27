use serde::Serialize;
use sqlx::{Any, Transaction};

use super::Model;
use crate::database::DBResult;

#[derive(Serialize)]
pub struct Ingredient {
    pub id: i64,
    pub name: String,
    pub energy_density: f64, // In J/kg.
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
