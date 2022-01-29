use serde::{Deserialize, Serialize};
use super::login::Login;


#[derive(Serialize, Deserialize, Clone)]
pub struct UserConfig {    
    pub(crate) logins: Vec<Login>,
}


impl std::default::Default for UserConfig {
    fn default() -> Self {
       Self { logins: Vec::<Login>::new() }
    }
}

