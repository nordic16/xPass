use super::login::Login;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserConfig {
    pub master_password: String,
    pub logins: Vec<Login>,
}
