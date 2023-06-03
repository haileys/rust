use crate::ffi::{CStr, c_void};
use crate::io;
use crate::num::NonZeroUsize;
use crate::ptr;
use crate::sys::c;
use crate::os::win9x::io::{OwnedHandle, HandleOrNull, AsRawHandle};
use crate::time::Duration;

pub struct Thread {
    handle: OwnedHandle,
}

pub const DEFAULT_MIN_STACK_SIZE: usize = 4096;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {
        let p = Box::into_raw(Box::new(p));

        // FIXME On UNIX, we guard against stack sizes that are too small but
        // that's because pthreads enforces that stacks are at least
        // PTHREAD_STACK_MIN bytes big. Windows has no such lower limit, it's
        // just that below a certain threshold you can't do anything useful.
        // That threshold is application and architecture-specific, however.
        let handle = c::CreateThread(
            ptr::null_mut(),
            stack as u32,
            Some(thread_start),
            p as *mut _,
            0,
            ptr::null_mut(),
        );

        let handle = HandleOrNull::from_raw_handle(handle);

        return handle.try_into()
            .map(|handle| Thread { handle })
            .map_err(|_| io::Error::last_os_error());

        unsafe extern "stdcall" fn thread_start(main: *mut c_void) -> c::DWORD {
            unsafe {
                // TODO: setup stack overflow handler somehow
                //
                // run the code:
                Box::from_raw(main as *mut Box<dyn FnOnce()>)();
            }
            0
        }
    }

    pub fn yield_now() {
        // do nothing
    }

    pub fn set_name(_name: &CStr) {
        // nope
    }

    pub fn sleep(duration: Duration) {
        unsafe { c::Sleep(duration.as_millis().try_into().unwrap()); }
    }

    pub fn join(self) {
        let rc = unsafe { c::WaitForSingleObject(self.handle.as_raw_handle(), c::INFINITE) };
        if rc == c::WAIT_FAILED {
            panic!("failed to join on thread: {}", io::Error::last_os_error());
        }
    }
}

pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    // Windows 95 doesn't support SMP:
    Ok(NonZeroUsize::new(1).unwrap())
}

pub mod guard {
    pub type Guard = !;
    pub unsafe fn current() -> Option<Guard> {
        None
    }
    pub unsafe fn init() -> Option<Guard> {
        None
    }
}
