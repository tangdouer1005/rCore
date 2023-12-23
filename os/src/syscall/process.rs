//! App management syscalls
//use crate::batch::run_next_app;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    //run_next_app()
    loop{}
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    println!("[kernel] Application try to yield");
    0
}

