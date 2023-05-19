#![stable(feature = "rust1", since = "1.0.0")]

//! System bindings for the win9x platform
//!
//! This module contains the facade (aka platform-specific) implementations of
//! OS level functionality for win9x, similar to sys/wasm.

pub mod alloc;
pub mod env;

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
#[path = "../unix/os_str.rs"]
pub mod os_str;
#[path = "../unix/path.rs"]
pub mod path;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
#[path = "../unsupported/stdio.rs"]
pub mod stdio;
#[path = "../unsupported/thread_local_dtor.rs"]
pub mod thread_local_dtor;
#[path = "../unsupported/thread_local_key.rs"]
pub mod thread_local_key;
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

