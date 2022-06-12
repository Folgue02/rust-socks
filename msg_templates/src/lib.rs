use lnpkg;
use std::collections::HashMap;
pub type lpv = lnpkg::LnPkgValue;   // LakenetPackageValue
pub type lpty = lnpkg::LnPkgType;   // LakeNetPackageType
pub type lnp = lnpkg::LnPkg;        // LakeNetPackage


/// Message templates used by the client
pub mod client {
    use super::*;

    /// Message that request the server to return a `server::self_identity` to the client
    pub fn selfid_request(client_id: lnpkg::ClientId, client_name: String) -> lnp {
        let mut hm = HashMap::new();
        hm.insert("id".to_string(), lpv::Int(client_id));
        hm.insert("name".to_string(), lpv::String(client_name));
        lnp::new(lpty::SelfIdentity)
    }

    /// Request for the identity of the client associated with the ID specified.
    pub fn id_request(client_id: lnpkg::ClientId, client_name: String) -> lnp {
        let mut hm = HashMap::new();
        hm.insert("id".to_string(), lpv::Int(client_id));
        hm.insert("name".to_string(), lpv::String(client_name));
        lnp::from_hashmap(hm, lpty::Identity)
    }

    /// Message **sent by the client** to the server, with the purpose of
    /// being broadcasted to the rest of clients 
    pub fn msg(msg: String) -> lnp {
        let mut hm = HashMap::new();
        hm.insert("msg".to_string(), lpv::String(msg));
        lnpkg::LnPkg::from_hashmap(hm, lpty::Message)
    }
}

/// Message templates used by the server
pub mod server {
    use super::*;
    /// Message **sent by client**, broadcasted by the server to the
    /// rest of clients connected. <br>*Side note: The `msg` parameter only refers to 
    /// the string that the client wants the other clients to see, not the `lnpkg` string.*
    pub fn msg(client_id: lnpkg::ClientId, msg: String) -> lnp {
        let mut hm = HashMap::new();
        hm.insert("client".to_string(), lpv::Int(client_id as i128));
        hm.insert("msg".to_string(), lpv::String(msg)); 
        lnp::from_hashmap(hm, lpty::Message)
    }

    /// Message sent back to the client to give self awareness of its identity.
    pub fn self_identity(client_id: lnpkg::ClientId, client_name: String) -> lnp {
        let mut hm = HashMap::new();
        hm.insert("id".to_string(), lpv::Int(client_id));
        hm.insert("name".to_string(), lpv::String(client_name));
        lnp::from_hashmap(hm, lpty::SelfIdentity)
    }

    /// Contains the identity of a client specified
    pub fn identity(client_id: lnpkg::ClientId, client_name: String) -> lnp {
        let mut hm = HashMap::new();
        hm.insert("id".to_string(), lpv::Int(client_id));
        hm.insert("name".to_string(), lpv::String(client_name));
        lnp::from_hashmap(hm, lpty::Identity)
    }

    /// Message that contains a list of all the clients
    pub fn list_clients(_: HashMap<lnpkg::ClientId, String>) -> lnp {
        todo!("Create format for the list_clients package")
    }

    pub fn event_client_connected(client_id: lnpkg::ClientId, client_name: String) -> lnp {
        let mut hm = HashMap::new();
        hm.insert("id".to_string(), lpv::Int(client_id));
        hm.insert("name".to_string(), lpv::String(client_name));
        lnp::from_hashmap(hm, lpty::EventClientConnected)
    }

    pub fn event_client_left(client_id: lnpkg::ClientId, client_name: String) -> lnp {
        let mut hm = HashMap::new();
        hm.insert("id".to_string(), lpv::Int(client_id));
        hm.insert("name".to_string(), lpv::String(client_name));
        lnp::from_hashmap(hm, lpty::EventClientLeft)
    }
}

/// Message templates used by both the server and client
pub mod shared {
    #[allow(unused_imports)]
    use super::*;
}
