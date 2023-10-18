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

        let dialog_view = construct_dialog("Info", ListView::new()
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
                    });
                
                x.add_layer(construct_dialog("Password deleted successfully!", TextView::new("Press ok to go back"))
                    .button("Ok", |y| {
                        y.pop_layer();
                        y.pop_layer();
                        y.pop_layer();
                        ListLoginsScreen::draw_window(y);
                    }));
                
                }
            ).button("Ok", |x| {
                x.add_layer(
                    construct_dialog("Not implemented", TextView::new("Feature is not implemented.")).dismiss_button("Ok"))});
            
      
        cursive.add_layer(dialog_view);
        
    }
}
