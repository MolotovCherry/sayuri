#[doc(hidden)]
pub trait FnPtr {
    type Output;
}

impl<R> FnPtr for fn() -> R {
    type Output = R;
}

/// ! type, but on stable!
pub type Never = <fn() -> ! as FnPtr>::Output;
