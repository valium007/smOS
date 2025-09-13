use core::arch::asm;
#[macro_export]
macro_rules! print {
    ($($t:tt)*) => { $crate::logger::print_fmt(format_args!($($t)*)) };
}

#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($t:tt)*) => { $crate::logger::print_fmt(format_args!("{}\n", format_args!($($t)*))) };
}

#[macro_export]
macro_rules! dbgbreak {
    () => {
        unsafe { core::arch::asm!("xchg bx,bx") }
    };
}

#[macro_export]
macro_rules! int3 {
    () => {
        unsafe { core::arch::asm!("int3") }
    };
}

#[macro_export]
macro_rules! BIT {
    ($x: expr) => {
        (1 << ($x))
    };
}

pub fn rdmsr(msr: u32) -> u64 {
    let lo: u32;
    let hi: u32;

    unsafe {
        asm!("rdmsr",
             in("rcx") msr, out("rax") lo, out("rdx") hi,
             options(nostack));
    }

    ((hi as u64) << 32) | lo as u64
}

pub fn wrmsr(msr: u32, value: u64) {
    let lo: u32 = value as u32;
    let hi: u32 = (value >> 32) as u32;
    unsafe {
        asm!("wrmsr",
             in("rcx") msr, in("rax") lo, in("rdx") hi,
             options(nostack));
    }
}
