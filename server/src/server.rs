use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, sync::{Mutex, Arc}};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
    clients: Arc<Mutex<Vec<TcpStream>>>,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server {
            listener: TcpListener::bind(address).unwrap(),
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn run(&self) {
        println!("Accepting connections on: {}", self.listener.local_addr().unwrap());
        
        for stream in self.listener.incoming() {
            match stream {
                Ok(s) => {
                    self.clients.lock().unwrap().push(s.try_clone().unwrap());
                    let clients = self.clients.clone();
                    std::thread::spawn(move || {
                        handle_client(s, clients);
                    });
                }
                Err(e) => panic!("encountered IO error: {e}"),
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    println!("New client connected: {}", stream.peer_addr().unwrap());
    println!("Thread id: {:?}", std::thread::current().id());

    let mut buffer = [0; 8192];
    loop {
        buffer.fill(0);
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client {} disconnected", stream.peer_addr().unwrap());
                break;
            },
            Ok(_) => {
                println!("Received: {}", String::from_utf8_lossy(&buffer).trim());
                clients.lock().unwrap().iter_mut().for_each(|client| {
                    println!("Sending to: {}", client.peer_addr().unwrap());
                    client.write(&buffer).unwrap();
                });
            },
            Err(e) => {
                println!("Failed to read from connection: {}", e);
                break;
            },
        }
    }
}
