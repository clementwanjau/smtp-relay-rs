extern crate clap;
extern crate num_cpus;
extern crate threadpool;

use clap::{App, Arg};
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use lettre::SmtpTransport;
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::transport::smtp::PoolConfig;
use log::{debug, error, info};
use simple_logger::SimpleLogger;
use threadpool::ThreadPool;
use crate::config::read_config;
use crate::smtp_relay::{Connection, relay_email};

mod smtp_relay;
mod config;


fn main() {
    SimpleLogger::new().init().unwrap();
    let config_path = parse_config_path();
    let config = read_config(config_path.as_str());
    let listener = format!("{}:{}", config.listener.host, config.listener.port);
    let listener = TcpListener::bind(&listener)
        .unwrap_or_else(|e| panic!("Binding to {} failed: {}", &listener, e));
    info!("*** STARTED SERVER ***");
    info!("*** LISTENING ON '{}:{}'...", config.listener.host, config.listener.port);
    // Handle incoming connections in parallel with workers equal to the number of cores
    let pool = ThreadPool::new(num_cpus::get());
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => pool.execute(|| {
                handle_connection(stream);
            }),
            Err(e) => error!("**** UNABLE TO HANDLE CLIENT CONNECTION : {}", e),
        }
    }
}

/// Handle a client connection.
/// If the SMTP communication was successful, print a list of messages on stdout.
fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let config_path = parse_config_path();
    let config = read_config(config_path.as_str());
    match Connection::handle(&mut reader, &mut stream) {
        Ok(result) => {
            let listener = format!("{}", config.outgoing_smtp.host);
            let mut smtp = SmtpTransport::starttls_relay(listener.as_str()).unwrap()
                .credentials(Credentials::new(config.outgoing_smtp.username, config.outgoing_smtp.password))
                // Configure expected authentication mechanism
                .authentication(vec![Mechanism::Plain])
                // Connection pool settings
                .pool_config(PoolConfig::new().max_size(20))
                .build();

            debug!("*** SENDER DOMAIN: '{}' ", result.get_sender_domain().unwrap());
            for message in result.get_messages().unwrap() {
                relay_email(&mut smtp, message);
            }
        }
        Err(e) => eprintln!("Error communicating with client: {}", e),
    }
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
