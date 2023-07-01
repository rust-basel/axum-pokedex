
pub mod controller {
    use std::collections::HashMap;

    use axum::extract::Path;

    use axum::{
        extract::{self, State},
        http::StatusCode,
        Json,
    };
    use axum_macros::debug_handler;

    use crate::model::{Pokemon, PokemonError};
    use crate::view::{Direction, PokemonCreate, PokemonIndexField, PokemonIndexRequest, PokemonList, PokemonShow, PokemonUpdate};

    #[debug_handler]
    pub async fn create_pokemon(
        State(db): State<HashMap<usize, Pokemon>>,
        extract::Json(pokemon_create_request): extract::Json<PokemonCreate>,
    ) -> StatusCode {
        // call business logic to create a pokemon
        let pokemon: Pokemon = pokemon_create_request.into();
        let mut db = db;
        match db.insert(pokemon.id, pokemon) {
            Some(_) => StatusCode::OK,
            None => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    #[debug_handler]
    pub async fn list_pokemon(
        State(db): State<HashMap<usize,Pokemon>>,
        index_request: extract::Query<PokemonIndexRequest>,
    ) -> axum::response::Json<Vec<PokemonList>> {
        let pokemons = index_pokemons(db, index_request.0).unwrap_or(vec![]);
        axum::response::Json(pokemons)
    }

    #[debug_handler]
    pub async fn show_pokemon(
        State(db): State<HashMap<usize,Pokemon>>,
        Path(id): Path<usize>,
    ) -> Result<Json<PokemonShow>, StatusCode> {
        match db.get(&id) {
            Some(pokemon) => Ok(Json(PokemonShow::from(pokemon))),
            None => Err(StatusCode::NOT_FOUND),
        }
    }

    #[debug_handler]
    pub async fn delete_pokemon(
        State(db): State<HashMap<usize,Pokemon>>,
        Path(id): Path<usize>,
    ) -> Result<StatusCode, StatusCode> {
        let mut db = db;
        match db.remove(&id) {
            Some(_) => Ok(StatusCode::NO_CONTENT),
            None => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    #[debug_handler]
    pub async fn update_pokemon(
        State(db): State<HashMap<usize,Pokemon>>,
        Path(id): Path<usize>,
        Json(update_request): Json<PokemonUpdate>,
    ) -> Result<StatusCode, StatusCode> {
        match update_request.name {
            None => Err(StatusCode::BAD_REQUEST),
            Some(name) => {
                match do_update_pokemon(db, Pokemon { name, id }) {
                    Ok(()) => Ok(StatusCode::NO_CONTENT),
                    Err(PokemonError::NotFound) => Err(StatusCode::NOT_FOUND),
                }
            }
        }
    }

    // helpers for update and index
    fn do_update_pokemon(mut db: HashMap<usize, Pokemon>, pokemon: Pokemon) -> Result<(), PokemonError> {
        match db.get_mut(&pokemon.id) {
            Some(p) => {
                p.name = pokemon.name;
                Ok(())
            }
            None => Err(PokemonError::NotFound),
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
}


