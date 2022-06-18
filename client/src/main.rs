use lnpkg;
use msg_templates;
use std::io::{Read, Write};
use std::{io, net, thread};

mod command;
mod syntax;
#[cfg(test)]
mod test; // TODO: Pass this to `/tests/` folder at the root of the project

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
        } else {
            println!("Message received: {}", String::from_utf8(buffer).unwrap());
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

        // Command
        if message.starts_with(":") {
            let input = syntax::Input::from_string(message[1..].to_string());
            println!("SENDING RAW MESSAGE: {:?}", &input);
            let template = msg_templates::client::command(input.command, input.arguments);
            println!("SENDING COMMAND: {:?}", &template.to_string());
            match server.write(template.as_bytes().as_slice()) {
                Ok(_) => println!("Command sent"),
                Err(e) => eprintln!("Error sending command: {:?}", e),
            };
        } else {
            match server.write(msg_templates::client::msg(message).as_bytes().as_slice()) {
                Ok(_) => println!("Messsage sent"),
                Err(e) => eprintln!("Error occurred: {:?}", e),
            }
        }
    }
}
