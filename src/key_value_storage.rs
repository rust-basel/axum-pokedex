use std::collections::HashMap;

use crate::{business_logic::Pokemon, storage::Storage};

pub struct KeyValueStorage {
    inner: HashMap<usize, Pokemon>,
}

impl Storage for KeyValueStorage {
    fn store_pokemon(
        &mut self,
        pokemon: crate::business_logic::Pokemon,
    ) -> Result<(), crate::storage::StorageError> {
        self.inner.insert(pokemon.id, pokemon);
        Ok(())
    }

    fn get_pokemon(
        &self,
        id: usize,
    ) -> Result<crate::business_logic::Pokemon, crate::storage::StorageError> {
        todo!()
    }

    fn delete_pokomen(
        &self,
        id: usize,
    ) -> Result<crate::business_logic::Pokemon, crate::storage::StorageError> {
        todo!()
    }

    fn update_pokemon(
        &self,
        pokemon: crate::business_logic::Pokemon,
    ) -> Result<(), crate::storage::StorageError> {
        todo!()
    }

    fn index_pokemons(
        &self,
        index_request: crate::models::PokemonIndexRequest,
    ) -> Result<Vec<crate::models::PokemonList>, crate::storage::StorageError> {
        let mut pokemon_list: Vec<crate::models::PokemonList> = Vec::new();
        self.inner.iter().for_each(|(key, value)| {
            pokemon_list.push(crate::models::PokemonList {
                name: value.name.clone(),
                id: *key,
            });
        });

        //sort pokemon_list by name ascending if index_request.sort == Name
        match (index_request.sort_field, index_request.sort_direction) {
            (crate::models::PokemonIndexField::Name, crate::models::Direction::Ascending) => {
                pokemon_list.sort_by(|a, b| a.name.cmp(&b.name))
            }
            (crate::models::PokemonIndexField::Name, crate::models::Direction::Descending) => {
                pokemon_list.sort_by(|b, a| a.name.cmp(&b.name))
            }
            (crate::models::PokemonIndexField::Id, crate::models::Direction::Ascending) => (),
            (crate::models::PokemonIndexField::Id, crate::models::Direction::Descending) => {
                pokemon_list = pokemon_list.into_iter().rev().collect()
            }
        };

        // filter pokemon_list by index_request.search
        let pokemon_list: Vec<crate::models::PokemonList> = pokemon_list
            .into_iter()
            .filter(|pokemon| pokemon.name.contains(&index_request.search))
            .collect();

        Ok(pokemon_list)
    }
}

impl KeyValueStorage {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{business_logic::Pokemon, storage::Storage};

    use super::KeyValueStorage;

    #[test]
    fn store_pokemon_when_called_with_pokemon_then_stores_pokemon() {
        // given
        let pokemon = Pokemon {
            name: "Bisasam".to_string(),
            id: 3,
        };
        let mut storage = KeyValueStorage::new();

        // when
        storage.store_pokemon(pokemon).unwrap();

        // then
        assert!(storage.inner.get(&3).is_some());
    }
}
