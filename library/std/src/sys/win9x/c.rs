#![stable(feature = "rust1", since = "1.0.0")]

// #![allow(nonstandard_style)]

#[stable(feature = "rust1", since = "1.0.0")]
pub use vc6_sys::*;

// bool
pub const TRUE: BOOL = 1i32;
// crt
pub const DLL_PROCESS_DETACH: u32 = 0u32;
pub const DLL_THREAD_DETACH: u32 = 3u32;
pub const TLS_OUT_OF_INDEXES: u32 = 4294967295u32;
// stdio
pub const ERROR_INVALID_HANDLE: DWORD = 6u32;
pub const INVALID_HANDLE_VALUE: HANDLE = ::core::ptr::invalid_mut(-1i32 as _);
pub const STD_ERROR_HANDLE: DWORD = 4294967284u32;
pub const STD_INPUT_HANDLE: DWORD = 4294967286u32;
pub const STD_OUTPUT_HANDLE: DWORD = 4294967285u32;
// sync
pub const WAIT_FAILED: DWORD = 0xFFFFFFFF;
