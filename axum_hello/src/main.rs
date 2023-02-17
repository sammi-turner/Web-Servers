#![allow(unused_imports)]

use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse, Json, Router
};

use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
