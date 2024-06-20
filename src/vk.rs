pub mod dispatcher;
pub mod extensions;
pub mod enums;
pub mod raw;
pub mod rs;
pub mod structs;
pub mod custom;

use std::{
    ffi::{c_char, CStr},
    fmt::Debug,
    ptr::NonNull,
};

pub use dispatcher::*;
pub use enums::*;
pub use structs::*;
pub use extensions::*;
pub use custom::*;

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
}

impl core::fmt::Display for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Debug>::fmt(&self, f)
    }
}

impl std::error::Error for Status {}

/// Result type most Vulkan Function return
/// You are guaranteed that if a vk::Result<A> is an Err
/// Then the status code is an error code
pub type Result<A> = core::result::Result<A, Status>;
