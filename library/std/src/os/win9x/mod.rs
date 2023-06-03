#![stable(feature = "rust1", since = "1.0.0")]
#![doc(cfg(target_os = "win9x"))]

pub mod ffi;
// pub mod fs;
pub mod io;
// pub mod process;
pub mod raw;
// pub mod thread;

/// A prelude for conveniently writing platform-specific code.
///
/// Includes all extension traits, and some important type definitions.
#[stable(feature = "rust1", since = "1.0.0")]
pub mod prelude {
    #[doc(no_inline)]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::ffi::{OsStrExt, OsStringExt};
//     #[doc(no_inline)]
//     #[stable(feature = "rust1", since = "1.0.0")]
//     pub use super::fs::FileExt;
//     #[doc(no_inline)]
    #[stable(feature = "rust1", since = "1.0.0")]
//     pub use super::fs::{MetadataExt, OpenOptionsExt};
//     #[doc(no_inline)]
    pub use super::io::{
        AsHandle, BorrowedHandle, FromRawHandle, HandleOrInvalid, IntoRawHandle, OwnedHandle,
//         AsHandle, AsSocket, BorrowedHandle, BorrowedSocket, FromRawHandle, FromRawSocket,
//         HandleOrInvalid, IntoRawHandle, IntoRawSocket, OwnedHandle, OwnedSocket,
    };
//     #[doc(no_inline)]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::io::{AsRawHandle, /*AsRawSocket,*/ RawHandle, /*RawSocket*/};
}
