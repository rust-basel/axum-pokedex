use axum::{extract, Json};

use crate::models;

pub struct Controller;

impl Controller {
    pub async fn create_pokemon(
        extract::Json(pokemon_create_request): extract::Json<models::PokemonCreateRequest>,
    ) {
        // call business logic to create a pokemon
        println!("{:?}", pokemon_create_request);
    }
}
