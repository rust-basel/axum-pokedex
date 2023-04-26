#[derive(serde::Deserialize, Debug)]
pub struct PokemonCreateRequest {
    name: String,
    id: usize,
}
