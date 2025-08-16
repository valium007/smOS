use crate::println;
use core::arch::asm;
use log::*;
use spin::Mutex;

pub unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("dx") port, in("al") val, options(preserves_flags, nomem, nostack));
}

pub unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    asm!("in al, dx", out("al") ret, in("dx") port, options(preserves_flags, nomem, nostack));
    return ret;
}

pub unsafe fn puts(s: &str) {
    for b in s.bytes() {
        putb(b);
    }
}

pub unsafe fn putb(b: u8) {
    outb(0xe9, b);
}

struct Writer;

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe { puts(s) };
        Ok(())
    }
}

static WRITER: Mutex<Writer> = Mutex::new(Writer);

pub fn print_fmt(args: core::fmt::Arguments) {
    let mut writer = WRITER.lock();
    <Writer as core::fmt::Write>::write_fmt(&mut writer, args).unwrap();
}

pub static LOGGER: Logger = Logger;
pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _m: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        println!("[{}] {}", record.level(), record.args())
    }

    fn flush(&self) {}
}

pub fn init_logger() {
    log::set_logger(&LOGGER).expect("failed to init logger");
    log::set_max_level(log::LevelFilter::Trace);
}
