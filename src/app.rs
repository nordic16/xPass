use cursive::theme::Color;
use cursive::{Cursive, theme::Theme};
use cursive::theme::PaletteColor::{Background, Shadow, View, Primary};

use crate::login::Login;
 

pub struct App {
    pub cursive: Cursive,
    pub logins: Vec<Login>
}

impl App {
    pub fn new(logins: &Vec<Login>) -> Self {
        let mut cursive = Cursive::default();

        let mut theme = Theme::default();

        theme.palette[Background] = Color::TerminalDefault;
        theme.palette[Shadow] = Color::TerminalDefault;
        theme.palette[View] = Color::TerminalDefault;
        theme.palette[Primary] = Color::TerminalDefault;

        cursive.set_theme(theme);

        App { cursive, logins: logins.to_owned() }
    }
}