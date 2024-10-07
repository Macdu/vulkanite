//! Unofficial bindings for the Vulkan Graphics API
//!
//! The goal is to provide a nice and safer way to use Vulkan with Rust
//! while having in most cases no overhead.
//!
//! These bindings try to replicate an experience similar to using the official Vulkan C++ bindings on Rust
//! which makes it differ from the popular Vulkan crate ash.
//!
//! # Features
//! ## Safety
//! Vulkan handles can be mostly be seen as references to Vulkan object. As such, using this crate, Vulkan handles **cannot be NULL**
//! (there is no vk::NULL_HANDLE value), being given a handle means you can assume it is not null and valid.
//! All vulkan functions which take as parameters handles which can be null now instead take as parameter an [`Option<Handle>`].
//!
//! Concerning command safety, I decided to take an approach similar to the `cxx` crate: guarantee safety at the boundary between rust and Vulkan API/driver code (likely written in C). These bindings also try to ensure that anything that can be checked to be according to the Vulkan Specification while being mostly cost-free / ran at compile time is done this way.
//!
//! When using the Vulkan API, driver code will be called which is possibly proprietary and on which you have no control. Even if you completely follow the Vulkan Specification and have no validation error, you might still get some surprise segfault when running your program on some GPUs/drivers (I speak from experience). As such the first solution would be to make every vulkan command or function calling a vulkan command unsafe, but this is from my point of view counter-productive. I chose to keep most Vulkan commands safe. The exceptions are destroy commands for which you must ensure everything created by what you are about to destroyed have already been destroyed.
//!
//! Note that these bindings assume the driver implementation complies, at least minimally, with the Vulkan Specification. In particular if the driver returns a completely unkown `VkStatus` code (which is not allowed by the specification), this
//! will lead to undefined behavior in the rust code.
//!
//! ## Smart handles
//! Similar to the C++ bindings, this binding groups vulkan commands by the handle which 'executes' it. Therefore the owing code on ash:
//! ```
//! unsafe {
//!     device.begin_command_buffer(
//!         &vk::CommandBufferBeginInfo::default()
//!             .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
//!     )
//!     ...
//!     swapchain_khr.queue_present_khr(queue, &present_info)?;
//! }
//! ```
//! becomes:
//! ```
//! cmd_buffer.begin(
//!     &vk::CommandBufferBeginInfo::default()
//!        .flags(vk::CommandBufferUsageFlags::OneTimeSubmit)
//! )?;
//!
//! queue.present_khr(&present_info)?;
//! ```
//!
//! ## Result
//! The Vulkan `VkResult` enum as been rename as [vk::Status] (this is the only enum/structure whose name is different ared to the C bindings).
//! Instead [`vk::Result<A>`] is defined as [Result<A, vk::Status>] and all Vulkan commands which return a Result in the inal specification instead
//! return a [vk::Result] now:
//! ```
//! let device = instance.create_device(&device_info)?;
//! ```
//! Moreover, if a Vulkan command can return multiple success codes, the status will be part of the result:
//! ```
//! let (status, image_idx) = self.device.acquire_next_image_khr(
//!    &self.swapchain_objects.swapchain,
//!    u64::MAX,
//!    Some(semaphore),
//!    None, // not signaling any fence
//! )?;
//! if status == vk::Status::vk::Status::SuboptimalKHR {
//!     ...
//! }
//! ```
//!
//! ## Slices
//! Every vulkan command or structure that takes as input a length along a raw pointer now takes as input a slice.
//! If a length is used for multiple raw pointers, the matching slices must be entered together and it is checked
//! that they have the same length.
//! ```
//! impl CommandBuffer {
//!     pub fn set_viewport(&self, first_viewport: u32, viewports: impl AsSlice<vk::Viewport>);
//! }
//!
//! cmd_buffer.set_viewport(0, &[vk::Viewport{..}, vk::Viewport{..}])
//!
//! let color_attachment = vec![...];
//! let resolve_attachment = vec![...];
//! let subpass_description = vk::SubpassDescription::default()
//!     // the resolve attachment field is optional for SubpassDescription
//!     // but if it is supplied it must have the same length as color_attachment
//!     .color_attachment(&color_attachment, Some(&resolve_attachment))
//!     ....;
//! ```
//!
//! ## Arrays as command outputs
//! For Vulkan commands that return an array (a length pointer and data pointer), it is instead possible
//! to use as output any structure implementing the [DynamicArray] (and [AdvancedDynamicArray] for the case of
//! smart handles). This is the case of [Vec]:
//! ```
//! let surface_formats : Vec<_> = physical_device.get_surface_formats_khr(Some(surface))?;
//! ```
//! The reason `: Vec<_>` has to be added is when using the `smallvec` feature, it is possible to do the following:
//! ```
//! // won't make any heap allocation if you have less than 3 GPUs
//! let physical_devices: SmallVec<[_; 3]> = instance.enumerate_physical_devices()?;
//! ```
//! Note that the case of [vk::Status::Incomplete] is handled by the implementation: you can assume [vk::Status::Incomplete]
//! is never returned
//!
//! ## Structure chain as command outputs
//! In case a vulkan command returns a structure that can be extended, a tuple can be used to specify which structure to ieve:
//! ```
//! let (vk_props, vk11_props) : (_, vk::PhysicalDeviceVulkan11Properties) = physical_device.get_properties2();
//! println!("Max supported API is {}, Subgroup size is {}", vk_props.properties.api_version,  vk11_props.subgroup_size);
//! ```
//! In case you don't want to use a structure chain, you have the 2 following choices (the type cannot be deduced explicitely in the default case):
//! ```
//! let vk_props: vk::PhysicalDeviceProperties2 = physical_device.get_properties2();
//! let (vk_props,) = physical_device.get_properties2();
//! ```
//!
//! Note that the structure chain integrity is checked as compile time: the following code will lead to a compile error ::PhysicalDeviceVulkan11Features] cannot
//! be used in a structure chain whose head is [vk::PhysicalDeviceFeatures]):
//! ```
//! let (_, _) : (_, vk::PhysicalDeviceVulkan11Features) = physical_device.get_properties2();
//! ```
//! This compile-time check applies also to the next part:
//!
//! ## Structure chain as command inputs
//! The first possible way to build a structure chain is to use `structure.push_next(&mut next_structure)`. There are also iple
//! macros provided to do the same in a way similar to the C++ bindings:
//!
//! ```
//! let mut device_info = vk_headers::structure_chain!(
//!     vk::DeviceCreateInfo::default()
//!         .queue_create_infos(&queue_info)
//!         .enabled_features(Some(&features))
//!         .enabled_extension(&required_extensions),
//!     vk::PhysicalDeviceShaderObjectFeaturesEXT::default().shader_object(true)
//! );
//!
//! if does_not_support_extension {
//!     device_info.unlink::<vk::PhysicalDeviceShaderObjectFeaturesEXT>()
//! }
//!
//! if does_not_support_feature {
//!     device_info.get_mut::<vk::PhysicalDeviceShaderObjectFeaturesEXT>().shader_object(false);
//! }
//!
//! let device = physical_device.create_device(device_info.as_ref())?;
//! ```
//!
//! # Features
//!
//! The following features are available:
//! - `loaded`: Allow the crate to dynamically load the vulkan library using the `libloading` crate, see [Dispatcher::new_loaded]
//! - `smallvec`: Add support for the smallvec crate to minimize heap allocations, enabling this feature allows the following: `let physical_devices: SmallVec<[_; 3]> = instance.enumerate_physical_devices()?;`.
//! - `arrayvec`: Add support for the arrayvec crate to minimize heap allocations, enabling this feature allows the following: `let pipeline: ArrayVec<_; 1> = device.create_compute_pipelines(None, &create_info)?;`.
//! - `raw-window-handle`: Add interoperability with the raw-window-handle crate, to create surfaces from raw handles, see the [window] module
//!
//! # MSRV
//! The current MSRV for this crate is Rust 1.77 (C-String literals are heavily used). It is not planned to increase
//! this version anytime soon and if this happens a reasonable (at least 6 months old) MSRV will be chosen.
//!
//! Note that these bindings are still a Work-In-Process, the public API may see breaking changes if this improves
//! safety or how nice to use the code is.
//!
//! Please be aware that this crate should not be considered production ready yet, breaking changes are to be expected in the future versions.

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

