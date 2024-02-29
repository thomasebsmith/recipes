use std::collections::HashMap;
use std::pin::Pin;

use log::debug;
use sqlx::any::Any;
use sqlx::{Pool, Transaction};

use super::{DBFut, DBResult};

// TODO: Switch this file to use sqlx's built-in migrations.
type Migration =
    dyn Fn(&mut Transaction<'static, Any>) -> Pin<Box<dyn DBFut<i64>>>;

fn get_migrations() -> HashMap<i64, Box<Migration>> {
    HashMap::<i64, Box<Migration>>::new()
}

pub struct Migrator<'a> {
    connection_pool: &'a Pool<Any>,
    current_version: i64,
}

impl<'a> Migrator<'a> {
    pub async fn new(connection_pool: &'a Pool<Any>) -> DBResult<Migrator<'a>> {
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

        debug!(
            "Running database migrations starting at version \
                {starting_version}"
        );

        let migrations = get_migrations();

        let mut transaction = self.connection_pool.begin().await?;
        while let Some(migration) = migrations.get(&self.current_version) {
            debug!("Applying migration from version {}", self.current_version);
            let new_version = migration(&mut transaction).await?;
            assert!(new_version > self.current_version);
            debug!("After applying migration, version is now {new_version}");
            self.current_version = new_version;
        }
        let version_update_result = sqlx::query(
            "UPDATE db_version SET version = $1 WHERE version = $2",
        )
        .bind(self.current_version)
        .bind(starting_version)
        .execute(&mut *transaction)
        .await?;
        if version_update_result.rows_affected() != 1 {
            return Err(sqlx::Error::RowNotFound.into());
        }

        transaction.commit().await?;

        Ok(())
    }

    pub fn get_current_version(&self) -> i64 {
        self.current_version
    }
}
