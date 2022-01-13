use cursive::Cursive;
use cursive::traits::Nameable;
use cursive::views::{Dialog, EditView, ListView, ViewRef};
use crate::utils::login::Login;
use crate::utils::user_config::UserConfig;

use super::{Window, login_info::LoginInfoScreen};
pub struct CreatePasswordScreen;

impl Window for CreatePasswordScreen {
    fn draw_window(&self, cursive: &mut Cursive) {
        let view = Dialog::new()
            .title("Create a new login")

            .content(ListView::new()
                .child("Username: ", EditView::new().with_name("username"))
                .child("Password: ", EditView::new().secret().with_name("password"))
            )

            .button("Cancel", |x| {
                x.pop_layer();
            })
            
            .button("Create login", |x| {
                //... create password
                let username: ViewRef<EditView> = x.find_name("username").unwrap();
                let password: ViewRef<EditView> = x.find_name("password").unwrap();

                let login = CreatePasswordScreen::create_password(username.get_content().as_str(), 
                password.get_content().as_str(), x);
                
                x.pop_layer();
                LoginInfoScreen::new(&login).draw_window(x);
            });


        cursive.add_layer(view);
    }

    fn new() -> Self {
        CreatePasswordScreen {}
    }
}


impl CreatePasswordScreen {
    fn create_password(username: &str, password: &str, cursive: &mut Cursive) -> Login {
        let login = Login::new(username, password);

        cursive.with_user_data(|cfg: &mut UserConfig| cfg.logins.push(login.to_owned()));

        return login;
    }
}