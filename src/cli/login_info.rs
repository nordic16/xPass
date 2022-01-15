use cursive::views::{Dialog, ListView, TextView};

use crate::utils::login::Login;

use super::Screen;

pub struct LoginInfoScreen {
    pub login: Login
}


/// Displays information about a single entry.
impl Screen for LoginInfoScreen {
    fn draw_window(&self, cursive: &mut cursive::Cursive) {
        let view = Dialog::new()
            .title("New password:")
            .content(ListView::new()
                .child("Username: ", TextView::new(&self.login.username))
                .child("Password: ", TextView::new(&self.login.password))
            )

            .button("Close", |x| {
                x.pop_layer();
            });
            
        cursive.add_layer(view);
    }


    /// Should never be used anyway.
    fn new() ->  Self {
        LoginInfoScreen { login: Login::new("", "") }
    }
}

impl LoginInfoScreen {
    pub fn new(login: &Login) ->  Self {
        LoginInfoScreen { login: login.to_owned() }
    }
}

