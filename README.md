# Recipes
**Recipes** is a recipe book web server. It's written in Rust using Axum and
SQLx.

It was originally created by Thomas Smith.

## Prerequisites
- A recent, stable Rust toolchain
- sqlite3

## Quick Start
```sh
$ sqlite3 /tmp/recipes.db
  > .read setup/create_tables.sql
  > ^D
$ cargo run resources/sample-config.toml
# A server will listen on localhost port 8000 until it's killed
```

## License
This project is available under the MIT License. See [LICENSE](./LICENSE) for
details.

## Copyright
This project is copyright © 2023 Thomas Smith.
