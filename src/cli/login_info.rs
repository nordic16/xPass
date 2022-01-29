use crate::utils::{login::Login, construct_info_dialog, crypto::decrypt};

pub struct LoginInfoScreen;


impl LoginInfoScreen {
    pub fn draw_window(cursive: &mut cursive::Cursive, login: &Login) {  
        // Formats the content nicely.      
        let content = format!("Name: {}\nUsername: {}\nPassword: {}", &login.name, &login.username, decrypt(&login.password));
        cursive.add_layer(construct_info_dialog("Info", &content));
    }
}

