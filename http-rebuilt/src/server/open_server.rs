// Install modules
use std::net::{SocketAddr, TcpListener};
use super::handle_connection;

// Constants
const PORT: u16 = 5555;

// Functions
pub fn open_server() {
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], PORT));
    let listener = TcpListener::bind(&addr).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        handle_connection::handle_connection(stream);
    }
}