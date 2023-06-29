use serde::Deserialize;
use std::net::IpAddr;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub connection_url: String,

    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub ip_address: IpAddr,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub log_file_path: PathBuf,
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
        Err(format!("Usage: {} <TOML config file>", executable_name))
    } else {
        let config_file_contents = stringify_err(fs::read_to_string(&args[1]))?;
        let config = stringify_err(toml::from_str(&config_file_contents))?;
        Ok(config)
    }
}

fn stringify_err<T, U: ToString>(result: Result<T, U>) -> Result<T, String> {
    result.map_err(|err| err.to_string())
}

fn default_max_connections() -> u32 {
    16
}
