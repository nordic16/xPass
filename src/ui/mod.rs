use cursive::Cursive;

pub mod create_password;

pub trait Window {
    fn draw_window(cursive: &mut Cursive);
}