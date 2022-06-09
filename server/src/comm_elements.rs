// TODO: Change the name of this file
use std::{
    collections::HashMap,
    io,
    io::{Read, Write},
    net,
    sync::{Arc, Mutex},
};

pub enum ClientInputError {
    NonValidFormat,
    NoMessageType,
    UnknownMessageType,
    UnknownUser,
}

pub struct Client {
    pub name: String,
    pub stream: net::TcpStream,
}

pub struct Server {
    pub clients: HashMap<u128, Client>,
    last_id: u128,
}

impl Server {
    pub fn default() -> Self {
        Self {
            clients: HashMap::new(),
            last_id: 0,
        }
    }
    pub fn add_client(&mut self, c: Client) -> u128 {
        self.last_id += 1;
        self.clients.insert(self.last_id, c);
        self.last_id
    }

    pub fn broadcast_msg(&mut self, msg: &[u8]) {
        let mut i = self.clients.iter_mut();
        loop {
            if let Some(kp) = i.next() {
                kp.1.stream.write(&msg).unwrap();
            } else {
                break;
            }
        }
    }
    pub fn send_msg(&mut self, client_id: &u128, msg: &[u8]) -> io::Result<usize> {
        if !self.clients.contains_key(client_id) {
            return Err(io::Error::new(io::ErrorKind::AddrNotAvailable, ""));
        } else {
            self.clients.get_mut(client_id).unwrap().stream.write(msg)
        }
    }

    /// Handles the input of the client, and returns a `Result` type containing an `Ok(())` to
    /// represent a success parsing and execution of the client's input, or an `Err(ClientInputError)`
    pub fn handle_client_input(
        &mut self,
        author_ide: i128,
        msg: &[u8],
    ) -> Result<(), ClientInputError> {
        // FIXME: I'm writing this code in the wrong way, I know it, but don't remember how to do it
        // well, :/
        let msg = if let Ok(m) = String::from_utf8(msg.to_vec()) {
            m
        } else {
            return Err(ClientInputError::NonValidFormat);
        };

        let parsed_message = lnpkg::LnPkg::from_string(msg);

        if parsed_message.pkg_type == lnpkg::LnPkgType::Unknown {
            return Err(ClientInputError::NoMessageType);
        }

        match parsed_message.pkg_type {
            lnpkg::LnPkgType::Message => {
                println!("Call message function and broadcast message")
            }
            lnpkg::LnPkgType::DirectMessage => {
                println!("Call direct message function and send message to author")
            }
            lnpkg::LnPkgType::Command => {
                println!("Call command function and execute command")
            }
            lnpkg::LnPkgType::Unknown => return Err(ClientInputError::UnknownMessageType),
        }
    }
}

pub fn handle_client(server: Arc<Mutex<Server>>, mut client_stream: net::TcpStream) {
    let client_id = server.lock().unwrap().add_client(Client {
        name: String::from("User"),
        stream: client_stream.try_clone().unwrap(),
    });
    let client_name = server.lock().unwrap().clients[&client_id].name.clone();
    let mut buffer = vec![0; crate::BUFF_SIZE];
    println!("Thread started for client {}", client_id);
    loop {
        if client_stream.read(&mut buffer).unwrap() == 0 {
            break;
        } else {
            let mut server_guard = server.lock().unwrap();
            server_guard
                .send_msg(&client_id, &buffer)
                .expect("Cannot send message");
            server_guard.broadcast_msg(
                format!(
                    "{}#{}: {}",
                    client_id,
                    client_name,
                    String::from_utf8(buffer).unwrap()
                )
                .as_bytes(),
            );
            buffer = vec![0; crate::BUFF_SIZE];
        }
    }
    eprintln!("Killed thread for client {}.", client_id);
}