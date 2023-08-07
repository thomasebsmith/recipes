use std::net::IpAddr;
use std::path::PathBuf;
use std::{env, fs};

use serde::Deserialize;

/// Configuration related to the database.
#[derive(Deserialize)]
pub struct DatabaseConfig {
    /// The URL to use to connect to the database (e.g.
    /// `"sqlite:///tmp/recipes.db"`).
    pub connection_url: String,

    /// The maximum number of connections to establish with the database.
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

/// Configuration related to the HTTP server.
#[derive(Deserialize)]
pub struct ServerConfig {
    /// The IP address on which to serve the HTTP server.
    pub ip_address: IpAddr,

    /// The port on which to serve the HTTP server.
    pub port: u16,
}

/// Configuration relate to logging.
#[derive(Deserialize)]
pub struct LoggingConfig {
    /// The path of the file to which logs are written.
    pub log_file_path: PathBuf,

    /// The minimum verbosity below which logs are ignored.
    pub verbosity: log::LevelFilter,
}

#[derive(Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub logging: LoggingConfig,
}

pub fn get_config() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        let executable_name =
            if args.is_empty() { "recipes" } else { &args[0] };
        return Err(format!("Usage: {executable_name} <TOML config file>"));
    }

    let config_file_contents = stringify_err(fs::read_to_string(&args[1]))?;
    let config = stringify_err(toml::from_str(&config_file_contents))?;
    Ok(config)
}

fn stringify_err<T, U: ToString>(result: Result<T, U>) -> Result<T, String> {
    result.map_err(|err| err.to_string())
}

fn default_max_connections() -> u32 {
    16
}
