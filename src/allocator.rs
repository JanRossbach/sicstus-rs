#![cfg(feature = "allocator")]

#[global_allocator]
static ALLOCATOR: SICStusAllocator = SICStusAllocator;

// It is recommended to use the sicstus memory management functions instead of the Rust ones in order to
// avoid memory fragmentation. In order to use the Rust allocator you can disable the alloc feature.

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

pub struct SICStusAllocator;

unsafe impl GlobalAlloc for SICStusAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        sicstus_sys::SP_malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        sicstus_sys::SP_free(ptr as *mut c_void)
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        sicstus_sys::SP_realloc(ptr as *mut c_void, new_size) as *mut u8
    }
}
