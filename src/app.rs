use cursive::CursiveExt;
use cursive::direction::Orientation;
use cursive::event::Key;
use cursive::theme::Color;
use cursive::views::{Button, LinearLayout, Dialog, TextView, Panel};
use cursive::{Cursive, theme::Theme};
use cursive::theme::PaletteColor::{Background, Shadow, View, Primary};
use crate::cli::Screen;
use crate::cli::{create_login::CreateLoginScreen, list_logins::ListLoginsScreen};
use crate::utils::user_config::UserConfig; 

pub struct App {
    pub cursive: Cursive,
}

impl App {
    pub fn new() -> Self {
        let mut cursive = Cursive::default();
        let mut theme = Theme::default();

        theme.palette[Background] = Color::TerminalDefault;
        theme.palette[Shadow] = Color::TerminalDefault;
        theme.palette[View] = Color::TerminalDefault;
        theme.palette[Primary] = Color::TerminalDefault;

        cursive.set_theme(theme);

        App { cursive }
    }


    /// Starts event loop and draws the main screen.
    pub fn start_event_loop(&mut self) {    
        let view = Panel::new(LinearLayout::new(Orientation::Vertical)
            .child(Button::new_raw("Create new password", |x| {
                CreateLoginScreen::draw_window(x);
            }))

            .child( Button::new_raw("List passwords", |x| {
                ListLoginsScreen::draw_window(x);
            }))

            .child(Button::new_raw("Clear passwords", |q| {
                q.with_user_data(|cfg: &mut UserConfig| cfg.logins.clear());

                let dialog = Dialog::new()
                    .title("Success!")
                    .content(TextView::new("Passwords cleared."))
                    .button("Ok", |x| {
                        x.pop_layer();
                    });
                q.add_layer(dialog);
            }))
        )
        .title("xPass");

        self.cursive.add_layer(view);
    
        self.cursive.add_global_callback('q', |s| s.quit());
        self.cursive.add_global_callback(Key::Esc, |s| {  
            // Prevents the user from being able to close the main view (lol).
            if s.screen_mut().len() > 1 {
                s.pop_layer();
            }
         });
    
        self.cursive.run();
    }
}