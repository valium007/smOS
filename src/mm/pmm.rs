use crate::MMAP_REQUEST;
use limine::memmap::*;

pub fn pmm_init() {
    let mmap = MMAP_REQUEST.response().unwrap().entries();

    for entry in mmap {
        if entry.type_ == MEMMAP_ACPI_NVS {
            log::info!(
                "ACPI_NVS: {:#X} {:#X} ",
                entry.base,
                entry.base + entry.length
            );
        }
    }
}
