use super::{Category, Model, RecipeVersion, RecipeVersionID, Ref};
use crate::database::DBResult;
use serde::Serialize;
use sqlx::{Any, Transaction};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Recipe {
    pub id: i64,
    pub name: String,
    pub versions: HashMap<i64, Ref<RecipeVersion>>,
    pub categories: Vec<Ref<Category>>,
}

impl Model for Recipe {
    type ID = i64;

    async fn get(
        transaction: &mut Transaction<'_, Any>,
        id: Self::ID,
    ) -> DBResult<Self> {
        let name: String = sqlx::query_scalar(
            "SELECT name FROM recipes WHERE id = $1 AND NOT hidden",
        )
        .bind(id)
        .fetch_one(&mut *transaction)
        .await?;

        let version_ids: Vec<i64> = sqlx::query_scalar(
            "SELECT DISTINCT version_id FROM recipes_ingredients WHERE recipe_id = $1",
        )
        .bind(id)
        .fetch_all(&mut *transaction)
        .await?;

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
