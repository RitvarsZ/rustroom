mod connect;

use std::io::Write;

use connect::*;

const SERVER_ADDR: &str = "127.0.0.1:42069";

fn main() -> Result<(), ()> {
    let mut socket = connect_to_server(SERVER_ADDR.to_string())
        .expect("Could not connect");

    println!("Connected!");

    socket.write("Hello?".as_bytes()).unwrap();

    Ok(())
}