use cursive::Cursive;
pub mod create_login;
pub mod generate_password;
pub mod list_logins;
pub mod login_info;

pub trait Screen {
    fn draw_window(_cursive: &mut Cursive);
}
