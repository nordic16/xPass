use cursive::views::TextView;

use crate::utils::{login::Login, construct_info_dialog, crypto::decrypt, user_config::UserConfig};

pub struct LoginInfoScreen;


impl LoginInfoScreen {
    pub fn draw_window(cursive: &mut cursive::Cursive, login: &Login) {  
        // Formats the content nicely.      
        let key = &cursive.user_data::<UserConfig>().unwrap().master_password;

        let content = format!("Name: {}\nUsername: {}\nPassword: {}", &login.name, &login.username, decrypt(&login.password, key));
        cursive.add_layer(construct_info_dialog("Info", TextView::new(content), |x| { x.pop_layer(); }));
    }
}

