#![cfg_attr(not(feature = "std"), no_std)]

pub mod convert;
pub mod sync;
#[cfg(feature = "std")]
pub mod time;
mod tri;

pub mod macros {
    #[doc(inline)]
    pub use super::tri::tri;
    pub use sayuri_proc::stringify_raw;
}
