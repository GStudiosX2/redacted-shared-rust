use core::{alloc::GlobalAlloc};

#[global_allocator]
static ALLOC: RedactedAllocator = RedactedAllocator;

unsafe extern "C" {
    pub unsafe fn malloc(size: usize) -> *mut u8;
    pub unsafe fn free(ptr: *mut u8, size: usize);
}

pub struct RedactedAllocator;

unsafe impl GlobalAlloc for RedactedAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe { malloc(layout.pad_to_align().size()) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        unsafe { free(ptr, layout.pad_to_align().size()); }
    }
}