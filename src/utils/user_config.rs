use serde::{Deserialize, Serialize};
use super::login::Login;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserConfig {
    pub master_password: String,
    pub logins: Vec<Login>
}


impl std::default::Default for UserConfig {
    fn default() -> Self {
       Self { logins: Vec::<Login>::new(), master_password: String::from("") }
    }
}

