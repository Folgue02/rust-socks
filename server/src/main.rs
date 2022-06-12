use comm_elements::*;
use std::net;
use std::sync::{Arc, Mutex};
use std::thread;

// Modules
mod comm_elements;

const SERVER_ADDR: &str = "127.0.0.1:8080";
const BUFF_SIZE: usize = 1024;
fn main() {
    println!("Server started.");
    let listener = net::TcpListener::bind(SERVER_ADDR).expect("Cannot bind socket.");
    let mut server = Arc::new(Mutex::new(Server::default()));
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let second_server = Arc::clone(&mut server);
        thread::spawn(move || handle_client(second_server, stream));
    }
}
