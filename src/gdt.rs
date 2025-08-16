use lazy_static::lazy_static;
use x86_64::instructions::segmentation::{CS, DS, SS};
use x86_64::registers::segmentation::Segment;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};

pub fn init() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.kernel_cs);
        DS::set_reg(GDT.1.kernel_ds);
        SS::set_reg(GDT.1.kernel_ss);
    }
    log::info!("GDT initialized")
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let kernel_cs = gdt.append(Descriptor::kernel_code_segment());
        let kernel_ds = gdt.append(Descriptor::kernel_data_segment());
        let kernel_ss = gdt.append(Descriptor::kernel_data_segment());
        (
            gdt,
            Selectors {
                kernel_cs,
                kernel_ds,
                kernel_ss,
            },
        )
    };
}

struct Selectors {
    kernel_cs: SegmentSelector,
    kernel_ds: SegmentSelector,
    kernel_ss: SegmentSelector,
}
