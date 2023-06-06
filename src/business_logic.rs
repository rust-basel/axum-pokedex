#[derive(Debug)]
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

pub fn get_pokemon<S: Storage>(id: usize, storage: &S) -> Result<Pokemon, BusinessError> {
    Ok(storage.get_pokemon(id)?)
}

#[cfg(test)]
mod tests {
    use crate::business_logic::get_pokemon;
    use mockall::predicate::eq;

    use crate::storage::MockStorage;

    use super::{create_pokemon, Pokemon};

    #[test]
    fn create_pokemon_given_valid_pokemon_when_called_with_pokemon_then_stores_pokemon_in_storage()
    {
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

    #[test]
    fn get_pokemon_given_id_when_called_with_ok_mock_then_returns_pokemon() {
        // given
        let id = 6;
        let mut mock = MockStorage::new();
        let expected_pokemon = Pokemon {
            name: "Glumanda".to_string(),
            id: 6,
        };
        mock.expect_get_pokemon()
            .with(eq(id))
            .times(1)
            .returning(move |_| Ok(expected_pokemon.clone()));

        // when
        let result = get_pokemon(6, &mock);

        // then
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Pokemon {
                name: "Glumanda".to_string(),
                id: 6
            }
        );
    }
}
