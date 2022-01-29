use cursive::views::{Dialog, TextView};

/// Contains methods useful for retrieving stuff from the config.

pub mod login;
pub mod user_config;
pub mod crypto;


/// Useful for retrieving a padded dialog.
pub fn construct_info_dialog(title: &str, content: &str) -> Dialog {
    Dialog::new()
        .title(title)
        .content(TextView::new(content))
        .button("Ok", |x| { x.pop_layer(); })
        .padding_lrtb(2, 2, 1, 1)
}