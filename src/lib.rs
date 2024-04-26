pub mod enums;
pub mod raw;

pub use enums::*;

mod private {
    /// For safety, prevent types outside this crate to implement Vulkan-specific traits
    pub trait Sealed {}
}

/// A dispatchable or non-dispatchable Vulkan Handle
trait Handle: private::Sealed + Sized {
    type InnerType;
    const TYPE: ObjectType;

    /// Retrieve the inner content of the vulkan handle, to be used by other Vulkan librairies not using this crate
    fn as_raw(self) -> Self::InnerType;

    /// Convert a pointer to a handle
    /// When calling this code, the user must ensure the following:
    /// - The pointer given is a valid Vulkan handle for the appropriate type
    /// - The handle must live at least as long as the object being created
    unsafe fn from_raw(x: Self::InnerType) -> Self;

    /// Same as [`from_raw`] but allows for types that can be zero (usize or u64 depending on the handle)
    /// Will fail if x is null/zero
    unsafe fn try_from_raw<T>(x: T) -> Option<Self>
    where
        Self::InnerType: TryFrom<T>,
    {
        Self::InnerType::try_from(x).ok().map(|t| Self::from_raw(t))
    }
}

#[macro_export]
macro_rules! vk_handle {
    ($name:ident, $obj_type:ident, $doc_tag:meta, $ty:ident) => {
        #[repr(transparent)]
        #[$doc_tag]
        #[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
        pub struct $name($ty);

        impl private::Sealed for $name {}
        impl Handle for $name {
            type InnerType = $ty;

            const TYPE: ObjectType = ObjectType::$obj_type;

            fn as_raw(self) -> $ty {
                self.0
            }

            unsafe fn from_raw(x: $ty) -> Self {
                Self(x)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "0x{:X}", self.0)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&format_args!("0x{:X}", self.0))
                    .finish()
            }
        }
    };
}


#[macro_export]
macro_rules! handle_dispatchable {
    ($name:ident, $obj_type:ident, $doc_tag:meta) => {
        vk_handle! {$name, $obj_type, $doc_tag, NonZeroUsize}
    };
}

#[macro_export]
macro_rules! handle_nondispatchable {
    ($name:ident, $obj_type:ident, $doc_tag:meta) => {
        vk_handle! {$name, $obj_type, $doc_tag, NonZeroU64}
    };
}
