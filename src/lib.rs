#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(unchecked_math))]

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod win32;

#[cfg(unix)]
pub use unix::*;
#[cfg(windows)]
pub use win32::*;