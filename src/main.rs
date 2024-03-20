mod api;
mod config;
mod database;
mod frontend;
mod models;
mod util;

use std::fs::File;
use std::net::SocketAddr;
use std::process::ExitCode;
use std::sync::Arc;

use axum::Router;
use log::{error, info, trace};
use simplelog::WriteLogger;
use tokio::net::TcpListener;

use crate::config::{get_config, LoggingConfig};
use crate::database::Database;
use crate::util::stringify_err;

/// Initializes logging based on the log file path and verbosity in `config`.
fn init_logging(config: &LoggingConfig) -> Result<(), String> {
    println!(
        "Writing logs to path {} with verbosity {}",
        config.log_file_path.display(),
        config.verbosity
    );

    let simplelog_config = simplelog::ConfigBuilder::new()
        .set_time_format_rfc3339()
        .build();

    stringify_err(WriteLogger::init(
        config.verbosity,
        simplelog_config,
        stringify_err(
            File::options()
                .create(true)
                .append(true)
                .open(&config.log_file_path),
        )?,
    ))
}

/// Starts the server asynchronously and runs it until shutdown or an error
/// occurs.
///
/// This function serves as the entrypoint to the asynchronous runtime.
#[tokio::main]
async fn run_server() -> Result<(), String> {
    println!("Reading configuration...");
    let config = get_config()?;

    init_logging(&config.logging)?;
    trace!("Logging initiated with verbosity {}", config.logging.verbosity);

    info!("Starting server: recipes");

    info!("Connecting to database...");
    let database =
        Arc::new(stringify_err(Database::new(config.database).await)?);
    info!("Database connected (version = {})", database.get_version());

    let app = Router::new()
        .nest("/", frontend::create_router(database.clone()))
        .nest("/api", api::create_router(database));

    info!(
        "Binding to {}:{}",
        config.server.ip_address, config.server.port
    );

    let listener = stringify_err(
        TcpListener::bind(&SocketAddr::new(
            config.server.ip_address,
            config.server.port,
        ))
        .await,
    )?;

    stringify_err(axum::serve(listener, app).await)
}

/// Runs the server until shutdown or an error occurs.
fn main() -> ExitCode {
    if let Err(err) = run_server() {
        eprintln!("{err}");
        error!("Server exiting with error: {err}");
        ExitCode::FAILURE
    } else {
        info!("Server exiting normally");
        ExitCode::SUCCESS
    }
}
