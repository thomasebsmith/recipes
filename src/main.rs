mod api;
mod config;
mod database;

use crate::config::{get_config, LoggingConfig};
use crate::database::Database;
use axum::{routing::get, Router};
use simplelog::{self, WriteLogger};
use std::fs::File;
use std::net::SocketAddr;
use std::process::ExitCode;

fn init_logging(config: &LoggingConfig) -> Result<(), String> {
    WriteLogger::init(
        config.verbosity,
        simplelog::Config::default(),
        File::create(&config.log_file_path).map_err(|err| err.to_string())?,
    )
    .map_err(|err| err.to_string())
}

#[tokio::main]
async fn run_server() -> Result<(), String> {
    let config = get_config()?;

    init_logging(&config.logging)?;

    let database = Database::new(config.database)
        .await
        .map_err(|err| err.to_string())?;

    database
        .run_test_query()
        .await
        .map_err(|err| err.to_string())?;
    println!("DB version is {}", database.get_version());

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/api", api::create_router());

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
        eprintln!("{}", err);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
