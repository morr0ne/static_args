#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(unchecked_math))]

#[cfg(any(unix, feature = "unsafe_impl"))]
mod unix;
#[cfg(windows)]
mod win32;

#[cfg(any(unix, feature = "unsafe_impl"))]
pub use unix::*;
#[cfg(windows)]
pub use win32::*;