#[cfg(test)]
    use scrypt::password_hash::{PasswordHash, PasswordVerifier};
    use scrypt::Scrypt;
    use crate::cli::generate_password::GeneratePasswordScreen;
    use crate::utils::crypto;

    #[test]
    /// Attempts to generate 32 passwords.
    fn test_password_generator() {
        let len = 20;

        for i in 0..32 {
            let password = GeneratePasswordScreen::gen_secure_password(len);
            println!("Attempt {}: {} ({})", (i + 1), password, password.len());
        }
    }

    #[test]
    fn test_calculate_hash() {
        // This is the best password of All Time
        let password = "hello there!";
        let hash = crypto::calculate_hash(&password);
        let parsed_hash = PasswordHash::new(hash.as_str()).unwrap();

        println!("{:?}", parsed_hash.hash);

        assert!(Scrypt.verify_password(password.as_bytes(), &parsed_hash).is_ok());
    }

