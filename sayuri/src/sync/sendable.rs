use std::thread::{self, ThreadId};

/// Make any T Send+Sync
pub struct Sendable<T: ?Sized>(ThreadId, T);

unsafe impl<T> Send for Sendable<T> {}
unsafe impl<T> Sync for Sendable<T> {}

impl<T> Sendable<T> {
    pub fn new(t: T) -> Self {
        let thread = thread::current().id();
        Self(thread, t)
    }

    /// # Safety
    /// All safety requirements for `T` apply
    pub unsafe fn as_ref(&self) -> &T {
        &self.1
    }

    /// # Safety
    /// All safety requirements for `T` apply
    pub unsafe fn as_mut(&mut self) -> &mut T {
        &mut self.1
    }

    /// # Safety
    /// All safety requirements for `T` apply
    pub unsafe fn into_inner(self) -> T {
        self.1
    }

    /// Get ref to inner if you are on the same thread it was created on
    pub fn get(&self) -> Option<&T> {
        if thread::current().id() == self.0 {
            Some(&self.1)
        } else {
            None
        }
    }

    /// Get mut ref to inner if you are on the same thread it was created on
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if thread::current().id() == self.0 {
            Some(&mut self.1)
        } else {
            None
        }
    }

    /// Get inner if you are on the same thread it was created on
    pub fn get_inner(self) -> Option<T> {
        if thread::current().id() == self.0 {
            Some(self.1)
        } else {
            None
        }
    }
}
