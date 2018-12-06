use core::mem::size_of;

#[repr(C)]
pub struct CPCommand {
    pub val: u32,
    pub rw: u8,
    pub cp: u8,
    pub tab: u8,
    pub col: u8,
    pub row: u8,
    pub end: u8,
}

#[repr(C)]
pub struct CPResponse {
    pub val: u32,
    pub valid: u8,
}

pub const CP_COMMAND_SIZE: usize = size_of::<CPCommand>();
pub const CP_RESPONSE_SIZE: usize = size_of::<CPResponse>();

impl CPCommand {
    pub fn as_raw(&self) -> [u8; CP_COMMAND_SIZE] {
        let mut bytes: [u8; CP_COMMAND_SIZE] = [0; CP_COMMAND_SIZE];
        let val = self.val.to_le_bytes();
        bytes = [val[3], val[2], val[1], val[0], self.rw, self.cp, self.tab, self.col, self.row, self.end, 0, 0];
        bytes
    }
}

impl CPResponse {
    pub fn as_raw(&self) -> [u8; CP_RESPONSE_SIZE] {
        let mut bytes: [u8; CP_RESPONSE_SIZE] = [0; CP_RESPONSE_SIZE];
        let val = self.val.to_le_bytes();
        bytes = [val[3], val[2], val[1], val[0], self.valid, 0, 0, 0];
        bytes
    }

    pub fn from_raw(raw: &[u8]) -> CPResponse {
        CPResponse {
            val: u32::from_le_bytes([raw[3], raw[2], raw[1], raw[0]]),
            valid: raw[4]
        }
    }
}
