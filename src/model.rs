use crate::view::{PokemonCreate, PokemonShow};

#[derive(Clone)]
pub struct Pokemon {
    pub name: String,
    pub id: usize,
}

#[derive(PartialEq, Debug)]
pub enum PokemonError {
    NotFound,
}

impl From<PokemonCreate> for Pokemon {
    fn from(value: PokemonCreate) -> Self {
        Pokemon {
            name: value.name,
            id: 999, //todo
        }
    }
}

impl From<&Pokemon> for PokemonShow{
    fn from(p: &Pokemon) -> Self {
        PokemonShow {
            name: p.name.clone(),
            id: p.id.clone(),
        }
    }
}