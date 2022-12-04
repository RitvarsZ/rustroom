use std::{net::{TcpListener, TcpStream}, io::Read};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server {
            listener: TcpListener::bind(address).unwrap()
        }
    }

    pub fn run(&self) {
        println!("Accepting connections on: {}", self.listener.local_addr().unwrap());
        
        for stream in self.listener.incoming() {
            match stream {
                Ok(s) => self.handle_client(s),
                Err(e) => panic!("encountered IO error: {e}"),
            }
        }
    }

    fn handle_client(&self, mut stream: TcpStream) {
        println!("New client connected: {}", stream.peer_addr().unwrap());

        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {
                println!("Received: {}", String::from_utf8_lossy(&buffer));
            },
            Err(e) => println!("Failed to read from connection: {}", e),
        }
    }
}