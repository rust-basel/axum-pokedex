#[derive(PartialEq, Debug)]
pub enum BusinessError {
    NotFound,
}

use crate::storage::{Storage, StorageError};

#[derive(Debug, PartialEq, Clone)]
pub struct Pokemon {
    pub(crate) name: String,
    pub(crate) id: usize,
}

impl From<StorageError> for BusinessError {
    fn from(_value: StorageError) -> Self {
        BusinessError::NotFound
    }
}

pub fn create_pokemon<S>(pokemon: Pokemon, storage: &mut S) -> Result<(), BusinessError>
where
    S: Storage,
{
    Ok(storage.store_pokemon(pokemon)?)
}

pub fn update_pokemon<S>(pokemon: Pokemon, storage: &mut S) -> Result<(), BusinessError>
where
    S: Storage,
{
    Ok(storage.update_pokemon(pokemon)?)
}

pub fn delete_pokemon<S>(id: usize, storage: &mut S) -> Result<(), BusinessError>
where
    S: Storage,
{
    storage.delete_pokemon(id)?;
    Ok(())
}

pub fn get_pokemon<S: Storage>(id: usize, storage: &S) -> Result<Pokemon, BusinessError> {
    Ok(storage.get_pokemon(id)?)
}

#[cfg(test)]
mod tests {
    use crate::business_logic::{delete_pokemon, get_pokemon, update_pokemon, BusinessError};
    use mockall::predicate::eq;

    use crate::storage::{MockStorage, StorageError};

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

    #[test]
    fn delete_pokemon_given_pokemon_when_called_with_id_then_returns_ok() {
        // given
        let id = 6;
        let mut mock = MockStorage::new();
        mock.expect_delete_pokemon()
            .with(eq(id))
            .times(1)
            .returning(|_| Ok(()));

        // when
        let result = delete_pokemon(6, &mut mock);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn update_pokemon_given_bad_mock_when_called_then_returns_not_found() {
        // given
        let mut mock = MockStorage::new();
        mock.expect_update_pokemon()
            .times(1)
            .returning(|_| Err(StorageError::NotFound));

        // when
        let result = update_pokemon(
            Pokemon {
                name: "Pika".to_string(),
                id: 24,
            },
            &mut mock,
        );

        // then
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), BusinessError::NotFound);
    }

    #[test]
    fn update_pokemon_given_good_mock_when_called_then_returns_ok() {
        // given
        let mut mock = MockStorage::new();
        mock.expect_update_pokemon().times(1).returning(|_| Ok(()));

        // when
        let result = update_pokemon(
            Pokemon {
                name: "Pika".to_string(),
                id: 24,
            },
            &mut mock,
        );

        // then
        assert!(result.is_ok());
    }
}
