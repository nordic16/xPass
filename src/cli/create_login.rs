use super::Screen;
use crate::utils::{construct_dialog, crypto, login::Login, user_config::UserConfig};
use cursive::{
    traits::Nameable, views::{
        Dialog, EditView, HideableView, LinearLayout, ListView, ResizedView, TextView, ViewRef
    }, Cursive
};
use rand::{self, thread_rng, Rng};

pub struct CreateLoginScreen;

impl Screen for CreateLoginScreen {
    fn draw_window(cursive: &mut Cursive) {
        let view = ResizedView::with_fixed_width(
            60,
            LinearLayout::new(cursive::direction::Orientation::Vertical)
                .child(
                    Dialog::new()
                        .title("Create a new login")
                        .content(
                            ListView::new()
                                .child("Name:", EditView::new().with_name("name"))
                                .child("Username: ", EditView::new().with_name("username"))
                                .child(
                                    "Password: ",
                                    EditView::new().secret().with_name("password"),
                                ),
                        )
                        // TODO: FIX UNINTENDED BEHAVIOR.
                        .dismiss_button("Cancel")
                        .button("Generate secure password", |x| {
                            let mut passwordref: ViewRef<EditView> =
                                x.find_name("password").unwrap();
                            let mut passwordlen_ref: ViewRef<HideableView<TextView>> =
                                x.find_name("passwordlen").unwrap();
                            let mut password_entropyref: ViewRef<HideableView<TextView>> =
                                x.find_name("password_entropy").unwrap();

                            let len = thread_rng().gen_range(16..50);
                            let password = crypto::gen_secure_password(len);

                            passwordref.set_content(&password);
                            passwordlen_ref
                                .get_inner_mut()
                                .set_content(format!("{}", len));
                            password_entropyref.get_inner_mut().set_content(format!(
                                "{} bits",
                                crypto::calculate_password_entropy(&password)
                            ));

                            // Display attributes to the user.
                            passwordlen_ref.set_visible(true);
                            password_entropyref.set_visible(true);
                        })
                        .button("Create login", |x| {
                            //... create password
                            let usernameref: ViewRef<EditView> = x.find_name("username").unwrap();
                            let passwordref: ViewRef<EditView> = x.find_name("password").unwrap();
                            let nameref: ViewRef<EditView> = x.find_name("name").unwrap();

                            CreateLoginScreen::create_password(
                                usernameref.get_content().as_str(),
                                passwordref.get_content().as_str(),
                                nameref.get_content().as_str(),
                                x,
                            );

                            x.add_layer(
                                construct_dialog(
                                    "Success!",
                                    TextView::new(
                                        "Password has been encrypted and stored successfully!",
                                    ),
                                )
                                .button("Ok", |x| {
                                    x.pop_layer();
                                    x.pop_layer();
                                }),
                            );
                        }),
                )
                .child(
                    ListView::new() // Password attrs
                        .child(
                            "Pasword length: ",
                            HideableView::new(TextView::new("")).with_name("passwordlen"),
                        )
                        .child(
                            "Password entropy",
                            HideableView::new(TextView::new("")).with_name("password_entropy"),
                        ),
                ),
        );

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
