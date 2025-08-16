#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(static_mut_refs)]
#![feature(abi_x86_interrupt)]

pub mod shim;
pub mod logger;
pub mod start;
pub mod gdt;
pub mod idt;
pub mod macros;

use core::f32::MANTISSA_DIGITS;

use limine::memory_map::Entry;
use limine::BaseRevision;
use limine::request::{FramebufferRequest, HhdmRequest, ExecutableAddressRequest, MemoryMapRequest, RequestsEndMarker, RequestsStartMarker};

use crate::shim::hcf;
use crate::start::startup;


#[used]
#[unsafe(link_section = ".request$a")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests$ba")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests$bb")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".request$bc")]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[unsafe(link_section = ".request$bd")]
static EXECUTABLE_ADDRESS_REQUEST: ExecutableAddressRequest = ExecutableAddressRequest::new();

#[used]
#[unsafe(link_section = ".request$be")]
static MMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[unsafe(link_section = ".requests$c")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();



#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    // All limine requests must also be referenced in a called function, otherwise they may be
    // removed by the linker.
    assert!(BASE_REVISION.is_supported());
    let hhdm_offset = HHDM_REQUEST.get_response().unwrap().offset();
    let exec_virtual = EXECUTABLE_ADDRESS_REQUEST.get_response().unwrap().virtual_base();
    let exec_vphy = EXECUTABLE_ADDRESS_REQUEST.get_response().unwrap().physical_base();
    println!("hhdm offset = {:#X}",hhdm_offset);
    println!("exec virtual_base = {:#X}",exec_virtual);
    println!("exec phy_base = {:#X}",exec_vphy);
    startup();
    hcf();
}



static mut base:u64 = 0;

fn pmm_init() {

    unsafe {
    base = HHDM_REQUEST.get_response().unwrap().offset();
    let mmap = MMAP_REQUEST.get_response().unwrap().entries();


    for entry in mmap {
        
    }

}

}





#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("PANIC!! {}",_info);
    hcf();
}