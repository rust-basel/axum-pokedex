use crate::business_logic::Pokemon;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PokemonCreateRequest {
    pub name: String,
    pub id: usize,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PokemonUpdateRequest {
    pub name: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct PokemonGetResponse {
    pub name: String,
    pub id: usize,
}

impl From<PokemonCreateRequest> for Pokemon {
    fn from(value: PokemonCreateRequest) -> Self {
        Pokemon {
            name: value.name,
            id: value.id,
        }
    }
}

impl From<Pokemon> for PokemonGetResponse {
    fn from(p: Pokemon) -> Self {
        PokemonGetResponse {
            name: p.name,
            id: p.id,
        }
    }
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

#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
pub struct PokemonIndexRequest {
    pub sort_field: PokemonIndexField,
    pub sort_direction: Direction,
    pub search: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PokemonList {
    pub name: String,
    pub id: usize,
}

#[cfg(test)]
mod tests {
    use crate::business_logic::Pokemon;

    use super::PokemonCreateRequest;

    #[test]
    fn from_request_to_entity() {
        // given
        let req = PokemonCreateRequest {
            name: "name".to_string(),
            id: 1,
        };

        // when
        let res: Pokemon = req.into();

        // then
        assert_eq!(
            res,
            Pokemon {
                name: "name".to_string(),
                id: 1
            }
        );
    }
}
