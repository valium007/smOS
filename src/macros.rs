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
    () => { unsafe { core::arch::asm!("xchg bx,bx") } };
}

#[macro_export]
macro_rules! int3 {
    () => { unsafe { core::arch::asm!("int3") } };
}
