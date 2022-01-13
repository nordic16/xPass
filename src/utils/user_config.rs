use std::{path::PathBuf, fs, io};

use serde::{Deserialize, Serialize};

use super::login::Login;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserConfig {
    pub(crate) e_key: u32,
    pub(crate) logins: Vec<Login>,
}


impl std::default::Default for UserConfig {
    fn default() -> Self {
       Self { e_key: 0, logins: Vec::<Login>::new() }
    }
}