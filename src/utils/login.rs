use serde::{Deserialize, Serialize};

/// Representation of a Login entry.
///
/// NOTE: Name is the name used to represent the entry!

/// TODO: implement IDs. This will simplify things a lot.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Login {
    pub(crate) name: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Login {
    pub fn new(username: &str, password: &str, name: &str) -> Self {
        Login {
            username: username.to_owned(),
            password: password.to_owned(),
            name: name.to_owned(),
        }
    }
}
