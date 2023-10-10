#[cfg(test)]
    use scrypt::password_hash::{PasswordHash, PasswordVerifier};
    use scrypt::Scrypt;
    use crate::cli::generate_password::GeneratePasswordScreen;
    use crate::utils::crypto;

    use std::time::{Duration, Instant};


    #[test]
    /// Attempts to generate 32 passwords.
    fn test_password_generator() {
        let len = 20;
        let num: usize = 32;
        println!("Attempting to generate 32 passwords.... ");

        let time = Instant::now();

        for i in 0..num {
            let password = GeneratePasswordScreen::gen_secure_password(len);
            println!("Attempt {}: {} ({})", (i + 1), password, password.len());
        }

        let elapsed_time = Instant::now() - time;

        // yeah, microseconds, because 10^-3 just ain't enough lmao
        println!("\nIt took {:?} microseconds to generate {} passwords", Duration::as_micros(&elapsed_time), num);
    }

    #[test]
    fn test_calculate_hash() {
        // This is the best password of All Time
        let password = "general kenobi!";

        println!("Attempting to calculate hash for password {}", password);

        let time = Instant::now();
        // resource-intensive ahh function
        let hash = crypto::calculate_hash(&password);
        let parsed_hash = PasswordHash::new(hash.as_str()).unwrap();

        println!("Hash: {:?}", parsed_hash.hash);

        // makes sure the hash was correctly generated.
        assert!(Scrypt.verify_password(password.as_bytes(), &parsed_hash).is_ok());

        let elapsed_time = Instant::now() - time;
        
        println!("It took {:?}ms to generate the correct hash for the given password", elapsed_time.as_millis());
    }

