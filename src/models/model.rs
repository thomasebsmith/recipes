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

    async fn fill_refs(
        &mut self,
        transaction: &mut Transaction<'_, Any>,
    ) -> DBResult<()>;

    async fn get_filled(
        transaction: &mut Transaction<'_, Any>,
        id: Self::ID,
    ) -> DBResult<Self> {
        let mut model = Self::get(transaction, id).await?;
        model.fill_refs(transaction).await?;
        Ok(model)
    }
}

pub trait MutableModel: Model {
    async fn store(
        &self,
        transaction: &mut Transaction<'_, Any>,
    ) -> DBResult<()>;
}
