# Fruit Jokes Microservice

This microservice, built in Rust, serves random fruit jokes to users. Users can also contribute their own jokes by sending a `POST` request.

## Features

- **Add Jokes**: Send a `POST` request with your joke, and it'll be added to the collection of fruit jokes.
- **Retrieve Random Jokes**: A `GET` request to the service returns a random fruit joke.

## Dependencies

This microservice leverages the following wasmCloud capabilities:

- `wasmbus_rpc::actor::prelude::*` for general Actor support.
- `wasmcloud_interface_httpserver` to serve the jokes via HTTP.
- `wasmcloud_interface_keyvalue` for storing the jokes in a set.
- `wasmcloud_interface_numbergen` to generate random numbers to retrieve random jokes.

## Endpoints

- `POST /`: Add a new joke to the collection. Ensure the body contains the joke text.
- `GET /`: Retrieve a random fruit joke from the collection.

## Constants

- `JOKES_KEY`: This key is used to store the jokes in a set.
- `FALLBACK_JOKE`: In case a joke isn't found, this fallback joke will be returned.

## Error Handling

If the sent joke in a `POST` request cannot be parsed, the microservice responds with a cheeky error message: "That joke was bad, I'm not even going to store it".

## Usage

Deploy this microservice with wasmCloud, and it's ready to serve jokes and collect new ones.

## Contributing

Feel free to submit pull requests or raise issues if you find any. Every joke is appreciated, but keep them fun and light-hearted!
