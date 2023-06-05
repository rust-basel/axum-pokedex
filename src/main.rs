use axum::{
    routing::{get, post},
    Router,
};
use key_value_storage::KeyValueStorage;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex, RwLock},
};

mod business_logic;
mod controllers;
mod key_value_storage;
mod models;
mod storage;

use controllers::Controller;

#[tokio::main]
async fn main() {
    let app: Router = app();
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

#[allow(dead_code)]
fn app() -> Router {
    let database = Arc::new(Mutex::new(KeyValueStorage::new()));
    let app = Router::new()
        .route("/", get(handler))
        .route("/pokemon/create", post(Controller::create_pokemon))
        .route("/pokemon/index", get(Controller::pokemon_index))
        .with_state(database);
    app
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use tower::util::ServiceExt; // app.oneshot(...)

    use crate::{app, models};

    #[tokio::test]
    async fn create_pokemon_when_called_with_correct_payload_returns_http_ok() {
        // given
        let app = app();
        let json_payload = models::PokemonCreateRequest {
            name: String::from("Glumanda"),
            id: 6usize,
        };
        let create_request = Request::builder()
            .method(http::Method::POST)
            .uri("/pokemon/create")
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&json_payload).unwrap()))
            .unwrap();

        // when
        let response = &app.clone().oneshot(create_request).await.unwrap();

        // then
        assert_eq!(response.status(), StatusCode::OK);

        let glumanda_list_request = Request::builder()
            .method(http::Method::GET)
            .uri("/pokemon/index?sort_field=Name&sort_direction=Ascending&search=Glumanda")
            .body(Body::empty())
            .unwrap();

        let response = &app.clone().oneshot(glumanda_list_request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        //TODO: check if response body contains Glumanda

        let bulbasaur_list_request = Request::builder()
            .method(http::Method::GET)
            .uri("/pokemon/index?sort_field=Name&sort_direction=Ascending&search=Bulbasaur")
            .body(Body::empty())
            .unwrap();

        let response = &app.clone().oneshot(bulbasaur_list_request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        //TODO: result should be empty
    }
}
