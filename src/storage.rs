#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::business_logic::Pokemon;

#[derive(Debug, PartialEq)]
pub enum StorageError {
    NotFound,
}

#[cfg_attr(test, automock)]
pub trait Storage {
    fn store_pokemon(&mut self, pokemon: Pokemon) -> Result<(), StorageError>;
    fn get_pokemon(&self, id: usize) -> Result<Pokemon, StorageError>;
    fn delete_pokemon(&mut self, id: usize) -> Result<(), StorageError>;
    fn update_pokemon(&mut self, pokemon: Pokemon) -> Result<(), StorageError>;
}
