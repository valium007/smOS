use crate::{logger::init_logger,gdt,idt,dbgbreak,int3};

pub fn startup() {

    dbgbreak!();
    
    init_logger();
    log::info!("Hello, World!");    

    gdt::init();
    idt::init();
    
    int3!();
    dbgbreak!();

}
