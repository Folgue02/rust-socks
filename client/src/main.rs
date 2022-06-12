use lnpkg;
use msg_templates;
use std::io::{Read, Write};
use std::{io, net, thread};

const SERVER: &str = "127.0.0.1:8080";
const BUFF_SIZE: usize = 1024;

fn main() {
    let mut server = net::TcpStream::connect(SERVER).unwrap();

    let server_clone = server.try_clone().unwrap();
    thread::spawn(move || sender(server_clone));

    // Reading from the tcp stream in a loop
    loop {
        let mut buffer = vec![0; BUFF_SIZE];

        if server.read(&mut buffer).unwrap() == 0 {
            println!("Connection closed.");
            std::process::exit(1);
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.trim().to_string();
    buffer
}

fn sender(mut server: net::TcpStream) {
    loop {
        let message = get_input("SEND ME> ");

        println!("Unlocking guard");

        match server.write(msg_templates::client::msg(message).as_bytes().as_slice()) {
            Ok(_) => println!("Message sent"),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}