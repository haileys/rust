use crate::ffi::{OsString, CString};
use crate::io;
use crate::os::win9x::ffi::{OsStrExt, OsStringExt};
use crate::path::{PathBuf, Path};
use crate::ptr;
use super::{c, cvt};

pub use super::os_unsupported::{
    Env,
    env,
    getenv,
    setenv,
    unsetenv,
    SplitPaths,
    split_paths,
    JoinPathsError,
    join_paths,
    home_dir,
    temp_dir,
    current_exe,
};

pub fn errno() -> i32 {
    unsafe { c::GetLastError() as i32 }
}

pub fn exit(code: i32) -> ! {
    unsafe { c::ExitProcess(code as c::UINT); }
    unreachable!();
}

pub fn getpid() -> u32 {
    unsafe { c::GetCurrentProcessId() as u32 }
}

pub fn getcwd() -> io::Result<PathBuf> {
    let mut buf = Vec::with_capacity(0);

    loop {
        let rc = cvt(unsafe {
            let ptr: *mut u8 = buf.as_mut_ptr();
            let ptr: *mut i8 = ptr as _;
            c::GetCurrentDirectoryA(buf.capacity() as u32, ptr)
        })? as usize;

        // error
        if rc == 0 {
            return Err(io::Error::last_os_error());
        }

        // on success GetCurrentDirectoryA returns number of bytes written to
        // buffer, not including null-terminator
        if rc < buf.capacity() {
            unsafe { buf.set_len(rc); }
            return Ok(PathBuf::from(OsString::from_vec(buf)));
        }

        // otherwise GetCurrentDirectoryA returns required size of buffer,
        // *including* null-terminator. we re-use some funky logic from
        // sys::unix::os to resize buffer to fit
        unsafe { buf.set_len(0); }
        buf.reserve(rc);
    }
}

pub fn chdir(path: &Path) -> io::Result<()> {
    let mut cstr = CString::new(path.as_os_str().as_bytes())?;
    let success = unsafe { c::SetCurrentDirectoryA(cstr.as_ptr() as *mut c::CHAR) };
    if success == c::TRUE {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

/// Gets a detailed string description for the given error number.
pub fn error_string(mut errnum: i32) -> String {
    // This value is calculated from the macro
    // MAKELANGID(LANG_SYSTEM_DEFAULT, SUBLANG_SYS_DEFAULT)
    let langId = 0x0800 as c::DWORD;

    let mut buf = [0 as u8; 2048];

    unsafe {
        let mut module = ptr::null_mut();
        let mut flags = 0;

        let res = c::FormatMessageA(
            flags | c::FORMAT_MESSAGE_FROM_SYSTEM | c::FORMAT_MESSAGE_IGNORE_INSERTS,
            module,
            errnum as c::DWORD,
            langId,
            buf.as_mut_ptr() as *mut i8,
            buf.len() as c::DWORD,
            ptr::null_mut(),
        ) as usize;
        if res == 0 {
            // Sometimes FormatMessageW can fail e.g., system doesn't like langId,
            let fm_err = errno();
            return format!("OS Error {errnum} (FormatMessageA() returned error {fm_err})");
        }

        // TODO this is not utf8 but ASCII
        match crate::str::from_utf8(&buf[..res]) {
            Ok(msg) => {
                let mut msg = msg.to_string();
                // Trim trailing CRLF inserted by FormatMessageA
                let len = msg.trim_end().len();
                msg.truncate(len);
                msg
            }
            Err(..) => format!(
                "OS Error {} (FormatMessageA() returned \
                 invalid UTF-16)",
                errnum
            ),
        }
    }
}
