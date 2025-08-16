use core::arch::asm;
use core::ffi::*;

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_load_16(_ptr: u64, _order: u64) -> ! {
    panic!("undefined function");
}

#[unsafe(no_mangle)]
pub extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe { *s.add(i) = c as u8 };
    }
    s
}

#[unsafe(no_mangle)]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe {
            *dest.add(i) = *src.add(i);
        }
    }
    dest
}

#[unsafe(no_mangle)]
pub extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        for i in (0..n).rev() {
            unsafe {
                *dest.add(i) = *src.add(i);
            }
        }
    } else {
        for i in 0..n {
            unsafe {
                *dest.add(i) = *src.add(i);
            }
        }
    }
    dest
}

#[unsafe(no_mangle)]
pub extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let a = unsafe { *s1.add(i) };
        let b = unsafe { *s2.add(i) };
        if a != b {
            return a as i32 - b as i32;
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlen(mut s: *const c_char) -> usize {
    let mut n = 0;
    while *s != 0 {
        s = s.add(1);
        n += 1;
    }
    n
}

#[unsafe(no_mangle)]
pub extern "C" fn fma(x: f64, y: f64, z: f64) -> f64 {
    return x * y + z;
}

#[unsafe(no_mangle)]
pub extern "C" fn fmaf(x: f32, y: f32, z: f32) -> f32 {
    return x * y + z;
}

#[unsafe(no_mangle)]
pub extern "C" fn _fltused() {
    panic!("undefined function");
}

#[unsafe(no_mangle)]
pub fn hcf() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn __chkstk() -> () {}
