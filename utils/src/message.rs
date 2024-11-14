// message types
pub const KEY_SIZE: usize = 16;
pub const VALUE_SIZE: usize = 64;

#[repr(u16)]
#[derive(Debug, Clone)]
pub enum MessageType {
    Empty = 0,
    Get = 500,
    Insert = 501,
    Remove = 502,
    GetSuccess = 600,
    GetNotFound = 601,
    InsertSuccess = 602,
    RemoveSuccess = 603,
    RemoveNotFound = 604,
}

#[repr(C)] // makes struct compatible with C's layout convention
#[derive(Debug, Clone)]
pub struct Message {
    pub typ: MessageType,
    pub key: [u8; KEY_SIZE],
    pub value: [u8; VALUE_SIZE],
}

impl Message {
    pub fn empty() -> Self {
        Message {
            typ: MessageType::Empty,
            key: [0; KEY_SIZE],
            value: [0; VALUE_SIZE]
        }
    }

    pub fn serliaize_key(s: &str) -> [u8; KEY_SIZE] {
        let mut array = [0u8; KEY_SIZE];
        let bytes = s.as_bytes();

        let len = bytes.len().min(KEY_SIZE);
        array[..len].copy_from_slice(&bytes[..len]);

        array
    }

    pub fn serliaize_value(s: &str) -> [u8; VALUE_SIZE] {
        let mut array = [0u8; VALUE_SIZE];
        let bytes = s.as_bytes();

        let len = bytes.len().min(VALUE_SIZE);
        array[..len].copy_from_slice(&bytes[..len]);

        array
    }

    pub fn deserialize_key(bytes: [u8; KEY_SIZE]) -> String {
        // trim trailing null characters
        String::from_utf8(bytes.to_vec())
            .map(|s| s.trim_end_matches('\0').to_string())
            .unwrap_or_default()
    }

    pub fn deserialize_value(bytes: [u8; VALUE_SIZE]) -> String {
        // trim trailing null characters
        String::from_utf8(bytes.to_vec())
            .map(|s| s.trim_end_matches('\0').to_string())
            .unwrap_or_default()
    }
}
