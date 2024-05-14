use cursive::{
    views::Dialog,
    View,
};

/// Useful cryptography-related stuff
pub mod crypto;
/// Contains methods useful for retrieving stuff from the config.
pub mod login;
pub mod user_config;

/// Useful for retrieving a padded info dialog and to reduce code duplication.
/// closure defines the Ok button's callback.
pub fn construct_dialog<T>(title: &str, content: T) -> Dialog
where
    T: View,
    // C: Fn(&mut Cursive) + 'static,
{
    Dialog::new().title(title).content(content).padding_lrtb(2, 2, 1, 1)
}
