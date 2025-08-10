# Transpaer

This repository defines the public API for the Transpaer project for Rust programming language.

More on the Transpaer project on its website: [transpaer.app](https://transpaer.app/).

The API definition itself can be found in [a separate repositiory](https://github.com/transpaer/transpaer-api).

# For manintainers

## Updates

The code here is auto-generated from OpenAPI API definition.

To update the code:

1. Install tools:

  - `openapi-generator`,

  - `cargo-typify` - JSON schema code generator (`cargo install cargo-typify`).

  - `yj` - YAML to JSON converter (`cargo install yj`) 

2. Copy the updated API definition from [its repository](https://github.com/transpaer/transpaer-api) to `openapi` directory.

3. Run `sh ./scripts/generate.sh`.
