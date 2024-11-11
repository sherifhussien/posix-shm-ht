pub mod message;
pub mod shared_mem;
pub mod sem;

// TODO: check
pub fn serliaize(s: &str) -> [u8; 16] {
    let mut array = [0u8; 16];
    let bytes = s.as_bytes();

    let len = bytes.len().min(16);
    array[..len].copy_from_slice(&bytes[..len]);

    array
}

// TODO: check
pub fn deserialize(bytes: [u8; 16]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}