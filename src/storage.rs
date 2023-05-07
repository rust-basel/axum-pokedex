#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::business_logic::Pokemon;

pub enum StorageError {
    NotFound,
}

#[cfg_attr(test, automock)]
pub trait Storage {
    fn store_pokemon(&mut self, pokemon: Pokemon) -> Result<(), StorageError>;
    fn get_pokemon(&self, id: usize) -> Result<Pokemon, StorageError>;
    fn delete_pokomen(&self, id: usize) -> Result<Pokemon, StorageError>;
    fn update_pokemon(&self, pokemon: Pokemon) -> Result<(), StorageError>;
}
