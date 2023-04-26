use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

mod business_logic;
mod controllers;
mod models;

use controllers::Controller;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/pokemon/create", post(Controller::create_pokemon));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, world!"
}
