#[cfg(feature = "loaded")]
mod loaded;
pub mod vk;
#[cfg(feature = "raw-window-handle")]
pub mod window;

use std::cell::Cell;
use std::ffi::c_char;
use std::marker::PhantomData;
use std::mem::{self, MaybeUninit};
use std::ptr::{self};

#[cfg(feature = "smallvec")]
use smallvec::SmallVec;

pub type GetInstanceProcAddrSignature =
    unsafe extern "system" fn(Option<vk::raw::Instance>, *const c_char) -> *const ();

pub trait Dispatcher: Clone {
    fn get_command_dispatcher(&self) -> &vk::CommandsDispatcher;

    unsafe fn new(get_instance_proc_addr: GetInstanceProcAddrSignature) -> Self;

    #[cfg(feature = "loaded")]
    unsafe fn new_loaded_and_lib(
    ) -> core::result::Result<(Self, libloading::Library), loaded::LoadingError> {
        let (proc_addr, lib) = loaded::load_proc_addr_and_lib()?;

        Ok((Self::new(proc_addr), lib))
    }

    fn clone_with_instance(&self, instance: &vk::raw::Instance) -> Self;
    fn clone_with_device(&self, device: &vk::raw::Device) -> Self;
}

// TODO: this is safe (Option<fn> being set to None is guaranteed to match memory being zero-ed)
// but this unsafe is not necessary (can't use Default::default because it is not const...)
static DYNAMIC_DISPATCHER: vk::CommandsDispatcher = unsafe { std::mem::zeroed() };

#[derive(Clone)]
pub struct DynamicDispatcher(pub(crate) ());

impl Dispatcher for DynamicDispatcher {
    fn get_command_dispatcher(&self) -> &vk::CommandsDispatcher {
        &DYNAMIC_DISPATCHER
    }

    unsafe fn new(get_instance_proc_addr: GetInstanceProcAddrSignature) -> Self {
        DYNAMIC_DISPATCHER.load_proc_addr(get_instance_proc_addr);
        Self(())
    }

    fn clone_with_instance(&self, instance: &vk::raw::Instance) -> Self {
        unsafe { DYNAMIC_DISPATCHER.load_instance(instance) };
        Self(())
    }

    fn clone_with_device(&self, device: &vk::raw::Device) -> Self {
        unsafe { DYNAMIC_DISPATCHER.load_device(device) };
        Self(())
    }
}

impl DynamicDispatcher {
    #[cfg(feature = "loaded")]
    pub unsafe fn new_loaded() -> core::result::Result<Self, loaded::LoadingError> {
        let (result, lib) = Self::new_loaded_and_lib()?;

        loaded::DYNAMIC_VULKAN_LIB.0.set(Some(lib));
        Ok(result)
    }

    #[cfg(feature = "loaded")]
    pub unsafe fn unload() {
        loaded::DYNAMIC_VULKAN_LIB.0.set(None);
    }
}

/// See https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation
pub unsafe trait Allocator: Sized + Clone {
    /// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkAllocationFunction.html
    fn alloc(
        &self,
        size: usize,
        alignment: usize,
        allocation_scope: vk::SystemAllocationScope,
    ) -> *mut ();
    /// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkReallocationFunction.html
    fn realloc(
        &self,
        original: *mut (),
        size: usize,
        alignment: usize,
        allocation_scope: vk::SystemAllocationScope,
    ) -> *mut ();
    /// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkFreeFunction.html
    fn free(&self, memory: *mut ());
    /// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkInternalAllocationNotification.html
    fn on_internal_alloc(
        &self,
        size: usize,
        allocation_type: vk::InternalAllocationType,
        allocation_scope: vk::SystemAllocationScope,
    );
    /// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkInternalFreeNotification.html
    fn on_internal_free(
        &self,
        size: usize,
        allocation_type: vk::InternalAllocationType,
        allocation_scope: vk::SystemAllocationScope,
    );

    extern "system" fn pfn_allocation(
        user_data: *mut (),
        size: usize,
        alignment: usize,
        allocation_scope: vk::SystemAllocationScope,
    ) -> *mut () {
        let allocator: &Self = unsafe { &*user_data.cast() };
        allocator.alloc(size, alignment, allocation_scope)
    }

    extern "system" fn pfn_reallocation(
        user_data: *mut (),
        original: *mut (),
        size: usize,
        alignment: usize,
        allocation_scope: vk::SystemAllocationScope,
    ) -> *mut () {
        let allocator: &Self = unsafe { &*user_data.cast() };
        allocator.realloc(original, size, alignment, allocation_scope)
    }

