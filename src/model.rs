use crate::view::{PokemonCreate, PokemonShow};

#[derive(Clone)]
pub struct Pokemon {
    pub name: String,
    pub number: usize,
    pub pokemon_type: String,
    pub nick_name: String,
}

impl From<PokemonCreate> for Pokemon {
    fn from(value: PokemonCreate) -> Self {
        Pokemon {
            name: value.name,
            number: value.number,
            pokemon_type: value.pokemon_type,
            nick_name: value.nick_name,
        }
    }
}

impl From<&Pokemon> for PokemonShow {
    fn from(p: &Pokemon) -> Self {
        PokemonShow {
            name: p.name.clone(),
            nick_name: p.nick_name.clone(),
            id: p.number,
            pokemon_type: p.pokemon_type.clone(),
        }
    }
}
