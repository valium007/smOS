#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(static_mut_refs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(abi_x86_interrupt)]

pub mod apic;
pub mod gdt;
pub mod globals;
pub mod idt;
pub mod logger;
pub mod mm;
pub mod shim;
pub mod start;
pub mod util;

use limine::BaseRevision;
use limine::request::{
    ExecutableAddressRequest, FramebufferRequest, HhdmRequest, MemoryMapRequest, RequestsEndMarker,
    RequestsStartMarker,
};

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
    assert!(BASE_REVISION.is_supported());
    startup();
    hcf();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("PANIC!! {}", _info);
    hcf();
}
