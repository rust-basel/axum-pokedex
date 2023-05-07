use crate::business_logic::Pokemon;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PokemonCreateRequest {
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
