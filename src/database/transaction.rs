use sqlx::{any::Any, Transaction};

use super::DBResult;

pub struct DBTransaction {
    transaction: Transaction<'static, Any>, /* The encapsulated SQLx
                                             * transaction */
}

impl DBTransaction {
    #[allow(dead_code)]
    /// Applies an action to the `SQLx` transaction contained within this
    /// object.
    pub fn apply<T, Func>(&mut self, action: Func) -> T
    where
        Func: for<'a> FnOnce(&'a mut Transaction<'static, Any>) -> T,
    {
        action(&mut self.transaction)
    }

    #[allow(dead_code)]
    pub async fn commit(self) -> DBResult<()> {
        Ok(self.transaction.commit().await?)
    }

    #[allow(dead_code)]
    pub async fn rollback(self) -> DBResult<()> {
        Ok(self.transaction.rollback().await?)
    }
}
