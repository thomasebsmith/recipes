#![feature(async_fn_in_trait)]

mod api;
mod config;
mod database;
mod models;

use crate::config::{get_config, LoggingConfig};
use crate::database::Database;
use axum::{routing::get, Router};
use log::{error, info};
use simplelog::{self, WriteLogger};
use std::fs::File;
use std::net::SocketAddr;
use std::process::ExitCode;
use std::sync::Arc;

fn init_logging(config: &LoggingConfig) -> Result<(), String> {
    println!(
        "Logs will be written to {} with verbosity {}",
        config.log_file_path.display(),
        config.verbosity
    );

    WriteLogger::init(
        config.verbosity,
        simplelog::Config::default(),
        File::options()
            .create(true)
            .append(true)
            .open(&config.log_file_path)
            .map_err(|err| err.to_string())?,
    )
    .map_err(|err| err.to_string())
}

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
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/api", api::create_router(database));

    info!(
        "Binding to {}:{}",
        config.server.ip_address, config.server.port
    );

    axum::Server::bind(&SocketAddr::new(
        config.server.ip_address,
        config.server.port,
    ))
    .serve(app.into_make_service())
    .await
    .map_err(|err| err.to_string())
}

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
