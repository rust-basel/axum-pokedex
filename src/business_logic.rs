use std::error::Error;

struct Pokemon {
    name: String,
    id: usize,
}

fn create_pokemon(pokemon: Pokemon) -> Result<(), Box<dyn Error>> {
    todo!()
}

fn update_pokemon(pokemon: Pokemon) -> Result<(), Box<dyn Error>> {
    todo!()
}

fn delete_pokomen(id: usize) -> Result<(), Box<dyn Error>> {
    todo!()
}

fn get_pokemon(id: usize) -> Result<(), Box<dyn Error>> {
    todo!()
}
