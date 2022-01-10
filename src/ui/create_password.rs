use cursive::traits::Nameable;
use cursive::{Cursive};
use cursive::views::{Dialog, EditView, TextView, ListView, ViewRef};

use super::Window;
pub struct CreatePasswordWindow;

impl Window for CreatePasswordWindow {
    fn draw_window(cursive: &mut Cursive) {
        let view = Dialog::around(TextView::new("username: "))
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

                CreatePasswordWindow::create_password(username.get_content().as_str(), 
                password.get_content().as_str());
            });


        cursive.add_layer(view);
    }
}


impl CreatePasswordWindow {
    fn create_password(username: &str, password: &str) {        
    }
}