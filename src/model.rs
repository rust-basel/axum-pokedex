use crate::view::{PokemonCreate, PokemonShow};

#[derive(Clone)]
pub struct Pokemon {
    pub name: String,
    pub id: usize,
    pub pokemon_type: String,
    pub nick_name: String,
}

#[derive(PartialEq, Debug)]
pub enum PokemonError {
    NotFound,
}

impl From<PokemonCreate> for Pokemon {
    fn from(value: PokemonCreate) -> Self {
        Pokemon {
            name: value.name,
            id: value.id,
            pokemon_type: value.pokemon_type,
            nick_name: value.nick_name,
        }
    }
}

impl From<&Pokemon> for PokemonShow {
    fn from(p: &Pokemon) -> Self {
        PokemonShow {
            name: p.name.clone(),
            id: p.id.clone(),
        }
    }
}
