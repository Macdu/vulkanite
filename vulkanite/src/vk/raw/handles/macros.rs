macro_rules! vk_handle {
    ($name:ident, $obj_type:ident, $doc_tag:meta, $vk_name:literal, $ty:ident) => {
        #[repr(transparent)]
        #[$doc_tag]
        #[doc(alias = $vk_name)]
        #[derive(Eq, PartialEq, PartialOrd, Ord, Hash)]
        pub struct $name($ty);

        impl private::Sealed for $name {}
        impl Handle for $name {
            type InnerType = $ty;

            const TYPE: ObjectType = ObjectType::$obj_type;

            #[inline]
            fn as_raw(&self) -> $ty {
                self.0
            }

            #[inline]
            unsafe fn from_raw(x: $ty) -> Self {
                Self(x)
            }

            #[inline]
            unsafe fn clone(&self) -> Self {
                Self(self.0)
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

macro_rules! handle_dispatchable {
    ($name:ident, $obj_type:ident, $doc_tag:meta, $vk_name:literal) => {
        vk_handle! {$name, $obj_type, $doc_tag, $vk_name, NonZeroUsize}
    };
}

macro_rules! handle_nondispatchable {
    ($name:ident, $obj_type:ident, $doc_tag:meta, $vk_name:literal) => {
        vk_handle! {$name, $obj_type, $doc_tag, $vk_name, NonZeroU64}
    };
}
