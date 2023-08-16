#![allow(unused)]

use std::net::SocketAddr;
use std::sync::mpsc::RecvTimeoutError;

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::get_service;
use axum::{routing::get, Router};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let routes_all: Router = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static()); //can't have overlapping routes ("/" in this case) - static routing used as a fallback

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("-> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
// e.g. /hello?name=Risto
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("world");
    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g. /hello/Hanh
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}
