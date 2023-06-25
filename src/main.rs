mod api;
mod config;
mod database;

use crate::database::Database;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::process::ExitCode;

#[tokio::main]
async fn run_server() -> Result<(), String> {
    let config = config::get_config()?;

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
