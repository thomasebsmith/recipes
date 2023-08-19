use serde::Serialize;
use sqlx::{Any, Transaction};

use crate::database::DBResult;

/// Represents a data type that can be the result of an API GET request.
pub trait Model: Serialize + Sized {
    type ID: Copy;

    /// Attempts to retrieve the model of this type with the ID `id` from the
    /// database using `transaction`.
    async fn get(
        transaction: &mut Transaction<'_, Any>,
        id: Self::ID,
    ) -> DBResult<Self>;

    /// Eagerly retrieves all data referenced by this model.
    ///
    /// When creating a model with `Model::get`, models may not actually
    /// retrieve all referenced submodels. Calling this method will retrieve
    /// all submodels.
    async fn fill_refs(
        &mut self,
        transaction: &mut Transaction<'_, Any>,
    ) -> DBResult<()>;

    /// Attempts to retrieve the model of this type with the ID `id` and its
    /// submodels from the database using `transaction`.
    ///
    /// The default implementation is equivalent to `get(transaction, id)`,
    /// followed by `fill_refs(transaction)` if the call to `get` succeeds.
    async fn get_filled(
        transaction: &mut Transaction<'_, Any>,
        id: Self::ID,
    ) -> DBResult<Self> {
        let mut model = Self::get(transaction, id).await?;
        model.fill_refs(transaction).await?;
        Ok(model)
    }
}
