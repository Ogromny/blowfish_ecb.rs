/// Converts an array of u8 to hexadecimal.
///
/// # Arguments
/// * `vec` - an array of u8.
pub fn vec_to_hex(vec: &[u8]) -> String {
    let mut hex = String::with_capacity(vec.len() * 2);
    for byte in vec {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}

/// Converts a hexadecimal to an array of u8.
///
/// # Arguments
/// * `hex` - a hexadecimal ```String```.
pub fn hex_to_vec(hex: &str) -> Vec<u8> {
    let mut vec = Vec::with_capacity(hex.len());
    for chunk in hex.as_bytes().chunks(2) {
        let src = unsafe { String::from_utf8_unchecked(chunk.to_vec()) };
        let byte = match u8::from_str_radix(&src, 16) {
            Ok(byte) => byte,
            Err(err) => panic!("{}", err),
        };
        vec.push(byte);
    }
    vec
}