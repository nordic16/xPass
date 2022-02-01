use cursive::{
    direction::Orientation,
    traits::Nameable,
    views::{Button, HideableView, LinearLayout, PaddedView, TextView},
};

use super::Screen;
use crate::utils::construct_dialog;
use rand::{rngs::OsRng, Rng};

pub struct GeneratePasswordScreen;

impl Screen for GeneratePasswordScreen {
    fn draw_window(cursive: &mut cursive::Cursive) {
        // The password field is hidden until a password is generated.
        let content = LinearLayout::new(Orientation::Vertical)
            .child(PaddedView::lrtb(
                0,
                0,
                0,
                1,
                HideableView::new(TextView::new("Password: generating password..."))
                    .hidden()
                    .with_name("gen_password"),
            ))
            .child(
                LinearLayout::new(Orientation::Horizontal)
                    .child(PaddedView::lrtb(
                        0,
                        2,
                        0,
                        0,
                        Button::new_raw("Generate Password", |x| {
                            let mut view =
                                x.find_name::<HideableView<TextView>>("gen_password").unwrap();
                            view.set_visible(true);

                            // Displays the newly generated password to the user.
                            view.get_inner_mut().set_content(format!(
                                "Password: {}",
                                GeneratePasswordScreen::gen_secure_password(16)
                            ));
                        }),
                    ))
                    .child(Button::new_raw("Copy to clipboard", |_| todo!())),
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
