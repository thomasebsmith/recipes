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
}

pub trait MutableModel: Model {
    async fn store(
        &self,
        transaction: &mut Transaction<'_, Any>,
    ) -> DBResult<()>;
}
