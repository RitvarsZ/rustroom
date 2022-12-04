mod server;

use server::Server;

fn main() -> Result<(), ()> {
    let server = Server::new("127.0.0.1:42069".to_string());
    server.run();

    Ok(())
}
