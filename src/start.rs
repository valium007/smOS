use crate::{dbgbreak, int3};

pub fn startup() {
    dbgbreak!();
    int3!();
    dbgbreak!();
}
