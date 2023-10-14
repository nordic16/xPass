use cursive::views::TextView;
use crate::utils::{construct_dialog, crypto::decrypt, login::Login, user_config::UserConfig};

use super::{list_logins::ListLoginsScreen, Screen};

pub struct LoginInfoScreen;

impl LoginInfoScreen {
    pub fn draw_window(cursive: &mut cursive::Cursive, login: &Login) {
        // Formats the content nicely.
        let key = &cursive.user_data::<UserConfig>().unwrap().master_password;
        let content = format!(
            "Name: {}\nUsername: {}\nPassword: {}",
            &login.name,
            &login.username,
            decrypt(&login.password, key)
        );

        let mut dialog = construct_dialog("Info", TextView::new(&content), |x| {
            x.pop_layer();
        });

        // This is some messy code right here...
        let login1 = login.clone();
        dialog.add_button("Delete", move |x| {
            x.with_user_data(|cfg: &mut UserConfig| {
                // Removes the selected password from the list.
                cfg.logins.remove(cfg.logins.iter().position(|c| c.name == login1.name).unwrap());
            });
            
            // This dialog will be shown after the user deletes a password. Upon pressing ok, it'll redraw the password list screen.
            x.add_layer(construct_dialog("Password deleted successfully!", TextView::new("Press ok to go back"), |x| {
                x.pop_layer();
                x.pop_layer();
                x.pop_layer();
                ListLoginsScreen::draw_window(x);
            }));
        });
        
        cursive.add_layer(dialog);
        
    }
}
