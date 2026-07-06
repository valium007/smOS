#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(static_mut_refs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(abi_custom)]
#![allow(unused_macros)]

pub mod arch;
pub mod hal;
pub mod logger;
pub mod mm;
pub mod start;
pub mod util;

use limine::BaseRevision;
use limine::RequestsEndMarker;
use limine::RequestsStartMarker;
use limine::request::RsdpRequest;
use limine::request::{ExecutableAddressRequest, FramebufferRequest, HhdmRequest, MemmapRequest};

use crate::arch::hcf;
use crate::start::startup;

#[used]
#[unsafe(link_section = ".request$a")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests$ba")]
static BASE_REVISION: BaseRevision = BaseRevision::with_revision(6);

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
static MMAP_REQUEST: MemmapRequest = MemmapRequest::new();

#[used]
#[unsafe(link_section = ".request$be")]
static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();

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
    log::error!("PANIC!! {}", _info);
    hcf();
}
