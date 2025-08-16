#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(static_mut_refs)]
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod idt;
pub mod logger;
pub mod macros;
pub mod shim;
pub mod start;

use limine::BaseRevision;
use limine::memory_map::EntryType;
use limine::request::{
    ExecutableAddressRequest, FramebufferRequest, HhdmRequest, MemoryMapRequest, RequestsEndMarker,
    RequestsStartMarker,
};

use crate::logger::init_logger;
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

    pmm_init();
    init_logger();

    log::info!("Hello, World!");
    gdt::init();
    idt::init();

    startup();
    hcf();
}

static mut BASE: u64 = 0;
static mut USABLE_SIZE: u64 = 0;

fn pmm_init() {
    unsafe {
        BASE = HHDM_REQUEST.get_response().unwrap().offset();
        let mut highest_addr: u64 = 0;
        let mmap = MMAP_REQUEST.get_response().unwrap().entries();
        for entry in mmap {
            if entry.entry_type == EntryType::USABLE {
                let end_addr = entry.base + entry.length;

                if end_addr > highest_addr {
                    highest_addr = end_addr;
                }

                USABLE_SIZE += entry.length;
            }
            println!(
                "MMAP BASE: {:#X} {:#X} ",
                entry.base,
                entry.base + entry.length
            );
        }

        println!("usable mem size: {:#X}", USABLE_SIZE);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("PANIC!! {}", _info);
    hcf();
}
