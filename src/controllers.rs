use std::sync::{Arc, Mutex};

use axum::{
    extract::{self, State},
    http::StatusCode,
};

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
}
