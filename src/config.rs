use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub(crate) listener: Listener,
    pub(crate) domain: ProxySMTPServer
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listener {
    pub(crate) host: String,
    pub(crate) port: u16
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxySMTPServer {
    pub(crate) host: String,
    pub(crate) username: String,
    pub(crate) password: String
}

/// Reads the configuration file and returns a configuration object.
pub fn read_config(file_path: &str) -> Config {
    let file_content = fs::read_to_string(file_path).expect("An error occurred reading the config file.");
    let config_yaml = serde_yaml::from_str::<Config>(file_content.as_str()).unwrap();
    config_yaml
}