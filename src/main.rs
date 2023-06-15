use axum::routing::{delete, patch};
use axum::{
    routing::{get, post},
    Router,
};
use key_value_storage::KeyValueStorage;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

mod business_logic;
mod controllers;
mod key_value_storage;
mod models;
mod storage;

use controllers::Controller;

#[tokio::main]
async fn main() {
    let app: Router = app(KeyValueStorage::new());
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

fn app(storage: KeyValueStorage) -> Router {
    let database = Arc::new(Mutex::new(storage));
    let app = Router::new()
        .route("/", get(handler))
        .route("/pokemon", post(Controller::create_pokemon))
        .route("/pokemon/:id", get(Controller::get_pokemon))
        .route("/pokemon/:id", delete(Controller::delete_pokemon))
        .route("/pokemon/:id", patch(Controller::update_pokemon))
        .with_state(database);
    app
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use std::collections::HashMap;
    use tower::util::ServiceExt; // app.oneshot(...)

    use crate::business_logic::Pokemon;
    use crate::key_value_storage::KeyValueStorage;
    use crate::models::{PokemonGetResponse, PokemonUpdateRequest};
    use crate::{app, models};

    #[tokio::test]
    async fn create_pokemon_when_called_with_correct_payload_returns_http_ok() {
        // given
        let app = app(KeyValueStorage::new());
        let json_payload = models::PokemonCreateRequest {
            name: String::from("Glumanda"),
            id: 6usize,
        };
        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/pokemon")
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&json_payload).unwrap()))
            .unwrap();

        // when
        let response = app.oneshot(request).await.unwrap();

        // then
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_pokemon_given_stored_pokemon_when_called_with_correct_payload_returns_http_ok_with_payload(
    ) {
        // given
        let mut inner_storage = HashMap::new();
        inner_storage.insert(
            6,
            Pokemon {
                name: "Glumanda".to_string(),
                id: 6,
            },
        );
        let storage = KeyValueStorage::with(inner_storage);
        let app = app(storage);
        let id = 6;
        let get_request = Request::builder()
            .method(http::Method::GET)
            .uri(format!("/pokemon/{id}"))
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::empty())
            .unwrap();

        // when
        let response = app.oneshot(get_request).await.unwrap();

        // then
        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: PokemonGetResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            body,
            PokemonGetResponse {
                name: "Glumanda".to_string(),
                id: 6
            }
        );
    }

    #[tokio::test]
    async fn delete_pokemon_given_stored_pokemon_when_called_with_id_then_returns_http_ok_no_content(
    ) {
        // given
        let id = 6;
        let mut inner_storage = HashMap::new();
        inner_storage.insert(
            id,
            Pokemon {
                name: "Glumanda".to_string(),
                id: 6,
            },
        );
        let storage = KeyValueStorage::with(inner_storage);
        let app = app(storage);

        let delete_request = Request::builder()
            .method(http::Method::DELETE)
            .uri(format!("/pokemon/{id}"))
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::empty())
            .unwrap();

        // when
        let response = app.oneshot(delete_request).await.unwrap();

        // then
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn update_pokemon_given_stored_pokemon_when_called_with_id_then_returns_http_ok_no_content(
    ) {
        // given
        let id = 6;
        let mut inner_storage = HashMap::new();
        inner_storage.insert(
            id,
            Pokemon {
                name: "Glumanda".to_string(),
                id: 6,
            },
        );
        let storage = KeyValueStorage::with(inner_storage);
        let app = app(storage);

        let patch_json_body = PokemonUpdateRequest {
            name: Some("LittleFirePokemon".to_string()),
        };
        let update_request = Request::builder()
            .method(http::Method::PATCH)
            .uri(format!("/pokemon/{id}"))
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&patch_json_body).unwrap()))
            .unwrap();

        // when
        let response = app.oneshot(update_request).await.unwrap();

        // then
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
