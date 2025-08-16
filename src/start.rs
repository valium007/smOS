use crate::logger::init_logger;
use crate::mm::pmm;
use crate::{dbgbreak, int3};
use crate::{gdt, idt};

pub fn startup() {
    dbgbreak!();

    pmm::pmm_init();
    init_logger();

    log::info!("Hello, World!");
    gdt::init();
    idt::init();

    dbgbreak!();
}
