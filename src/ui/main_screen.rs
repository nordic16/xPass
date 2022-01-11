use cursive::direction::Orientation;
use cursive::event::Key;
use cursive::{Cursive, CursiveExt};
use cursive::views::{TextView, Button, Dialog, LinearLayout};

use super::Window;
use super::create_password::CreatePasswordWindow;

pub struct MainScreen;


impl MainScreen {
    pub fn start_event_loop(cursive: &mut Cursive) {        
    
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
    

        cursive.add_layer(view);
    
        cursive.add_global_callback('q', |s| s.quit());
        cursive.add_global_callback(Key::Esc, |s| {  
            // Prevents the user from being able to close the main view (lol).
            if s.screen_mut().len() > 1 {
                s.pop_layer();
            }
         });
    
        cursive.run();
    }
}

fn retrieve_future_dialog() -> Dialog {
    Dialog::new()
             .title("Future")
             .content(
                 TextView::new("Soon I will have a new apprentice...One far younger, and more powerful!"))
}