# Recipes
**Recipes** is a web server for creating, viewing, and editing your recipes.
It is written in Rust, and it uses SQLite (via SQLx) and Axum internally.

It is designed to have good performance in limited-resource deployments, such as
on Raspberry Pis.

It was created and is developed by Thomas Smith.

## Prerequisites
- A recent nightly Rust toolchain
- sqlite3 (if desired to initialize the database)

## Quick Start
To start the server, run these commands in a Unix-like environment:
```sh
$ sqlite3 /tmp/recipes.db < setup/create_tables.sql
$ sqlite3 /tmp/recipes.db < setup/sample_data.sql
$ cargo run resources/sample-config.toml
# A server will listen on localhost port 8000 until it's killed
# Log messages will go to /tmp/recipes.log
```

To run code checks (compilation, formatting, linting, and tests), run the
`check` script:
```sh
$ ./scripts/check
    Checking recipes v0.1.0
    ...
```

## Project Status
This project is currently in early development. It is not stable. SQL schemas,
code layout, etc. are subject to arbitrary changes.

## Roadmap
### v0.1 [In Progress]
- API to view ingredients and recipes [complete]
- API to create new ingredients and recipes [complete]
- API to create new versions of recipes
- Basic database optimizations (low query count, ID cache)
- Validation of all API endpoints

### v0.2
- HTML UI for viewing, creating, and editing recipes and ingredients
- Basic CSS styling for the HTML UI

### v0.3
- API and UI for searching for recipes and ingredients by name, composition,
  time, etc.
- SQLite-specific database optimizations

### v0.4
- Ability to record and view recipe attempts with notes
- Complete API documentation
- Visual design cleanup
- Testing on a Raspberry Pi

### v0.5
- Admin interface with server status
- Traffic statistics (optionally collected)
- Database backup integration

### v0.6
- Ability to track current food supplies and determine possible recipes

## License
This project is available as open-source software under the MIT License. See
[LICENSE](./LICENSE) for details.

## Contributing
To contribute to this repository, please find an existing issue or create one
that follows the project roadmap. Then, post on the issue that you intend to
work on it. Finally, create a PR to `master` that resolves the issue.

All contributed work will be incorporated under the terms of the MIT License.

## Copyright
This project is copyright © 2023-2024 Thomas Smith.
