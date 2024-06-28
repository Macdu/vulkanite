mod custom;
mod dispatcher;
pub mod enums;
pub mod extensions;
pub mod raw;
pub mod rs;
pub mod structs;

use std::ffi::CStr;

#[doc(inline)]
pub use custom::*;
#[doc(inline)]
pub use dispatcher::*;
pub use enums::*;
pub use extensions::*;
pub use structs::*;

impl Status {
    #[inline]
    pub const fn is_success(self) -> bool {
        (self as i32) >= 0
    }

    #[inline]
    pub const fn is_error(self) -> bool {
        (self as i32) < 0
    }

    pub fn map_success<T, F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> T,
    {
        if self.is_success() {
            Ok(f())
        } else {
            Err(self)
        }
    }

    pub fn map_successes<T, F>(self, f: F) -> Result<(Self, T)>
    where
        F: FnOnce() -> T,
    {
        if self.is_success() {
            Ok((self, f()))
        } else {
            Err(self)
        }
    }

    pub fn map_always<T>(self, result: T) -> core::result::Result<(Self, T), (Self, T)> {
        if self.is_success() {
            Ok((self, result))
        } else {
            Err((self, result))
        }
    }

    pub fn into_result(self) -> Result<Self> {
        if self.is_success() {
            Ok(self)
        } else {
            Err(self)
        }
    }
}

impl core::fmt::Display for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Debug>::fmt(&self, f)
    }
}

impl std::error::Error for Status {}

/// Result type most Vulkan Function return
/// You are guaranteed when calling a vulkan function that if a [`Result<A>`] is an Err
/// Then the status code is an error code
pub type Result<A> = core::result::Result<A, Status>;
