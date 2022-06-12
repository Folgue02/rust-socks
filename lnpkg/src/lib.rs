#[cfg(test)]
mod test;
use std::collections::HashMap;
pub type ClientId = i128;

#[derive(Debug, PartialEq, Clone)]
pub struct LnPkg {
    pub content: HashMap<String, LnPkgValue>,
    pub pkg_type: LnPkgType,
}

impl LnPkg {

    pub fn new(pkg_type: LnPkgType) -> Self {
        Self {
            pkg_type,
            content: HashMap::new()
        }
    }


    pub fn from_string(pkg: String) -> Self {
        let mut hm: HashMap<String, LnPkgValue> = HashMap::new();
        let mut pkg_type = LnPkgType::Unknown;

        for segment in pkg.split(':') {
            // Ignore empty segments
            if segment == "" {
                continue;
            }

            // Value=key
            if let Some(index) = segment.find('=') {
                let key = segment[..index].to_string();
                let value_str = segment[index + 1..].to_string();
                let value: LnPkgValue;

                // Difference between a normal key=value and a type segment
                if key == "type" {
                    pkg_type = LnPkgType::from_string(value_str);
                    continue;
                } else {
                    value = LnPkgValue::from_string(value_str);
                }
                hm.insert(key, value);
            } else {
                // Key, null value
                hm.insert(segment.to_string(), LnPkgValue::Null);
            }
        }
        Self {
            content: hm,
            pkg_type,
        }
    }
    /// Returns an instance of `LnPkg` built with a hashmap
    pub fn from_hashmap(target: HashMap<String, LnPkgValue>, pkg_type: LnPkgType) -> Self {
        let mut pkg_type = pkg_type; // Make it mutable
        let target = target
            .into_iter()
            .filter(|pair| {
                // Do not store the `type` segment in `self.content`
                if pair.0 == "type" {
                    // Store the type of pkg in `self.pkg_type`
                    pkg_type = match &pair.1 {
                        LnPkgValue::String(v) => LnPkgType::from_string(v.clone()),
                        _ => LnPkgType::Unknown,
                    };
                    return false;
                } else {
                    // Store anything else
                    return true;
                }
            })
            .collect();
        Self {
            content: target,
            pkg_type,
        }
    }
    /// Returns the formatted version of the package to a string that can be parsed back
    /// into an identical `LnPkg`
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        if self.pkg_type != LnPkgType::Unknown {
            result += format!("type={}:", self.pkg_type).as_str(); //TODO HERE
        }
        for (k, v) in &self.content {
            result += format!("{}={}:", k, v).as_str()
        }
        result
    }

    /// Returns a vector of bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum LnPkgType {
    /// Sent by server, contains the identity of the one who requested it
    SelfIdentity,
    /// Sent by server, contains the identity of whoever was specified, and gets returned to who
    /// requested it. 
    Identity,
    /// Sent by the client, and the broadcasted by the server to the rest of the users
    Message,
    /// Sent by the client (*tries to send a message to a certain user*) or the server (*resends a message to a user*)
    DirectMessage,
    /// Sent by the client, represents command and parameters
    Command,

    // ------------------------- EVENTS
    /// Message sent by the server to communicate that a client has connected the server.
    EventClientConnected,
    /// Message sent by the server to communicate that a client has left the server.
    EventClientLeft,

    /// No message type or non existent
    Unknown,
}

#[derive(PartialEq, Debug, Clone)]
pub enum LnPkgValue {
    String(String),
    Int(ClientId),
    Bool(bool),
    Null,
}

impl LnPkgType {
    /// Returns a variant of the enum by parsing the string provided
    pub fn from_string(target: String) -> Self {
        let target = target.as_str();
        match target {
            "msg" => Self::Message,
            "cmd" => Self::Command,
            "dmsg" => Self::DirectMessage,
            "selfid" => Self::SelfIdentity,
            "id" => Self::Identity,
            "event-connection" => Self::EventClientConnected,
            "event-left" => Self::EventClientLeft,
            _ => Self::Unknown,
        }
    }
}

impl std::fmt::Display for LnPkgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Message => "msg",
                Self::Command => "cmd",
                Self::DirectMessage => "dmsg",
                Self::SelfIdentity => "selfid",
                Self::Identity => "id",
                Self::EventClientConnected => "event-connection",
                Self::EventClientLeft => "event-left",
                Self::Unknown => "",
            }
        )
    }
}

impl std::fmt::Display for LnPkgValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bool(b) => format!("{}", b),
                Self::Int(i) => format!("{}", i),
                Self::String(s) => s.clone(),
                _ => "".to_string(),
            }
        )
    }
}

impl LnPkgValue {
    pub fn from_string(target: String) -> LnPkgValue {
        let result;
        if let Ok(int) = target.parse::<ClientId>() {
            result = LnPkgValue::Int(int)
        } else if let Ok(boolean) = target.parse::<bool>() {
            result = LnPkgValue::Bool(boolean)
        } else if target == "" {
            result = LnPkgValue::Null;
        } else {
            // String
            result = LnPkgValue::String(target);
        }
        result
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}
