// message types
pub const SERVER_RESPONSE: u16 = 500;
pub const CLIENT_GET: u16 = 600;
pub const CLIENT_INSERT: u16 = 601;
pub const CLIENT_REMOVE: u16 = 602;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Message {
    pub typ: u16,
    pub content: [u8; 16], // thorws segmentation error for large size
}