use axum::routing::{delete, patch};
use axum::{
    routing::{get, post},
    Router,
};
use std::{
    net::SocketAddr,
};
use std::collections::HashMap;
use crate::controller::controller::{create_pokemon, delete_pokemon, list_pokemon, show_pokemon, update_pokemon};

mod model;
mod view;
mod controller;

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

async fn handler() -> &'static str {
    "Hello, world!"
}

fn app(db: HashMap<usize, Pokemon>) -> Router {
    let app = Router::new()
        .route("/", get(handler))
        .route("/pokemon/create", post(create_pokemon))
        .route("/pokemon/index", get(list_pokemon))
        .route("/pokemon", post(create_pokemon))
        .route("/pokemon/:id", get(show_pokemon))
        .route("/pokemon/:id", delete(delete_pokemon))
        .route("/pokemon/:id", patch(update_pokemon))
        .with_state(db);
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

    use crate::{app};
    use crate::model::Pokemon;
    use crate::view::{PokemonCreate, PokemonShow, PokemonUpdate};

    #[tokio::test]
    async fn create_pokemon_when_called_with_correct_payload_returns_http_ok() {
        // given
        let app = app(HashMap::new());
        let json_payload = PokemonCreate {
            name: String::from("Glumanda"),
            id: 6usize,
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
    async fn index_pokemon_given_glumanda_when_called_with_search_then_returns_list_containing_glumanda()
    {
        // given
        let mut initial_db: HashMap<usize, Pokemon> = HashMap::new();
        initial_db.insert(6, Pokemon{ name: "Glumanda".to_string(), id: 6 });
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
        assert_eq!(body[0], PokemonShow{ id: 6, name: "Glumanda".to_string() });
    }

    #[tokio::test]
    async fn index_pokemon_indexed_given_empty_db_when_called_then_returns_empty_list(){
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
    async fn get_pokemon_given_stored_pokemon_when_called_with_correct_payload_returns_http_ok_with_payload() {
        // given
        let mut db = HashMap::new();
        db.insert(
            6_usize,
            Pokemon {
                name: "Glumanda".to_string(),
                id: 6,
            },
        );
        let app = app(db);
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
        let body: PokemonShow = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            body,
            PokemonShow {
                name: "Glumanda".to_string(),
                id: 6,
            }
        );
    }

    #[tokio::test]
    async fn delete_pokemon_given_stored_pokemon_when_called_with_id_then_returns_http_ok_no_content() {
        // given
        let id = 6;
        let mut db = HashMap::new();
        db.insert(
            id,
            Pokemon {
                name: "Glumanda".to_string(),
                id: 6,
            },
        );
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
    async fn update_pokemon_given_stored_pokemon_when_called_with_id_then_returns_http_ok_no_content() {
        // given
        let id = 6;
        let mut db = HashMap::new();
        db.insert(
            id,
            Pokemon {
                name: "Glumanda".to_string(),
                id: 6,
            },
        );
        let app = app(db);

        let patch_json_body = PokemonUpdate {
            name: Some("LittleFirePokemon".to_string())
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
}
