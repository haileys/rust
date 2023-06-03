#![stable(feature = "rust1", since = "1.0.0")]

use core::ptr;
use crate::io;
use crate::mem;
use crate::ffi::CString;
use super::c;

// Don't cache handles but get them fresh for every read/write. This allows us to track changes to
// the value over time (such as if a process calls `SetStdHandle` while it's running). See #40490.
pub struct Stdin(());
pub struct Stdout(());
pub struct Stderr(());

impl Stdin {
    pub const fn new() -> Stdin {
        Stdin(())
    }
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout(())
    }
}

impl Stderr {
    pub const fn new() -> Stderr {
        Stderr(())
    }
}

const MAX_BUFFER_SIZE: usize = 4096;
pub const STDIN_BUF_SIZE: usize = MAX_BUFFER_SIZE;

pub fn get_handle(handle_id: c::DWORD) -> io::Result<c::HANDLE> {
    let handle = unsafe { c::GetStdHandle(handle_id) };
    if handle == c::INVALID_HANDLE_VALUE {
        Err(io::Error::last_os_error())
    } else if handle.is_null() {
        Err(io::Error::from_raw_os_error(c::ERROR_INVALID_HANDLE as i32))
    } else {
        Ok(handle)
    }
}

fn is_console(handle: c::HANDLE) -> bool {
    // `GetConsoleMode` will return false (0) if this is a pipe (we don't care about the reported
    // mode). This will only detect Windows Console, not other terminals connected to a pipe like
    // MSYS. Which is exactly what we need, as only Windows Console needs a conversion to UTF-16.
    let mut mode = 0;
    unsafe { c::GetConsoleMode(handle, &mut mode) != 0 }
}

fn read(handle_id: c::DWORD, buf: &mut [u8]) -> io::Result<usize> {
    let handle = get_handle(handle_id)?;

    let mut nread = 0;
    let success = unsafe {
        c::ReadConsoleA(
            handle,
            buf.as_mut_ptr() as *mut _,
            buf.len() as u32,
            &mut nread,
            ptr::null_mut(),
        )
    };

    if success != c::TRUE {
        return Err(io::Error::last_os_error());
    }

    Ok(nread as usize)
}

fn write(handle_id: c::DWORD, buf: &[u8]) -> io::Result<usize> {
    let handle = get_handle(handle_id)?;
    if !is_console(handle) {
        // TODO!!! handle not console case
    }

    let mut nwritten = 0;
    let success = unsafe {
        c::WriteConsoleA(
            handle,
            buf.as_ptr() as *mut _,
            buf.len() as u32,
            &mut nwritten,
            ptr::null_mut(),
        )
    };

    if success != c::TRUE {
        return Err(io::Error::last_os_error());
    }

    Ok(nwritten as usize)
}

impl io::Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        read(c::STD_INPUT_HANDLE, buf)
    }
}

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        write(c::STD_OUTPUT_HANDLE, buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl io::Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        write(c::STD_ERROR_HANDLE, buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub fn is_ebadf(err: &io::Error) -> bool {
    err.raw_os_error() == Some(c::ERROR_INVALID_HANDLE as i32)
}

pub fn panic_output() -> Option<impl io::Write> {
    Some(PanicMessageBox::default())
}

#[derive(Default)]
struct PanicMessageBox {
    message: Vec<u8>,
}

impl io::Write for PanicMessageBox {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.message.extend(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Drop for PanicMessageBox {
    fn drop(&mut self) {
        if self.message.len() == 0 {
            return;
        }

        let message = CString::new(mem::take(&mut self.message)).unwrap();

        unsafe {
            c::MessageBoxA(
                ptr::null_mut(),
                message.as_ptr(),
                b"Panic\0".as_ptr() as *const i8,
                c::MB_OK | c::MB_ICONSTOP,
            );
        }
    }
}
