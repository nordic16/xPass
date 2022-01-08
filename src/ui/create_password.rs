use cursive::traits::Nameable;
use cursive::{Cursive};
use cursive::views::{Dialog, EditView, TextView, ListView};

use super::Window;

// This window is just a dialog, really xD
pub struct CreatePasswordWindow;

impl Window for CreatePasswordWindow {
    fn draw_window(cursive: &mut Cursive) {
        let view = Dialog::around(TextView::new("username: "))
            .title("Create a new login")

            .content(ListView::new()
            .child("Username: ", EditView::new().with_name("username"))
            .child("password: ", EditView::new().with_name("Password"))
        )

            .button("Cancel", |x| {
                x.pop_layer();
            })
            .button("Create login", |x| {
                //... create password
            });


        cursive.add_layer(view);
    }
}


impl CreatePasswordWindow {
    fn create_password() {
        // do something
    }
}