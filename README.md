# Recipes
**Recipes** is a recipe book web server. It is written in Rust, and it uses SQLite (via SQLx) and Axum.

It is designed for limited-resource deployment, such as on a Raspberry Pi.

It was originally created by Thomas Smith.

## Prerequisites
- A recent nightly Rust toolchain
- sqlite3 (to initialize the database)

## Quick Start
Run these commands in a Unix-like environment:
```sh
$ sqlite3 /tmp/recipes.db < setup/create_tables.sql
$ sqlite3 /tmp/recipes.db < setup/sample_data.sql
$ cargo run resources/sample-config.toml
# A server will listen on localhost port 8000 until it's killed
# Log messages will go to /tmp/recipes.log
```

## Project Status
This project is currently in early development. It is not stable. SQL schemas,
code layout, etc. are subject to arbitrary changes.

## Roadmap
### v0.1 (In Progress)
- API to view ingredients and recipes
- API to create new ingredients and recipes
- API to edit recipes (i.e. create new versions)
- Basic database optimizations (minimal query count, ID cache)

### v0.2
- HTML UI for viewing, creating, and editing recipes and ingredients

### v0.3
- API and UI for searching for recipes and ingredients by name, composition,
  time, etc.
- SQLite-specific database optimizations

### v0.4
- Ability to record recipe attempts with notes
- API documentation
- Testing on a Raspberry Pi

### v0.5
- Admin interface with server status
- Traffic statistics
- Database backup integration

### v0.6
- Ability to track current food supplies and determine possible recipes

## License
This project is available under the MIT License. See [LICENSE](./LICENSE) for
details.

## Copyright
This project is copyright © 2023 Thomas Smith.
