use core::ptr::addr_of;
use spin;

use bitfield::bitfield;

use crate::arch::{DescriptorTablePointer, cli, lgdt};

bitfield! {
    pub struct SegmentDescriptor(u64);
    impl Debug;
    pub get_limit_low, set_limit_low: 15, 0;            // [0-15]
    pub get_base_low, set_base_low: 31, 16;             // [16-31]
    pub get_base_middle, set_base_middle: 39, 32;       // [32-39]
    pub get_access, set_access: 40, 40;
    pub get_rw, set_rw: 41, 41;
    pub get_dc, set_dc: 42, 42;
    pub get_executable, set_executable: 43, 43;
    pub get_code_data, set_code_data: 44, 44;                 // [44]
    pub get_dpl, set_dpl: 46, 45;                       // [45-46]
    pub get_present, set_present: 47, 47;               // [47]
    pub get_limit_high, set_limit_high: 51, 48;         // [48-51]
    pub get_avl, set_avl: 52, 52;                       // [52]
    pub get_long_mode, set_long_mode: 53, 53;           // [53]
    pub get_default_bit, set_default_bit: 54, 54;       // [54]
    pub get_granularity, set_granularity: 55, 55;       // [55]
    pub get_base_high, set_base_high: 63, 56;           // [56-63]
}

static KGDT: spin::Once<[u64; 16]> = spin::Once::new();

fn create_kernel_code_64() -> u64 {
    let mut descriptor = SegmentDescriptor(0);
    descriptor.set_rw(1);
    descriptor.set_executable(1);
    descriptor.set_code_data(1);
    descriptor.set_present(1);
    descriptor.set_long_mode(1);
    descriptor.0
}

fn create_kernel_data_64() -> u64 {
    let mut descriptor = SegmentDescriptor(0);
    descriptor.set_rw(1);
    descriptor.set_access(1);
    descriptor.set_code_data(1);
    descriptor.set_present(1);
    descriptor.0
}

fn create_null() -> u64 {
    0u64
}

pub fn init() {
    let null = create_null();
    let kernel_code_64 = create_kernel_code_64();
    let kernel_data_64 = create_kernel_data_64();

    let mut kgdt_: [u64; 16] = [0; 16];

    kgdt_[0] = null;
    kgdt_[1] = kernel_code_64;
    kgdt_[2] = kernel_data_64;

    KGDT.call_once(|| kgdt_);

    let gdtr = DescriptorTablePointer {
        limit: (size_of::<[u64; 16]>() - 1) as u16,
        base: KGDT.as_mut_ptr().addr() as u64,
    };

    cli();
    lgdt(addr_of!(gdtr) as u64);

    unsafe {
        core::arch::asm!(
            "
            2:
            push 0x08
            lea rax, [3f]
            push rax
            retfq
            3:
            mov ax, 0x10
            mov ds, ax
            mov es, ax
            mov fs, ax
            mov gs, ax
            mov ss, ax
            "
        );
    }
}
