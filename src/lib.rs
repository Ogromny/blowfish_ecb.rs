pub mod utils;

extern crate crypto;

use crypto::blowfish::Blowfish;
use crypto::symmetriccipher::{BlockEncryptor, BlockDecryptor};

const PADDING_BYTE: u8 = 0;

pub fn encrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    let blowfish = Blowfish::new(key);
    let block_size = <Blowfish as BlockEncryptor>::block_size(&blowfish);

    let data_len = round_len(data.len(), block_size);
    let mut data = data.to_vec();
    data.resize(data_len, PADDING_BYTE);

    let mut encrypted = data.clone();

    for (data_chunk, mut encrypted_chunk) in data.chunks_mut(block_size).zip(encrypted.chunks_mut(block_size)) {
        blowfish.encrypt_block(data_chunk, encrypted_chunk);
    }

    encrypted
}

pub fn decrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    let blowfish = Blowfish::new(key);
    let block_size = <Blowfish as BlockDecryptor>::block_size(&blowfish);

    let data_len = round_len(data.len(), block_size);
    let mut data = data.to_vec();
    data.resize(data_len, PADDING_BYTE);

    let mut decrypted = data.clone();

    for (data_chunk, mut decrypted_chunk) in data.chunks_mut(block_size).zip(decrypted.chunks_mut(block_size)) {
        blowfish.decrypt_block(data_chunk, decrypted_chunk);
    }

    match decrypted.iter().position(|&byte| byte == PADDING_BYTE) {
        Some(index) => decrypted.truncate(index),
        None => {},
    }

    decrypted
}


fn round_len(len: usize, block_size: usize) -> usize {
    let remainder = len % block_size;
    if remainder == 0 {
        len
    } else {
        len + block_size - remainder
    }
}