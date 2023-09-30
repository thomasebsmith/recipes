use serde::{Deserialize, Serialize};
use sqlx::{Any, Transaction};

use super::Model;
use crate::database::DBResult;

/// Represents a category of recipes.
#[derive(Clone, Deserialize, Serialize)]
pub struct Category {
    /// The category's internal ID.
    pub id: i64,

    /// The human-readable name of the category. This should be unique.
    pub name: String,
}

impl Category {
    pub async fn store_new(
        transaction: &mut Transaction<'_, Any>,
        name: &str,
    ) -> DBResult<i64> {
        let last_category_id: Option<i64> =
            sqlx::query_scalar("SELECT MAX(id) FROM categories")
                .fetch_optional(&mut *transaction)
                .await?;

        let id = last_category_id.map_or(0, |old_id| old_id + 1);

        sqlx::query(
            "INSERT INTO categories (id, name)
             VALUES ($1, $2)",
        )
        .bind(id)
        .bind(name)
        .execute(transaction)
        .await?;

        Ok(id)
    }
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
