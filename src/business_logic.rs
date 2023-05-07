pub enum BusinessError {
    NotFound,
}

use crate::storage::{self, Storage};

#[derive(Debug, PartialEq, Clone)]
pub struct Pokemon {
    pub(crate) name: String,
    pub(crate) id: usize,
}

impl From<storage::StorageError> for BusinessError {
    fn from(value: storage::StorageError) -> Self {
        BusinessError::NotFound
    }
}

pub fn create_pokemon<S>(pokemon: Pokemon, storage: &mut S) -> Result<(), BusinessError>
where
    S: Storage,
{
    storage.store_pokemon(pokemon)?;
    Ok(())
}

fn update_pokemon(pokemon: Pokemon) -> Result<(), BusinessError> {
    todo!()
}

fn delete_pokomen(id: usize) -> Result<(), BusinessError> {
    todo!()
}

fn get_pokemon(id: usize) -> Result<(), BusinessError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::storage::MockStorage;

    use super::{create_pokemon, Pokemon};

    #[test]
    fn create_pokemon_when_called_with_pokemon_then_stores_pokemon_in_storage() {
        // given
        let pokemon = Pokemon {
            name: String::from("Pikachu"),
            id: 24usize,
        };
        let mut mock = MockStorage::new();
        mock.expect_store_pokemon()
            .with(eq(pokemon.clone()))
            .times(1)
            .returning(|_| Ok(()));

        // when
        let result = create_pokemon(pokemon, &mut mock);

        // then
        assert!(result.is_ok());
    }
}
