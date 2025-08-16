use crate::println;
use crate::{HHDM_REQUEST, MMAP_REQUEST};
use limine::memory_map::EntryType;
use spin::Mutex;

static BASE: Mutex<u64> = Mutex::new(0);
static USABLE_SIZE: Mutex<u64> = Mutex::new(0);

pub fn pmm_init() {
    let mut highest_addr: u64 = 0;
    let mut base = BASE.lock();
    let mut usable_size = USABLE_SIZE.lock();

    *base = HHDM_REQUEST.get_response().unwrap().offset();
    let mmap = MMAP_REQUEST.get_response().unwrap().entries();

    for entry in mmap {
        if entry.entry_type == EntryType::USABLE {
            let end_addr = entry.base + entry.length;

            if end_addr > highest_addr {
                highest_addr = end_addr;
            }

            *usable_size += entry.length;
        }
        println!(
            "MMAP BASE: {:#X} {:#X} ",
            entry.base,
            entry.base + entry.length
        );
    }
    println!("usable mem size: {:#X}", *usable_size);
}
