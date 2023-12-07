#![no_main]
#![no_std]
#![feature(panic_info_message)]

use core::{arch::global_asm, ptr::write_volatile};
global_asm!(include_str!("entry.asm"));

mod sbi;
#[macro_use]
mod console;
mod lang_items;

#[no_mangle]
pub fn rust_main() {
    clear_bss();
    println!("hello world");
    panic!("fuck");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize .. ebss as usize).for_each(|addr|{
        unsafe {(addr as *mut u8).write_volatile(0);}
    })
}