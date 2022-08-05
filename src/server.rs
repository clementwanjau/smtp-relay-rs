use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use lettre::SmtpTransport;
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::transport::smtp::PoolConfig;
use log::{debug, error, info};
use crate::{Connection, parse_config_path, read_config, relay_email};

pub struct Server{
    host: String,
    port: u16
}

impl Server{
    pub fn new(address: (String, u16)) -> Self{
        Server {
            host: address.0,
            port: address.1
        }
    }
    
    pub async fn serve(&mut self) {
        let listener_addr = format!("{}:{}", self.host, self.port); 
        let listener = TcpListener::bind(&listener_addr)
            .unwrap_or_else(|e| panic!("Binding to {} failed: {}", &listener_addr, e));
        info!("*** STARTED SERVER ***");
        info!("*** LISTENING ON '{}'...", listener_addr);
        // Handle incoming connections in parallel with workers equal to the number of cores
        // let pool = ThreadPool::new(num_cpus::get());
        for stream_result in listener.incoming() {
            match stream_result {
                Ok(stream) =>{
                    Server::handle_connection(stream.try_clone().unwrap()).await;
                },
                Err(e) => error!("**** UNABLE TO HANDLE CLIENT CONNECTION : {}", e),
            }
        }
    }


    /// Handle a client connection.
    /// If the SMTP communication was successful, print a list of messages on stdout.
    pub async fn handle_connection(mut stream: TcpStream) {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let config_path = parse_config_path();
        let config = read_config(config_path.as_str());
        match Connection::handle(&mut reader, &mut stream) {
            Ok(result) => {
                let listener = format!("{}", config.domain.host);
                let mut smtp = SmtpTransport::starttls_relay(listener.as_str()).unwrap()
                    .credentials(Credentials::new(config.domain.username, config.domain.password))
                    // Configure expected authentication mechanism
                    .authentication(vec![Mechanism::Plain])
                    // Connection pool settings
                    .pool_config(PoolConfig::new().max_size(20))
                    .build();

                debug!("*** SENDER DOMAIN: '{}' ", result.get_sender_domain().unwrap());
                for message in result.get_messages().unwrap() {
                    relay_email(&mut smtp, message).await;
                }
            }
            Err(e) => eprintln!("Error communicating with client: {}", e),
        }
    }
}