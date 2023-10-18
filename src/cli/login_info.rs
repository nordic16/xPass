use clipboard::{ClipboardContext, ClipboardProvider};
use cursive::views::{TextView, ListView, EditView};
use crate::utils::{construct_dialog, crypto::decrypt, login::Login, user_config::UserConfig};

use super::{list_logins::ListLoginsScreen, Screen};

pub struct LoginInfoScreen;

impl LoginInfoScreen {
    pub fn draw_window(cursive: &mut cursive::Cursive, login: &Login) {
        // Formats the content nicely.
        let key = &cursive.user_data::<UserConfig>().unwrap().master_password;
        let decrypted_password = decrypt(&login.password, key);
        let login1 = login.clone();

        let login_fields = construct_dialog("Info", ListView::new()
            .child("Name:", EditView::new().content(&login.name))
            .child("Username:", EditView::new().content(&login.username))
            .child("Password:", EditView::new().content(&login.password)))
            .button("Copy to clipboard", move |x| { // messy code!
                let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();

                if let Ok(_) = clipboard.set_contents(String::from(&decrypted_password)) {
                    x.add_layer(construct_dialog(
                        "Success!",
                        TextView::new("Password has been copied to the clipboard."),
            ))}})

            .button("Delete", move |x| {
                x.with_user_data(|cfg: &mut UserConfig| {
                    // Removes the selected password from the list.
                    cfg.logins.remove(cfg.logins.iter().position(|c| c.name == login1.name).unwrap());
                    let mut dialog = construct_dialog("Password deleted successfully!", TextView::new("Press ok to go back"));
                    
                    dialog.add_button("Ok", |x| {
                        x.pop_layer();
                        x.pop_layer();
                        x.pop_layer();
                        ListLoginsScreen::draw_window(x);
                })});
            })
            .dismiss_button("Ok");
            
      
        cursive.add_layer(login_fields);
        
    }
}
