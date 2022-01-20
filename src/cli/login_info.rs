use cursive::views::{Dialog, ListView, TextView};
use crate::utils::{login::Login};

pub struct LoginInfoScreen;


impl LoginInfoScreen {
    pub fn draw_window(cursive: &mut cursive::Cursive, login: &Login) {        
        let view = Dialog::new()
            .title("Info")
            .content(ListView::new()
                .child("Name: ", TextView::new(&login.name))
                .child("Username: ", TextView::new(&login.username))
                .child("Password: ", TextView::new(&login.password))
            )

            .button("Close", |x| {
                x.pop_layer();
            });

            
        cursive.add_layer(view);
    }
}

