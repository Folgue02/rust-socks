use lnpkg;
use std::collections::HashMap;

type LnPkgMsg = HashMap<String, lnpkg::LnPkgValue>;

/// Message templates used by the client
pub mod client {
    use super::LnPkgMsg;

    /// Message **sent by the client** to the server, with the purpose of
    /// being broadcasted to the rest of users
    pub fn msg(msg: String) -> LnPkgMsg {
        unimplemented!()
    }
}

/// Message templates used by the server
pub mod server {
    use super::LnPkgMsg;
    /// Message **sent by user**, broadcasted by the server to the
    /// rest of users connected.
    pub fn msg(author: i32, msg: String) -> LnPkgMsg {
        unimplemented!()
    }
}

/// Message templates used by both the server and client
pub mod shared {
    #[allow(unused_imports)]
    use super::LnPkgMsg;
}
