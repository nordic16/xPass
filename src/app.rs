use cursive::CursiveExt;
use cursive::direction::Orientation;
use cursive::event::Key;
use cursive::theme::Color;
use cursive::views::{Button, LinearLayout, Dialog, TextView};
use cursive::{Cursive, theme::Theme};
use cursive::theme::PaletteColor::{Background, Shadow, View, Primary};

use crate::login::Login;
use crate::ui::Window;
use crate::ui::create_password::CreatePasswordScreen; 

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
        let view = LinearLayout::new(Orientation::Vertical)
            .child( Button::new("List passwords", |q| {
                let dialog = retrieve_future_dialog();
                    q.add_layer(dialog);
            }))
         
            .child(Button::new("Clear passwords", |q| {
                q.with_user_data(|logins: &mut Vec<Login>| logins.clear());

                let dialog = Dialog::new()
                    .title("Success!")
                    .content(TextView::new("Passwords cleared."))
                    .button("Ok", |x| {
                        x.pop_layer();
                    });
                q.add_layer(dialog);
            }))

            .child(Button::new("Create new password", |x: &mut Cursive| {
                CreatePasswordScreen::new().draw_window(x);
        }));


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


/// TODO: Delete this later.
fn retrieve_future_dialog() -> Dialog {
    Dialog::new()
             .title("Future")
             .content(
                 TextView::new("Soon I will have a new apprentice...One far younger, and more powerful!"))
}