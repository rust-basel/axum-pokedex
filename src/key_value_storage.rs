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
