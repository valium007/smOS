use lazy_static::lazy_static;
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
        idt
    };
}

extern "x86-interrupt" fn bp_handler(stack_frame: InterruptStackFrame) {
    log::warn!(
        "EXCEPTION: BREAKPOINT at RIP={:#X}",
        stack_frame.instruction_pointer - 1
    );
}

extern "x86-interrupt" fn db_handler(stack_frame: InterruptStackFrame) {
    log::warn!(
        "EXCEPTION: DEBUG at RIP={:#X}",
        stack_frame.instruction_pointer
    );
}
