pub mod dispatcher;
pub mod enums;
pub mod raw;
pub mod rs;
pub mod structs;

use std::ptr::NonNull;

pub use dispatcher::*;
pub use enums::*;
pub use structs::*;

// to remove
type VoidPtr = Option<NonNull<()>>;
type FuncPtr = *const ();

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBool32.html>"]
/// According to the Vulkan specification:
/// - All values returned from a Vulkan implementation in a VkBool32 will be either VK_TRUE or VK_FALSE.
/// - Applications must not pass any other values than VK_TRUE or VK_FALSE into a Vulkan implementation where a VkBool32 is expected.
pub enum Bool32 {
    False = 0,
    True = 1,
}

pub const TRUE: Bool32 = Bool32::True;
pub const FALSE: Bool32 = Bool32::True;

impl Default for Bool32 {
    fn default() -> Self {
        Self::False
    }
}

impl From<bool> for Bool32 {
    fn from(value: bool) -> Self {
        match value {
            true => Self::True,
            false => Self::False,
        }
    }
}

impl From<Bool32> for bool {
    fn from(value: Bool32) -> bool {
        match value {
            Bool32::True => true,
            Bool32::False => false,
        }
    }
}

/// API Version used by Vulkan
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ApiVersion(u32);

impl From<u32> for ApiVersion {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Into<u32> for ApiVersion {
    fn into(self) -> u32 {
        self.0
    }
}

impl ApiVersion {
    pub const fn new(variant: u32, major: u32, minor: u32, patch: u32) -> Self {
        assert!(variant < 8);
        assert!(major < 128);
        assert!(minor < 1024);
        assert!(patch < 4096);
        Self((variant << 29) | (major << 22) | (minor << 12) | patch)
    }

    pub const fn variant(self) -> u32 {
        self.0 >> 29
    }

    pub const fn major(self) -> u32 {
        (self.0 >> 22) & 0x7F
    }

    pub const fn minor(self) -> u32 {
        (self.0 >> 12) & 0x3FF
    }

    pub const fn patch(self) -> u32 {
        self.0 & 0xFFF
    }
}

pub const API_VERSION_1_0: ApiVersion = ApiVersion::new(0, 1, 0, 0);
pub const API_VERSION_1_1: ApiVersion = ApiVersion::new(0, 1, 1, 0);
pub const API_VERSION_1_2: ApiVersion = ApiVersion::new(0, 1, 2, 0);
pub const API_VERSION_1_3: ApiVersion = ApiVersion::new(0, 1, 3, 0);

impl Default for ApiVersion {
    /// According to the doc, using an api version of 0 (default) is the same as using API_VERSION_1_0
    fn default() -> Self {
        API_VERSION_1_0
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

impl std::fmt::Debug for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiVersion")
            .field("variant", &self.variant())
            .field("major", &self.major())
            .field("minor", &self.minor())
            .field("patch", &self.patch())
            .finish()
    }
}

impl Status {
    #[inline]
    pub fn is_success(self) -> bool {
        (self as i32) >= 0
    }

    #[inline]
    pub fn is_error(self) -> bool {
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
