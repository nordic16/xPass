use cursive::CursiveExt;
use cursive::direction::Orientation;
use cursive::event::Key;
use cursive::theme::Color;
use cursive::views::{Button, LinearLayout, Dialog, TextView};
use cursive::{Cursive, theme::Theme};
use cursive::theme::PaletteColor::{Background, Shadow, View, Primary};

use crate::ui::Window;
use crate::ui::create_password::CreatePasswordWindow; 

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

    pub fn start_event_loop(&mut self) {        
    
        let b_listpasswd = Button::new("List passwords", |q| {
           let dialog = retrieve_future_dialog();
                q.add_layer(dialog);
        });
    
        let b_createpasswd = Button::new("Create new password", |x: &mut Cursive| CreatePasswordWindow::draw_window(x));

    
         let b_clearpasswd = Button::new("Clear passwords", |q| {
            let dialog = retrieve_future_dialog();
                 q.add_layer(dialog);
         });
    
        let mut view = LinearLayout::new(Orientation::Vertical);
        
        view.add_child(b_listpasswd);   
        view.add_child(b_createpasswd);   
        view.add_child(b_clearpasswd);   
    

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

fn retrieve_future_dialog() -> Dialog {
    Dialog::new()
             .title("Future")
             .content(
                 TextView::new("Soon I will have a new apprentice...One far younger, and more powerful!"))
}