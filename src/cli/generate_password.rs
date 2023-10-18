use clipboard::{ClipboardContext, ClipboardProvider};
use cursive::{
    direction::Orientation,
    traits::Nameable,
    views::{Button, HideableView, LinearLayout, PaddedView, TextView, ListView, ResizedView, TextArea, ViewRef},
};

use super::Screen;
use crate::utils::{construct_dialog, crypto};
pub struct GeneratePasswordScreen;

impl Screen for GeneratePasswordScreen {
    fn draw_window(cursive: &mut cursive::Cursive) {
        // The password field is hidden until a password is generated.
        let content = LinearLayout::new(Orientation::Vertical)
            .child(ListView::new().child("length: ", ResizedView::with_fixed_width(10, 
            TextArea::new().with_name("password_len"))))
            .child(PaddedView::lrtb(0, 0, 1, 0,
                HideableView::new(TextView::new("generating password...").h_align(cursive::align::HAlign::Center))
                        .hidden()
                        .with_name("gen_password"),
            )).child(PaddedView::lrtb(0, 0, 0, 1, HideableView::new(TextView::new("Entropy")
                    .h_align(cursive::align::HAlign::Center)).hidden().with_name("entropy")))
            .child(
                LinearLayout::new(Orientation::Horizontal)
                    .child(PaddedView::lrtb(0, 2, 0, 0, 
                        Button::new_raw("Generate Password", |x| {
                            let mut passref = x.find_name::<HideableView<TextView>>("gen_password").unwrap();
                            let mut entropyref = x.find_name::<HideableView<TextView>>("entropy").unwrap();
                            let lenref: ViewRef<TextArea> = x.find_name("password_len").unwrap();

                            // Some dumbass might think it's a good idea to try to create a password with *no* length or with letters.
                            if !lenref.get_content().is_empty() && lenref.get_content().parse::<i32>().is_ok() {
                                let len = lenref.get_content().parse::<usize>().unwrap();

                                entropyref.set_visible(true);
                                passref.set_visible(true);
                            
                                // these constraints might change one day (except for the 0 one ofc, no password can have a negative length.)
                                if len < 50 && len > 0 {
                                    let password = crypto::gen_secure_password(len as usize);

                                    // Displays the newly generated password to the user.
                                    passref.get_inner_mut().set_content(&password);
                                    entropyref.get_inner_mut().set_content(format!("Password entropy: {} bits", crypto::calculate_password_entropy(&password)));
                                }
                            }
                        }),
                    ))
                    .child(PaddedView::lrtb(0, 2, 0, 0,
                        Button::new_raw("Copy to clipboard", |x| {
                                let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
                                let passref = x.find_name::<HideableView<TextView>>("gen_password").unwrap();

                                if clipboard.set_contents(String::from(passref.get_inner().get_content().source())).is_ok() {
                                    x.add_layer(construct_dialog("Success!", TextView::new("Password has been copied to the clipboard.")
                                    ).dismiss_button("Ok"));
                                }
                    })))
                    .child(Button::new_raw("Ok", |x| { x.pop_layer(); }))
            );

        cursive.add_layer(construct_dialog("Generate password", content));
    }
}