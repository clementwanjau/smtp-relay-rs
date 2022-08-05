extern crate clap;
extern crate num_cpus;

use clap::{App, Arg};
use log::Level;
use simple_logger::{init_with_level};
use crate::config::read_config;
use crate::server::Server;
use crate::smtp_relay::{Connection, relay_email};

mod smtp_relay;
mod config;
mod server;


#[tokio::main]
pub async fn main() {
    init_with_level(Level::Debug).unwrap();
    let config_path = parse_config_path();
    let config = read_config(config_path.as_str());
    let mut server = Server::new((config.listener.host, config.listener.port));
    server.serve().await;
}

pub fn parse_config_path() -> String {
    let matches = App::new("Rust SMTP Relay server")
        .version("1.0.0")
        .author("Marirs <marirs@gmail.com>")
        .about("Simple SMTP relay server that will relay the received messages.")
        .arg(
            Arg::with_name("config")
                .long("config")
                .short("c")
                .help("The file path to the configuration file.")
                .takes_value(true),
        )
        .get_matches();

    let config_path = matches.value_of("config").unwrap();
    config_path.to_string()
}
