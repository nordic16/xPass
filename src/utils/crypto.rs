use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rand::rngs::OsRng;
use scrypt::{
    password_hash::{PasswordHasher, SaltString},
    Scrypt,
};

/// Returns a cipher from val.
pub fn encrypt(val: &str, key: &str) -> String {
    let mc = new_magic_crypt!(key, 256);
    mc.encrypt_to_base64(val)
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
    Scrypt.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}


pub fn calculate_password_entropy(password: &str) -> f32 {
    let len = password.len() as i32;
    let pool = calculate_password_pool(password);

    return len as f32 * (pool as f32).log2();
}


/// This function probably sucks. Refactor it some day.
fn calculate_password_pool(password: &str) -> i32 {
    let mut pool = 0;
    
    // character ranges
    let minescules = ['a'..'z'];
    let capitals = ['A'..'Z'];
    let numbers = ['0'..'9'];
    let digits = [['!'..'/'], [':'..'@'], ['['..'`'], ['{'..'~']];

    if password.chars().any(|f| matches!(f, minescules)) {
        pool += 26;
    }

    if password.chars().any(|f| matches!(f, capitals)) {
        pool += 26;
    }

    if password.chars().any(|f| matches!(f, numbers)) {
        pool += 10;
    }

    if password.chars().any(|f| matches!(f, digits)) {
        pool += 32;
    }

    return pool;
}