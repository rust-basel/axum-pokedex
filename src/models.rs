#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PokemonCreateRequest {
    pub name: String,
    pub id: usize,
}
