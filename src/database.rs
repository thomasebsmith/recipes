mod error;
mod migrator;
mod modelcache;
mod transaction;
mod types;

use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;

pub use error::{to_internal_db_error, Error};
use log::{debug, trace, LevelFilter};
use migrator::Migrator;
use sqlx::any::{
    install_default_drivers, Any, AnyConnectOptions, AnyPoolOptions,
};
use sqlx::{ConnectOptions, Pool, Transaction};
pub use types::{DBFut, DBResult};

use crate::config::DatabaseConfig;

/// Represents a recipes database.
///
/// Allows connecting to the database, querying the database, and updating the
/// database.
///
/// Automatically performs migrations as needed.
pub struct Database {
    connection_pool: Pool<Any>, // The pool of connections to the database
    version: i64,               // The version number reached after migrations
}

impl Database {
    /// Creates a new `Database` based on `config`.
    ///
    /// The database will connect using `config.connection_url` and will
    /// maintain at most `config.max_connections` connections at a time.
    ///
    /// Migrations will be applied while the `Database` is created.
    pub async fn new(config: DatabaseConfig) -> DBResult<Self> {
        trace!("Installing SQLx default drivers");
        install_default_drivers();

        debug!(
            "Connecting to database using connection URL \"{}\"",
            config.connection_url
        );

        // Log SQL statements at debug level. This way, they don't show up with
        // the default logging configuration, but they can easily be enabled for
        // debugging.
        let connect_options =
            AnyConnectOptions::from_str(&config.connection_url)?
                .log_statements(LevelFilter::Debug);

        let connection_pool = AnyPoolOptions::new()
            .max_connections(config.max_connections)
            .connect_with(connect_options)
            .await?;

        // TODO: Use SQLx's built-in migration system
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
