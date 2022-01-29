use magic_crypt::{new_magic_crypt, MagicCryptTrait};

// temporary.
static KEY: &'static str = "256235403290482";

/// Returns a cipher from val.
pub fn encrypt(val: &str) -> String {
    let mc = new_magic_crypt!(KEY, 256);
    mc.encrypt_to_base64(val)
}


/// Returns the decrypted cipher.
pub fn decrypt(cipher: &str) -> String {
    let mc = new_magic_crypt!(KEY, 256);
    mc.decrypt_base64_to_string(cipher).unwrap()
}