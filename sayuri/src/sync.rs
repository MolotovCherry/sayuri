#[cfg(feature = "std")]
mod mutex;
mod sendable;

#[cfg(feature = "std")]
pub use mutex::{Mutex, MutexGuard};
pub use sendable::Sendable;
