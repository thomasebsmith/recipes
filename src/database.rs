mod error;
mod migrator;
mod modelcache;

use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;

pub use error::{to_internal_db_error, Error};
use log::{debug, LevelFilter};
use migrator::Migrator;
use sqlx::any::{Any, AnyConnectOptions, AnyPoolOptions};
use sqlx::{ConnectOptions, Pool, Transaction};

use crate::config::DatabaseConfig;

/// The result of a database query or operation. Contains either the result of
/// the successful operation, or a database error.
pub type DBResult<T> = Result<T, Error>;

/// A future that returns a `DBResult<T>`.
pub trait DBFut<T>: Future<Output = DBResult<T>> {}
impl<T, U: Future<Output = DBResult<T>>> DBFut<T> for U {}

/// Represents a recipes database.
///
/// Allows connecting to the database, querying the database, and updating the
/// database.
///
/// Automatically performs migrations as needed.
pub struct Database {
    connection_pool: Pool<Any>,
    version: i64,
}

impl Database {
    /// Creates a new `Database` based on `config`.
    ///
    /// The database will connect using `config.connection_url` and will
    /// maintain at most `config.max_connections` connections at a time.
    ///
    /// Migrations will be applied while the `Database` is created.
    pub async fn new(config: DatabaseConfig) -> DBResult<Self> {
        debug!(
            "Connecting to database using connection URL \"{}\"",
            config.connection_url
        );

        let connect_options =
            AnyConnectOptions::from_str(&config.connection_url)?
                .log_statements(LevelFilter::Debug);

        let connection_pool = AnyPoolOptions::new()
            .max_connections(config.max_connections)
            .connect_with(connect_options)
            .await?;

        let mut migrator = Migrator::new(&connection_pool).await?;
        migrator.run_migrations().await?;

        let version = migrator.get_current_version();

        debug!("Database connected and migrated to version {version}");

        Ok(Self {
            connection_pool,
            version,
        })
    }

    /// Returns the current migration version of the database.
    ///
    /// This is the latest known version at the time of `Database` creation.
    pub fn get_version(&self) -> i64 {
        self.version
    }

    /// Executes `action` during a database transaction.
    ///
    /// If `action(&mut transaction).await` returns an `Ok` result, the
    /// `transaction` will be committed. Otherwise, `transaction` will be
    /// aborted.
    pub async fn with_transaction<T, Func>(&self, action: Func) -> DBResult<T>
    where
        Func: for<'a> FnOnce(
            &'a mut Transaction<'static, Any>,
        ) -> Pin<
            Box<dyn Send + Future<Output = Result<T, Error>> + 'a>,
        >,
    {
        let mut transaction = self.connection_pool.begin().await?;
        let result = action(&mut transaction).await?;
        transaction.commit().await?;
        Ok(result)
    }
}

#[allow(dead_code)]
pub struct DBTransaction {
    transaction: Transaction<'static, Any>,
}

impl DBTransaction {
    #[allow(dead_code)]
    pub fn apply<T, Func>(&mut self, action: Func) -> T
    where
        Func: for<'a> FnOnce(&'a mut Transaction<'static, Any>) -> T,
    {
        action(&mut self.transaction)
    }
}
