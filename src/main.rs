mod api;

use axum::{routing::get, Router};
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::process::ExitCode;

struct Args {
    ip_address: IpAddr,
    port: u16,
}

fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        let executable_name =
            if args.is_empty() { "recipes" } else { &args[0] };
        Err(format!("Usage: {} <IP address> <port>", executable_name))
    } else {
        let ip_address =
            args[1].parse::<IpAddr>().map_err(|err| err.to_string())?;
        let port = args[2].parse::<u16>().map_err(|err| err.to_string())?;
        Ok(Args { ip_address, port })
    }
}

#[tokio::main]
async fn run_server() -> Result<(), String> {
    let args = parse_args()?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/api", api::create_router());

    axum::Server::bind(&SocketAddr::new(args.ip_address, args.port))
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
