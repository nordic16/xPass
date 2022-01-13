use cursive::Cursive;
pub mod create_password;
pub mod settings;
pub mod login_info;

pub trait Screen {
    fn draw_window(&self, cursive: &mut Cursive);
    fn new() -> Self;
}