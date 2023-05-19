use crate::alloc::{GlobalAlloc, Layout, System};
use win9x_sync::critical_section::CriticalSection;

static mut DLMALLOC: dlmalloc::Dlmalloc = dlmalloc::Dlmalloc::new();
static mut LOCK: CriticalSection = CriticalSection::new();

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: DLMALLOC access is guaranteed to be safe because the lock gives us unique and non-reentrant access.
        // Calling malloc() is safe because preconditions on this function match the trait method preconditions.
        let _guard = LOCK.lock();
        unsafe { DLMALLOC.malloc(layout.size(), layout.align()) }
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: DLMALLOC access is guaranteed to be safe because the lock gives us unique and non-reentrant access.
        // Calling calloc() is safe because preconditions on this function match the trait method preconditions.
        let _guard = LOCK.lock();
        unsafe { DLMALLOC.calloc(layout.size(), layout.align()) }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: DLMALLOC access is guaranteed to be safe because the lock gives us unique and non-reentrant access.
        // Calling free() is safe because preconditions on this function match the trait method preconditions.
        let _guard = LOCK.lock();
        unsafe { DLMALLOC.free(ptr, layout.size(), layout.align()) }
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: DLMALLOC access is guaranteed to be safe because the lock gives us unique and non-reentrant access.
        // Calling realloc() is safe because preconditions on this function match the trait method preconditions.
        let _guard = LOCK.lock();
        unsafe { DLMALLOC.realloc(ptr, layout.size(), layout.align(), new_size) }
    }
}
