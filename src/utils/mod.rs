use cursive::{views::{Dialog}, View, Cursive};

/// Contains methods useful for retrieving stuff from the config.

pub mod login;
pub mod user_config;
pub mod crypto;


/// Useful for retrieving a padded info dialog and to reduce code duplication.
/// closure defines the Ok button's callback.
pub fn construct_dialog<T, C>(title: &str, content: T, closure: C) -> Dialog 
    where T: View,
    C: Fn(&mut Cursive) + 'static
{
    Dialog::new()
        .title(title)
        .content(content)
        .button("Ok", closure)
        .padding_lrtb(2, 2, 1, 1)
}