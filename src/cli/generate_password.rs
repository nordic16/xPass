use clipboard::{ClipboardContext, ClipboardProvider};
use cursive::{
    direction::Orientation,
    traits::Nameable,
    views::{Button, HideableView, LinearLayout, PaddedView, TextView},
};

use super::Screen;
use crate::utils::{construct_dialog, crypto};
use rand::{rngs::OsRng, Rng};

pub struct GeneratePasswordScreen;

impl Screen for GeneratePasswordScreen {
    fn draw_window(cursive: &mut cursive::Cursive) {
        // The password field is hidden until a password is generated.
        let content = LinearLayout::new(Orientation::Vertical)
            .child(PaddedView::lrtb(0, 0, 0, 1,
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

                            entropyref.set_visible(true);
                            passref.set_visible(true);

                            // Displays the newly generated password to the user.
                            let password = GeneratePasswordScreen::gen_secure_password(16);

                            passref
                                .get_inner_mut()
                                .set_content(&password);
                            
                            
                            entropyref.get_inner_mut().set_content(format!("Password entropy: {} bits", crypto::calculate_password_entropy(&password)));
                        }),
                    ))
                    .child(Button::new_raw("Copy to clipboard", |x| {
                        let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
                        let passref = x.find_name::<HideableView<TextView>>("gen_password").unwrap();

                        if let Ok(_) = clipboard.set_contents(String::from(passref.get_inner().get_content().source())) {
                            x.add_layer(construct_dialog(
                                "Success!",
                                TextView::new("Password has been copied to the clipboard."),
                                |x| {
                                    x.pop_layer();
                                },
                            ));
                        }
                    })),
            );

        cursive.add_layer(construct_dialog("Generate password", content, |x| {
            x.pop_layer();
        }));
    }
}

impl GeneratePasswordScreen {
    /// Generates a *possibly secure* password.
    pub fn gen_secure_password(len: usize) -> String {
        let min_bound = 33u8; // Equivalent of '!'
        let max_bound = 126u8; // Equivalent of '~'

        let invalid_characters = vec!['\'', '`', '´', '\"', '{', '}'];

        let mut password = Vec::<char>::with_capacity(len + 1);
        let mut rng = OsRng::default();

        // Does some magic :troll:
        (0..len).for_each(|f| {
            let mut ch = rng.gen_range(min_bound..min_bound + (max_bound - min_bound) - f as u8) as char;

            // Brute forcing is not very efficient, but it'll do for now.
            while invalid_characters.contains(&ch) {
                ch = rng.gen_range(min_bound..min_bound + (max_bound - min_bound) - f as u8) as char;
            }
            password.push(ch);
        });

        password.into_iter().collect()
    }
}
