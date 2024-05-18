pub mod enums;
pub mod raw;
pub mod structs;
pub mod dispatcher;

use std::cell::Cell;
use std::marker::PhantomData;
use std::ptr::{self, NonNull};

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

mod private {
    /// For safety, prevent types outside this crate to implement Vulkan-specific traits
    pub trait Sealed {}
}

/// If A implements Alias<B>, this means A and B have exactly the same memory representation
/// Thus transmuting from A to B is safe
pub unsafe trait Alias<T: Sized>: Sized {}

/// T has always the same memory representation as itself
unsafe impl<T: Sized> Alias<T> for T {}

/// A dispatchable or non-dispatchable Vulkan Handle
pub trait Handle: private::Sealed + Sized {
    type InnerType;
    const TYPE: ObjectType;

    /// Retrieve the inner content of the vulkan handle, to be used by other Vulkan librairies not using this crate
    fn as_raw(&self) -> Self::InnerType;

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

    fn borrow<'a>(&'a self) -> BorrowedHandle<'a, Self> {
        BorrowedHandle {
            value: self.as_raw(),
            phantom: PhantomData,
        }
    }

    fn borrow_mut<'a>(&'a mut self) -> BorrowedMutHandle<'a, Self> {
        BorrowedMutHandle {
            value: self.as_raw(),
            phantom: PhantomData,
        }
    }

    /// clone the current object, this function is unsafe as the caller must ensure that only one of the two
    /// handles is destroyed, moreover, the second handle must not be used after the first has been destroyed
    unsafe fn clone(&self) -> Self;
}

/// This represents a reference to an handle
/// Its internal representation is the same as the handle
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct BorrowedHandle<'a, T: Handle> {
    value: T::InnerType,
    phantom: PhantomData<&'a T>,
}

/// BorrowedHandle<'a, T> is repr(transparent) of T
unsafe impl<'a, T: Handle> Alias<T> for BorrowedHandle<'a, T> {}

impl<'a, T: Handle> AsRef<T> for BorrowedHandle<'a, T> {
    fn as_ref(&self) -> &T {
        // SAFETY: BorrowedHandle<T> and T have the same internal representation
        // Moreover, the reference will only live as long as the borrowed handle
        // (it cannot live as long as the original one as we are not tracking it location)
        unsafe {
            ptr::from_ref(&self.value)
                .cast::<T>()
                .as_ref()
                .unwrap_unchecked()
        }
    }
}

/// This represents a reference to a mutable handle
/// Its internal representation is the same as the handle
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct BorrowedMutHandle<'a, T: Handle> {
    value: T::InnerType,
    phantom: PhantomData<&'a mut T>,
}

/// BorrowedMutHandle<'a, T> is repr(transparent) of T
unsafe impl<'a, T: Handle> Alias<T> for BorrowedMutHandle<'a, T> {}

impl<'a, T: Handle> AsMut<T> for BorrowedMutHandle<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        // SAFETY: Same as [BorrowedHandle::AsRef]
        unsafe {
            ptr::from_mut(&mut self.value)
                .cast::<T>()
                .as_mut()
                .unwrap_unchecked()
        }
    }
}

/// A trait implemented by Vulkan C structs whose first 2 fields are:
///     VkStructureType        sType;
///     const void*            pNext;
/// sType must always be set to STRUCTURE_TYPE
pub unsafe trait ExtendableStructure: Default {
    const STRUCTURE_TYPE: StructureType;

    unsafe fn retrieve_next(&self) -> &Cell<*const Header> {
        unsafe {
            &ptr::from_ref(self)
                .cast::<Header>()
                .as_ref()
                .unwrap_unchecked()
                .p_next
        }
    }

    /// Assuming the current structure chain is the following:
    /// Self -> Ext1 -> Ext2 -> Ext3
    /// calling this function with Ext4 will result in:
    /// Self -> Ext4 -> Ext1 -> Ext2 -> Ext3
    /// This function will never cause cycles in the structure chain
    /// This function is unsafe because it discards the lifetime (ExtendableStructure does not have a lifetime parameter)
    /// Also it does not check that T is a valid extension to be added to Self and only requires references (and not mutable references)
    unsafe fn push_next_unchecked<T: ExtendableStructure>(&self, ext: &T) {
        let my_next = self.retrieve_next();
        let other_next = ext.retrieve_next();
        other_next.set(my_next.get());
        my_next.set(ptr::from_ref(ext).cast());
    }
}

/// If an extendable structure A implements ExtendingStructure< B >
/// This means A can be used to extend B
/// For example, VkPhysicalDeviceFeatures2 can be used to extend VkDeviceCreateInfo
/// So vk::PhysicalDeviceFeatures2 has the trait ExtendingStructure<vk::DeviceCreateInfo>
/// This is used for additional security, making it impossible to extend a structure
/// with an extension that wasn't planed for this structure
pub unsafe trait ExtendingStructure<T: ExtendableStructure>: ExtendableStructure {}

