use crate::apic::*;
use crate::dbgbreak;
use crate::logger::init_logger;
use crate::{gdt, idt};

pub fn startup() {
    dbgbreak!();

    init_logger();

    log::info!("Hello, World!");
    gdt::init();
    idt::init();

    apic_status();
    enable_x2apic();
    apic_status();

    self_nmi();

    X2APIC_ICR::ICR(
        0,
        DeliveryMode::NMI,
        DestinationMode::Physical,
        Level::Assert,
        TriggerMode::Edge,
        DestinationShorthand::Myself,
        0u32,
    );
    self_ipi(39);

    dbgbreak!();
}
