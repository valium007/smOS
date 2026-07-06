//! x86_64 helpers

use core::arch::asm;

/// A struct describing a pointer to a descriptor table (GDT / IDT).
/// This is in a format suitable for giving to 'lgdt' or 'lidt'.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct DescriptorTablePointer {
    pub limit: u16, // Limit
    pub base: u64,  // Base address
}

/// Read from port
#[inline]
pub fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        asm!(
            "in al, dx",
            out("al") value,
            in("dx") port,
            options(nomem, nostack)
        );
    }
    value
}

/// Write to port
#[inline]
pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack)
        );
    }
}

/// Read 16-bit value from port
#[inline]
pub fn inw(port: u16) -> u16 {
    let value: u16;
    unsafe {
        asm!(
            "in ax, dx",
            out("ax") value,
            in("dx") port,
            options(nomem, nostack)
        );
    }
    value
}

/// Write 16-bit value to port
#[inline]
pub fn outw(port: u16, value: u16) {
    unsafe {
        asm!(
            "out dx, ax",
            in("dx") port,
            in("ax") value,
            options(nomem, nostack)
        );
    }
}

/// Read 32-bit value from port
#[inline]
pub fn inl(port: u16) -> u32 {
    let value: u32;
    unsafe {
        asm!(
            "in eax, dx",
            out("eax") value,
            in("dx") port,
            options(nomem, nostack)
        );
    }
    value
}

/// Write 32-bit value to port
#[inline]
pub fn outl(port: u16, value: u32) {
    unsafe {
        asm!(
            "out dx, eax",
            in("dx") port,
            in("eax") value,
            options(nomem, nostack)
        );
    }
}

/// Read MSR (Model Specific Register)
#[inline]
pub fn rdmsr(msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") low,
            out("edx") high,
            options(nomem, nostack)
        );
    }
    ((high as u64) << 32) | (low as u64)
}

/// Write MSR (Model Specific Register)
#[inline]
pub fn wrmsr(msr: u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;
    unsafe {
        asm!(
            "wrmsr",
            in("ecx") msr,
            in("eax") low,
            in("edx") high,
            options(nomem, nostack)
        );
    }
}

/// Read CR0 register
#[inline]
pub fn read_cr0() -> u64 {
    let value: u64;
    unsafe {
        asm!("mov {}, cr0", out(reg) value, options(nomem, nostack));
    }
    value
}

/// Write CR0 register
#[inline]
pub fn write_cr0(value: u64) {
    unsafe {
        asm!("mov cr0, {}", in(reg) value, options(nomem, nostack));
    }
}

/// Read CR2 register (page fault address)
#[inline]
pub fn read_cr2() -> u64 {
    let value: u64;
    unsafe {
        asm!("mov {}, cr2", out(reg) value, options(nomem, nostack));
    }
    value
}

/// Read CR3 register (page table base)
#[inline]
pub fn read_cr3() -> u64 {
    let value: u64;
    unsafe {
        asm!("mov {}, cr3", out(reg) value, options(nomem, nostack));
    }
    value
}

/// Write CR3 register
#[inline]
pub fn write_cr3(value: u64) {
    unsafe {
        asm!("mov cr3, {}", in(reg) value, options(nomem, nostack));
    }
}

/// Read CR4 register
#[inline]
pub fn read_cr4() -> u64 {
    let value: u64;
    unsafe {
        asm!("mov {}, cr4", out(reg) value, options(nomem, nostack));
    }
    value
}

/// Write CR4 register
#[inline]
pub fn write_cr4(value: u64) {
    unsafe {
        asm!("mov cr4, {}", in(reg) value, options(nomem, nostack));
    }
}

/// Invalidate TLB entry for address
#[inline]
pub fn invlpg(addr: u64) {
    unsafe {
        asm!("invlpg [{}]", in(reg) addr, options(nostack));
    }
}

#[inline]
pub fn load_tss(sel: u32) {
    unsafe {
        asm!("ltr {0:x}", in(reg) sel);
    }
}

#[inline]
pub fn store_tss(sel: *const u64) {
    unsafe {
        asm!("str [{}]", in(reg) sel);
    }
}

#[inline]
pub fn lidt(idt: u64) {
    unsafe {
        asm!("lidt [{}]", in(reg) idt);
    }
}

#[inline]
pub fn sidt(idt: *const u64) {
    unsafe {
        asm!("sidt [{}]", in(reg) idt);
    }
}

#[inline]
pub fn lgdt(gdt: u64) {
    unsafe {
        asm!("lgdt [{}]", in(reg) gdt);
    }
}

#[inline]
pub fn sgdt(gdt: *const u64) {
    unsafe {
        asm!("sgdt [{}]", in(reg) gdt);
    }
}

#[inline]
pub fn sti() {
    unsafe {
        asm!("sti");
    }
}

pub fn cli() {
    unsafe {
        asm!("cli");
    }
}

/// Get CPU features using CPUID
pub fn cpuid(leaf: u32) -> (u32, u32, u32, u32) {
    let (eax, ebx, ecx, edx): (u32, u32, u32, u32);
    unsafe {
        asm!(
            "push rbx",
            "cpuid",
            "mov {ebx_out:e}, ebx",
            "pop rbx",
            inout("eax") leaf => eax,
            ebx_out = out(reg) ebx,
            out("ecx") ecx,
            out("edx") edx,
            options(nomem, nostack)
        );
    }
    (eax, ebx, ecx, edx)
}

#[inline]
pub fn hcf() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn __chkstk() {}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_load_16() {
    panic!("unknown function")
}

#[unsafe(no_mangle)]
pub extern "C" fn _fltused() {
    panic!("unknown function")
}
