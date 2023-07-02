use crate::model::Pokemon;
use crate::view::{
    Direction, PokemonCreate, PokemonIndexField, PokemonIndexRequest, PokemonShow, PokemonUpdate,
};
use axum::extract::Path;
use axum::{
    extract::{self, State},
    http::StatusCode,
    Json,
};
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

type ThreadSafeDb = Arc<Mutex<HashMap<usize, Pokemon>>>;

pub async fn create_pokemon(
    State(db): State<ThreadSafeDb>,
    Json(pokemon_create_request): Json<PokemonCreate>,
) -> StatusCode {
    let mut db = db.lock().unwrap();
    let pokemon: Pokemon = pokemon_create_request.into();
    db.insert(pokemon.number, pokemon);
    StatusCode::CREATED
}

pub async fn list_pokemon(
    State(db): State<ThreadSafeDb>,
    index_request: extract::Query<PokemonIndexRequest>,
) -> Json<Vec<PokemonShow>> {
    let mut db = db.lock().unwrap();
    let pokemons = index_pokemons(db.deref_mut(), index_request.0);
    Json(pokemons)
}

pub async fn show_pokemon(
    State(db): State<ThreadSafeDb>,
    Path(id): Path<usize>,
) -> Result<Json<PokemonShow>, StatusCode> {
    let db = db.lock().unwrap();
    match db.get(&id) {
        Some(pokemon) => Ok(Json(PokemonShow::from(pokemon))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_pokemon(
    State(db): State<ThreadSafeDb>,
    Path(id): Path<usize>,
) -> Result<StatusCode, StatusCode> {
    let mut db = db.lock().unwrap();
    match db.remove(&id) {
        Some(_) => Ok(StatusCode::NO_CONTENT),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn update_pokemon(
    State(db): State<ThreadSafeDb>,
    Path(id): Path<usize>,
    Json(update_request): Json<PokemonUpdate>,
) -> Result<StatusCode, StatusCode> {
    let mut db = db.lock().unwrap();
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
    db: &mut HashMap<usize, Pokemon>,
    index_request: PokemonIndexRequest,
) -> Vec<PokemonShow> {
    let mut pokemon_list: Vec<PokemonShow> = db.iter().map(|(_, model)| model.into()).collect();

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
    let pokemon_list: Vec<PokemonShow> = pokemon_list
        .into_iter()
        .filter(|pokemon| pokemon.name.contains(&index_request.search))
        .collect();

    pokemon_list
}
