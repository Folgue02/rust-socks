use lnpkg;
use std::collections::HashMap;
pub type lpv = lnpkg::LnPkgValue;   // LakenetPackageValue
pub type lpty = lnpkg::LnPkgType;   // LakeNetPackageType
pub type lnp = lnpkg::LnPkg;        // LakeNetPackage


/// Message templates used by the client
pub mod client {
    use super::*;

    /// Message **sent by the client** to the server, with the purpose of
    /// being broadcasted to the rest of users
    pub fn msg(msg: String) -> lnp {
        let mut hm = HashMap::new();
        hm.insert("msg".to_string(), lpv::String(msg));
        lnpkg::LnPkg::from_hashmap(hm, lpty::Message)
    }
}

/// Message templates used by the server
pub mod server {
    use super::*;
    /// Message **sent by user**, broadcasted by the server to the
    /// rest of users connected. <br>*Side note: The `msg` parameter only refers to 
    /// the string that the client wants the other users to see, not the `lnpkg` string.*
    pub fn msg(client_id: u128, msg: String) -> lnp {
        println!("lnpkg::lib::server::msg> String message {}", &msg);
        let mut hm = HashMap::new();
        hm.insert("client".to_string(), lpv::Int(client_id as i128));
        hm.insert("msg".to_string(), lpv::String(msg)); 
        println!("lnpkg::lib::server::msg> Parsed message {:?}", lnp::from_hashmap(hm.clone(), lpty::Message));
        lnp::from_hashmap(hm, lpty::Message)
    }

    pub fn self_identity(client_id: u128)
}

/// Message templates used by both the server and client
pub mod shared {
    #[allow(unused_imports)]
    use super::*;
}
