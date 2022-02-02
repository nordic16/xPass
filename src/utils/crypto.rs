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
