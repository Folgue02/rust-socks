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
    pub clients: HashMap<lnpkg::ClientId, Client>,
    last_id: lnpkg::ClientId,
}

impl Server {
    pub fn default() -> Self {
        Self {
            clients: HashMap::new(),
            last_id: 0,
        }
    }
    pub fn add_client(&mut self, c: Client) -> lnpkg::ClientId {
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
    pub fn send_msg(&mut self, client_id: &lnpkg::ClientId, msg: &[u8]) -> io::Result<usize> {
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
        author_id: lnpkg::ClientId,
        msg: &[u8],
    ) -> Result<(), ClientInputError> {
        let msg = if let Ok(m) = String::from_utf8(msg.to_vec()) {
            m
        } else {
            return Err(ClientInputError::NonValidFormat);
        };

        let parsed_message = lnpkg::LnPkg::from_string(msg);

        // Return error if the type of the message its unknown
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
            lnpkg::LnPkgType::Unknown | _ => return Err(ClientInputError::UnknownMessageType),
        };

        // Return ClientInputError depending on the result of the message handling
        if let Err(e) = result {
            println!("{:?}", e);
            Err(ClientInputError::NoMessageType) // TODO: Provide different errors depending on the situation
        } else {
            Ok(())
        }
    }

    pub fn disconnect_client(&mut self, client_id: lnpkg::ClientId) -> Option<()> { // TODO: Give this better error handling
        if !self.clients.contains_key(&client_id) {
            return None 
        } else {
            let _ = self.clients.remove_entry(&client_id);
            return Some(())
        }
    }
}

/// Handles the incoming events from the client
pub fn handle_client(server: Arc<Mutex<Server>>, mut client_stream: net::TcpStream) {

    // Define the user
    let client_name = String::from("Generic user name");
    let client_id = server.lock().unwrap().add_client(Client {
        name: client_name.clone(),
        stream: client_stream.try_clone().unwrap(),
    });
    println!("Thread started for client {}", client_id);
    // Send identity msg
    client_stream.write(msg_templates::server::identity(client_id, client_name.clone()).as_bytes().as_slice()).unwrap();
    // Send event msg
    server.lock().unwrap().broadcast_msg(msg_templates::server::event_client_connected(client_id, client_name.clone()).as_bytes().as_slice()).unwrap(); 

    // Mainloop
    let mut buffer = vec![0; crate::BUFF_SIZE];
    loop {
        if client_stream.read(&mut buffer).unwrap() == 0 {
            // Empty packet (Connection closed)
            break;
        } else {
            let mut server_guard = server.lock().unwrap();

            // Remove NUL characters
            buffer = buffer.into_iter().filter(|c|  *c != 0).collect();
            let result = server_guard.handle_client_input(client_id, &buffer);
            drop(server_guard); // Avoid double locking 
            match  result {
                Ok(_) => (),
                Err(e) => {
                    println!("Error when handling message: {:?}", e); // BUG: Stuck here when disconnecting user
                    let mut server_guard = server.lock().unwrap();
                    server_guard.disconnect_client(client_id);
                    server_guard.broadcast_msg(msg_templates::server::event_client_left(client_id, client_name).as_bytes().as_slice()).unwrap();
                    println!("Client ({}) disconnected from the server.", client_id);
                    break
                }
            };
            buffer = vec![0; crate::BUFF_SIZE];
        }
    }
    eprintln!("Killed thread for client {}.", client_id);
}