use cursive::Cursive;
pub mod create_login;
pub mod settings;
pub mod login_info;
pub mod list_logins;

pub trait Screen {
    fn draw_window(_cursive: &mut Cursive) {}
}