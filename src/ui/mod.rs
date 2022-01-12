use cursive::Cursive;
pub mod create_password;
//pub mod list_passwords;
pub mod login_info;

pub trait Window {
    fn draw_window(&self, cursive: &mut Cursive);
    fn new() -> Self;
}