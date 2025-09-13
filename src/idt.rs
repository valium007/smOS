use lazy_static::lazy_static;
use paste::paste;
use x86_64::instructions::interrupts::*;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub fn init() {
    IDT.load();
    enable();
    log::info!("IDT initialized")
}

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(bp_handler);
        idt.debug.set_handler_fn(db_handler);
        idt.general_protection_fault.set_handler_fn(gp_handler);
        idt.non_maskable_interrupt.set_handler_fn(nmi_handler);
        unsafe { idt[39].set_handler_fn(my_handler) };
        idt
    };
}

macro_rules! build_handler {
    ($name:ident, $error_code:expr) => {
        paste! {
            extern "x86-interrupt" fn [<$name _handler>](stack_frame: InterruptStackFrame, error_code: u64) {
                log::warn!("#{} at RIP={:#X} ERR={}", stringify!([<$name:upper>]),stack_frame.instruction_pointer,error_code);
            }
        }
    };
    ($name:ident) => {
        paste! {
            extern "x86-interrupt" fn [<$name _handler>](stack_frame: InterruptStackFrame) {
                log::warn!("#{} at RIP={:#X}", stringify!([<$name:upper>]),stack_frame.instruction_pointer);
            }
        }
    };
}

build_handler!(gp, 1);
build_handler!(nmi);
build_handler!(bp);
build_handler!(db);
build_handler!(my);