#[cfg(feature = "arrayvec")]
use arrayvec::ArrayVec;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkGetInstanceProcAddr.html>
/// Entry point of the vulkan library, this is used to retrieve Vulkan functions
/// This function can be retrieved by loading the library using [Dispatcher::new_loaded], using your own library
/// loading code or some external libraries provide it like SDL with `SDL_Vulkan_GetVkGetInstanceProcAddr`
pub type GetInstanceProcAddrSignature =
    unsafe extern "system" fn(Option<vk::raw::Instance>, *const c_char) -> *const ();

/// Dispatcher type used to hold the [vk::CommandsDispatcher] object
/// The dispatcher type loads once all the vulkan commands that can be used then is used
/// every time a vulkan command is called.
/// It is initialized by calling [Dispatcher::new] with the [GetInstanceProcAddrSignature] entry point
/// Or when the `loaded` feature is enabled by calling [Dispatcher::new_loaded] which will load the library
/// There are two Dispatcher implementation provided:
/// - [DynamicDispatcher]: Store the commands in static memory, this allows for a cost-free command retrieval
/// but requires only at most one instance and one device to exist at any time. If this is not the case use the following implementation
/// - [MultiDispatcher]: Allocate the commands storage on the heap and reference count it. This allows for any number of vulkan
/// devices and instances to co-exist with their own dispatch table but incurs a small overhead cost (smart handles need to store an additional pointer
/// and arc cloning needs to be done each time a new smart handle is created)
pub trait Dispatcher: Clone {
    /// Return the associated [vk::CommandsDispatcher] with this Dispatcher
    /// You can then use the command table to call any command that has been loaded or load new commands
    fn get_command_dispatcher(&self) -> &vk::CommandsDispatcher;

