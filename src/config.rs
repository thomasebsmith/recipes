use std::net::IpAddr;
use std::path::PathBuf;
use std::{env, fs};

use serde::Deserialize;

use crate::util::stringify_err;

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

/// A model for the web server's configuration.
#[derive(Deserialize)]
pub struct Config {
    /// The server's database-related configuration.
    pub database: DatabaseConfig,

    /// The server's HTTP/hosting-related configuration.
    pub server: ServerConfig,

    /// The server's logging-related configuration.
    pub logging: LoggingConfig,
}

/// Attempts to retrieve the configuration based on the file name in this
/// program's arguments (retrieved from the global program environment).
///
/// Returns the retrieved configuration or a string describing the error.
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

fn default_max_connections() -> u32 {
    16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let toml = "
            [database]
            connection_url = \"sqlite:///path/to/file.db\"

            [server]
            ip_address = \"1.2.3.4\"
            port = 5678

            [logging]
            log_file_path = \"/path/to/file.log\"
            verbosity = \"debug\"
        ";

        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.database.connection_url, "sqlite:///path/to/file.db");
        assert_eq!(config.database.max_connections, default_max_connections());
        assert_eq!(
            config.server.ip_address,
            "1.2.3.4".parse::<IpAddr>().unwrap()
        );
        assert_eq!(config.server.port, 5678);
        assert_eq!(
            config.logging.log_file_path,
            PathBuf::from("/path/to/file.log")
        );
        assert_eq!(config.logging.verbosity, log::LevelFilter::Debug);
    }

    #[test]
    fn test_all_specified() {
        let toml = "
            [database]
            connection_url = \"sqlite:///database-file.db\"
            max_connections = 9876

            [server]
            ip_address = \"0.0.0.0\"
            port = 80

            [logging]
            log_file_path = \"/log-file.log\"
            verbosity = \"warn\"
        ";

        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(
            config.database.connection_url,
            "sqlite:///database-file.db"
        );
        assert_eq!(config.database.max_connections, 9876);
        assert_eq!(
            config.server.ip_address,
            "0.0.0.0".parse::<IpAddr>().unwrap()
        );
        assert_eq!(config.server.port, 80);
        assert_eq!(
            config.logging.log_file_path,
            PathBuf::from("/log-file.log")
        );
        assert_eq!(config.logging.verbosity, log::LevelFilter::Warn);
    }
}
