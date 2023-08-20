use serde::Serialize;
use sqlx::{Any, Transaction};

use super::Model;
use crate::database::DBResult;

/// Represents a category of recipes.
#[derive(Serialize)]
pub struct Category {
    /// The category's internal ID.
    pub id: i64,

    /// The human-readable name of the category. This should be unique.
    pub name: String,
}

impl Model for Category {
    type ID = i64;

    async fn get(
        transaction: &mut Transaction<'_, Any>,
        id: Self::ID,
    ) -> DBResult<Self> {
        let name: String =
            sqlx::query_scalar("SELECT name FROM categories WHERE id = $1")
                .bind(id)
                .fetch_one(transaction)
                .await?;
        Ok(Self { id, name })
    }

    async fn fill_refs(
        &mut self,
        _transaction: &mut Transaction<'_, Any>,
    ) -> DBResult<()> {
        Ok(())
    }
}