    /// Create a new dispatcher given the get_instance_proc_addr entry point
    /// this will load basic (non-instance and non-device dependent) commands
    /// # Safety
    /// `get_instance_proc_addr` must behave as expected (for any input it should either return [ptr::null()]
    /// or a pointer to a function with the expected parameters and return value)
    unsafe fn new(get_instance_proc_addr: GetInstanceProcAddrSignature) -> Self;

    /// Internal function used to load a library and return the dispatcher along with a library object which ensures the llibrary
    /// is kept loaded while the object is alive
    /// # Safety
    /// The [libloading::Library] object must be dropped only after Vulkan is done being used (all objects have been unitialized and no
    /// vulkan command is called after)
    #[cfg(feature = "loaded")]
    unsafe fn new_loaded_and_lib(
    ) -> core::result::Result<(Self, libloading::Library), loaded::LoadingError> {
        let (proc_addr, lib) = loaded::load_proc_addr_and_lib()?;

        Ok((Self::new(proc_addr), lib))
    }

    fn clone_with_instance(&self, instance: &vk::raw::Instance) -> Self;
    fn clone_with_device(&self, device: &vk::raw::Device) -> Self;

    /// Create a loads the Vulkan library, retrieve the entry point from it and initialize the dispatcher using it*
    /// This will return an error if the vulkan library or its entry point cannot be found
    /// This function is unsafe because it needs to assume that the Vulkan library being loaded follows the Vulkan specification
    /// Library unloading depends on the implementation, for [MultiDispatcher] it happends as soon as all dispatcher are dropped.
    /// While for [DynamicDispatcher] one should call [DynamicDispatcher::unload()]
    #[cfg(feature = "loaded")]
    unsafe fn new_loaded() -> core::result::Result<Self, loaded::LoadingError>;
}

// TODO: this is safe (Option<fn> being set to None is guaranteed to match memory being zero-ed)
// but this unsafe is not necessary (can't use Default::default because it is not const...)
static DYNAMIC_DISPATCHER: vk::CommandsDispatcher = unsafe { std::mem::zeroed() };

/// Dynamic dispatcher
/// Dispatcher implementation loading commands in static memory. This is a cost-free abstraction
/// assuming you follow the safety rule below.
/// Cloning this object is free (the object has size 0 and the clone function is empty) and it never
/// makes any heap allocation. Use this dispatcher if you can.
///
/// # Safety
/// Using a dynamic dispatcher means that at any point, only at most one vulkan instance
/// and at most one vulkan device exists. This is the case for most Vulkan program but if you cannot
/// guarantee it, use [MultiDispatcher] instead
#[derive(Clone, Copy)]
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

    #[cfg(feature = "loaded")]
    unsafe fn new_loaded() -> core::result::Result<Self, loaded::LoadingError> {
        let (result, lib) = Self::new_loaded_and_lib()?;

        loaded::DYNAMIC_VULKAN_LIB.0.set(Some(lib));
        Ok(result)
    }
}

#[cfg(feature = "loaded")]
impl DynamicDispatcher {
    /// Unloads the loaded library
    /// # Safety:
    /// Only call this function if the dispatcher was loaded with [Dispatcher::new_loaded].
    /// Only call this function after all vulkan handles have been freed/destroyed
    /// You cannot call any vulkan command before creating a new dispatcher.
    pub unsafe fn unload() {
        loaded::DYNAMIC_VULKAN_LIB.0.set(None);
    }
}