    extern "system" fn pfn_free(user_data: *mut (), memory: *mut ()) {
        let allocator: &Self = unsafe { &*user_data.cast() };
        allocator.free(memory)
    }

    extern "system" fn pfn_internal_allocation(
        user_data: *mut (),
        size: usize,
        allocation_type: vk::InternalAllocationType,
        allocation_scope: vk::SystemAllocationScope,
    ) {
        let allocator: &Self = unsafe { &*user_data.cast() };
        allocator.on_internal_alloc(size, allocation_type, allocation_scope)
    }

    extern "system" fn pfn_internal_free(
        user_data: *mut (),
        size: usize,
        allocation_type: vk::InternalAllocationType,
        allocation_scope: vk::SystemAllocationScope,
    ) {
        let allocator: &Self = unsafe { &*user_data.cast() };
        allocator.on_internal_free(size, allocation_type, allocation_scope)
    }

    /// SAFETY:
    /// When re-implementing this function and using the provided pfn_* functions, you must ensure that the user_data value is a reference
    /// to self that lives as long as the allocation callback
    /// Moreover, as stated in https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAllocationCallbacks.html
    /// pfn_internal_allocation and pfn_internal_free can only either be both None or be both Some
    fn get_allocation_callbacks(&self) -> Option<vk::AllocationCallbacks> {
        Some(vk::AllocationCallbacks {
            p_user_data: (self as *const Self).cast(),
            pfn_allocation: Self::pfn_allocation as *const (),
            pfn_reallocation: Self::pfn_reallocation as *const (),
            pfn_free: Self::pfn_free as *const (),
            pfn_internal_allocation: Self::pfn_internal_allocation as *const (),
            pfn_internal_free: Self::pfn_free as *const (),
        })
    }
}

/// The default vulkan allocator, Using this allocator will let Vulkan use the default allocator
/// It is the same as specifying NULL (on C) or None (on Ash) every time the parameter pAllocator is required
#[derive(Clone)]
pub struct DefaultAllocator;

unsafe impl Allocator for DefaultAllocator {
    fn alloc(&self, _: usize, _: usize, _: vk::SystemAllocationScope) -> *mut () {
        ptr::null_mut()
    }

    fn realloc(&self, _: *mut (), _: usize, _: usize, _: vk::SystemAllocationScope) -> *mut () {
        ptr::null_mut()
    }

    fn free(&self, _: *mut ()) {}

    fn on_internal_alloc(
        &self,
        _: usize,
        _: vk::InternalAllocationType,
        _: vk::SystemAllocationScope,
    ) {
    }

    fn on_internal_free(
        &self,
        _: usize,
        _: vk::InternalAllocationType,
        _: vk::SystemAllocationScope,
    ) {
    }

    #[inline(always)]
    /// By returning None, we ask Vulkan to use its default allocator
    fn get_allocation_callbacks(&self) -> Option<vk::AllocationCallbacks> {
        None
    }
}

#[macro_export]
macro_rules! flagbits {
    ( $enum:ident ::{ $($variant:ident)|+ } ) => {
        $($enum::$variant)|+
    };
    ( vk :: $enum:ident ::{ $($variant:ident)|+ } ) => {
        $(vk::$enum::$variant)|+
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
    const TYPE: vk::ObjectType;

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
        unsafe { mem::transmute(self) }
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
        unsafe { mem::transmute(self) }
    }
}

/// A trait implemented by types which can allocate memory for an array of given size in a contiguous memory
/// This is used for vulkan commands returning arrays
/// Vec<T> implements this trait as well as SmallVec<[T;N]> is the small-vec feature is enabled
/// This trait is unsafe because no allocating a memory area of the proper size when calling
/// allocate_with_capacity can cause undefined behavior when using this library
pub unsafe trait DynamicArray<T: Sized>: IntoIterator<Item = T> {
    /// Returns an array with at least the given capacity available
    /// Calling get_content_mut_ptr on an object allocated with allocate_with_capacity(capacity) should return
    /// A contiguous properly aligned allocated region of memory which can hold capacity elements of T
    fn create_with_capacity(capacity: usize) -> Self;

    /// Called after creation (in the case where a Vulkan command returns VK_INCOMPLETE)
    /// The new capacity should be strictly greater than the current one
    /// You can assume the length of the vector is 0 when calling this function
    fn update_with_capacity(&mut self, new_capacity: usize);

