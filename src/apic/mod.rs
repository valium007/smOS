use crate::globals::*;
use crate::util::*;
use bitfield::bitfield;

pub enum DeliveryMode {
    Fixed = 0b000,
    LowestPriority = 0b001,
    SMI = 0b010,
    NMI = 0b100,
    INIT = 0b101,
    StartUp = 0b110,
}

pub enum DestinationMode {
    Physical = 0,
    Logical = 1,
}

pub enum Level {
    DeAssert = 0,
    Assert = 1,
}

pub enum TriggerMode {
    Edge = 0,
    Level = 1,
}

pub enum DestinationShorthand {
    NoShorthand = 0b00,
    Myself = 0b01,
    AllIncludingSelf = 0b10,
    AllExcludingSelf = 0b11,
}

bitfield! {
    pub struct X2APIC_ICR(u64);
    impl Debug;
    pub get_vector, set_vector: 7,0;
    pub get_delivery_mode, set_delivery_mode: 10,8;
    pub get_destination_mode, set_destination_mode: 11,11;
    pub get_level, set_level: 14,14;
    pub get_trigger_mode, set_trigger_mode: 15,15;
    pub get_destination_shorthand, set_destination_shorthand: 19,18;
    pub get_destination, set_destination: 63,32;
}

//this ICR is strictly for x2apic, for other apic its different
impl X2APIC_ICR {
    pub fn ICR(
        vector: u8,
        delivery_mode: DeliveryMode,
        destination_mode: DestinationMode,
        level: Level,
        trigger_mode: TriggerMode,
        destination_shorthand: DestinationShorthand,
        destination: u32,
    ) {
        let mut icr = X2APIC_ICR(0);
        icr.set_vector(vector as u64);
        icr.set_delivery_mode(delivery_mode as u64);
        icr.set_destination_mode(destination_mode as u64);
        icr.set_level(level as u64);
        icr.set_trigger_mode(trigger_mode as u64);
        icr.set_destination_shorthand(destination_shorthand as u64);
        icr.set_destination(destination as u64);

        wrmsr(X2APIC_ICR_MSR, icr.0);
    }
}

pub fn self_nmi() {
    let mut icr = X2APIC_ICR(0);
    icr.set_delivery_mode(DeliveryMode::NMI as u64);
    icr.set_destination_mode(DestinationMode::Physical as u64);
    icr.set_level(Level::Assert as u64);
    icr.set_trigger_mode(TriggerMode::Edge as u64);
    icr.set_destination_shorthand(DestinationShorthand::Myself as u64);
    wrmsr(X2APIC_ICR_MSR, icr.0);
}

pub fn apic_status() {
    let value: u64 = rdmsr(IA32_APIC_BASE_MSR);
    if value & IA32_APIC_BASE_EN != 0 {
        log::info!("apic enabled")
    } else {
        log::info!("apic disabled")
    }
    if value & IA32_APIC_BASE_EXTD != 0 {
        log::info!("x2apic enabled")
    } else {
        log::info!("x2apic disabled")
    }
}

pub fn enable_x2apic() {
    wrmsr(
        IA32_APIC_BASE_MSR,
        rdmsr(IA32_APIC_BASE_MSR) | (IA32_APIC_BASE_EN | IA32_APIC_BASE_EXTD),
    ); //bitmask to set global_enable and x2apic bit
}

//only available when x2apic is enabled
pub fn self_ipi(vector: u8) {
    wrmsr(SELF_IPI_MSR, vector as u64);
}
