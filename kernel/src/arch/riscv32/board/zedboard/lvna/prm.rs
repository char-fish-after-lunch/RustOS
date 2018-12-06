
use super::cp_request::*;
use super::serial::{putbyte, getbyte, putfmt};
use super::control_plane::*;
use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt::Arguments;

lazy_static! {
    static ref prm_mutex: Mutex<()> = Mutex::new(());
}

pub fn print(args: Arguments) {
    let mutex = prm_mutex.lock();
    putfmt(args);
}

pub fn prm_send_command(cmd: CPCommand) -> CPResponse {
    let mutex = prm_mutex.lock();
    for i in 0..2 {
        putbyte(CP_PROTOCOL_BEGIN[i]);
    }

    let raw_bytes = cmd.as_raw();
    for i in 0..CP_COMMAND_SIZE {
        putbyte(raw_bytes[i]);
    }

    let mut res_bytes: [u8; CP_RESPONSE_SIZE] = [0; CP_RESPONSE_SIZE];
    for i in 0..CP_RESPONSE_SIZE {
        res_bytes[i] = getbyte();
    }

    CPResponse::from_raw(&res_bytes)
}

pub fn test_prm() {
    let res1 = CPCoreHartid.read(0);
    match res1 {
        Ok(x) => println!("ok: {}", x),
        Err(()) => println!("err"),
    }
    
    let res2 = CPCoreHartid.write(0, 10);
    match res2 {
        Ok(()) => println!("ok"),
        Err(()) => println!("err"),
    }

    let res3 = CPCoreHartid.read(0);
    match res3 {
        Ok(x) => println!("ok: {}", x),
        Err(()) => println!("err"),
    }
}