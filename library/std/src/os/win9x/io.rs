#![stable(feature = "rust1", since = "1.0.0")]

#[path = "../windows/io/handle.rs"]
mod handle;
#[stable(feature = "rust1", since = "1.0.0")]
pub use handle::*;

// TODO #[path = "../windows/io/raw.rs"]
mod raw;
#[stable(feature = "rust1", since = "1.0.0")]
pub use raw::*;
