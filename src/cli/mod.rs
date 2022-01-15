use cursive::Cursive;
pub mod create_login;
pub mod settings;
pub mod login_info;
pub mod list_logins;

pub trait Screen {
    fn draw_window(&self, cursive: &mut Cursive);
    fn new() -> Self;
}