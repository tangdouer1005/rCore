mod heap_allocator;
mod address;
mod frame_allocator;
mod page_table;
mod memory_set;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum, VPNRange, StepByOne};
pub use frame_allocator::{FrameTracker, frame_alloc};
pub use page_table::{PageTableEntry, PTEFlags, PageTable, translated_byte_buffer};
pub use super::config::{PAGE_SIZE, PAGE_SIZE_BITS};
pub use memory_set::{MapPermission, MemorySet, KERNEL_SPACE};

pub fn init(){
    heap_allocator::init_heap();
    heap_allocator::heap_test();
    frame_allocator::init_frame_allocator();
    frame_allocator::frame_allocator_test();
}