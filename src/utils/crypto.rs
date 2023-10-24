use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rand::{rngs::OsRng, Rng};
use scrypt::{
    password_hash::{PasswordHasher, SaltString}, Scrypt
};

/// Returns a cipher from val.
pub fn encrypt(val: &str, key: &str) -> String {
    let mc = new_magic_crypt!(key, 256);
    mc.encrypt_to_base64(val)
}

/// Generates a *possibly secure* password.
pub fn gen_secure_password(len: usize, special_chars: bool) -> String {
    let mut min_bound = 33u8; // Equivalent of '!'
    let mut max_bound = 126u8; // Equivalent of '~'


    let mut invalid_characters: Vec<char> = vec!['\'', '`', '´', '\"'];

    let mut password = Vec::<char>::with_capacity(len + 1);
    let mut rng = OsRng;

    if !special_chars {
        min_bound = 48u8; // 0
        max_bound = 122u8; // Z

        // TODO: this code can probably be optimized
        let mut new_invalid = (':'..'@').collect::<Vec<_>>();
        new_invalid.extend(('['..'`').collect::<Vec<_>>());
        invalid_characters.append(&mut new_invalid);
    }

    // Does some magic :troll:
    for f in 0..len {
        let mut ch =
            rng.gen_range(min_bound..min_bound + (max_bound - min_bound) - f as u8) as char;

        // Brute forcing is not very efficient, but it'll do for now.
        while invalid_characters.contains(&ch) {
            ch = rng.gen_range(min_bound..min_bound + (max_bound - min_bound) - f as u8) as char;
        }
        password.push(ch);
    }

    password.into_iter().collect()
}

/// Returns the decrypted cipher.
pub fn decrypt(cipher: &str, key: &str) -> String {
    let mc = new_magic_crypt!(key, 256);
    mc.decrypt_base64_to_string(cipher).unwrap()
}

/// Computes the hash of the given T.
/// NOTE: THIS IS WAY TOO SLOW: TAKES MORE THAN 10 seconds.
pub fn calculate_hash(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Scrypt
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn calculate_password_entropy(password: &str) -> f32 {
    let len = password.len() as f32;
    let pool = calculate_password_pool(password);

    return len * (pool as f32).log2();
}

/// This function probably sucks. Refactor it some day.
fn calculate_password_pool(password: &str) -> i32 {
    let mut pool = 0;

    if password.chars().any(|f| f.is_ascii_lowercase()) {
        pool += 26;
    }

    if password.chars().any(|f| f.is_ascii_uppercase()) {
        pool += 26;
    }

    if password.chars().any(|f| f.is_ascii_digit()) {
        pool += 10;
    }

    // Bunch of special characters, i know...
    if password
        .chars()
        .any(|f| matches!(f, '!'..='/' | ':'..='@' | '['..='`' | '{'..='~'))
    {
        pool += 32;
    }

    return pool;
}
