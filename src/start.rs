use crate::logger::init_logger;
use crate::{dbgbreak, hal};

pub fn startup() {
    init_logger();

    log::info!("Hello, World!");

    hal::gdt::init();
    hal::idt::init();

    // Demonstrate recoverable MSR access via the exception table.
    use hal::extable::{rdmsr_safe, wrmsr_safe};
    const IA32_APIC_BASE: u32 = 0x1B; // valid on every x86_64 CPU
    const BOGUS_MSR: u32 = 0xFFFF_FFFF; // does not exist -> #GP
    match rdmsr_safe(IA32_APIC_BASE) {
        Ok(v) => log::info!("rdmsr(IA32_APIC_BASE) = {:#X}", v),
        Err(()) => log::error!("rdmsr(IA32_APIC_BASE) unexpectedly faulted"),
    }
    match rdmsr_safe(BOGUS_MSR) {
        Ok(v) => log::error!("rdmsr(bogus) unexpectedly succeeded: {:#X}", v),
        Err(()) => log::info!("rdmsr(bogus) faulted and was recovered (#GP)"),
    }
    match wrmsr_safe(BOGUS_MSR, 0) {
        Ok(()) => log::error!("wrmsr(bogus) unexpectedly succeeded"),
        Err(()) => log::info!("wrmsr(bogus) faulted and was recovered (#GP)"),
    }

    hal::acpi::find_apic();

    dbgbreak!();
}
