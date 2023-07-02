use crate::controller::{
    create_pokemon, delete_pokemon, list_pokemon, show_pokemon, update_pokemon,
};
use axum::routing::{delete, patch};
use axum::{
    routing::{get, post},
    Router,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

mod controller;
mod model;
mod view;

use crate::model::Pokemon;

#[tokio::main]
async fn main() {
    let app: Router = app(HashMap::new());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn app(db: HashMap<usize, Pokemon>) -> Router {
    let thread_safe_db = Arc::new(Mutex::new(db));
    let app = Router::new()
        .route("/pokemon", post(create_pokemon))
        .route("/pokemon/:id", get(show_pokemon))
        .route("/pokemon/:id", patch(update_pokemon))
        .route("/pokemon/:id", delete(delete_pokemon))
        .route("/pokemon/index", get(list_pokemon))
        .with_state(thread_safe_db);
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

    use crate::app;
    use crate::model::Pokemon;
    use crate::view::{PokemonCreate, PokemonShow, PokemonUpdate};

    const GLUMANDA_ID: usize = 6;
    const GLUMANDA_NAME: &str = "Glumanda";

    fn create_glumanda_test_pokemon() -> Pokemon {
        Pokemon {
            name: GLUMANDA_NAME.to_string(),
            number: GLUMANDA_ID,
            pokemon_type: "Fire".to_string(),
            nick_name: "MyFirePokemon".to_string(),
        }
    }

    #[tokio::test]
    async fn create_pokemon_when_called_with_correct_payload_returns_http_created() {
        // given
        let app = app(HashMap::new());
        let json_payload = PokemonCreate {
            name: GLUMANDA_NAME.to_string(),
            number: GLUMANDA_ID,
            nick_name: "MyFirePokemon".to_string(),
            pokemon_type: "Fire".to_string(),
        };
        let create_request = Request::builder()
            .method(http::Method::POST)
            .uri("/pokemon")
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&json_payload).unwrap()))
            .unwrap();

        // when
        let response = &app.clone().oneshot(create_request).await.unwrap();

        // then
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn index_pokemon_given_glumanda_when_called_with_search_then_returns_list_containing_glumanda(
    ) {
        // given
        let mut initial_db: HashMap<usize, Pokemon> = HashMap::new();
        initial_db.insert(GLUMANDA_ID, create_glumanda_test_pokemon());
        let app = app(initial_db);

        let glumanda_list_request = Request::builder()
            .method(http::Method::GET)
            .uri("/pokemon/index?sort_field=Name&sort_direction=Ascending&search=Glumanda")
            .body(Body::empty())
            .unwrap();

        // when
        let response = app.clone().oneshot(glumanda_list_request).await.unwrap();

        // then
        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<PokemonShow> = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            body[0],
            PokemonShow {
                id: GLUMANDA_ID,
                name: GLUMANDA_NAME.to_string(),
                nick_name: "MyFirePokemon".to_string(),
                pokemon_type: "Fire".to_string(),
            }
        );
    }

    #[tokio::test]
    async fn index_pokemon_indexed_given_empty_db_when_called_then_returns_empty_list() {
        // given
        let app = app(HashMap::new());
        let bulbasaur_list_request = Request::builder()
            .method(http::Method::GET)
            .uri("/pokemon/index?sort_field=Name&sort_direction=Ascending&search=Bulbasaur")
            .body(Body::empty())
            .unwrap();

        // when
        let response = app.clone().oneshot(bulbasaur_list_request).await.unwrap();

        // then
        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<PokemonShow> = serde_json::from_slice(&body).unwrap();
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn get_pokemon_given_stored_pokemon_when_called_with_correct_payload_returns_http_ok_with_payload(
    ) {
        // given
        let mut db = HashMap::new();
        let id = GLUMANDA_ID;
        db.insert(id, create_glumanda_test_pokemon());
        let app = app(db);
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
        let body: PokemonShow = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            body,
            PokemonShow {
                name: GLUMANDA_NAME.to_string(),
                nick_name: "MyFirePokemon".to_string(),
                id: GLUMANDA_ID,
                pokemon_type: "Fire".to_string(),
            }
        );
    }

    #[tokio::test]
    async fn delete_pokemon_given_stored_pokemon_when_called_with_id_then_returns_http_ok_no_content(
    ) {
        // given
        let id = 6;
        let mut db = HashMap::new();
        db.insert(GLUMANDA_ID, create_glumanda_test_pokemon());
        let app = app(db);

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
        let mut db = HashMap::new();
        db.insert(GLUMANDA_ID, create_glumanda_test_pokemon());
        let app = app(db);

        let patch_json_body = PokemonUpdate {
            name: Some("LittleFirePokemon".to_string()),
            ..Default::default() // syntactic sugar - Options get initialized with None
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

    #[tokio::test]
    async fn delete_pokemon_given_no_pokemon_when_called_then_returns_not_found() {
        // given
        let id = 9;
        let app = app(HashMap::new());
        let delete_request = Request::builder()
            .method(http::Method::DELETE)
            .uri(format!("/pokemon/{id}"))
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::empty())
            .unwrap();

        // when
        let response = app.oneshot(delete_request).await.unwrap();

        // then
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn update_pokemon_given_no_pokemon_when_called_then_returns_not_found() {
        let app = app(HashMap::new());
        let update_view = PokemonUpdate {
            ..Default::default()
        };
        let some_id = 42;
        let update_request = Request::builder()
            .method(http::Method::PATCH)
            .uri(format!("/format/{some_id}"))
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&update_view).unwrap()))
            .unwrap();

        // when
        let response = app.oneshot(update_request).await.unwrap();

        // then
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
