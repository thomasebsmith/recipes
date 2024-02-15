mod api;
mod config;
mod database;
mod frontend;
mod models;

use std::fs::File;
use std::net::SocketAddr;
use std::process::ExitCode;
use std::sync::Arc;

use axum::Router;
use log::{error, info};
use simplelog::{self, WriteLogger};
use tokio::net::TcpListener;

use crate::config::{get_config, LoggingConfig};
use crate::database::Database;

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

    WriteLogger::init(
        config.verbosity,
        simplelog_config,
        File::options()
            .create(true)
            .append(true)
            .open(&config.log_file_path)
            .map_err(|err| err.to_string())?,
    )
    .map_err(|err| err.to_string())
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

    info!("Starting server: recipes");

    info!("Connecting to database...");
    let database = Arc::new(
        Database::new(config.database)
            .await
            .map_err(|err| err.to_string())?,
    );
    info!("Database connected (version = {})", database.get_version());

    let app = Router::new()
        .nest("/", frontend::create_router(database.clone()))
        .nest("/api", api::create_router(database));

    info!(
        "Binding to {}:{}",
        config.server.ip_address, config.server.port
    );

    let listener = TcpListener::bind(&SocketAddr::new(
        config.server.ip_address,
        config.server.port,
    ))
    .await
    .map_err(|err| err.to_string())?;

    axum::serve(listener, app)
        .await
        .map_err(|err| err.to_string())
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
