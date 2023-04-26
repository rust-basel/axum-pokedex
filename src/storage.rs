pub trait Storage {
    fn store_pokemon(pokemon: Pokemon) -> Result<(), Box<dyn Error>>;
    fn get_pokemon(id: usize) -> Result<Pokemon, Box<dyn Error>>;
    fn delete_pokomen(id: usize) -> Result<Pokemon, Box<dyn Error>>;
    fn update_pokemon(pokemon: Pokemon) -> Result<(), Box<dyn Error>>;
}
