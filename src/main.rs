mod app;
mod cli;
mod utils;

use std::error::Error;

use app::App;
use utils::user_config::UserConfig;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg: UserConfig = confy::load("xPass")?;
    let mut app = App::new();

    app.cursive.set_user_data(cfg);
    app.start_event_loop();

    let app_data = app.cursive.user_data::<UserConfig>().unwrap();

    confy::store("xPass", app_data)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use scrypt::password_hash::{PasswordHash, PasswordVerifier};
    use scrypt::Scrypt;

    use crate::cli::generate_password::GeneratePasswordScreen;
    use crate::utils::crypto;

    #[test]
    /// Attempts to generate 32 passwords.
    fn test_password_generator() {
        let len = 20;

        for i in 0..32 {
            println!("Attempt {}: {}", (i + 1), GeneratePasswordScreen::gen_secure_password(len));
        }
    }

    #[test]
    fn test_calculate_hash() {
        let password = "hello there!";
        let hash = crypto::calculate_hash(&password);
        let parsed_hash = PasswordHash::new(hash.as_str()).unwrap();

        println!("{:?}", parsed_hash.hash);

        assert!(Scrypt.verify_password(password.as_bytes(), &parsed_hash).is_ok());
    }
}
