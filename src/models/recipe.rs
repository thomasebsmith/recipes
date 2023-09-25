use std::collections::HashMap;

use serde::Serialize;
use sqlx::{Any, Transaction};

use super::{Category, Model, RecipeVersion, RecipeVersionID, Ref};
use crate::database::DBResult;

/// Represents a recipe for making something edible.
///
/// The recipe may have multiple revisions.
#[derive(Serialize)]
pub struct Recipe {
    /// The recipe's internal ID.
    pub id: i64,

    /// The human-readable name of the recipe.
    pub name: String,

    /// A map from (revision number) to (recipe revision).
    pub versions: HashMap<i64, Ref<RecipeVersion>>,

    /// A list of all the categories that this recipe is a part of.
    pub categories: Vec<Ref<Category>>,
}

impl Recipe {
    // TODO: Validate arguments
    pub async fn store_new(
        transaction: &mut Transaction<'_, Any>,
        name: &str,
        categories: Vec<Category>,
    ) -> DBResult<i64> {
        let last_recipe_id: i64 =
            sqlx::query_scalar("SELECT MAX(id) FROM recipes")
                .fetch_one(&mut *transaction)
                .await?;

        let id = last_recipe_id + 1;

        sqlx::query(
            "INSERT INTO recipes (id, name, hidden) \
             VALUES ($1, $2, FALSE)",
        )
        .bind(id)
        .bind(name)
        .execute(&mut *transaction)
        .await?;

        for category in categories.into_iter() {
            sqlx::query(
                "INSERT INTO recipes_categories (recipe_id, category_id) \
                 VALUES ($1, $2)",
            )
            .bind(id)
            .bind(category.id)
            .execute(&mut *transaction)
            .await?;
        }

        Ok(id)
    }
}

impl Model for Recipe {
    type ID = i64;

    async fn get(
        transaction: &mut Transaction<'_, Any>,
        id: Self::ID,
    ) -> DBResult<Self> {
        // Get the name of the recipe. If the recipe is hidden, fetch_one will
        // fail and so this method will fail.
        let name: String = sqlx::query_scalar(
            "SELECT name FROM recipes WHERE id = $1 AND NOT hidden",
        )
        .bind(id)
        .fetch_one(&mut *transaction)
        .await?;

        // Get all version IDs associated with this recipe.
        let version_ids: Vec<i64> = sqlx::query_scalar(
            "SELECT version_id FROM recipes_versions WHERE recipe_id = $1",
        )
        .bind(id)
        .fetch_all(&mut *transaction)
        .await?;

        // For now, just create refs to the RecipeVersions.
        let versions = version_ids
            .into_iter()
            .map(|version_id| {
                (
                    version_id,
                    Ref::new(RecipeVersionID {
                        recipe_id: id,
                        version_id,
                    }),
                )
            })
            .collect::<HashMap<i64, Ref<RecipeVersion>>>();

        // Get all category IDs and create refs for them, too.
        let category_ids: Vec<i64> = sqlx::query_scalar(
            "SELECT category_id FROM recipes_categories WHERE recipe_id = $1",
        )
        .bind(id)
        .fetch_all(transaction)
        .await?;

        let categories =
            category_ids.into_iter().map(Ref::new).collect::<Vec<_>>();

        Ok(Self {
            id,
            name,
            versions,
            categories,
        })
    }

    async fn fill_refs(
        &mut self,
        transaction: &mut Transaction<'_, Any>,
    ) -> DBResult<()> {
        for version in self.versions.values_mut() {
            version.fill(transaction).await?;
        }
        for category in &mut self.categories {
            category.fill(transaction).await?;
        }
        Ok(())
    }
}
