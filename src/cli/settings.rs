use cursive::{views::{Dialog, ListView, Checkbox, PaddedView, TextView, EnableableView}, view::Margins};

use super::Screen;

pub struct SettingsScreen;

impl Screen for SettingsScreen {
    fn draw_window(cursive: &mut cursive::Cursive) {
        let view = Dialog::new()
            .title("Settings")
            .content(
                ListView::new()
                        .child("Key", EnableableView::new(PaddedView::new(
                      Margins { left: 1, right: 0, top: 1, bottom: 0 },
                        ListView::new()
                                .child("Custom key: ", Checkbox::new())      
                        )))
                        
                        .child("test", TextView::new("test"))
                    );

                    
        cursive.add_layer(view);

    }
}