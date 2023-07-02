use crate::model::{Pokemon, PokemonError};
use crate::view::{
    Direction, PokemonCreate, PokemonIndexField, PokemonIndexRequest, PokemonList, PokemonShow,
    PokemonUpdate,
};
use axum::extract::Path;
use axum::{
    extract::{self, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;
use std::collections::HashMap;

#[debug_handler]
pub async fn create_pokemon(
    State(db): State<HashMap<usize, Pokemon>>,
    Json(pokemon_create_request): Json<PokemonCreate>,
) -> StatusCode {
    let pokemon: Pokemon = pokemon_create_request.into();
    let mut db = db;
    db.insert(pokemon.id, pokemon);
    StatusCode::CREATED
}

#[debug_handler]
pub async fn list_pokemon(
    State(db): State<HashMap<usize, Pokemon>>,
    index_request: extract::Query<PokemonIndexRequest>,
) -> Json<Vec<PokemonList>> {
    let pokemons = index_pokemons(db, index_request.0).unwrap_or(vec![]);
    Json(pokemons)
}

#[debug_handler]
pub async fn show_pokemon(
    State(db): State<HashMap<usize, Pokemon>>,
    Path(id): Path<usize>,
) -> Result<Json<PokemonShow>, StatusCode> {
    match db.get(&id) {
        Some(pokemon) => Ok(Json(PokemonShow::from(pokemon))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[debug_handler]
pub async fn delete_pokemon(
    State(db): State<HashMap<usize, Pokemon>>,
    Path(id): Path<usize>,
) -> Result<StatusCode, StatusCode> {
    let mut db = db;
    match db.remove(&id) {
        Some(_) => Ok(StatusCode::NO_CONTENT),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[debug_handler]
pub async fn update_pokemon(
    State(mut db): State<HashMap<usize, Pokemon>>,
    Path(id): Path<usize>,
    Json(update_request): Json<PokemonUpdate>,
) -> Result<StatusCode, StatusCode> {
    let pokemon_to_update = db.get_mut(&id);
    if let Some(pokemon) = pokemon_to_update {
        if let Some(nick_name) = update_request.nick_name {
            pokemon.nick_name = nick_name;
        }
        if let Some(name) = update_request.name {
            pokemon.name = name;
        }
        if let Some(pokemon_type) = update_request.pokemon_type {
            pokemon.pokemon_type = pokemon_type;
        }

        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

fn index_pokemons(
    db: HashMap<usize, Pokemon>,
    index_request: PokemonIndexRequest,
) -> Result<Vec<PokemonList>, PokemonError> {
    let mut pokemon_list: Vec<PokemonList> = Vec::new();
    db.iter().for_each(|(key, value)| {
        pokemon_list.push(PokemonList {
            name: value.name.clone(),
            id: *key,
        });
    });

    //sort pokemon_list by name ascending if index_request.sort == Name
    match (index_request.sort_field, index_request.sort_direction) {
        (PokemonIndexField::Name, Direction::Ascending) => {
            pokemon_list.sort_by(|a, b| a.name.cmp(&b.name))
        }
        (PokemonIndexField::Name, Direction::Descending) => {
            pokemon_list.sort_by(|b, a| a.name.cmp(&b.name))
        }
        (PokemonIndexField::Id, Direction::Ascending) => (),
        (PokemonIndexField::Id, Direction::Descending) => {
            pokemon_list = pokemon_list.into_iter().rev().collect()
        }
    };

    // filter pokemon_list by index_request.search
    let pokemon_list: Vec<PokemonList> = pokemon_list
        .into_iter()
        .filter(|pokemon| pokemon.name.contains(&index_request.search))
        .collect();

    Ok(pokemon_list)
}