    /// Returns a pointer to the array memory
    fn get_content_mut_ptr(&mut self) -> *mut T;

    /// Set the array length to size len
    /// The array must have been allocated with allocate_with_capacity(capacity)
    /// With capacity >= len and the first len elements of the array
    /// must be well defined
    unsafe fn resize_with_len(&mut self, len: usize);
}

/// When using advanced commands, we must be able to provide a dynamic array for both the type and the underlying type
/// This trait allows given a type T with a dynamic array to get a dynamic array for another type S
pub trait AdvancedDynamicArray<T: Sized, S: Sized>: DynamicArray<T> + FromIterator<T> {
    type InnerArrayType: DynamicArray<S>;
}

unsafe impl<T: Sized> DynamicArray<T> for Vec<T> {
    fn create_with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn update_with_capacity(&mut self, new_capacity: usize) {
        // we assume the length is 0, otherwise the appropriate value would be
        // (with underflow checking) new_capacity - self.len()
        self.reserve(new_capacity)
    }

    fn get_content_mut_ptr(&mut self) -> *mut T {
        self.as_mut_ptr()
    }

    unsafe fn resize_with_len(&mut self, len: usize) {
        self.set_len(len)
    }
}

impl<T: Sized, S: Sized> AdvancedDynamicArray<T, S> for Vec<T> {
    type InnerArrayType = Vec<S>;
}

#[cfg(feature = "smallvec")]
unsafe impl<T: Sized, A> DynamicArray<T> for SmallVec<A>
where
    A: smallvec::Array<Item = T>,
{
    fn create_with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn update_with_capacity(&mut self, new_capacity: usize) {
        self.reserve(new_capacity)
    }

    fn get_content_mut_ptr(&mut self) -> *mut T {
        self.as_mut_ptr()
    }

    unsafe fn resize_with_len(&mut self, len: usize) {
        self.set_len(len)
    }
}

#[cfg(feature = "smallvec")]
impl<T: Sized, S: Sized, const N: usize> AdvancedDynamicArray<T, S> for SmallVec<[T; N]> {
    type InnerArrayType = SmallVec<[S; N]>;
}

/// A trait implemented by Vulkan C structs whose first 2 fields are:
///     VkStructureType        sType;
///     const void*            pNext;
/// sType must always be set to STRUCTURE_TYPE
pub unsafe trait ExtendableStructure: Default {
    const STRUCTURE_TYPE: vk::StructureType;

