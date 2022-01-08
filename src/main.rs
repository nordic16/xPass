mod ui;
mod app;

use cursive::{Cursive, theme::{Color, Theme}};
use ui::{main_screen::MainWindow, Window};
use cursive::theme::PaletteColor::{Background, Shadow, View, Primary};


fn main() {
    let mut cursive = Cursive::default();
    let mut theme = Theme::default();
        
    theme.palette[Background] = Color::TerminalDefault;
    theme.palette[Shadow] = Color::TerminalDefault;
    theme.palette[View] = Color::TerminalDefault;
    theme.palette[Primary] = Color::TerminalDefault;
    
    cursive.set_theme(theme);

    MainWindow::draw_window(&mut cursive);
}