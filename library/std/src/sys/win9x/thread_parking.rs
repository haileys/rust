use crate::pin::Pin;
use crate::time::Duration;

use vc6_sys as c;

pub struct Parker {
    event: c::HANDLE,
}

impl Drop for Parker {
    fn drop(&mut self) {
        // ignore error return, nothing we can do in drop
        c::CloseHandle(self.event);
    }
}

impl Parker {
    pub unsafe fn new_in_place(_parker: *mut Parker) {}

    pub unsafe fn park(self: Pin<&Self>) {}
    pub unsafe fn park_timeout(self: Pin<&Self>, _dur: Duration) {}
    pub fn unpark(self: Pin<&Self>) {}
}
