// TODO: Change the name of this file
use msg_templates;
use std::{
    collections::HashMap,
    io,
    io::{Read, Write},
    net,
    sync::{Arc, Mutex},
};


#[derive(Debug)]
/// Different errors that can occur when elements of the server interact between each other
pub enum ClientInputError {
    /// This variant appears when the message sent by a user has an invalid format
    NonValidFormat,
    /// The lnpkg doesn't contain a type definition
    NoMessageType,
    /// The lnpkg contains a message type identifier that cannot be recognized
    UnknownMessageType,
    /// The user referenced doesn't exist 
    /// (*eg. client `X` tried to send a direcct message to client `Y`, but `Y` doesn't exist*)
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

    pub fn broadcast_msg(&mut self, msg: &[u8]) -> io::Result<()> {
        let mut i = self.clients.iter_mut();
        loop {
            if let Some(kp) = i.next() {
                kp.1.stream.write(&msg)?;
            } else {
                break;
            }
        }
        Ok(())
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
        author_id: u128,
        msg: &[u8],
    ) -> Result<(), ClientInputError> {
        let msg = if let Ok(m) = String::from_utf8(msg.to_vec()) {
            m
        } else {
            return Err(ClientInputError::NonValidFormat);
        };

        println!("comm_elements::Server::handle_client_input> Parsed message: {:?}", &msg);

        let parsed_message = lnpkg::LnPkg::from_string(msg);

        if parsed_message.pkg_type == lnpkg::LnPkgType::Unknown {
            return Err(ClientInputError::NoMessageType);
        }


        let result: io::Result<()> = match parsed_message.pkg_type {
            lnpkg::LnPkgType::Message => {
                let client_message = if let lnpkg::LnPkgValue::String(s) = parsed_message.content[&"msg".to_string()].clone() {
                    s
                } else {
                    return Err(ClientInputError::NonValidFormat)
                };
                //println!("comm_elements::Server::handle_client_input> {}: {:?}", author_id, &parsed_message.content[&"msg".to_string()]);
                //println!("comm_elements::Server::handle_client_input> {:?}", msg_templates::server::msg(author_id, client_message.clone())); 
                self.broadcast_msg(
                    msg_templates::server::msg(author_id, client_message)
                        .as_bytes()
                        .as_slice(),
                ) 
            }
            lnpkg::LnPkgType::DirectMessage => {
                todo!()
            }
            lnpkg::LnPkgType::Command => {
                todo!()
            }
            lnpkg::LnPkgType::Unknown => return Err(ClientInputError::UnknownMessageType),
        };

        // Return ClientInputError depending on the result of the message handling
        if let Err(e) = result {
            println!("{:?}", e);
            Err(ClientInputError::NoMessageType) // TODO: Provide different errors depending on the situation
        } else {
            Ok(())
        }
    }
}

/// Handles the incoming events from the client
pub fn handle_client(server: Arc<Mutex<Server>>, mut client_stream: net::TcpStream) {
    let client_id = server.lock().unwrap().add_client(Client {
        name: String::from("Generic User name"),
        stream: client_stream.try_clone().unwrap(),
    });
    let client_name = server.lock().unwrap().clients[&client_id].name.clone();
    let mut buffer = vec![0; crate::BUFF_SIZE];
    println!("Thread started for client {}", client_id);
    loop {
        if client_stream.read(&mut buffer).unwrap() == 0 {
            // Empty packet (Connection closed)
            break;
        } else {
            let mut server_guard = server.lock().unwrap();

            // Remove NUL characters
            buffer = buffer.into_iter().filter(|c|  *c != 0).collect();
            match server_guard.handle_client_input(client_id, &buffer) {
                Ok(_) => (),
                Err(e) => println!("Error when handling message: {:?}", e)
            };
            buffer = vec![0; crate::BUFF_SIZE];
        }
    }
    eprintln!("Killed thread for client {}.", client_id);
}