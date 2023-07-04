use crate::database::DBResult;
use serde::Serialize;
use sqlx::{Any, Transaction};

pub trait Model: Serialize + Sized {
    type ID: Copy;

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