struct DispatcherWithLib {
    dispatcher: vk::CommandsDispatcher,
    #[cfg(feature = "loaded")]
    library: Option<std::sync::Arc<libloading::Library>>,
}

/// MultiDispatcher
/// Dispatcher implementation which stores vulkan commands on the heap using smart pointers
/// This adds a small overhead:
/// - Smart Vulkan handles must store an additional pointer
/// - Creating/dropping smart Vulkan Handles has the additional cost of cloning/dropping a [std::sync::Arc]
/// If you only use at most one vulkan instance and one device at any given time, you should use [DynamicDispatcher] instead
/// When the `loaded` feature is enabled and the dispatcher is loaded with [MultiDispatcher::new_loaded],
/// The vulkan library will be unloaded as soon as all dispatchers are dropped
#[derive(Clone)]
pub struct MultiDispatcher(std::sync::Arc<DispatcherWithLib>);

impl Dispatcher for MultiDispatcher {
    fn get_command_dispatcher(&self) -> &vk::CommandsDispatcher {
        &self.0.dispatcher
    }

    unsafe fn new(get_instance_proc_addr: GetInstanceProcAddrSignature) -> Self {
        let dispatcher = vk::CommandsDispatcher::default();
        dispatcher.load_proc_addr(get_instance_proc_addr);
        Self(std::sync::Arc::new(DispatcherWithLib {
            dispatcher,
            #[cfg(feature = "loaded")]
            library: None,
        }))
    }

    fn clone_with_instance(&self, instance: &vk::raw::Instance) -> Self {
        let dispatcher = self.0.dispatcher.clone();
        unsafe { dispatcher.load_instance(instance) };
        Self(std::sync::Arc::new(DispatcherWithLib {
            dispatcher,
            #[cfg(feature = "loaded")]
            library: self.0.library.clone(),
        }))
    }

    fn clone_with_device(&self, device: &vk::raw::Device) -> Self {
        let dispatcher = self.0.dispatcher.clone();
        unsafe { dispatcher.load_device(device) };
        Self(std::sync::Arc::new(DispatcherWithLib {
            dispatcher,
            #[cfg(feature = "loaded")]
            library: self.0.library.clone(),
        }))
    }

    #[cfg(feature = "loaded")]
    unsafe fn new_loaded() -> core::result::Result<Self, loaded::LoadingError> {
        let (mut result, lib) = Self::new_loaded_and_lib()?;

        // result holds the only reference to the inner dispatcher
        // so unwrap will never fail
        let library = std::sync::Arc::new(lib);
        std::sync::Arc::get_mut(&mut result.0).unwrap().library = Some(library);
        Ok(result)
    }
}

/// See <https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation>
/// Cost-free allocator implementation for Vulkan
/// Vulkan allows a custom memory allocator to be specified for host allocations
/// Note that the vulkan implementation is not required to use this allocator (for example it might have to allocate
/// memory with execute permissions), but you will at least receive the [Allocator::on_internal_alloc] and [Allocator::on_internal_free]
/// notifications
/// # Safety
/// The implementations of alloc/realloc/free must satisfy an allocator behavior and the requirements of the specification
/// If for some reason you choose the re-implement the pfn_* functions, they also need to follow the specification
pub unsafe trait Allocator: Sized + Clone {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkAllocationFunction.html>
    fn alloc(
        &self,
        size: usize,
        alignment: usize,
        allocation_scope: vk::SystemAllocationScope,
    ) -> *mut ();
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkReallocationFunction.html>
    fn realloc(
        &self,
        original: *mut (),
        size: usize,
        alignment: usize,
        allocation_scope: vk::SystemAllocationScope,
    ) -> *mut ();
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkFreeFunction.html>
    fn free(&self, memory: *mut ());
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkInternalAllocationNotification.html>
    fn on_internal_alloc(
        &self,
        size: usize,
        allocation_type: vk::InternalAllocationType,
        allocation_scope: vk::SystemAllocationScope,
    );
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkInternalFreeNotification.html>
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
    /// Moreover, as stated in <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAllocationCallbacks.html>
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
#[derive(Clone, Copy)]
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

/// Quality-Of-Life macro to create bitflags with multiple flags.
/// # Example
/// ```
/// let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
///     .message_severity(
///         flagbits!(vk::DebugUtilsMessageSeverityFlagsEXT::{Info | Warning | Error}),
///     );
/// ```
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

/// If A implements [`Alias<B>`], this means A and B have exactly the same memory representation
/// Thus transmuting from A to B is safe
pub unsafe trait Alias<T>: Sized {}

/// T has always the same memory representation as itself
unsafe impl<T> Alias<T> for T {}

/// A dispatchable or non-dispatchable Vulkan Handle
pub trait Handle: private::Sealed + Sized {
    type InnerType: Copy;
    const TYPE: vk::ObjectType;

