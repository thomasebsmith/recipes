mod migrator;

use crate::config::DatabaseConfig;
use log::LevelFilter;
use migrator::Migrator;
use sqlx::any::{Any, AnyConnectOptions, AnyPoolOptions};
use sqlx::{ConnectOptions, Pool, Transaction};
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;

pub type DBResult<T> = Result<T, sqlx::Error>;

pub trait SqlxFut<T>: Future<Output = DBResult<T>> {}
impl<T, U: Future<Output = Result<T, sqlx::Error>>> SqlxFut<T> for U {}

pub struct Database {
    connection_pool: Pool<Any>,
    version: i64,
}

impl Database {
    pub async fn new(config: DatabaseConfig) -> DBResult<Self> {
        let mut connect_options =
            AnyConnectOptions::from_str(&config.connection_url)?;
        connect_options.log_statements(LevelFilter::Debug);

        let connection_pool = AnyPoolOptions::new()
            .max_connections(config.max_connections)
            .connect_with(connect_options)
            .await?;

        let mut migrator = Migrator::new(&connection_pool).await?;
        migrator.run_migrations().await?;

        let version = migrator.get_current_version();

        Ok(Self {
            connection_pool,
            version,
        })
    }

    pub fn get_version(&self) -> i64 {
        self.version
    }

    pub async fn with_transaction<T, Func>(&self, action: Func) -> DBResult<T>
    where
        Func: for<'a> FnOnce(
            &'a mut Transaction<'static, Any>,
        ) -> Pin<
            Box<dyn Send + Future<Output = Result<T, sqlx::Error>> + 'a>,
        >,
    {
        let mut transaction = self.connection_pool.begin().await?;
        let result = action(&mut transaction).await?;
        transaction.commit().await?;
        Ok(result)
    }

    pub async fn run_test_query(&self) -> DBResult<()> {
        let test_result: (f64,) = sqlx::query_as("SELECT $1")
            .bind(std::f64::consts::PI)
            .fetch_one(&self.connection_pool)
            .await?;

        println!(
            "Query result: expected = {}, actual = {}",
            std::f64::consts::PI,
            test_result.0
        );

        Ok(())
    }
}
