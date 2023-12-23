// 用户库（包括入口函数、初始化函数、I/O 函数和系统调用接口等）
#![feature(linkage)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]

pub mod syscall;
#[macro_use]
pub mod console;
pub mod lang_items;


use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }
pub fn exit(exit_code: i32) -> isize { sys_exit(exit_code) }
pub fn yield_() -> isize {
    sys_yield()
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
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
