extern crate blowfish_ecb;

use blowfish_ecb::{encrypt, decrypt};
use blowfish_ecb::utils::{hex_to_vec, vec_to_hex};

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

#[test]
fn hex_to_vec_and_vec_to_hex() {
    let message = "lorem ipsum.".as_bytes();

    let hex = vec_to_hex(&message);
    let vec = hex_to_vec(&hex);

    assert_eq!(message.to_vec(), vec);
}