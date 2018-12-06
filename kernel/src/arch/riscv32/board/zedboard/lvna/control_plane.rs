// Control Plane(CP) definitions in LvNA
use super::cp_request::{CPCommand, CPResponse};
use super::prm::prm_send_command;
use lazy_static::lazy_static;

pub const CP_PROTOCOL_BEGIN: [u8; 2] = [0xca, 0xfe];

pub const CP_PROTOCOL_END: u8 = 0xed;
pub const CP_CTRL_READ: u8 =    0;
pub const CP_CTRL_WRITE: u8 =   1;

pub const CP_REG_CORE: u8 =  0;
pub const CP_REG_MEM: u8 =   1;
pub const CP_REG_CACHE: u8 = 2;
pub const CP_REG_IO: u8 =    3;

pub const CP_TAB_P: u8 = 0;
pub const CP_TAB_S: u8 = 1;
pub const CP_TAB_T: u8 = 2;

pub struct CPRegister {
    pub cp: u8,
    pub tab: u8,
    pub col: u8
}

impl CPRegister {
    pub const fn new(cp: u8, tab: u8, col: u8) -> Self {
        CPRegister {cp, tab, col}
    }

    pub fn read(&self, row: u8) -> Result<u32, ()> {
        let cmd = CPCommand {
            val: 0,
            rw: CP_CTRL_READ,
            cp: self.cp,
            tab: self.tab,
            col: self.col,
            row: row,
            end: CP_PROTOCOL_END
        };

        let res: CPResponse = prm_send_command(cmd);
        match res.valid {
            1 => Ok(res.val),
            _ => Err(())
        }
    }

    pub fn write(&self, row: u8, val: u32) -> Result<(), ()> {
        let cmd = CPCommand {
            val: val,
            rw: CP_CTRL_WRITE,
            cp: self.cp,
            tab: self.tab,
            col: self.col,
            row: row,
            end: CP_PROTOCOL_END
        };
        
        let res: CPResponse = prm_send_command(cmd);
        match res.valid {
            1 => Ok(()),
            _ => Err(())
        }
    }
}

pub const CPCoreDSID    : CPRegister = CPRegister::new(CP_REG_CORE , CP_TAB_P, 0);
pub const CPCoreBase    : CPRegister = CPRegister::new(CP_REG_CORE , CP_TAB_P, 1);
pub const CPCoreSize    : CPRegister = CPRegister::new(CP_REG_CORE , CP_TAB_P, 2);
pub const CPCoreHartid  : CPRegister = CPRegister::new(CP_REG_CORE , CP_TAB_P, 3);

pub const CPMemSize     : CPRegister = CPRegister::new(CP_REG_MEM  , CP_TAB_P, 0);
pub const CPMemFreq     : CPRegister = CPRegister::new(CP_REG_MEM  , CP_TAB_P, 1);
pub const CPMemInc      : CPRegister = CPRegister::new(CP_REG_MEM  , CP_TAB_P, 2);

pub const CPMemCached   : CPRegister = CPRegister::new(CP_REG_MEM  , CP_TAB_S, 0);
pub const CPMemUncached : CPRegister = CPRegister::new(CP_REG_MEM  , CP_TAB_S, 1);

pub const CPCacheMask   : CPRegister = CPRegister::new(CP_REG_CACHE, CP_TAB_P, 0);

pub const CPCacheAccess : CPRegister = CPRegister::new(CP_REG_CACHE, CP_TAB_S, 0);
pub const CPCacheMiss   : CPRegister = CPRegister::new(CP_REG_CACHE, CP_TAB_S, 1);
pub const CPCacheUsage  : CPRegister = CPRegister::new(CP_REG_CACHE, CP_TAB_S, 2);