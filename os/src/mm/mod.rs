mod heap_allocator;
mod address;
mod frame_allocator;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};


pub fn init(){
    heap_allocator::init_heap();
    heap_allocator::heap_test();
    frame_allocator::init_frame_allocator();
    frame_allocator::frame_allocator_test();
}