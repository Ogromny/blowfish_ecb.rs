extern crate blowfish_ecb;

use blowfish_ecb::{encrypt, decrypt};

#[test]
fn encrypt_and_decrypt() {
    let key = "blowfish_ecb.rs rocks !";
    let message = "lorem ipsum";

    let encrypted_message = encrypt(
        &key.as_bytes(),
        &message.as_bytes(),
    );

    let decrypted_message = decrypt(
        &key.as_bytes(),
        &encrypted_message,
    );

    assert_eq!(message.as_bytes().to_vec(), decrypted_message);
}