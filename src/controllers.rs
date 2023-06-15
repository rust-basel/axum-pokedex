use std::sync::{Arc, Mutex};

use axum::extract::{Path, Query};
use axum::{
    extract::{self, State},
    http::StatusCode,
    Json,
};

use crate::business_logic::{delete_pokemon, get_pokemon, BusinessError, Pokemon};
use crate::models::PokemonGetResponse;
use crate::{business_logic::create_pokemon, models, storage::Storage};

pub struct Controller;

impl Controller {
    pub async fn create_pokemon<T: Storage>(
        State(db): State<Arc<Mutex<T>>>,
        extract::Json(pokemon_create_request): extract::Json<models::PokemonCreateRequest>,
    ) -> StatusCode {
        // call business logic to create a pokemon
        let pokemon = pokemon_create_request.into();
        let mut db = db.lock().unwrap();
        match create_pokemon(pokemon, &mut *db) {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub async fn get_pokemon<T: Storage>(
        State(db): State<Arc<Mutex<T>>>,
        Path(id): Path<usize>,
    ) -> Result<Json<PokemonGetResponse>, StatusCode> {
        let mut db = db.lock().unwrap();
        match get_pokemon(id, &mut *db) {
            Ok(pokemon) => Ok(Json(PokemonGetResponse::from(pokemon))),
            Err(_) => Err(StatusCode::NOT_FOUND),
        }
    }

    pub async fn delete_pokemon<T: Storage>(
        State(db): State<Arc<Mutex<T>>>,
        Path(id): Path<usize>,
    ) -> Result<StatusCode, StatusCode> {
        let mut db = db.lock().unwrap();
        match delete_pokemon(id, &mut *db) {
            Ok(()) => Ok(StatusCode::NO_CONTENT),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn update_pokemon<T: Storage>(
        State(_db): State<Arc<Mutex<T>>>,
        Path(_id): Path<usize>,
        Json(_update_request): Json<models::PokemonUpdateRequest>,
    ) -> Result<StatusCode, StatusCode> {
        todo!()
    }
}
