use crate::business_logic::Pokemon;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PokemonCreateRequest {
    pub name: String,
    pub id: usize,
}

#[derive(serde::Deserialize)]
pub struct IdQuery {
    pub id: usize,
}

#[derive(serde::Serialize)]
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