    /// Retrieve the inner content of the vulkan handle, to be used by other Vulkan librairies not using this crate
    fn as_raw(&self) -> Self::InnerType;

    /// Convert a pointer to a handle
    /// When calling this code, the user must ensure the following:
    /// - The pointer given is a valid Vulkan handle for the appropriate type
    /// - The handle must live at least as long as the object being created
    unsafe fn from_raw(x: Self::InnerType) -> Self;

    /// Same as [Handle::from_raw] but allows for types that can be zero (usize or u64 depending on the handle)
    /// Will fail if x is null/zero
    unsafe fn try_from_raw<T>(x: T) -> Option<Self>
    where
        Self::InnerType: TryFrom<T>,
    {
        Self::InnerType::try_from(x).ok().map(|t| Self::from_raw(t))
    }

    /// Return a representation of &self
    /// The advantage is that BorrowedHandle<'a, Self> has internally the exact same memory
    /// representation as the raw handle it represents and therefore should be used when a deref is not enough
    /// like for vulkan commands that require arrays of handles
    fn borrow<'a>(&'a self) -> BorrowedHandle<'a, Self> {
        BorrowedHandle {
            value: self.as_raw(),
            phantom: PhantomData,
        }
    }

    /// See [Handle::borrow]
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

/// A trait implemented by Vulkan C structs whose first 2 fields are:
///     VkStructureType        sType;
///     const void*            pNext;
/// sType must always be set to STRUCTURE_TYPE
/// This trait contains the minimum to be object safe, [ExtendableStructure] extends on it
pub unsafe trait ExtendableStructureBase {
    fn header(&self) -> *const Header {
        ptr::from_ref(self).cast()
    }

    fn header_mut(&mut self) -> *mut Header {
        ptr::from_mut(self).cast()
    }
}

pub unsafe trait ExtendableStructure: ExtendableStructureBase + Default {
    const STRUCTURE_TYPE: vk::StructureType;

