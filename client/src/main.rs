mod connect;

use std::io::{Write, Read};

use connect::*;

const SERVER_ADDR: &str = "127.0.0.1:42069";

fn main() -> Result<(), ()> {
    let mut socket = connect_to_server(SERVER_ADDR.to_string())
        .expect("Could not connect");

    println!("Connected!");

    {
        let mut buffer = [0; 8192];
        let mut socket = socket.try_clone().unwrap();
        std::thread::spawn(move || {
            println!("Thread id: {:?}, started a new thread. listening:", std::thread::current().id());
            loop {
                buffer.fill(0);
                match socket.read(&mut buffer) {
                    Ok(0) => {
                        println!("Server disconnected");
                        break;
                    },
                    Ok(_) => {
                        println!("Received: {}", String::from_utf8_lossy(&buffer).trim());
                    },
                    Err(e) => {
                        println!("Failed to read from connection: {}", e);
                        break;
                    },
                }
            }
        });
    }

    let mut input = String::new();
    loop {
        input.clear();
        std::io::stdin().read_line(&mut input).expect("Could not read line");
        input = input.trim().to_string();
        
        match socket.write(input.as_bytes()) {
            Err(_) => {
                println!("Could not send message");
                break;
            },
            _ => (),
        }
    }

    println!("Disconnected");
    Ok(())
}
