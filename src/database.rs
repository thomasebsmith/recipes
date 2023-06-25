mod migrator;

use crate::config::DatabaseConfig;
use migrator::Migrator;
use sqlx::any::{Any, AnyPoolOptions};
use sqlx::Pool;

pub struct Database {
    connection_pool: Pool<Any>,
    version: i64,
}

impl Database {
    pub async fn new(config: DatabaseConfig) -> Result<Self, sqlx::Error> {
        let connection_pool = AnyPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.connection_url)
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

    pub async fn run_test_query(&self) -> Result<(), sqlx::Error> {
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
