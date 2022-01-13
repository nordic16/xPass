use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Login {
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Login {
    pub fn new(username: &str, password: &str) -> Self {
        Login { username: username.to_owned(), password: password.to_owned() }
    }
}