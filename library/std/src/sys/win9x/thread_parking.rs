use crate::pin::Pin;
use crate::time::Duration;
use crate::sys::c;
use crate::os::win9x::io::{OwnedHandle, HandleOrNull, AsRawHandle};
use crate::ptr;

pub struct Parker {
    event: OwnedHandle,
}

impl Parker {
    pub unsafe fn new_in_place(parker: *mut Parker) {
        let handle = c::CreateEventA(
            ptr::null_mut(),
            0,
            0,
            ptr::null_mut(),
        );

        let handle = HandleOrNull::from_raw_handle(handle)
            .try_into()
            .expect("CreateEvent");

        ptr::write(parker, Parker { event: handle });
    }

    pub unsafe fn park(self: Pin<&Self>) {
        unsafe { c::WaitForSingleObject(self.event.as_raw_handle(), c::INFINITE) };
    }

    pub unsafe fn park_timeout(self: Pin<&Self>, duration: Duration) {
        let handle = self.event.as_raw_handle();
        let duration = duration.as_secs().try_into().unwrap();
        unsafe { c::WaitForSingleObject(handle, duration) };
    }

    pub fn unpark(self: Pin<&Self>) {
        unsafe { c::SetEvent(self.event.as_raw_handle()); }
    }
}
