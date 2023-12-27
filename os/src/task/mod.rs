//! Task management implementation
//!
//! Everything about task management, like starting and switching tasks is
//! implemented here.
//!
//! A single global instance of [`TaskManager`] called `TASK_MANAGER` controls
//! all the tasks in the operating system.
//!
//! Be careful when you see `__switch` ASM function in `switch.S`. Control flow around this function
//! might not be what you expect.

mod context;
mod switch;
mod pid;
mod manager;
mod processor;

#[allow(clippy::module_inception)]
mod task;

use crate::loader::{get_app_data_by_name};
use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
use lazy_static::*;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};
pub use context::TaskContext;
use alloc::vec::Vec;
pub use pid::{KernelStack, pid_alloc, PidHandle};
pub use manager::{fetch_task, add_task};
pub use processor::{
    current_task, current_trap_cx, current_user_token, run_tasks, schedule, take_current_task,
    Processor,
};
use alloc::sync::Arc;

lazy_static! {
    ///Globle process that init user shell
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
        get_app_data_by_name("initproc").unwrap()
    ));
}
///Add init process to the manager
pub fn add_initproc() {
    add_task(INITPROC.clone());
}