    /// SAFETY: Same as [ExtendableStructureBase::header]
    unsafe fn retrieve_next(&self) -> &Cell<*const Header> {
        &unsafe { &*self.header() }.p_next
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

#[repr(C)]
pub struct Header {
    s_type: vk::StructureType,
    p_next: Cell<*const Header>,
}

/// Represent an object that can be used as the return value of a vulkan function that outputs a structure chain
/// It must therefore internally represent what vulkan recognizes as a structure chain
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

#[doc(hidden)]
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

        pub fn new(head: H, $($ext_name: $ext_ty),*) -> Self {
            Self {
                head,
                $($ext_name: ($ext_name, true),)*
                has_changed: Cell::new(true),
            }
        }

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

impl<H, $($ext_ty),*> Default for $name<H, $($ext_ty),*>
where
    H: ExtendableStructure,
    $($ext_ty: ExtendingStructure<H>),*
{
    fn default() -> Self {
        Self {
            head: Default::default(),
            $($ext_name: (Default::default(), true),)*
            has_changed: Cell::new(true),
        }
    }
}

unsafe impl<H, $($ext_ty),*> StructureChain<H> for $name<H, $($ext_ty),*>
where
    H: ExtendableStructure,
    $($ext_ty: ExtendingStructure<H>),*
{
    fn get_mut<T: ExtendingStructure<H>>(&mut self) -> &mut T {
        if H::STRUCTURE_TYPE == T::STRUCTURE_TYPE {
            self.perform_linking();
            unsafe {
                mem::transmute(self)
            }
        } $(else if $ext_ty::STRUCTURE_TYPE == T::STRUCTURE_TYPE {
            unsafe {
                mem::transmute(self)
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
            self.perform_linking();
            unsafe {
                mem::transmute(self)
            }
        } $(else if $ext_ty::STRUCTURE_TYPE == T::STRUCTURE_TYPE {
            unsafe {
                mem::transmute(self)
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

/// Structure Chain that can take an arbitrary number of structures extending it
/// This is done by putting the structures on the heap
pub struct StructureChainVec<H: ExtendableStructure> {
    head: H,
    content: Vec<(Box<dyn ExtendableStructureBase>, Cell<bool>)>,
    has_changed: Cell<bool>,
}

impl<H> StructureChainVec<H>
where
    H: ExtendableStructure,
{
    pub fn new(head: H) -> Self {
        Self::new_with_capacity(head, 0)
    }

    pub fn new_with_capacity(head: H, capacity: usize) -> Self {
        Self {
            head,
            content: Vec::with_capacity(capacity),
            has_changed: Cell::new(true),
        }
    }

    /// Add a new structure to the structure chain
    /// Note: No check is done that the structure is not already part of this structure chain
    /// When pushing a structure, it is pushed in a linked state
    pub fn push<T: ExtendingStructure<H> + 'static>(&mut self, structure: T) {
        self.has_changed.set(true);
        self.content.push((Box::new(structure), Cell::new(true)));
    }

    fn perform_linking(&self) {
        self.has_changed.set(false);
        let mut prev_ptr = ptr::null();
        for (structure, is_linked) in &self.content {
            if is_linked.get() {
                let next_header = structure.header();
                unsafe { &*next_header }.p_next.set(prev_ptr);
                prev_ptr = next_header;
            }
        }
        unsafe { self.head.retrieve_next().set(prev_ptr) };
    }
}

impl<H> AsRef<H> for StructureChainVec<H>
where
    H: ExtendableStructure,
{
    fn as_ref(&self) -> &H {
        if self.has_changed.get() {
            self.perform_linking();
        }
        &self.head
    }
}

impl<H> AsMut<H> for StructureChainVec<H>
where
    H: ExtendableStructure,
{
    fn as_mut(&mut self) -> &mut H {
        if self.has_changed.get() {
            self.perform_linking();
        }
        &mut self.head
    }
}

unsafe impl<H> StructureChain<H> for StructureChainVec<H>
where
    H: ExtendableStructure,
{
    fn get_mut<T: ExtendingStructure<H>>(&mut self) -> &mut T {
        if H::STRUCTURE_TYPE == T::STRUCTURE_TYPE {
            self.perform_linking();
            return unsafe { mem::transmute(self) };
        }

        for (structure, _) in &mut self.content {
            let header = structure.header_mut();
            if unsafe { (*header).s_type } == H::STRUCTURE_TYPE {
                return unsafe { mem::transmute(header) };
            }
        }

        panic!(
            "Type {} is not part of the structure chain",
            std::any::type_name::<H>()
        )
    }

    fn get<T: ExtendingStructure<H>>(&self) -> &T {
        if H::STRUCTURE_TYPE == T::STRUCTURE_TYPE {
            self.perform_linking();
            return unsafe { mem::transmute(self) };
        }

        for (structure, _) in &self.content {
            let header = structure.header();
            if unsafe { (*header).s_type } == H::STRUCTURE_TYPE {
                return unsafe { mem::transmute(header) };
            }
        }

        panic!(
            "Type {} is not part of the structure chain",
            std::any::type_name::<H>()
        )
    }

    fn unlink<T: ExtendingStructure<H>>(&mut self) {
        if H::STRUCTURE_TYPE == T::STRUCTURE_TYPE {
            panic!("Cannot unlink head structure!");
        }
        self.has_changed.set(true);

        for (structure, is_linked) in &self.content {
            let header = structure.header();
            if unsafe { (*header).s_type } == H::STRUCTURE_TYPE {
                is_linked.set(false);
                return;
            }
        }

        panic!(
            "Type {} is not part of the structure chain",
            std::any::type_name::<H>()
        )
    }

    fn link<T: ExtendingStructure<H>>(&mut self) {
        if H::STRUCTURE_TYPE == T::STRUCTURE_TYPE {
            panic!("Head structure is always linked!");
        }
        self.has_changed.set(true);

        for (structure, is_linked) in &self.content {
            let header = structure.header();
            if unsafe { (*header).s_type } == H::STRUCTURE_TYPE {
                is_linked.set(true);
                return;
            }
        }

        panic!(
            "Type {} is not part of the structure chain",
            std::any::type_name::<H>()
        )
    }
}

#[macro_export]
macro_rules! create_structure_chain {
    ($head:ty $(,)?) => {
        $crate::StructureChain0::<$head>::default()
    };
    ($head:ty, $ext1:ty $(,)?) => {
        $crate::StructureChain1::<$head, $ext1>::default()
    };
    ($head:ty, $ext1:ty, $ext2:ty $(,)?) => {
        $crate::StructureChain2::<$head, $ext1, $ext2>::default()
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty $(,)?) => {
        $crate::StructureChain3::<$head, $ext1, $ext2, $ext3>::default()
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty, $ext4:ty $(,)?) => {
        $crate::StructureChain4::<$head, $ext1, $ext2, $ext3, $ext4>::default()
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty, $ext4:ty, $ext5:ty $(,)?) => {
        $crate::StructureChain5::<$head, $ext1, $ext2, $ext3, $ext4, $ext5>::default()
    };
    ($head:ty, $ext1:ty, $ext2:ty, $ext3:ty, $ext4:ty, $ext5:ty, $ext6:ty $(,)?) => {
        $crate::StructureChain6::<$head, $ext1, $ext2, $ext3, $ext4, $ext5, $ext6>::default()
    };
}

#[macro_export]
macro_rules! structure_chain {
    ($head:expr) => {
        $crate::StructureChain0::new($head)
    };
    ($head:expr, $ext1:expr $(,)?) => {
        $crate::StructureChain1::new($head, $ext1)
    };
    ($head:expr, $ext1:expr, $ext2:expr $(,)?) => {
        $crate::StructureChain2::new($head, $ext1, $ext2)
    };
    ($head:expr, $ext1:expr, $ext2:expr, $ext3:expr $(,)?) => {
        $crate::StructureChain3::new($head, $ext1, $ext2, $ext3)
    };
    ($head:expr, $ext1:expr, $ext2:expr, $ext3:expr, $ext4:expr $(,)?) => {
        $crate::StructureChain4::new($head, $ext1, $ext2, $ext3, $ext4)
    };
    ($head:expr, $ext1:expr, $ext2:expr, $ext3:expr, $ext4:expr, $ext5:expr $(,)?) => {
        $crate::StructureChain5::new($head, $ext1, $ext2, $ext3, $ext4, $ext5)
    };
    ($head:expr, $ext1:expr, $ext2:expr, $ext3:expr, $ext4:expr, $ext5:expr, $ext6:expr $(,)?) => {
        $crate::StructureChain6::new($head, $ext1, $ext2, $ext3, $ext4, $ext5, $ext6)
    };
    ($head:expr, $($ext:expr),*  $(,)?) => {{
        // TODO: this can be optimized using new_with_capacity
        let mut chain = $crate::StructureChainVec::new($head);
        $(
            chain.push($ext);
        )*
        chain
    }}
}

/// Includes a file as a reference to a u32 array.
/// This macro is really similar to rust macro [include_bytes], the main difference is that data is provided as a u32 array instead of a u8 array
/// As a consequence the data is 4-byte aligned. Moreover, if the file included has not a size which is a multiple of 4 bytes, it will cause a compile-time error
/// The main purpose of this macro in this library is to embed spirv code in a program, as include_bytes! requires at least an additional copy and can easily be misused for this case
///
/// The file is located relative to the current file (similarly to how modules are found). The provided path is interpreted in a platform-specific way at compile time. So, for instance, an invocation with a Windows path containing backslashes \ would not compile correctly on Unix.
///
/// This macro will yield an expression of type &'static \[u32; N\] which is the contents of the file.
/// This macro is inspired by <https://users.rust-lang.org/t/can-i-conveniently-compile-bytes-into-a-rust-program-with-a-specific-alignment/24049>
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
            bytes: Bytes,
        }

        static ALIGNED: &'static AlignedStruct<[u8]> = {
            let bytes = include_bytes!($path);
            assert!(
                bytes.len() % 4 == 0,
                concat!(
                    "The file ",
                    $path,
                    " must have a size which is a multiple of 4 bytes"
                )
            );
            &AlignedStruct { bytes: *bytes }
        };

        unsafe {
            std::slice::from_raw_parts(
                ALIGNED.bytes.as_ptr() as *const u32,
                ALIGNED.bytes.len() / 4,
            )
        }
    }};
}

/// A trait implemented by types which can allocate memory for an array of given size in a contiguous memory
/// This is used for vulkan commands returning arrays
/// [`Vec<T>`] implements this trait as well as [SmallVec] if the smallvec feature is enabled and [ArrayVec] if the arrayvec feature is enabled
/// This trait is unsafe because no allocating a memory area of the proper size when calling
/// allocate_with_capacity can cause undefined behavior when using this library
pub unsafe trait DynamicArray<T>: IntoIterator<Item = T> {
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
pub trait AdvancedDynamicArray<T, S>: DynamicArray<T> + FromIterator<T> {
    type InnerArrayType: DynamicArray<S>;
}

unsafe impl<T> DynamicArray<T> for Vec<T> {
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

impl<T, S> AdvancedDynamicArray<T, S> for Vec<T> {
    type InnerArrayType = Vec<S>;
}

#[cfg(feature = "smallvec")]
unsafe impl<T, A> DynamicArray<T> for SmallVec<A>
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
impl<T, S, const N: usize> AdvancedDynamicArray<T, S> for SmallVec<[T; N]> {
    type InnerArrayType = SmallVec<[S; N]>;
}

#[cfg(feature = "arrayvec")]
unsafe impl<T, const N: usize> DynamicArray<T> for ArrayVec<T, N> {
    fn create_with_capacity(capacity: usize) -> Self {
        if capacity > N {
            panic!("Trying to use an ArrayVec of size {N} with capacity {capacity}")
        }
        Self::new()
    }

    fn update_with_capacity(&mut self, new_capacity: usize) {
        if new_capacity > N {
            panic!("Trying to use an ArrayVec of size {N} with capacity {new_capacity}")
        }
        // ArrayVecs always have a fixed capacity
    }

    fn get_content_mut_ptr(&mut self) -> *mut T {
        self.as_mut_ptr()
    }

    unsafe fn resize_with_len(&mut self, len: usize) {
        self.set_len(len)
    }
}

#[cfg(feature = "arrayvec")]
impl<T, S, const N: usize> AdvancedDynamicArray<T, S> for ArrayVec<T, N> {
    type InnerArrayType = ArrayVec<S, N>;
}

/// Custom type which represents types that can be seen as slices
/// This is especially useful for this crate as there are multiple commands/structs
/// which accept slices but for which one would usually only supply one element
/// using [std::slice::from_ref].
///
/// With this trait, all of these `from_ref` calls
/// are avoided. There is also an implementation for [`Option<&T>`]
pub trait AsSlice<'a, T>: Copy {
    fn as_slice(self) -> &'a [T];
}

impl<'a, T> AsSlice<'a, T> for &'a [T] {
    fn as_slice(self) -> &'a [T] {
        self
    }
}

impl<'a, T, const N: usize> AsSlice<'a, T> for &'a [T; N] {
    fn as_slice(self) -> &'a [T] {
        self
    }
}

