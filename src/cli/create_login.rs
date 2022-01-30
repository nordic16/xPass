use super::Screen;
use crate::utils::login::Login;
use crate::utils::user_config::UserConfig;
use crate::utils::{construct_dialog, crypto};
use cursive::traits::Nameable;
use cursive::views::{Dialog, EditView, ListView, TextView, ViewRef};
use cursive::Cursive;

pub struct CreateLoginScreen;

impl Screen for CreateLoginScreen {
    fn draw_window(cursive: &mut Cursive) {
        let view = Dialog::new()
            .title("Create a new login")
            .content(
                ListView::new()
                    .child("Name:", EditView::new().with_name("name"))
                    .child("Username: ", EditView::new().with_name("username"))
                    .child("Password: ", EditView::new().secret().with_name("password")),
            )
            .dismiss_button("Cancel")
            .button("Create login", |x| {
                //... create password
                let username: ViewRef<EditView> = x.find_name("username").unwrap();
                let password: ViewRef<EditView> = x.find_name("password").unwrap();
                let name: ViewRef<EditView> = x.find_name("name").unwrap();

                CreateLoginScreen::create_password(
                    username.get_content().as_str(),
                    password.get_content().as_str(),
                    name.get_content().as_str(),
                    x,
                );

                x.add_layer(construct_dialog(
                    "Success!",
                    TextView::new("Password has been encrypted and stored successfully!"),
                    |x| {
                        x.pop_layer();
                        x.pop_layer();
                    },
                ));
            });

        cursive.add_layer(view);
    }
}

impl CreateLoginScreen {
    fn create_password(username: &str, password: &str, name: &str, cursive: &mut Cursive) {
        // It will have to encrypt the password first!
        let key = &cursive.user_data::<UserConfig>().unwrap().master_password;

        let e_passwd = crypto::encrypt(password, key);

        let login = Login::new(username, &e_passwd, name);

        cursive.with_user_data(|cfg: &mut UserConfig| cfg.logins.push(login));
    }
}
