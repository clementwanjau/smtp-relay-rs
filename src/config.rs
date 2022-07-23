use std::fs;
use serde_yaml::Value;

#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) listener: Listener,
    pub(crate) outgoing_smtp: ProxySMTPServer
}

#[derive(Debug, Clone)]
pub struct Listener {
    pub(crate) host: String,
    pub(crate) port: i32
}

#[derive(Debug, Clone)]
pub struct ProxySMTPServer {
    pub(crate) host: String,
    pub(crate) port: i32,
    pub(crate) username: String,
    pub(crate) password: String
}

/// Reads the configuration file and returns a configuration object.
pub fn read_config(file_path: &str) -> Config {
    let file_content = fs::read_to_string(file_path).expect("An error occurred reading the config file.");
    let config_yaml = serde_yaml::from_str::<Value>(file_content.as_str()).unwrap();
    Config {
        listener: Listener { 
            host: config_yaml["listener"]["host"].as_str().unwrap().to_string(), 
            port: config_yaml["listener"]["port"].as_i64().unwrap() as i32
        },
        outgoing_smtp: ProxySMTPServer {
            host: config_yaml["outgoing_smtp"]["host"].as_str().unwrap().to_string(),
            port: config_yaml["outgoing_smtp"]["port"].as_i64().unwrap() as i32,
            username: config_yaml["outgoing_smtp"]["username"].as_str().unwrap().to_string(),
            password: config_yaml["outgoing_smtp"]["password"].as_str().unwrap().to_string()
        }
    }
}