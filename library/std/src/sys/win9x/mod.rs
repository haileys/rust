#![stable(feature = "rust1", since = "1.0.0")]

//! System bindings for the win9x platform
//!
//! This module contains the facade (aka platform-specific) implementations of
//! OS level functionality for win9x, similar to sys/wasm.

pub mod c;

pub mod alloc;
pub mod env;
pub mod stdio;
pub mod thread_local_dtor;
pub mod thread_local_key;

#[path = "../unix/os_str.rs"]
pub mod os_str;

#[path = "../unsupported/args.rs"]
pub mod args;
#[path = "../unix/cmath.rs"]
pub mod cmath;
#[path = "../unsupported/fs.rs"]
pub mod fs;
#[path = "../unsupported/io.rs"]
pub mod io;
#[path = "../unsupported/net.rs"]
pub mod net;
#[path = "../unsupported/os.rs"]
pub mod os;
#[path = "../unix/path.rs"]
pub mod path;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
#[path = "../unsupported/time.rs"]
pub mod time;

cfg_if::cfg_if! {
    if #[cfg(target_feature = "atomics")] {
        #[path = "../unix/locks"]
        pub mod locks {
            #![allow(unsafe_op_in_unsafe_fn)]
            mod futex_condvar;
            mod futex_mutex;
            mod futex_rwlock;
            pub(crate) use futex_condvar::Condvar;
            pub(crate) use futex_mutex::Mutex;
            pub(crate) use futex_rwlock::RwLock;
        }
        #[path = "atomics/futex.rs"]
        pub mod futex;
        #[path = "atomics/thread.rs"]
        pub mod thread;
    } else {
        #[path = "../unsupported/locks/mod.rs"]
        pub mod locks;
        pub mod once;
        #[path = "../unsupported/thread.rs"]
        pub mod thread;
        #[path = "../unsupported/thread_parking.rs"]
        pub mod thread_parking;
    }
}

#[path = "../unsupported/common.rs"]
#[deny(unsafe_op_in_unsafe_fn)]
mod common;
pub use common::*;

pub trait IsZero {
    fn is_zero(&self) -> bool;
}

macro_rules! impl_is_zero {
    ($($t:ident)*) => ($(impl IsZero for $t {
        fn is_zero(&self) -> bool {
            *self == 0
        }
    })*)
}

impl_is_zero! { i8 i16 i32 i64 isize u8 u16 u32 u64 usize }

pub fn cvt<I: IsZero>(i: I) -> crate::io::Result<I> {
    if i.is_zero() { Err(crate::io::Error::last_os_error()) } else { Ok(i) }
}