/// For simplicity, say that every structure can extend itself
unsafe impl<T: ExtendableStructure> ExtendingStructure<T> for T {}

#[macro_export]
macro_rules! vk_handle {
    ($name:ident, $obj_type:ident, $doc_tag:meta, $ty:ident) => {
        #[repr(transparent)]
        #[$doc_tag]
        #[derive(Eq, PartialEq, PartialOrd, Ord, Hash)]
        pub struct $name($ty);

        impl private::Sealed for $name {}
        impl Handle for $name {
            type InnerType = $ty;

            const TYPE: ObjectType = ObjectType::$obj_type;

            fn as_raw(&self) -> $ty {
                self.0
            }

            unsafe fn from_raw(x: $ty) -> Self {
                Self(x)
            }

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

#[repr(C)]
pub struct Header {
    s_type: StructureType,
    p_next: Cell<*const Header>,
}

/// Structure chain trait
pub unsafe trait StructureChain<H>: AsRef<H> + AsMut<H>
where
    H: ExtendableStructure,
{
    /// Create a new structure chain, the head is created with the default constructor
    /// If the extension types can be known at this times (not a dynamic structure chain),
    /// they are also created with their default constructor and pushed
    fn new() -> Self;

    /// Return a mutable reference to the given structure
    /// Will panic if this structure is not part of the structure chain
    fn get_mut<T: ExtendingStructure<H>>(&mut self) -> &mut T;

    /// Return a reference to the given structure
    /// Will panic if this structure is not part of the structure chain
    fn get<T: ExtendingStructure<H>>(&self) -> &T;

    /// Unlink the given structure from the chain
    /// Will panic if this structure is not part of the structure chain
    fn unlink<T: ExtendingStructure<H>>(&mut self);

    /// Link the given structure from the chain
    /// Do not call this on a structure that has not been unlinked previously
    /// Calling link on an already linked structure is safe but has the side effect of unlinking
    /// all the other structures linked before the two link calls (which you probably do not want)
    /// Will panic if this structure is not part of the structure chain
    fn link<T: ExtendingStructure<H>>(&mut self);
}

macro_rules! make_structure_chain_type {
    ($name: ident, $($ext_ty:ident => $ext_name:ident),*) => {

pub struct $name<H, $($ext_ty),*>
where
    H: ExtendableStructure,
    $($ext_ty: ExtendingStructure<H>),*
{
    head: H,
    $($ext_name: ($ext_ty, bool),)*
    has_changed: Cell<bool>,
}

impl<H, $($ext_ty),*>  $name<H, $($ext_ty),*>
where
    H: ExtendableStructure,
    $($ext_ty: ExtendingStructure<H>),* {

        fn perform_linking(&self) {
            self.has_changed.set(false);
            let mut _prev_ptr = ptr::null();
            $(
                if self.$ext_name.1 {
                    unsafe { self.$ext_name.0.retrieve_next().set(_prev_ptr) };
                    _prev_ptr = ptr::from_ref(&self.$ext_name.0).cast();
                }
            )*
            unsafe { self.head.retrieve_next().set(_prev_ptr) };
        }
    }

impl<H, $($ext_ty),*> AsRef<H> for $name<H, $($ext_ty),*>
where
    H: ExtendableStructure,
    $($ext_ty: ExtendingStructure<H>),* {
        fn as_ref(&self) -> &H {
            if self.has_changed.get(){
                self.perform_linking();
            }
            &self.head
        }
    }

impl<H, $($ext_ty),*> AsMut<H> for $name<H, $($ext_ty),*>
    where
        H: ExtendableStructure,
        $($ext_ty: ExtendingStructure<H>),* {
            fn as_mut(&mut self) -> &mut H {
                if self.has_changed.get(){
                    self.perform_linking();
                }
                &mut self.head
            }
    }

unsafe impl<H, $($ext_ty),*> StructureChain<H> for $name<H, $($ext_ty),*>
where
    H: ExtendableStructure,
    $($ext_ty: ExtendingStructure<H>),*
{
    fn new() -> Self {
        Self {
            head: Default::default(),
            $($ext_name: (Default::default(), true),)*
            has_changed: Cell::new(true),
        }
    }

    fn get_mut<T: ExtendingStructure<H>>(&mut self) -> &mut T {
        if H::STRUCTURE_TYPE == T::STRUCTURE_TYPE {
            unsafe {
                ptr::from_mut(AsMut::<H>::as_mut(self))
                    .cast::<T>()
                    .as_mut()
                    .unwrap_unchecked()
            }
        } $(else if $ext_ty::STRUCTURE_TYPE == T::STRUCTURE_TYPE {
            unsafe {
                ptr::from_mut(&mut self.$ext_name)
                    .cast::<T>()
                    .as_mut()
                    .unwrap_unchecked()
            }
        })* else {
            panic!(
                "Unexpected type for structure chain {}",
                std::any::type_name::<H>()
            )
        }
    }

    fn get<T: ExtendingStructure<H>>(&self) -> &T {
        if H::STRUCTURE_TYPE == T::STRUCTURE_TYPE {
            unsafe {
                ptr::from_ref(AsRef::<H>::as_ref(self))
                    .cast::<T>()
                    .as_ref()
                    .unwrap_unchecked()
            }
        } $(else if $ext_ty::STRUCTURE_TYPE == T::STRUCTURE_TYPE {
            unsafe {
                ptr::from_ref(&self.$ext_name)
                    .cast::<T>()
                    .as_ref()
                    .unwrap_unchecked()
            }
        })* else {
            panic!(
                "Unexpected type for structure chain {}",
                std::any::type_name::<H>()
            )
        }
    }

    fn unlink<T: ExtendingStructure<H>>(&mut self) {
        if H::STRUCTURE_TYPE == T::STRUCTURE_TYPE {
            panic!("Cannot unlink head structure!");
        }
        self.has_changed.set(true);

        if false {
        } $(else if $ext_ty::STRUCTURE_TYPE == T::STRUCTURE_TYPE {
            self.$ext_name.1 = false;
        })* else {
            panic!(
                "Unexpected type for structure chain {}",
                std::any::type_name::<H>()
            )
        }
    }

    fn link<T: ExtendingStructure<H>>(&mut self) {
        if H::STRUCTURE_TYPE == T::STRUCTURE_TYPE {
            panic!("Head structure is always linked!");
        }
        self.has_changed.set(true);

        if false {
        } $(else if $ext_ty::STRUCTURE_TYPE == T::STRUCTURE_TYPE {
            self.$ext_name.1 = true;
        })* else {
            panic!(
                "Unexpected type for structure chain {}",
                std::any::type_name::<H>()
            )
        }
    }
}
};
}

make_structure_chain_type! {StructureChain0,}
make_structure_chain_type! {StructureChain1, V1 => ext1}
make_structure_chain_type! {StructureChain2, V1 => ext1, V2 => ext2}
make_structure_chain_type! {StructureChain3, V1 => ext1, V2 => ext2, V3 => ext3}
make_structure_chain_type! {StructureChain4, V1 => ext1, V2 => ext2, V3 => ext3, V4 => ext4}
make_structure_chain_type! {StructureChain5, V1 => ext1, V2 => ext2, V3 => ext3, V4 => ext4, V5 => ext5}
make_structure_chain_type! {StructureChain6, V1 => ext1, V2 => ext2, V3 => ext3, V4 => ext4, V5 => ext5, V6 => ext6}

#[macro_export]
macro_rules! create_structure_chain {
    ($head:ty $(,)?) => {
        StructureChain0::<$head>::new()
    };
    ($head:ty, $ext1:ty $(,)?) => {
        StructureChain1::<$head, $ext1>::new()
    };
    ($head:ty, $ext1:ty, $ext2:ty $(,)?) => {
        StructureChain2::<$head, $ext1, $ext2>::new()
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty $(,)?) => {
        StructureChain3::<$head, $ext1, $ext2, $ext3>::new()
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty, $ext4:ty $(,)?) => {
        StructureChain4::<$head, $ext1, $ext2, $ext3, $ext4>::new()
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty, $ext4:ty, $ext5:ty $(,)?) => {
        StructureChain5::<$head, $ext1, $ext2, $ext3, $ext4, $ext5>::new()
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty, $ext4:ty, $ext5:ty, $ext6:ty $(,)?) => {
        StructureChain6::<$head, $ext1, $ext2, $ext3, $ext4, $ext5, $ext6>::new()
    };
}

#[macro_export]
macro_rules! structure_chain_type {
    ($head:ty $(,)?) => {
        StructureChain0<$head>
    };
    ($head:ty, $ext1:ty $(,)?) => {
        StructureChain1<$head, $ext1>
    };
    ($head:ty, $ext1:ty, $ext2:ty $(,)?) => {
        StructureChain2<$head, $ext1, $ext2>
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty $(,)?) => {
        StructureChain3<$head, $ext1, $ext2, $ext3>
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty, $ext4:ty $(,)?) => {
        StructureChain4<$head, $ext1, $ext2, $ext3, $ext4>
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty, $ext4:ty, $ext5:ty $(,)?) => {
        StructureChain5<$head, $ext1, $ext2, $ext3, $ext4, $ext5>
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty, $ext4:ty, $ext5:ty, $ext6:ty $(,)?) => {
        StructureChain6<$head, $ext1, $ext2, $ext3, $ext4, $ext5, $ext6>
    };
}
