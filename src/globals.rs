use crate::BIT;

pub const IA32_APIC_BASE_MSR: u32 = 0x1b;
pub const X2APIC_ICR_MSR: u32 = 0x830;
pub const SELF_IPI_MSR: u32 = 0x83f;

pub const IA32_APIC_BASE_EN: u64 = BIT!(11);
pub const IA32_APIC_BASE_EXTD: u64 = BIT!(10);
pub const IA32_APIC_BASE_BSP: u64 = BIT!(8);