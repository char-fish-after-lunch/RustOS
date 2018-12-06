use core::fmt::{Write, Result, Arguments};
use core::ptr::{read_volatile, write_volatile};
use bbl::sbi;
use spin::Mutex;
use alloc::string::String;
use lazy_static::lazy_static;

struct PRMSerial;

impl Write for PRMSerial {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.bytes() {
            if c == 127 {
                putbyte(8);
                putbyte(b' ');
                putbyte(8);
            } else if c == 10 {
                putbyte(13); // \r
                putbyte(10); // \n
            } else {
                putbyte(c);
            }
        }
        Ok(())
    }
}

pub fn putbyte(c: u8) {
    sbi::prm_putchar(c as usize);
}

pub fn getbyte() -> u8 {
    sbi::prm_getchar() as u8
}

pub fn prm_getline() -> String {
    let mut s = String::new();
    loop {
        let c = getbyte() as char;
        match c {
            '\u{7f}' /* '\b' */ => {
                if s.pop().is_some() {
                    print!("\u{7f}");
                }
            }
            ' '...'\u{7e}' => {
                s.push(c);
                print!("{}", c);
            }
            '\n' | '\r' => {
                print!("\n");
                return s;
            }
            _ => {}
        }
    }
}

pub fn putfmt(fmt: Arguments) {
    PRMSerial.write_fmt(fmt).unwrap();
}

#[macro_export]
macro_rules! print_prm {
    ($($arg:tt)*) => ({
        $crate::arch::board::prm::print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println_prm {
    ($fmt:expr) => (print_prm!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print_prm!(concat!($fmt, "\n"), $($arg)*));
}
