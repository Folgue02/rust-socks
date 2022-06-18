use lnpkg;
use std::collections::HashMap;
pub type Lpv = lnpkg::LnPkgValue; // LakenetPackageValue
pub type Lpty = lnpkg::LnPkgType; // LakeNetPackageType
pub type Lnp = lnpkg::LnPkg; // LakeNetPackage

/// Message templates used by the client
pub mod client {
    use super::*;

    /// Message that request the server to return a `server::self_identity` to the client
    pub fn selfid_request(client_id: lnpkg::ClientId, client_name: String) -> Lnp {
        let mut hm = HashMap::new();
        hm.insert("id".to_string(), Lpv::Int(client_id));
        hm.insert("name".to_string(), Lpv::String(client_name));
        Lnp::new(Lpty::SelfIdentity)
    }

    /// Request for the identity of the client associated with the ID specified.
    pub fn id_request(client_id: lnpkg::ClientId, client_name: String) -> Lnp {
        let mut hm = HashMap::new();
        hm.insert("id".to_string(), Lpv::Int(client_id));
        hm.insert("name".to_string(), Lpv::String(client_name));
        Lnp::from_hashmap(hm, Lpty::Identity)
    }

    /// Message **sent by the client** to the server, with the purpose of
    /// being broadcasted to the rest of clients
    pub fn msg(msg: String) -> Lnp {
        let mut hm = HashMap::new();
        hm.insert("msg".to_string(), Lpv::String(msg));
        lnpkg::LnPkg::from_hashmap(hm, Lpty::Message)
    }

    /// Message directly **sent to the server**, which will be redirected to the
    /// specified client by the server without being sent to any other client.
    pub fn direct_message(client_id: lnpkg::ClientId, msg: String) -> Lnp {
        let mut hm = HashMap::new();
        hm.insert("id".to_string(), Lpv::Int(client_id));
        hm.insert("msg".to_string(), Lpv::String(msg));
        lnpkg::LnPkg::from_hashmap(hm, Lpty::DirectMessage)
    }

    /// Message **sent by the client**, requesting the **server** to do an operation
    /// which can result in success or error
    pub fn command(command: String, arguments: Vec<String>) -> Lnp {
        let mut hm = HashMap::new();
        hm.insert("command".to_string(), Lpv::String(command));
        hm.insert("args".to_string(), Lpv::List(arguments));

        lnpkg::LnPkg::from_hashmap(hm, Lpty::Command)
    }
}

/// Message templates used by the server
pub mod server {
    use super::*;
    /// Message **sent by client**, broadcasted by the server to the
    /// rest of clients connected. <br>*Side note: The `msg` parameter only refers to
    /// the string that the client wants the other clients to see, not the `lnpkg` string.*
    pub fn msg(client_id: lnpkg::ClientId, msg: String) -> Lnp {
        let mut hm = HashMap::new();
        hm.insert("client".to_string(), Lpv::Int(client_id as i128));
        hm.insert("msg".to_string(), Lpv::String(msg));
        Lnp::from_hashmap(hm, Lpty::Message)
    }

    /// Message sent back to the client to give self awareness of its identity.
    pub fn self_identity(client_id: lnpkg::ClientId, client_name: String) -> Lnp {
        let mut hm = HashMap::new();
        hm.insert("id".to_string(), Lpv::Int(client_id));
        hm.insert("name".to_string(), Lpv::String(client_name));
        Lnp::from_hashmap(hm, Lpty::SelfIdentity)
    }

    /// Contains the identity of a client specified
    pub fn identity(client_id: lnpkg::ClientId, client_name: String) -> Lnp {
        let mut hm = HashMap::new();
        hm.insert("id".to_string(), Lpv::Int(client_id));
        hm.insert("name".to_string(), Lpv::String(client_name));
        Lnp::from_hashmap(hm, Lpty::Identity)
    }

    /// Message that contains a list of all the clients
    pub fn list_clients(_: HashMap<lnpkg::ClientId, String>) -> Lnp {
        todo!("Create format for the list_clients package")
    }

    pub fn event_client_connected(client_id: lnpkg::ClientId, client_name: String) -> Lnp {
        let mut hm = HashMap::new();
        hm.insert("id".to_string(), Lpv::Int(client_id));
        hm.insert("name".to_string(), Lpv::String(client_name));
        Lnp::from_hashmap(hm, Lpty::EventClientConnected)
    }

    pub fn event_client_left(client_id: lnpkg::ClientId, client_name: String) -> Lnp {
        let mut hm = HashMap::new();
        hm.insert("id".to_string(), Lpv::Int(client_id));
        hm.insert("name".to_string(), Lpv::String(client_name));
        Lnp::from_hashmap(hm, Lpty::EventClientLeft)
    }
}

/// Message templates used by both the server and client
pub mod shared {
    #[allow(unused_imports)]
    use super::*;
}
