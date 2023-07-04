use super::{DBResult, SqlxFut};
use sqlx::any::Any;
use sqlx::{Pool, Transaction};
use std::collections::HashMap;
use std::pin::Pin;

type Migration =
    dyn Fn(&mut Transaction<'static, Any>) -> Pin<Box<dyn SqlxFut<i64>>>;

fn get_migrations() -> HashMap<i64, Box<Migration>> {
    HashMap::<i64, Box<Migration>>::new()
}

pub struct Migrator<'a> {
    connection_pool: &'a Pool<Any>,
    current_version: i64,
}

impl<'a> Migrator<'a> {
    pub async fn new(
        connection_pool: &'a Pool<Any>,
    ) -> Result<Migrator<'a>, sqlx::Error> {
        let current_version: i64 =
            sqlx::query_scalar("SELECT version FROM db_version")
                .fetch_one(connection_pool)
                .await?;
        Ok(Self {
            connection_pool,
            current_version,
        })
    }

    pub async fn run_migrations(&mut self) -> DBResult<()> {
        let starting_version = self.current_version;

        let migrations = get_migrations();

        let mut transaction = self.connection_pool.begin().await?;
        while let Some(migration) = migrations.get(&self.current_version) {
            let new_version = migration(&mut transaction).await?;
            assert!(new_version > self.current_version);
            self.current_version = new_version;
        }
        let version_update_result = sqlx::query(
            "UPDATE db_version SET version = $1 WHERE version = $2",
        )
        .bind(self.current_version)
        .bind(starting_version)
        .execute(&mut transaction)
        .await?;
        if version_update_result.rows_affected() != 1 {
            return Err(sqlx::Error::RowNotFound);
        }

        transaction.commit().await?;

        Ok(())
    }

    pub fn get_current_version(&self) -> i64 {
        self.current_version
    }
}
