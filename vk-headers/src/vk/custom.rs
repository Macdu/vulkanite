//! Some custom Vulkan types which are implemented as Quality-Of-Life improvements over the existing ones
use std::ffi::c_char;
use std::ffi::{c_void, CStr};

use crate::vk;

// to remove
pub(crate) type VoidPtr = *const c_void;
pub(crate) type FuncPtr = *const ();

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBool32.html>
///
/// According to the Vulkan specification:
/// - All values returned from a Vulkan implementation in a VkBool32 will be either VK_TRUE or VK_FALSE.
/// - Applications must not pass any other values than VK_TRUE or VK_FALSE into a Vulkan implementation where a VkBool32 is expected.
pub enum Bool32 {
    False = 0,
    True = 1,
}

pub const TRUE: Bool32 = Bool32::True;
pub const FALSE: Bool32 = Bool32::False;

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

macro_rules! extension_name_decl {
    ($name:ident) => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        pub struct $name(*const c_char);

        impl $name {
            /// Safety: name must live at least as long as the resulting object
            /// This name must also be a valid instance/device extension name
            pub const unsafe fn new(name: &CStr) -> Self {
                Self(name.as_ptr())
            }

            pub fn get(&self) -> &CStr {
                unsafe { CStr::from_ptr(self.0) }
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.get() == other.get()
            }
        }

        impl Eq for $name {}

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.get().partial_cmp(other.get())
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.get().cmp(other.get())
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.get().fmt(f)
            }
        }

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.get().hash(state)
            }
        }
    };
}

extension_name_decl!(InstanceExtensionName);
extension_name_decl!(DeviceExtensionName);

#[derive(Clone, PartialEq, Debug, Hash)]
pub struct InstanceExtension {
    pub name: InstanceExtensionName,
    pub spec: u32,
}

#[derive(Clone, PartialEq, Debug, Hash)]
pub struct DeviceExtension {
    pub name: DeviceExtensionName,
    pub spec: u32,
}

/// Replacement for PFN_vkDebugUtilsMessengerCallbackEXT (<https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkDebugUtilsMessengerCallbackEXT.html>)
/// 
/// Used by [vk::DebugUtilsMessengerCreateInfoEXT]
pub type DebugUtilsMessengerCallbackEXT = Option<
    extern "system" fn(
        vk::DebugUtilsMessageSeverityFlagsEXT,
        vk::DebugUtilsMessageTypeFlagsEXT,
        &vk::DebugUtilsMessengerCallbackDataEXT,
        *const (),
    ) -> Bool32,
>;
