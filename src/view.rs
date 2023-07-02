// write models
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PokemonCreate {
    pub name: String,
    pub id: usize,
    pub nick_name: String,
    #[serde(rename = "type")]
    pub pokemon_type: String,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct PokemonUpdate {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub nick_name: Option<String>,
    pub pokemon_type: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum PokemonIndexField {
    Name,
    Id,
}

impl Default for PokemonIndexField {
    fn default() -> Self {
        Self::Name
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Direction {
    Ascending,
    Descending,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Ascending
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct PokemonIndexRequest {
    pub sort_field: PokemonIndexField,
    pub sort_direction: Direction,
    pub search: String,
}

// read models
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct PokemonShow {
    pub id: usize,
    pub name: String,
    pub nick_name: String,
    #[serde(rename = "type")]
    pub pokemon_type: String,
}