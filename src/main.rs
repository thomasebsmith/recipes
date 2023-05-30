mod api;

use axum::{routing::get, Router};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

const IP_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/api", api::create_router());

    axum::Server::bind(&SocketAddr::new(IP_ADDRESS, PORT))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
