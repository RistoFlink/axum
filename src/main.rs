#![allow(unused)]

use std::net::SocketAddr;
use std::sync::mpsc::RecvTimeoutError;

use axum::{Router, routing::get};
use axum::response::Html;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
    get(|| async { Html("Hello, <strong> world!</strong>")})
    ); 

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("-> LISTENING on {addr}\n");
    axum::Server::bind(&addr).serve(routes_hello.into_make_service()).await.unwrap();
}
