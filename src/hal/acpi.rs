use crate::HHDM_REQUEST;
use crate::RSDP_REQUEST;
use core::ptr::addr_of;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct XSDP {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oemid: [u8; 6],
    pub revision: u8,
    pub rsdt_addr: u32,
    pub length: u32,
    pub xsdt_addr: u64,
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct SDT_HEADER {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct XSDT {
    pub h: SDT_HEADER,
    pub sdt_pointers: [u64; 1],
}

pub fn hal_acpi_find_sdt(signature: &str) -> *const SDT_HEADER {
    let xsdp_ptr = RSDP_REQUEST.response().unwrap().address as *const u64;
    let hhdm_offset = HHDM_REQUEST.response().unwrap().offset;

    log::info!("hhdm offset: {:X}", hhdm_offset);

    let xsdp = unsafe { *xsdp_ptr.cast::<XSDP>() };

    let xsdt = ((xsdp.xsdt_addr + hhdm_offset) as *const u64).cast::<XSDT>();

    let entries = unsafe { ((*xsdt).h.length - size_of::<SDT_HEADER>() as u32) / 8 };

    unsafe {
        let sdt_table_ptr = addr_of!((*xsdt).sdt_pointers) as *mut u64;
        let mut header: *const SDT_HEADER = core::ptr::null();

        for i in 0..entries {
            header =
                (sdt_table_ptr.add(i as usize).read_unaligned() + hhdm_offset) as *const SDT_HEADER;
            let sig = str::from_utf8(&(*header).signature).expect("invalid utf-8");
            if sig == signature {
                log::info!("found {}", sig);
                break;
            }
        }
        return header;
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct MADT {
    pub h: SDT_HEADER,
    pub local_apic_addr: u32,
    pub flags: u32,
    pub entry0: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct MADT_HEADER {
    pub typ: u8,
    pub length: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct MADT_LAPIC {
    pub h: MADT_HEADER,
    pub acpi_processor_uid: u8,
    pub apic_id: u8,
    pub flags: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct MADT_IOAPIC {
    pub h: MADT_HEADER,
    pub ioapic_id: u8,
    pub reserved: u8,
    pub ioapic_addr: u32,
    pub gsi_base: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct MADT_INTERRUPT_SOURCE_OVERRIDE {
    pub h: MADT_HEADER,
    pub bus: u8,
    pub source: u8,
    pub gsi: u32,
    pub flags: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct MADT_LAPIC_NMI {
    pub h: MADT_HEADER,
    pub acpi_processor_uid: u8,
    pub flags: u16,
    pub lapic_lint: u8,
}

const MADT_TYPE_LAPIC: u8 = 0;
const MADT_TYPE_IOAPIC: u8 = 1;
const MADT_TYPE_INTERRUPT_SOURCE_OVERRIDE: u8 = 2;
const MADT_TYPE_LAPIC_NMI: u8 = 4;

pub fn find_apic() {
    let madt = hal_acpi_find_sdt("APIC") as *const MADT;

    let lapic_addr = unsafe { (*madt).local_apic_addr };

    log::info!("local apic addr: {:X}", lapic_addr);

    let mut start = unsafe { addr_of!((*madt).entry0) as usize };

    let end = unsafe { madt.addr() + (*madt).h.length as usize };
    while start < end {
        let madt_header = (start as *const u64).cast::<MADT_HEADER>();
        let typ = unsafe { (*madt_header).typ };
        let len = unsafe { (*madt_header).length };

        if typ == MADT_TYPE_LAPIC {
            let lapic = unsafe { *(start as *const u64).cast::<MADT_LAPIC>() };
            log::info!("Found CPU #{:X}", lapic.apic_id);
        } else if typ == MADT_TYPE_IOAPIC {
            let ioapic = unsafe { *(start as *const u64).cast::<MADT_IOAPIC>() };
            let ioapic_addr = ioapic.ioapic_addr;
            let id = ioapic.ioapic_id;
            let gsi_base = ioapic.gsi_base;
            log::info!(
                "ioapic base: {:X} id: {:X} gsi base: {:X}",
                ioapic_addr,
                id,
                gsi_base
            );
        } else if typ == MADT_TYPE_INTERRUPT_SOURCE_OVERRIDE {
            let interrupt_source_override =
                unsafe { *(start as *const u64).cast::<MADT_INTERRUPT_SOURCE_OVERRIDE>() };
            let source = interrupt_source_override.source;
            let gsi = interrupt_source_override.gsi;
            log::info!("interrupt override source: {:X} gsi: {:?}", source, gsi);
        } else if typ == MADT_TYPE_LAPIC_NMI {
            let lapic_nmi = unsafe { *(start as *const u64).cast::<MADT_LAPIC_NMI>() };
            log::info!("lapic nmi lint: {:X}", lapic_nmi.lapic_lint);
        }

        start += len as usize;
    }
}
