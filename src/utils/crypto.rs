use magic_crypt::{new_magic_crypt, MagicCryptTrait};

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