impl<'a, T> AsSlice<'a, T> for &'a T {
    fn as_slice(self) -> &'a [T] {
        std::slice::from_ref(self)
    }
}

impl<'a, T> AsSlice<'a, T> for &'a Option<T> {
    fn as_slice(self) -> &'a [T] {
        self.as_slice()
    }
}

impl<'a, T> AsSlice<'a, T> for Option<&'a T> {
    fn as_slice(self) -> &'a [T] {
        self.map_or(&[], std::slice::from_ref)
    }
}

impl<'a, T> AsSlice<'a, T> for &'a Vec<T> {
    fn as_slice(self) -> &'a [T] {
        self.as_slice()
    }
}

impl<'a, T> AsSlice<'a, T> for &'a Box<T> {
    fn as_slice(self) -> &'a [T] {
        std::slice::from_ref(self)
    }
}

/// Implement the AsSlice trait for `()``, some vulkan commands/structs take as parameter `Option<impl AsSlice<...>>`
/// With this type, if you want to to give as parameter None (the compiler cannot infer the type, although this is not useful)
/// you can use [`None::<()>`] (instead of `None::<&vk::AttachmentReference>` for example):
///
/// # Example
///
/// ```
/// let subpass = vk::SubpassDescription::default()
///     .pipeline_bind_point(vk::PipelineBindPoint::Graphics)
///     .color_attachment(&color_ref, None::<()>);
/// ```
impl<'a, T> AsSlice<'a, T> for () {
    fn as_slice(self) -> &'a [T] {
        &[]
    }
}

#[cfg(feature = "smallvec")]
impl<'a, T, const N: usize> AsSlice<'a, T> for &'a SmallVec<[T; N]> {
    fn as_slice(self) -> &'a [T] {
        self.as_slice()
    }
}

#[cfg(feature = "arrayvec")]
impl<'a, T, const N: usize> AsSlice<'a, T> for &'a ArrayVec<T, N> {
    fn as_slice(self) -> &'a [T] {
        self.as_slice()
    }
}
