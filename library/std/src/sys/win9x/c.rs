#![allow(nonstandard_style)]

use core::ffi::{c_void, c_ulong};

pub type BOOL = i32;
pub type CONSOLE_MODE = u32;
pub type DWORD = c_ulong;
pub type HANDLE = *mut ::core::ffi::c_void;
pub type LPVOID = *mut c_void;
pub type STD_HANDLE = u32;
pub type WIN32_ERROR = u32;

// bool
pub const TRUE: BOOL = 1i32;
pub const FALSE: BOOL = 0i32;
// crt
pub const DLL_PROCESS_DETACH: u32 = 0u32;
pub const DLL_THREAD_DETACH: u32 = 3u32;
pub const TLS_OUT_OF_INDEXES: u32 = 4294967295u32;
// stdio
pub const ERROR_INVALID_HANDLE: WIN32_ERROR = 6u32;
pub const INVALID_HANDLE_VALUE: HANDLE = ::core::ptr::invalid_mut(-1i32 as _);
pub const STD_ERROR_HANDLE: STD_HANDLE = 4294967284u32;
pub const STD_INPUT_HANDLE: STD_HANDLE = 4294967286u32;
pub const STD_OUTPUT_HANDLE: STD_HANDLE = 4294967285u32;

#[link(name = "kernel32")]
extern "system" {
    // error:
    pub fn GetLastError() -> WIN32_ERROR;
    // tls:
    pub fn TlsAlloc() -> DWORD;
    pub fn TlsGetValue(index: DWORD) -> *mut c_void;
    pub fn TlsSetValue(index: DWORD, value: *mut c_void) -> BOOL;
    // stdio:
    pub fn GetConsoleMode(hconsolehandle: HANDLE, lpmode: *mut CONSOLE_MODE) -> BOOL;
    pub fn GetStdHandle(nstdhandle: STD_HANDLE) -> HANDLE;
    pub fn ReadConsoleA(hConsoleInput: HANDLE, lpBuffer: LPVOID, nNumberOfCharsToRead: DWORD, lpNumberOfCharsRead: *mut DWORD, lpReserved: LPVOID) -> BOOL;
    pub fn WriteConsoleA(hConsoleOutput: HANDLE, lpBuffer: *const c_void, nNumberOfCharsToWrite: DWORD, lpNumberOfCharsWritten: *mut DWORD, lpReserved: LPVOID) -> BOOL;
}
