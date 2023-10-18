use clipboard::{ClipboardContext, ClipboardProvider};
use cursive::views::{TextView, ListView, EditView, NamedView};
use crate::utils::{construct_dialog, crypto::{decrypt, encrypt}, login::Login, user_config::UserConfig};

use super::{list_logins::ListLoginsScreen, Screen};

pub struct LoginInfoScreen;

impl LoginInfoScreen {
    pub fn draw_window(cursive: &mut cursive::Cursive, login: &Login) {
        let key = cursive.user_data::<UserConfig>().unwrap().master_password.to_owned();
        let decrypted_password = decrypt(&login.password, &key);
        let login1 = login.clone();
        let login2 = login.clone(); // i hate this code so much.

        let dialog_view = construct_dialog("Info", ListView::new()
            .child("Name:", NamedView::new("login_name", EditView::new().content(&login.name)))
            .child("Username:", NamedView::new("login_uname", EditView::new().content(&login.username)))
            .child("Password:", NamedView::new("login_password", EditView::new().content(&decrypted_password))))
            .button("Copy to clipboard", move |x| { // messy code!
                let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
                if clipboard.set_contents(String::from(&decrypted_password)).is_ok() {
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
            ).button("Save changes", move |x| {
                let name = x.find_name::<EditView>("login_name").unwrap().get_content().to_string();
                let uname = x.find_name::<EditView>("login_uname").unwrap().get_content().to_string();
                let password = x.find_name::<EditView>("login_password").unwrap().get_content().to_string();

                x.with_user_data(|cfg: &mut UserConfig| {
                    let lg = cfg.logins.iter_mut().find(|lg| lg.name ==  login2.name).unwrap();
                    lg.name = name;
                    lg.username = uname;
                    lg.password = encrypt(&password, &key);
                });

                x.add_layer(
                    construct_dialog("Operation completed.", TextView::new("Changes saved successfully!"))
                        .button("Ok", |cx| {
                            cx.pop_layer(); 
                            cx.pop_layer();
                            cx.pop_layer();
                            ListLoginsScreen::draw_window(cx);
                        }))});
            
      
        cursive.add_layer(dialog_view);
        
    }
}