    unsafe fn retrieve_next(&self) -> &Cell<*const Header> {
        unsafe { &mem::transmute::<_, &Header>(self).p_next }
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

    /// Return a unitialized structure except the structure type being correctly set
    /// and the p_next pointer being set to null
    fn new_uninit() -> MaybeUninit<Self> {
        let mut result: MaybeUninit<Self> = MaybeUninit::uninit();
        let header = Header {
            s_type: Self::STRUCTURE_TYPE,
            p_next: Cell::new(ptr::null()),
        };
        // SAFETY: result is a C struct which starts with the fields from Header
        unsafe { result.as_mut_ptr().cast::<Header>().write(header) };
        result
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
    s_type: vk::StructureType,
    p_next: Cell<*const Header>,
}

pub unsafe trait StructureChainOut<H>: Sized
where
    H: ExtendableStructure,
{
    /// Setup an uninitialized structure chain
    /// After this call, for the structure chain to be initialized, each structure field (with the exception of the structure type
    /// and the p_next pointer) must be initialized (usually by calling the appropriate vulkan command)
    /// The structure type and p_next pointer of each struct are set so that a vulkan commands sees a pointer to the head
    /// as a valid chain containing all structures
    /// Calling setup_uninit should be enough to then call a vulkan command filling this structure chain, moreover after
    /// the call to this vulkan command, the whole structure chain should be considered initialized
    fn setup_uninit(chain: &mut MaybeUninit<Self>);

    /// Return a mutable pointer to the head structure, which can then be passed to vulkan commands
    fn get_uninit_head_ptr(chain: *mut Self) -> *mut H;

    /// Function to call after a vulkan function initialized this structure to make sure there is no dangling pointer
    /// or anything which could cause undefined behavior
    fn setup_cleanup(chain: *mut Self) {
        // Clearing the dangling pointer from the head should be enough
        // A user should not be able to use the p_next pointer from the chain structure without unsafe code
        let head = Self::get_uninit_head_ptr(chain).cast::<Header>();
        unsafe { ptr::addr_of_mut!((*head).p_next).write(Cell::new(ptr::null())) };
    }
}

/// Structure chain trait
pub unsafe trait StructureChain<H>: AsRef<H> + AsMut<H> + Sized
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

unsafe impl<H: ExtendableStructure> StructureChainOut<H> for H {
    fn setup_uninit(chain: &mut MaybeUninit<Self>) {
        // SAFETY: H is a C struct which starts with Header
        unsafe {
            chain.as_mut_ptr().cast::<Header>().write(Header {
                s_type: Self::STRUCTURE_TYPE,
                p_next: Cell::new(ptr::null()),
            })
        }
    }

    fn get_uninit_head_ptr(chain: *mut Self) -> *mut H {
        chain
    }

    fn setup_cleanup(_: *mut Self) {
        // self.s_type is already empty, nothing to be done here
    }
}

macro_rules! make_structure_chain_type {
    ($name: ident, $($ext_ty:ident => ($ext_nb:tt, $ext_name:ident)),*) => {

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

unsafe impl<H, $($ext_ty),*> StructureChainOut<H> for $name<H, $($ext_ty),*>
where
    H: ExtendableStructure,
    $($ext_ty: ExtendingStructure<H>),*
{
    fn setup_uninit(chain: &mut MaybeUninit<Self>) {
        let chain_ptr = chain.as_mut_ptr();

        // SAFETY: Each structure in this chain is a C struct which start with
        // the fields from Header
        unsafe {
            ptr::addr_of_mut!((*chain_ptr).has_changed).write(Cell::new(false));

            let mut _prev_header = Header {
                s_type: H::STRUCTURE_TYPE,
                p_next: Cell::new(ptr::null()),
            };
            let prev_ptr: *mut Header = ptr::addr_of_mut!((*chain_ptr).head).cast();

            $(
                let ptr = ptr::addr_of_mut!((*chain_ptr).$ext_name.0).cast();
                _prev_header.p_next = Cell::new(ptr);
                prev_ptr.write(_prev_header);

                let prev_ptr = ptr;
                let mut _prev_header = Header {
                    s_type: $ext_ty::STRUCTURE_TYPE,
                    p_next: Cell::new(ptr::null()),
                };

                ptr::addr_of_mut!((*chain_ptr).$ext_name.1).write(true);
            )*

            prev_ptr.write(_prev_header);
        }
    }

    fn get_uninit_head_ptr(chain: *mut Self) -> *mut H {
        unsafe { ptr::addr_of_mut!((*chain).head).cast() }
    }
}

unsafe impl<H, $($ext_ty),*> StructureChainOut<H> for (H, $($ext_ty,)*)
where
    H: ExtendableStructure,
    $($ext_ty: ExtendingStructure<H>),*
{
    fn setup_uninit(chain: &mut MaybeUninit<Self>) {
        let chain_ptr = chain.as_mut_ptr();

        // SAFETY: Each structure in this chain is a C struct which start with
        // the fields from Header
        unsafe {
            let mut _prev_header = Header {
                s_type: H::STRUCTURE_TYPE,
                p_next: Cell::new(ptr::null()),
            };
            let prev_ptr: *mut Header = ptr::addr_of_mut!((*chain_ptr).0).cast();

            $(
                let ptr = ptr::addr_of_mut!((*chain_ptr).$ext_nb).cast();
                _prev_header.p_next = Cell::new(ptr);
                prev_ptr.write(_prev_header);

                let prev_ptr = ptr;
                let mut _prev_header = Header {
                    s_type: $ext_ty::STRUCTURE_TYPE,
                    p_next: Cell::new(ptr::null()),
                };
            )*

            prev_ptr.write(_prev_header);
        }
    }

    fn get_uninit_head_ptr(chain: *mut Self) -> *mut H {
        unsafe { ptr::addr_of_mut!((*chain).0).cast() }
    }
}
};
}

make_structure_chain_type! {StructureChain0,}
make_structure_chain_type! {StructureChain1, V1 => (1,ext1)}
make_structure_chain_type! {StructureChain2, V1 => (1,ext1), V2 => (2,ext2)}
make_structure_chain_type! {StructureChain3, V1 => (1,ext1), V2 => (2,ext2), V3 => (3,ext3)}
make_structure_chain_type! {StructureChain4, V1 => (1,ext1), V2 => (2,ext2), V3 => (3,ext3), V4 => (4,ext4)}
make_structure_chain_type! {StructureChain5, V1 => (1,ext1), V2 => (2,ext2), V3 => (3,ext3), V4 => (4,ext4), V5 => (5,ext5)}
make_structure_chain_type! {StructureChain6, V1 => (1,ext1), V2 => (2,ext2), V3 => (3,ext3), V4 => (4,ext4), V5 => (5,ext5), V6 => (6,ext6) }

#[macro_export]
macro_rules! create_structure_chain {
    ($head:ty $(,)?) => {
        $crate::StructureChain0::<$head>::new()
    };
    ($head:ty, $ext1:ty $(,)?) => {
        $crate::StructureChain1::<$head, $ext1>::new()
    };
    ($head:ty, $ext1:ty, $ext2:ty $(,)?) => {
        $crate::StructureChain2::<$head, $ext1, $ext2>::new()
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty $(,)?) => {
        $crate::StructureChain3::<$head, $ext1, $ext2, $ext3>::new()
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty, $ext4:ty $(,)?) => {
        $crate::StructureChain4::<$head, $ext1, $ext2, $ext3, $ext4>::new()
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty, $ext4:ty, $ext5:ty $(,)?) => {
        $crate::StructureChain5::<$head, $ext1, $ext2, $ext3, $ext4, $ext5>::new()
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty, $ext4:ty, $ext5:ty, $ext6:ty $(,)?) => {
        $crate::StructureChain6::<$head, $ext1, $ext2, $ext3, $ext4, $ext5, $ext6>::new()
    };
}

#[macro_export]
macro_rules! structure_chain {
    ($head:ty $(,)?) => {
        $crate::StructureChain0<$head>
    };
    ($head:ty, $ext1:ty $(,)?) => {
        $crate::StructureChain1<$head, $ext1>
    };
    ($head:ty, $ext1:ty, $ext2:ty $(,)?) => {
        $crate::StructureChain2<$head, $ext1, $ext2>
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty $(,)?) => {
        $crate::StructureChain3<$head, $ext1, $ext2, $ext3>
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty, $ext4:ty $(,)?) => {
        $crate::StructureChain4<$head, $ext1, $ext2, $ext3, $ext4>
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty, $ext4:ty, $ext5:ty $(,)?) => {
        $crate::StructureChain5<$head, $ext1, $ext2, $ext3, $ext4, $ext5>
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty, $ext4:ty, $ext5:ty, $ext6:ty $(,)?) => {
        $crate::StructureChain6<$head, $ext1, $ext2, $ext3, $ext4, $ext5, $ext6>
    };
}

/// Includes a file as a reference to a u32 array.
/// This macro is really similar to rust macro [include_bytes], the main difference is that data is provided as a u32 array instead of a u8 array
/// As a consequence the data is 4-byte aligned. Moreover, if the file included has not a size which is a multiple of 4 bytes, it will cause a compile-time error
/// The main purpose of this macro in this library is to embed spirv code in a program, as include_bytes! requires at least an additional copy and can easily be misused for this case
///
/// The file is located relative to the current file (similarly to how modules are found). The provided path is interpreted in a platform-specific way at compile time. So, for instance, an invocation with a Windows path containing backslashes \ would not compile correctly on Unix.
///
/// This macro will yield an expression of type &'static \[u32; N\] which is the contents of the file.
/// This macro is inspired by https://users.rust-lang.org/t/can-i-conveniently-compile-bytes-into-a-rust-program-with-a-specific-alignment/24049
/// # Example
/// ```
/// let vertex_shader = include_spirv!("vert.spirv");
/// let vertex_module = device.create_shader_module(
///     &vk::ShaderModuleCreateInfo::default().code(vertex_shader),
/// )?;
/// ```
#[macro_export]
macro_rules! include_spirv {
    ($path:literal) => {{
        #[repr(align(4))]
        struct AlignedStruct<Bytes: ?Sized> {
            bytes: Bytes
        }

        static ALIGNED: &'static AlignedStruct<[u8]> = {
            let bytes = include_bytes!($path);
            assert!(bytes.len() % 4 == 0, concat!("The file ", $path, " must have a size which is a multiple of 4 bytes"));
            &AlignedStruct{
                bytes: *bytes
            }
        };

        unsafe { std::slice::from_raw_parts(ALIGNED.bytes.as_ptr() as *const u32, ALIGNED.bytes.len() / 4) }
    }};
}
