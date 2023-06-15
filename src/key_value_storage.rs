use std::collections::HashMap;

use crate::storage::StorageError;
use crate::{business_logic::Pokemon, storage::Storage};

pub struct KeyValueStorage {
    inner: HashMap<usize, Pokemon>,
}

impl Storage for KeyValueStorage {
    fn store_pokemon(&mut self, pokemon: Pokemon) -> Result<(), StorageError> {
        self.inner.insert(pokemon.id, pokemon);
        Ok(())
    }

    fn get_pokemon(&self, id: usize) -> Result<Pokemon, StorageError> {
        match self.inner.get(&id) {
            None => Err(StorageError::NotFound),
            Some(p) => Ok(p.clone()),
        }
    }

    fn delete_pokemon(&mut self, id: usize) -> Result<(), StorageError> {
        self.inner.remove(&id);
        Ok(())
    }

    fn update_pokemon(&mut self, pokemon: Pokemon) -> Result<(), StorageError> {
        match self.inner.get_mut(&pokemon.id) {
            Some(p) => {
                p.name = pokemon.name;
                Ok(())
            }
            None => Err(StorageError::NotFound),
        }
    }
}

impl KeyValueStorage {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn with(storage: HashMap<usize, Pokemon>) -> Self {
        Self { inner: storage }
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::StorageError;
    use crate::{business_logic::Pokemon, storage, storage::Storage};
    use std::collections::HashMap;

    use super::KeyValueStorage;

    #[test]
    fn store_pokemon_given_pokemon_when_called_with_pokemon_then_stores_pokemon() {
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

    #[test]
    fn get_pokemon_given_id_with_existing_pokemon_when_called_then_ok_pokemon() {
        // given
        let id: usize = 6;
        let pokemon_name = "Glumanda";
        let mut inner_storage: HashMap<usize, Pokemon> = HashMap::new();
        inner_storage.insert(
            6,
            Pokemon {
                name: pokemon_name.to_string(),
                id,
            },
        );
        let storage = KeyValueStorage {
            inner: inner_storage,
        };

        // when
        let result = storage.get_pokemon(id);

        // then
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Pokemon {
                name: pokemon_name.to_string(),
                id
            }
        )
    }

    #[test]
    fn get_pokemon_given_id_with_not_existing_pokemon_when_called_then_err_not_found() {
        // given
        let id: usize = 6;
        let storage = KeyValueStorage::new();

        // when
        let result = storage.get_pokemon(id);

        // then
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), storage::StorageError::NotFound);
    }

    #[test]
    fn delete_pokemon_given_id_with_when_called_with_pokemon_then_deletes_it() {
        // given
        let id = 6;
        let mut storage = HashMap::new();
        storage.insert(
            id,
            Pokemon {
                name: "Glumanda".to_string(),
                id,
            },
        );
        let mut storage = KeyValueStorage::with(storage);

        // when
        let result = storage.delete_pokemon(id);

        // then
        assert!(result.is_ok());
        assert_eq!(storage.inner.get(&id), None);
    }

    #[test]
    fn update_pokemon_given_pokemon_when_called_with_pokemon_then_adjusts_name() {
        // given
        let id = 6;
        let mut storage = HashMap::new();
        storage.insert(
            id,
            Pokemon {
                name: "Glumanda".to_string(),
                id,
            },
        );
        let mut storage = KeyValueStorage::with(storage);
        let expected_pokemon = Pokemon {
            name: "NotGlumanda".to_string(),
            id,
        };

        // when
        let result = storage.update_pokemon(expected_pokemon.clone());

        // then
        assert!(result.is_ok());
        assert_eq!(storage.inner.get(&id), Some(&expected_pokemon));
    }

    #[test]
    fn update_pokemon_given_no_pokemon_when_called_with_pokemon_then_err_not_found() {
        // given
        let id = 6;
        let storage = HashMap::new();
        let mut storage = KeyValueStorage::with(storage);
        let expected_pokemon = Pokemon {
            name: "NotGlumanda".to_string(),
            id,
        };

        // when
        let result = storage.update_pokemon(expected_pokemon.clone());

        // then
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StorageError::NotFound);
    }
}
