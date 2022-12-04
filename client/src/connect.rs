use std::{net::TcpStream, io::Error};

pub fn connect_to_server (addr: String) -> Result<TcpStream, Error> {
    println!("Trying to connect: {}", addr);

    TcpStream::connect(addr)
}