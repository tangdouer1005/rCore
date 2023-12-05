#![no_main]
#![no_std]
mod lang_items;

use core::{arch::global_asm, ptr::write_volatile};
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() {
    bss_clear();
    loop{}
}

fn bss_clear() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize .. ebss as usize).for_each(|addr|{
        unsafe {(addr as *mut u8).write_volatile(0);}
    })
}