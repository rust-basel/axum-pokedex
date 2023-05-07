pub enum BusinessError {
    NotFound,
}

use crate::storage::Storage;

#[derive(Debug, PartialEq)]
pub struct Pokemon {
    name: String,
    id: usize,
}

fn create_pokemon<S>(pokemon: Pokemon, storage: &S) -> Result<(), BusinessError>
where
    S: Storage,
{
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
        // mock
        let pokemon = Pokemon {
            name: String::from("Pikachu"),
            id: 24usize,
        };
        let mut mock = MockStorage::new();
        mock.expect_store_pokemon()
            .with(eq(pokemon))
            .times(1)
            .returning(|_| Ok(()));

        // given
        let pokemon = Pokemon {
            name: String::from("Pikachu"),
            id: 24usize,
        };

        // when
        let result = create_pokemon(pokemon, &mock);

        // then
        assert!(result.is_ok());
    }
}
