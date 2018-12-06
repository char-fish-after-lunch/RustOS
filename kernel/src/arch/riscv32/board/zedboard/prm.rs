use core::fmt::{Write, Result, Arguments};
use core::ptr::{read_volatile, write_volatile};
use super::super::bbl::sbi;
use spin::Mutex;
use alloc::string::String;

lazy_static! {
    static ref prm_mutex: Mutex<()> = Mutex::new(());
}

struct PRM;

impl Write for PRM {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.bytes() {
            if c == 127 {
                putchar(8);
                putchar(b' ');
                putchar(8);
            } else if c == 10 {
                putchar(13); // \r
                putchar(10); // \n
            } else {
                putchar(c);
            }
        }
        Ok(())
    }
}

fn putchar(c: u8) {
    sbi::prm_putchar(c as usize);
}

pub fn prm_getbyte() -> u8 {
    sbi::prm_getchar() as u8
}

pub fn prm_getline() -> String {
    let mut s = String::new();
    loop {
        let c = prm_getbyte() as char;
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

fn putfmt(fmt: Arguments) {
    PRM.write_fmt(fmt).unwrap();
}

#[macro_export]
macro_rules! print_prm {
    ($($arg:tt)*) => ({
        $crate::arch::prm::print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println_prm {
    ($fmt:expr) => (print_prm!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print_prm!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: Arguments) {
    let mutex = prm_mutex.lock();
    putfmt(args);
}

pub fn test_prm() {
    print_prm!("test\n");
    let hello = prm_getline();
    println!("{}", hello);
}