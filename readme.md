# Pokédex

This is a possible solution to the [challenge](https://github.com/rust-basel/axum-pokedex-starter)
from our [workshop meetup#8](https://github.com/rust-basel/rust-meetup-8).

# Tests

Integration tests were written, to check if the endpoints do, what they are promised to do.
You could also use the [postman collection](https://github.com/rust-basel/axum-pokedex-starter/blob/main/Pokedex.postman_collection.json) 
from the [starter template](https://github.com/rust-basel/axum-pokedex-starter).

## Integration Tests

To execute the tests, run:

```sh
cargo test
```

## With Postman

If you like to use the postman collection, you have to run
this server first with:

```sh
cargo run
```

Afterwards you can use the postman collection.