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
    /// The resource requested by the client (*such as communication with another client*) is not
    /// available for the client, or not available anymore
    ResourceNotAvailable,
    /// The client has requested the execution of an unknown command.
    UnknownCommand,
    /// The client has issued a command in a non valid way (*not enough arguments, string instead of int...*)
    NonValidCommandUsage,
    /// An error occurred in the server internals functioning.
    InternalServerError,
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
            println!(
                "server::comm_elements::Server::handle_client> Non utf-8 valid format: {:?}",
                msg
            );
            return Err(ClientInputError::NonValidFormat);
        };

        let parsed_message = lnpkg::LnPkg::from_string(&msg);

        // Return error if the type of the message its unknown
        if parsed_message.pkg_type == lnpkg::LnPkgType::Unknown {
            println!("Client sent message without type, '{:?}'", parsed_message);
            return Err(ClientInputError::NoMessageType);
        }

        let result: Result<(), ClientInputError> = match parsed_message.pkg_type {
            lnpkg::LnPkgType::Message => {
                if !parsed_message.exist(&["msg"]) {
                    eprintln!(
                        "The `Message` didn't contain the `msg` key. {:?}",
                        parsed_message.content
                    );
                    return Err(ClientInputError::NonValidFormat);
                }

                self.broadcast_msg(
                    msg_templates::server::msg(
                        author_id,
                        parsed_message.content["msg"].to_string(),
                    )
                    .as_bytes()
                    .as_slice(),
                ).unwrap(); // TODO: Give this better error handling
                Ok(())
            }
            lnpkg::LnPkgType::DirectMessage => {
                // Check the integrity of the message
                if !parsed_message.exist(&["msg", "id"]) {
                    println!(
                        "Direct message has incorrect format: {:?}",
                        parsed_message.content
                    );
                    return Err(ClientInputError::NonValidFormat);
                }

                let destination_id: lnpkg::ClientId = match parsed_message.content["id"] {
                    lnpkg::LnPkgValue::Int(i) => i,
                    _ => {
                        println!("The destination id provided by the client for the direct message wasn't an integer. {:?}", parsed_message.content["id"]);
                        return Err(ClientInputError::NonValidFormat);
                    }
                };

                // Check for errors
                match self.send_msg(
                    &destination_id,
                    parsed_message.content["msg"].to_string().as_bytes(),
                ) {
                    Err(e) => {
                        match e.kind() {
                            std::io::ErrorKind::AddrNotAvailable => {
                                // Message sent to client that's not connected anymore
                                eprintln!("A direct message was tried to be sent to {}, but the client wasn't connected anymore", destination_id);
                                return Err(ClientInputError::ResourceNotAvailable);
                            }
                            _ => {
                                // Unknown error
                                eprintln!("An error has occurred when sending a direct message (to {}): {:?}", destination_id, e);
                                return Err(ClientInputError::InternalServerError);
                            }
                        }
                    }
                    Ok(_) => (),
                };

                Ok(())
            }
            lnpkg::LnPkgType::Command => {
                // Check package vality
                if !parsed_message.exist(&["args", "command"]) {
                    return Err(ClientInputError::NonValidFormat);
                }

                let arguments: Vec<String>;
                if let lnpkg::LnPkgValue::List(l) = &parsed_message.content["args"] {
                    arguments = l.into_iter().map(|x| x.to_owned()).collect();
                } else if let lnpkg::LnPkgValue::String(s) = &parsed_message.content["args"] {
                    arguments = vec![s.clone()];
                } else {
                    eprintln!("Command didn't have a list or a string as arguments");
                    return Err(ClientInputError::NonValidFormat);
                }

                return self.execute_client_command(
                    author_id,
                    parsed_message.content["command"].to_string(),
                    arguments,
                );
            }
            lnpkg::LnPkgType::SelfIdentity => {
                let template = msg_templates::server::self_identity(author_id, self.clients[&author_id].name.clone());
                self.send_msg(&author_id, template.as_bytes().as_slice()).unwrap(); // TODO: Give this better error handling
                Ok(())
            }
            lnpkg::LnPkgType::Identity => {
                todo!()
            }
            lnpkg::LnPkgType::Unknown | _ => {
                eprintln!("Message with unknown lnpkg type.");
                return Err(ClientInputError::UnknownMessageType);
            }
        };

        // Return ClientInputError depending on the result of the message handling
        if let Err(e) = result {
            println!("{:?}", e);
            return Err(e);
        } else {
            Ok(())
        }
    }

    /// Disconnects an specific client from the server and removes it from the `self.clients` hashmap
    pub fn disconnect_client(&mut self, client_id: lnpkg::ClientId) -> Result<(), ClientInputError> {
        if !self.clients.contains_key(&client_id) {
            return Err(ClientInputError::UnknownUser);
        } else {
            let _ = self.clients.remove_entry(&client_id);
            return Ok(());
        }
    }

    /// Executes commands sent by the client that might take changes on the server
    pub fn execute_client_command(
        &mut self,
        client_id: lnpkg::ClientId,
        command: String,
        arguments: Vec<String>,
    ) -> Result<(), ClientInputError> {
        println!("Executing command {} sent by {}", command, client_id);

        let command = command.as_str();
        return match command {
            "chnick" => {
                if let Some(new_name) = arguments.get(0) {
                    self.change_name(client_id, new_name.to_string())?
                } else {
                    return Err(ClientInputError::NonValidCommandUsage)
                }
                Ok(())
            }
            "whoami" => {
                let template = msg_templates::server::self_identity(client_id, self.clients[&client_id].name.clone());
                self.send_msg(&client_id, template.as_bytes().as_slice()).unwrap(); // TODO: Give this better error handling
                Ok(())
            }
            _ => return Err(ClientInputError::UnknownCommand)
        }
    }

    /// Changes the name of the client specified
    pub fn change_name(
        &mut self,
        target_id: lnpkg::ClientId,
        new_name: String,
    ) -> Result<(), ClientInputError> {
        if !self.clients.contains_key(&target_id) {
            return Err(ClientInputError::UnknownUser);
        } else {
            let cl_obj = self.clients.get_mut(&target_id).unwrap();
            cl_obj.name = new_name;
        }
        Ok(())
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
    client_stream
        .write(
            msg_templates::server::identity(client_id, client_name.clone())
                .as_bytes()
                .as_slice(),
        )
        .unwrap();
    // Send event msg
    server
        .lock()
        .unwrap()
        .broadcast_msg(
            msg_templates::server::event_client_connected(client_id, client_name.clone())
                .as_bytes()
                .as_slice(),
        )
        .unwrap();

    // Mainloop
    let mut buffer = vec![0; crate::BUFF_SIZE];
    loop {
        if client_stream.read(&mut buffer).unwrap() == 0 {
            // Empty packet (Connection closed)
            break;
        } else {
            let mut server_guard = server.lock().unwrap();

            // Remove NUL characters
            buffer = buffer.into_iter().filter(|c| *c != 0).collect();
            let result = server_guard.handle_client_input(client_id, &buffer);
            std::mem::drop(server_guard); // Avoid double locking
            match result {
                Ok(_) => (),
                Err(e) => {
                    // Act according the type of error (do not disconnect in certain cases)
                    println!("Error when handling message: {:?}", e);
                    let mut server_guard = server.lock().unwrap();
                    server_guard.disconnect_client(client_id).unwrap();
                    server_guard
                        .broadcast_msg(
                            msg_templates::server::event_client_left(client_id, client_name)
                                .as_bytes()
                                .as_slice(),
                        )
                        .unwrap();
                    println!("Client ({}) disconnected from the server.", client_id);
                    break;
                }
            };
            buffer = vec![0; crate::BUFF_SIZE];
        }
    }
    eprintln!("Killed thread for client {}.", client_id);
}
