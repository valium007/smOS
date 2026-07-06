use bitfield::bitfield;
use core::{arch::naked_asm, ptr::addr_of};
use paste::paste;
use spin::once::Once;

use crate::arch::{DescriptorTablePointer, cli, lidt, sti};

static IDT: Once<[u128; 256]> = Once::new();

bitfield! {
    pub struct IdtEntry(u128);
    impl Debug;
    pub get_offset_low, set_offset_low: 15, 0;       // [0-15]   offset bits 0-15
    pub get_selector, set_selector: 31, 16;           // [16-31]  segment selector
    pub get_ist, set_ist: 34, 32;                     // [32-34]  interrupt stack table
                                                      // [35-39]  reserved (zeros)
    pub get_gate_type, set_gate_type: 43, 40;         // [40-43]  0xE=interrupt, 0xF=trap
                                                      // [44]     must be zero
    pub get_dpl, set_dpl: 46, 45;                     // [45-46]  privilege level
    pub get_present, set_present: 47, 47;             // [47]     present
    pub get_offset_high, set_offset_high: 63, 48;     // [48-63]  offset bits 16-31
    pub get_offset_upper, set_offset_upper: 95, 64;   // [64-95]  offset bits 32-63
                                                      // [96-127] reserved (zeros)
}

impl IdtEntry {
    pub fn new(handler: u64, selector: u16, ist: u8) -> Self {
        let mut entry = IdtEntry(0);
        entry.set_offset_low(handler as u128);
        entry.set_offset_high((handler >> 16) as u128);
        entry.set_offset_upper((handler >> 32) as u128);
        entry.set_selector(selector as u128);
        entry.set_gate_type(0xE); //interrupt gate
        entry.set_present(1);
        entry.set_dpl(0);
        entry.set_ist(ist as u128);
        entry
    }
}

pub fn init() {
    let de_handler = de_handler as *const u64 as u64;
    let gp_handler = gp_handler as *const u64 as u64;

    let de_entry = IdtEntry::new(de_handler, 0x8, 0).0;
    let gp_entry = IdtEntry::new(gp_handler, 0x8, 0).0;

    let mut idt: [u128; 256] = [0; 256];
    idt[0] = de_entry;
    idt[13] = gp_entry;

    IDT.call_once(|| idt);

    let idtr = DescriptorTablePointer {
        limit: (size_of::<[u128; 256]>() - 1) as u16,
        base: IDT.as_mut_ptr().addr() as u64,
    };

    cli();
    lidt(addr_of!(idtr) as u64);
    sti();
}

#[derive(Debug)]
#[repr(C)]
struct TrapFrame {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rdi: u64,
    rsi: u64,
    rbp: u64,
    rbx: u64,
    rdx: u64,
    rcx: u64,
    rax: u64,
    exception_number: u64, // Software saved (see interrupt_handler.S)
    error_code: u64,       // Software or hardware saved (see interrupt_handler.S)
    rip: u64,              // Hardware saved
    cs: u64,               // Hardware saved
    rflags: u64,           // Hardware saved
    rsp: u64,              // Hardware saved
    ss: u64,               // Hardware saved
}

#[unsafe(no_mangle)]
extern "win64" fn handle_exception(stack_frame: *mut TrapFrame) {
    let frame = unsafe { &mut *stack_frame };

    // Recoverable fault? If the faulting RIP is registered in the exception
    // table, redirect execution to its fixup stub and return; the handler
    // epilogue restores registers and `iretq`s into the stub.
    if let Some(fixup) = crate::hal::extable::fixup_exception(frame.rip) {
        frame.rip = fixup;
        return;
    }

    // Otherwise this is a genuine fault we can't handle: oops.
    log::error!(
        "EXCEPTION: {:#X} (err={:#X}) at RIP={:#X}",
        frame.exception_number,
        frame.error_code,
        frame.rip
    );
    panic!("unhandled CPU exception {:#X}", frame.exception_number);
}

macro_rules! handler {
    ($name:ident, $exception_number:expr) => {
        paste! {
            #[unsafe(naked)]
            pub unsafe extern "custom" fn [<$name _handler>]() {
                naked_asm!(
                    "push 0",
                    stringify!(push $exception_number),
                    "push    rax
                        push    rcx
                        push    rdx
                        push    rbx
                        push    rbp
                        push    rsi
                        push    rdi
                        push    r8
                        push    r9
                        push    r10
                        push    r11
                        push    r12
                        push    r13
                        push    r14
                        push    r15",
                    "mov rcx, rsp",
                    "sub rsp, 0x20",
                     "call handle_exception",
                     "add rsp, 0x20",
                     "
                     pop     r15
                        pop     r14
                        pop     r13
                        pop     r12
                        pop     r11
                        pop     r10
                        pop     r9
                        pop     r8
                        pop     rdi
                        pop     rsi
                        pop     rbp
                        pop     rbx
                        pop     rdx
                        pop     rcx
                        pop     rax
                        add rsp, 0x10
                        iretq
                        "

                )
            }
        }
    };
}

macro_rules! handler_with_code {
    ($name:ident, $exception_number:expr) => {
        paste! {
            #[unsafe(naked)]
            pub unsafe extern "custom" fn [<$name _handler>]() {
                naked_asm!(
                    stringify!(push $exception_number),
                    "push    rax
                        push    rcx
                        push    rdx
                        push    rbx
                        push    rbp
                        push    rsi
                        push    rdi
                        push    r8
                        push    r9
                        push    r10
                        push    r11
                        push    r12
                        push    r13
                        push    r14
                        push    r15",
                    "mov rcx, rsp",
                    "sub rsp, 0x20",
                     "call handle_exception",
                     "add rsp, 0x20",
                     "
                     pop     r15
                        pop     r14
                        pop     r13
                        pop     r12
                        pop     r11
                        pop     r10
                        pop     r9
                        pop     r8
                        pop     rdi
                        pop     rsi
                        pop     rbp
                        pop     rbx
                        pop     rdx
                        pop     rcx
                        pop     rax
                        add rsp, 0x10
                        iretq
                        "
                )
            }
        }
    };
}

handler!(de, 0);
handler_with_code!(gp, 13);
