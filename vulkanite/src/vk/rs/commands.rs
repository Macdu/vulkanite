#![allow(unused_unsafe)]
use crate::{
    vk::*, AdvancedDynamicArray, Alias, Allocator, AsSlice, DefaultAllocator, Dispatcher,
    DynamicArray, DynamicDispatcher, Handle, StructureChainOut,
};
use std::{
    ffi::{c_int, CStr},
    ops::Deref,
};
#[derive(Clone)]
pub struct Entry<D: Dispatcher = DynamicDispatcher, A: Allocator = DefaultAllocator> {
    disp: D,
    alloc: A,
}
impl<D: Dispatcher, A: Allocator> Copy for Entry<D, A>
where
    D: Copy,
    A: Copy,
{
}
impl<D: Dispatcher, A: Allocator> Entry<D, A> {
    pub fn new(disp: D, alloc: A) -> Self {
        Self { disp, alloc }
    }
    pub fn get_dispatcher(&self) -> &D {
        &self.disp
    }
    pub fn get_allocator(&self) -> &A {
        &self.alloc
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html>"]
    #[doc(alias = "vkCreateInstance")]
    pub fn create_instance(&self, p_create_info: &InstanceCreateInfo) -> Result<Instance<D, A>> {
        let vk_result = unsafe {
            raw::create_instance(
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|instance| {
            let disp = self.disp.clone_with_instance(&instance);
            unsafe { Instance::from_inner(instance, disp, self.alloc.clone()) }
        })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html>"]
    #[doc(alias = "vkEnumerateInstanceExtensionProperties")]
    pub fn enumerate_instance_extension_properties<R: DynamicArray<ExtensionProperties>>(
        &self,
        p_layer_name: Option<&CStr>,
    ) -> Result<R> {
        unsafe {
            raw::enumerate_instance_extension_properties(
                p_layer_name,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html>"]
    #[doc(alias = "vkEnumerateInstanceLayerProperties")]
    pub fn enumerate_instance_layer_properties<R: DynamicArray<LayerProperties>>(
        &self,
    ) -> Result<R> {
        unsafe { raw::enumerate_instance_layer_properties(self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceVersion.html>"]
    #[doc(alias = "vkEnumerateInstanceVersion")]
    pub fn enumerate_instance_version(&self) -> Result<u32> {
        unsafe { raw::enumerate_instance_version(self.disp.get_command_dispatcher()) }
    }
}
#[repr(C)]
#[derive(Clone)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInstance.html>"]
#[doc(alias = "VkInstance")]
pub struct Instance<D: Dispatcher = DynamicDispatcher, A: Allocator = DefaultAllocator> {
    inner: <raw::Instance as Handle>::InnerType,
    disp: D,
    alloc: A,
}
unsafe impl Alias<raw::Instance> for Instance {}
impl<D: Dispatcher, A: Allocator> Copy for Instance<D, A>
where
    D: Copy,
    A: Copy,
{
}
impl<D: Dispatcher, A: Allocator> Deref for Instance<D, A> {
    type Target = raw::Instance;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl<D: Dispatcher, A: Allocator> Instance<D, A> {
    pub unsafe fn from_inner(handle: raw::Instance, disp: D, alloc: A) -> Self {
        Self {
            inner: handle.as_raw(),
            disp,
            alloc,
        }
    }
    pub fn get_dispatcher(&self) -> &D {
        &self.disp
    }
    pub fn get_allocator(&self) -> &A {
        &self.alloc
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html>"]
    #[doc(alias = "vkDestroyInstance")]
    pub unsafe fn destroy(&self) {
        unsafe {
            raw::destroy_instance(
                Some(self),
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html>"]
    #[doc(alias = "vkEnumeratePhysicalDevices")]
    pub fn enumerate_physical_devices<
        R: AdvancedDynamicArray<PhysicalDevice<D, A>, raw::PhysicalDevice>,
    >(
        &self,
    ) -> Result<R> {
        let vk_result: Result<R::InnerArrayType> =
            unsafe { raw::enumerate_physical_devices(self, self.disp.get_command_dispatcher()) };
        vk_result.map(|vk_result| {
            vk_result
                .into_iter()
                .map(|el| unsafe {
                    PhysicalDevice::from_inner(el, self.disp.clone(), self.alloc.clone())
                })
                .collect()
        })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html>"]
    #[doc(alias = "vkGetInstanceProcAddr")]
    pub fn get_proc_addr(&self, p_name: &CStr) -> FuncPtr {
        unsafe {
            raw::get_instance_proc_addr(Some(self), p_name, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html>"]
    #[doc(alias = "vkEnumeratePhysicalDeviceGroups")]
    pub fn enumerate_physical_device_groups<
        R: DynamicArray<PhysicalDeviceGroupProperties<'static>>,
    >(
        &self,
    ) -> Result<R> {
        unsafe { raw::enumerate_physical_device_groups(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroupsKHR.html>"]
    #[doc(alias = "vkEnumeratePhysicalDeviceGroupsKHR")]
    pub fn enumerate_physical_device_groups_khr<
        R: DynamicArray<PhysicalDeviceGroupProperties<'static>>,
    >(
        &self,
    ) -> Result<R> {
        unsafe {
            raw::enumerate_physical_device_groups_khr(self, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html>"]
    #[doc(alias = "vkDestroySurfaceKHR")]
    pub unsafe fn destroy_surface_khr(&self, surface: Option<&raw::SurfaceKHR>) {
        unsafe {
            raw::destroy_surface_khr(
                self,
                surface,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html>"]
    #[doc(alias = "vkCreateDisplayPlaneSurfaceKHR")]
    pub fn create_display_plane_surface_khr(
        &self,
        p_create_info: &DisplaySurfaceCreateInfoKHR,
    ) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_display_plane_surface_khr(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SurfaceKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateXlibSurfaceKHR.html>"]
    #[doc(alias = "vkCreateXlibSurfaceKHR")]
    pub fn create_xlib_surface_khr(
        &self,
        p_create_info: &XlibSurfaceCreateInfoKHR,
    ) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_xlib_surface_khr(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SurfaceKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateXcbSurfaceKHR.html>"]
    #[doc(alias = "vkCreateXcbSurfaceKHR")]
    pub fn create_xcb_surface_khr(
        &self,
        p_create_info: &XcbSurfaceCreateInfoKHR,
    ) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_xcb_surface_khr(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SurfaceKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateWaylandSurfaceKHR.html>"]
    #[doc(alias = "vkCreateWaylandSurfaceKHR")]
    pub fn create_wayland_surface_khr(
        &self,
        p_create_info: &WaylandSurfaceCreateInfoKHR,
    ) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_wayland_surface_khr(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SurfaceKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAndroidSurfaceKHR.html>"]
    #[doc(alias = "vkCreateAndroidSurfaceKHR")]
    pub fn create_android_surface_khr(
        &self,
        p_create_info: &AndroidSurfaceCreateInfoKHR,
    ) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_android_surface_khr(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SurfaceKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateWin32SurfaceKHR.html>"]
    #[doc(alias = "vkCreateWin32SurfaceKHR")]
    pub fn create_win32_surface_khr(
        &self,
        p_create_info: &Win32SurfaceCreateInfoKHR,
    ) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_win32_surface_khr(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SurfaceKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugReportCallbackEXT.html>"]
    #[doc(alias = "vkCreateDebugReportCallbackEXT")]
    pub fn create_debug_report_callback_ext(
        &self,
        p_create_info: &DebugReportCallbackCreateInfoEXT,
    ) -> Result<DebugReportCallbackEXT> {
        let vk_result = unsafe {
            raw::create_debug_report_callback_ext(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { DebugReportCallbackEXT::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugReportCallbackEXT.html>"]
    #[doc(alias = "vkDestroyDebugReportCallbackEXT")]
    pub unsafe fn destroy_debug_report_callback_ext(
        &self,
        callback: Option<&raw::DebugReportCallbackEXT>,
    ) {
        unsafe {
            raw::destroy_debug_report_callback_ext(
                self,
                callback,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugReportMessageEXT.html>"]
    #[doc(alias = "vkDebugReportMessageEXT")]
    pub fn debug_report_message_ext(
        &self,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        p_layer_prefix: &CStr,
        p_message: &CStr,
    ) {
        unsafe {
            raw::debug_report_message_ext(
                self,
                flags,
                object_type,
                object,
                location,
                message_code,
                p_layer_prefix,
                p_message,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateStreamDescriptorSurfaceGGP.html>"]
    #[doc(alias = "vkCreateStreamDescriptorSurfaceGGP")]
    pub fn create_stream_descriptor_surface_ggp(
        &self,
        p_create_info: &StreamDescriptorSurfaceCreateInfoGGP,
    ) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_stream_descriptor_surface_ggp(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SurfaceKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateViSurfaceNN.html>"]
    #[doc(alias = "vkCreateViSurfaceNN")]
    pub fn create_vi_surface_nn(
        &self,
        p_create_info: &ViSurfaceCreateInfoNN,
    ) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_vi_surface_nn(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SurfaceKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateIOSSurfaceMVK.html>"]
    #[doc(alias = "vkCreateIOSSurfaceMVK")]
    pub fn create_iossurface_mvk(
        &self,
        p_create_info: &IOSSurfaceCreateInfoMVK,
    ) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_iossurface_mvk(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SurfaceKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMacOSSurfaceMVK.html>"]
    #[doc(alias = "vkCreateMacOSSurfaceMVK")]
    pub fn create_mac_ossurface_mvk(
        &self,
        p_create_info: &MacOSSurfaceCreateInfoMVK,
    ) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_mac_ossurface_mvk(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SurfaceKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html>"]
    #[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
    pub fn create_debug_utils_messenger_ext(
        &self,
        p_create_info: &DebugUtilsMessengerCreateInfoEXT,
    ) -> Result<DebugUtilsMessengerEXT> {
        let vk_result = unsafe {
            raw::create_debug_utils_messenger_ext(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { DebugUtilsMessengerEXT::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html>"]
    #[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
    pub unsafe fn destroy_debug_utils_messenger_ext(
        &self,
        messenger: Option<&raw::DebugUtilsMessengerEXT>,
    ) {
        unsafe {
            raw::destroy_debug_utils_messenger_ext(
                self,
                messenger,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html>"]
    #[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
    pub fn submit_debug_utils_message_ext(
        &self,
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: &DebugUtilsMessengerCallbackDataEXT,
    ) {
        unsafe {
            raw::submit_debug_utils_message_ext(
                self,
                message_severity,
                message_types,
                p_callback_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html>"]
    #[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
    pub fn create_image_pipe_surface_fuchsia(
        &self,
        p_create_info: &ImagePipeSurfaceCreateInfoFUCHSIA,
    ) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_image_pipe_surface_fuchsia(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SurfaceKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMetalSurfaceEXT.html>"]
    #[doc(alias = "vkCreateMetalSurfaceEXT")]
    pub fn create_metal_surface_ext(
        &self,
        p_create_info: &MetalSurfaceCreateInfoEXT,
    ) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_metal_surface_ext(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SurfaceKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateHeadlessSurfaceEXT.html>"]
    #[doc(alias = "vkCreateHeadlessSurfaceEXT")]
    pub fn create_headless_surface_ext(
        &self,
        p_create_info: &HeadlessSurfaceCreateInfoEXT,
    ) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_headless_surface_ext(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SurfaceKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDirectFBSurfaceEXT.html>"]
    #[doc(alias = "vkCreateDirectFBSurfaceEXT")]
    pub fn create_direct_fbsurface_ext(
        &self,
        p_create_info: &DirectFBSurfaceCreateInfoEXT,
    ) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_direct_fbsurface_ext(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SurfaceKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateScreenSurfaceQNX.html>"]
    #[doc(alias = "vkCreateScreenSurfaceQNX")]
    pub fn create_screen_surface_qnx(
        &self,
        p_create_info: &ScreenSurfaceCreateInfoQNX,
    ) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_screen_surface_qnx(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SurfaceKHR::from_inner(vk_result) })
    }
}
#[repr(C)]
#[derive(Clone)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice.html>"]
#[doc(alias = "VkPhysicalDevice")]
pub struct PhysicalDevice<D: Dispatcher = DynamicDispatcher, A: Allocator = DefaultAllocator> {
    inner: <raw::PhysicalDevice as Handle>::InnerType,
    disp: D,
    alloc: A,
}
unsafe impl Alias<raw::PhysicalDevice> for PhysicalDevice {}
impl<D: Dispatcher, A: Allocator> Copy for PhysicalDevice<D, A>
where
    D: Copy,
    A: Copy,
{
}
impl<D: Dispatcher, A: Allocator> Deref for PhysicalDevice<D, A> {
    type Target = raw::PhysicalDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl<D: Dispatcher, A: Allocator> PhysicalDevice<D, A> {
    pub unsafe fn from_inner(handle: raw::PhysicalDevice, disp: D, alloc: A) -> Self {
        Self {
            inner: handle.as_raw(),
            disp,
            alloc,
        }
    }
    pub fn get_dispatcher(&self) -> &D {
        &self.disp
    }
    pub fn get_allocator(&self) -> &A {
        &self.alloc
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures.html>"]
    #[doc(alias = "vkGetPhysicalDeviceFeatures")]
    pub fn get_features(&self) -> PhysicalDeviceFeatures {
        unsafe { raw::get_physical_device_features(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html>"]
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties")]
    pub fn get_format_properties(&self, format: Format) -> FormatProperties {
        unsafe {
            raw::get_physical_device_format_properties(
                self,
                format,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html>"]
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties")]
    pub fn get_image_format_properties(
        &self,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
    ) -> Result<ImageFormatProperties> {
        unsafe {
            raw::get_physical_device_image_format_properties(
                self,
                format,
                ty,
                tiling,
                usage,
                flags,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties.html>"]
    #[doc(alias = "vkGetPhysicalDeviceProperties")]
    pub fn get_properties(&self) -> PhysicalDeviceProperties {
        unsafe { raw::get_physical_device_properties(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html>"]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties")]
    pub fn get_queue_family_properties<R: DynamicArray<QueueFamilyProperties>>(&self) -> R {
        unsafe {
            raw::get_physical_device_queue_family_properties(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html>"]
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties")]
    pub fn get_memory_properties(&self) -> PhysicalDeviceMemoryProperties {
        unsafe {
            raw::get_physical_device_memory_properties(self, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html>"]
    #[doc(alias = "vkCreateDevice")]
    pub fn create_device(&self, p_create_info: &DeviceCreateInfo) -> Result<Device<D, A>> {
        let vk_result = unsafe {
            raw::create_device(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|device| {
            let disp = self.disp.clone_with_device(&device);
            unsafe { Device::from_inner(device, disp, self.alloc.clone()) }
        })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceExtensionProperties.html>"]
    #[doc(alias = "vkEnumerateDeviceExtensionProperties")]
    pub fn enumerate_device_extension_properties<R: DynamicArray<ExtensionProperties>>(
        &self,
        p_layer_name: Option<&CStr>,
    ) -> Result<R> {
        unsafe {
            raw::enumerate_device_extension_properties(
                self,
                p_layer_name,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceLayerProperties.html>"]
    #[doc(alias = "vkEnumerateDeviceLayerProperties")]
    pub fn enumerate_device_layer_properties<R: DynamicArray<LayerProperties>>(&self) -> Result<R> {
        unsafe { raw::enumerate_device_layer_properties(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html>"]
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties")]
    pub fn get_sparse_image_format_properties<R: DynamicArray<SparseImageFormatProperties>>(
        &self,
        format: Format,
        ty: ImageType,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
    ) -> R {
        unsafe {
            raw::get_physical_device_sparse_image_format_properties(
                self,
                format,
                ty,
                samples,
                usage,
                tiling,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2.html>"]
    #[doc(alias = "vkGetPhysicalDeviceFeatures2")]
    pub fn get_features2<S: StructureChainOut<PhysicalDeviceFeatures2<'static>>>(&self) -> S {
        unsafe { raw::get_physical_device_features2(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2KHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceFeatures2KHR")]
    pub fn get_features2_khr<S: StructureChainOut<PhysicalDeviceFeatures2<'static>>>(&self) -> S {
        unsafe { raw::get_physical_device_features2_khr(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2.html>"]
    #[doc(alias = "vkGetPhysicalDeviceProperties2")]
    pub fn get_properties2<S: StructureChainOut<PhysicalDeviceProperties2<'static>>>(&self) -> S {
        unsafe { raw::get_physical_device_properties2(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2KHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceProperties2KHR")]
    pub fn get_properties2_khr<S: StructureChainOut<PhysicalDeviceProperties2<'static>>>(
        &self,
    ) -> S {
        unsafe {
            raw::get_physical_device_properties2_khr(self, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html>"]
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties2")]
    pub fn get_format_properties2<S: StructureChainOut<FormatProperties2<'static>>>(
        &self,
        format: Format,
    ) -> S {
        unsafe {
            raw::get_physical_device_format_properties2(
                self,
                format,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2KHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties2KHR")]
    pub fn get_format_properties2_khr<S: StructureChainOut<FormatProperties2<'static>>>(
        &self,
        format: Format,
    ) -> S {
        unsafe {
            raw::get_physical_device_format_properties2_khr(
                self,
                format,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html>"]
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2")]
    pub fn get_image_format_properties2<S: StructureChainOut<ImageFormatProperties2<'static>>>(
        &self,
        p_image_format_info: &PhysicalDeviceImageFormatInfo2,
    ) -> Result<S> {
        unsafe {
            raw::get_physical_device_image_format_properties2(
                self,
                p_image_format_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2KHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2KHR")]
    pub fn get_image_format_properties2_khr<
        S: StructureChainOut<ImageFormatProperties2<'static>>,
    >(
        &self,
        p_image_format_info: &PhysicalDeviceImageFormatInfo2,
    ) -> Result<S> {
        unsafe {
            raw::get_physical_device_image_format_properties2_khr(
                self,
                p_image_format_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html>"]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2")]
    pub fn get_queue_family_properties2<R: DynamicArray<QueueFamilyProperties2<'static>>>(
        &self,
    ) -> R {
        unsafe {
            raw::get_physical_device_queue_family_properties2(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2KHR")]
    pub fn get_queue_family_properties2_khr<R: DynamicArray<QueueFamilyProperties2<'static>>>(
        &self,
    ) -> R {
        unsafe {
            raw::get_physical_device_queue_family_properties2_khr(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html>"]
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties2")]
    pub fn get_memory_properties2<
        S: StructureChainOut<PhysicalDeviceMemoryProperties2<'static>>,
    >(
        &self,
    ) -> S {
        unsafe {
            raw::get_physical_device_memory_properties2(self, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2KHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties2KHR")]
    pub fn get_memory_properties2_khr<
        S: StructureChainOut<PhysicalDeviceMemoryProperties2<'static>>,
    >(
        &self,
    ) -> S {
        unsafe {
            raw::get_physical_device_memory_properties2_khr(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html>"]
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2")]
    pub fn get_sparse_image_format_properties2<
        R: DynamicArray<SparseImageFormatProperties2<'static>>,
    >(
        &self,
        p_format_info: &PhysicalDeviceSparseImageFormatInfo2,
    ) -> R {
        unsafe {
            raw::get_physical_device_sparse_image_format_properties2(
                self,
                p_format_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2KHR")]
    pub fn get_sparse_image_format_properties2_khr<
        R: DynamicArray<SparseImageFormatProperties2<'static>>,
    >(
        &self,
        p_format_info: &PhysicalDeviceSparseImageFormatInfo2,
    ) -> R {
        unsafe {
            raw::get_physical_device_sparse_image_format_properties2_khr(
                self,
                p_format_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html>"]
    #[doc(alias = "vkGetPhysicalDeviceExternalBufferProperties")]
    pub fn get_external_buffer_properties<
        S: StructureChainOut<ExternalBufferProperties<'static>>,
    >(
        &self,
        p_external_buffer_info: &PhysicalDeviceExternalBufferInfo,
    ) -> S {
        unsafe {
            raw::get_physical_device_external_buffer_properties(
                self,
                p_external_buffer_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceExternalBufferPropertiesKHR")]
    pub fn get_external_buffer_properties_khr<
        S: StructureChainOut<ExternalBufferProperties<'static>>,
    >(
        &self,
        p_external_buffer_info: &PhysicalDeviceExternalBufferInfo,
    ) -> S {
        unsafe {
            raw::get_physical_device_external_buffer_properties_khr(
                self,
                p_external_buffer_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html>"]
    #[doc(alias = "vkGetPhysicalDeviceExternalFenceProperties")]
    pub fn get_external_fence_properties<S: StructureChainOut<ExternalFenceProperties<'static>>>(
        &self,
        p_external_fence_info: &PhysicalDeviceExternalFenceInfo,
    ) -> S {
        unsafe {
            raw::get_physical_device_external_fence_properties(
                self,
                p_external_fence_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFencePropertiesKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceExternalFencePropertiesKHR")]
    pub fn get_external_fence_properties_khr<
        S: StructureChainOut<ExternalFenceProperties<'static>>,
    >(
        &self,
        p_external_fence_info: &PhysicalDeviceExternalFenceInfo,
    ) -> S {
        unsafe {
            raw::get_physical_device_external_fence_properties_khr(
                self,
                p_external_fence_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html>"]
    #[doc(alias = "vkGetPhysicalDeviceExternalSemaphoreProperties")]
    pub fn get_external_semaphore_properties<
        S: StructureChainOut<ExternalSemaphoreProperties<'static>>,
    >(
        &self,
        p_external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo,
    ) -> S {
        unsafe {
            raw::get_physical_device_external_semaphore_properties(
                self,
                p_external_semaphore_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")]
    pub fn get_external_semaphore_properties_khr<
        S: StructureChainOut<ExternalSemaphoreProperties<'static>>,
    >(
        &self,
        p_external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo,
    ) -> S {
        unsafe {
            raw::get_physical_device_external_semaphore_properties_khr(
                self,
                p_external_semaphore_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolProperties.html>"]
    #[doc(alias = "vkGetPhysicalDeviceToolProperties")]
    pub fn get_tool_properties<R: DynamicArray<PhysicalDeviceToolProperties<'static>>>(
        &self,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_tool_properties(self, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html>"]
    #[doc(alias = "vkGetPhysicalDeviceToolPropertiesEXT")]
    pub fn get_tool_properties_ext<R: DynamicArray<PhysicalDeviceToolProperties<'static>>>(
        &self,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_tool_properties_ext(self, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
    pub fn get_surface_support_khr(
        &self,
        queue_family_index: u32,
        surface: &raw::SurfaceKHR,
    ) -> Result<bool> {
        unsafe {
            raw::get_physical_device_surface_support_khr(
                self,
                queue_family_index,
                surface,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
    pub fn get_surface_capabilities_khr(
        &self,
        surface: &raw::SurfaceKHR,
    ) -> Result<SurfaceCapabilitiesKHR> {
        unsafe {
            raw::get_physical_device_surface_capabilities_khr(
                self,
                surface,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
    pub fn get_surface_formats_khr<R: DynamicArray<SurfaceFormatKHR>>(
        &self,
        surface: Option<&raw::SurfaceKHR>,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_surface_formats_khr(
                self,
                surface,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
    pub fn get_surface_present_modes_khr<R: DynamicArray<PresentModeKHR>>(
        &self,
        surface: Option<&raw::SurfaceKHR>,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_surface_present_modes_khr(
                self,
                surface,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html>"]
    #[doc(alias = "vkGetPhysicalDevicePresentRectanglesKHR")]
    pub fn get_present_rectangles_khr<R: DynamicArray<Rect2D>>(
        &self,
        surface: &raw::SurfaceKHR,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_present_rectangles_khr(
                self,
                surface,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceDisplayPropertiesKHR")]
    pub fn get_display_properties_khr<R: DynamicArray<DisplayPropertiesKHR<'static>>>(
        &self,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_display_properties_khr(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")]
    pub fn get_display_plane_properties_khr<R: DynamicArray<DisplayPlanePropertiesKHR<'static>>>(
        &self,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_display_plane_properties_khr(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html>"]
    #[doc(alias = "vkGetDisplayPlaneSupportedDisplaysKHR")]
    pub fn get_display_plane_supported_displays_khr<
        R: AdvancedDynamicArray<DisplayKHR, raw::DisplayKHR>,
    >(
        &self,
        plane_index: u32,
    ) -> Result<R> {
        let vk_result: Result<R::InnerArrayType> = unsafe {
            raw::get_display_plane_supported_displays_khr(
                self,
                plane_index,
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| {
            vk_result
                .into_iter()
                .map(|el| unsafe { DisplayKHR::from_inner(el) })
                .collect()
        })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModePropertiesKHR.html>"]
    #[doc(alias = "vkGetDisplayModePropertiesKHR")]
    pub fn get_display_mode_properties_khr<R: DynamicArray<DisplayModePropertiesKHR<'static>>>(
        &self,
        display: &raw::DisplayKHR,
    ) -> Result<R> {
        unsafe {
            raw::get_display_mode_properties_khr(self, display, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayModeKHR.html>"]
    #[doc(alias = "vkCreateDisplayModeKHR")]
    pub fn create_display_mode_khr(
        &self,
        display: &raw::DisplayKHR,
        p_create_info: &DisplayModeCreateInfoKHR,
    ) -> Result<DisplayModeKHR> {
        let vk_result = unsafe {
            raw::create_display_mode_khr(
                self,
                display,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { DisplayModeKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html>"]
    #[doc(alias = "vkGetDisplayPlaneCapabilitiesKHR")]
    pub fn get_display_plane_capabilities_khr(
        &self,
        mode: &raw::DisplayModeKHR,
        plane_index: u32,
    ) -> Result<DisplayPlaneCapabilitiesKHR> {
        unsafe {
            raw::get_display_plane_capabilities_khr(
                self,
                mode,
                plane_index,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
    pub fn get_xlib_presentation_support_khr(
        &self,
        queue_family_index: u32,
        dpy: &VoidPtr,
        visual_id: VoidPtr,
    ) -> bool {
        unsafe {
            raw::get_physical_device_xlib_presentation_support_khr(
                self,
                queue_family_index,
                dpy,
                visual_id,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceXcbPresentationSupportKHR")]
    pub fn get_xcb_presentation_support_khr(
        &self,
        queue_family_index: u32,
        connection: &VoidPtr,
        visualid: VoidPtr,
    ) -> bool {
        unsafe {
            raw::get_physical_device_xcb_presentation_support_khr(
                self,
                queue_family_index,
                connection,
                visualid,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
    pub fn get_wayland_presentation_support_khr(
        &self,
        queue_family_index: u32,
        display: &VoidPtr,
    ) -> bool {
        unsafe {
            raw::get_physical_device_wayland_presentation_support_khr(
                self,
                queue_family_index,
                display,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceWin32PresentationSupportKHR")]
    pub fn get_win32_presentation_support_khr(&self, queue_family_index: u32) -> bool {
        unsafe {
            raw::get_physical_device_win32_presentation_support_khr(
                self,
                queue_family_index,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html>"]
    #[doc(alias = "vkGetPhysicalDeviceExternalImageFormatPropertiesNV")]
    pub fn get_external_image_format_properties_nv(
        &self,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        external_handle_type: ExternalMemoryHandleTypeFlagsNV,
    ) -> Result<ExternalImageFormatPropertiesNV> {
        unsafe {
            raw::get_physical_device_external_image_format_properties_nv(
                self,
                format,
                ty,
                tiling,
                usage,
                flags,
                external_handle_type,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseDisplayEXT.html>"]
    #[doc(alias = "vkReleaseDisplayEXT")]
    pub fn release_display_ext(&self, display: &raw::DisplayKHR) -> Result<()> {
        unsafe { raw::release_display_ext(self, display, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireXlibDisplayEXT.html>"]
    #[doc(alias = "vkAcquireXlibDisplayEXT")]
    pub fn acquire_xlib_display_ext(&self, dpy: &VoidPtr, display: &raw::DisplayKHR) -> Result<()> {
        unsafe {
            raw::acquire_xlib_display_ext(self, dpy, display, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRandROutputDisplayEXT.html>"]
    #[doc(alias = "vkGetRandROutputDisplayEXT")]
    pub fn get_rand_routput_display_ext(
        &self,
        dpy: &VoidPtr,
        rr_output: VoidPtr,
    ) -> Result<DisplayKHR> {
        let vk_result = unsafe {
            raw::get_rand_routput_display_ext(
                self,
                dpy,
                rr_output,
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { DisplayKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html>"]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2EXT")]
    pub fn get_surface_capabilities2_ext<S: StructureChainOut<SurfaceCapabilities2EXT<'static>>>(
        &self,
        surface: &raw::SurfaceKHR,
    ) -> Result<S> {
        unsafe {
            raw::get_physical_device_surface_capabilities2_ext(
                self,
                surface,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")]
    pub fn get_queue_family_performance_query_passes_khr(
        &self,
        p_performance_query_create_info: &QueryPoolPerformanceCreateInfoKHR,
    ) -> u32 {
        unsafe {
            raw::get_physical_device_queue_family_performance_query_passes_khr(
                self,
                p_performance_query_create_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2KHR")]
    pub fn get_surface_capabilities2_khr<S: StructureChainOut<SurfaceCapabilities2KHR<'static>>>(
        &self,
        p_surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<S> {
        unsafe {
            raw::get_physical_device_surface_capabilities2_khr(
                self,
                p_surface_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceFormats2KHR")]
    pub fn get_surface_formats2_khr<R: DynamicArray<SurfaceFormat2KHR<'static>>>(
        &self,
        p_surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_surface_formats2_khr(
                self,
                p_surface_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
    pub fn get_display_properties2_khr<R: DynamicArray<DisplayProperties2KHR<'static>>>(
        &self,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_display_properties2_khr(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
    pub fn get_display_plane_properties2_khr<
        R: DynamicArray<DisplayPlaneProperties2KHR<'static>>,
    >(
        &self,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_display_plane_properties2_khr(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModeProperties2KHR.html>"]
    #[doc(alias = "vkGetDisplayModeProperties2KHR")]
    pub fn get_display_mode_properties2_khr<R: DynamicArray<DisplayModeProperties2KHR<'static>>>(
        &self,
        display: &raw::DisplayKHR,
    ) -> Result<R> {
        unsafe {
            raw::get_display_mode_properties2_khr(self, display, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html>"]
    #[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
    pub fn get_display_plane_capabilities2_khr<
        S: StructureChainOut<DisplayPlaneCapabilities2KHR<'static>>,
    >(
        &self,
        p_display_plane_info: &DisplayPlaneInfo2KHR,
    ) -> Result<S> {
        unsafe {
            raw::get_display_plane_capabilities2_khr(
                self,
                p_display_plane_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html>"]
    #[doc(alias = "vkGetPhysicalDeviceMultisamplePropertiesEXT")]
    pub fn get_multisample_properties_ext<
        S: StructureChainOut<MultisamplePropertiesEXT<'static>>,
    >(
        &self,
        samples: SampleCountFlags,
    ) -> S {
        unsafe {
            raw::get_physical_device_multisample_properties_ext(
                self,
                samples,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFragmentShadingRatesKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceFragmentShadingRatesKHR")]
    pub fn get_fragment_shading_rates_khr<
        R: DynamicArray<PhysicalDeviceFragmentShadingRateKHR<'static>>,
    >(
        &self,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_fragment_shading_rates_khr(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html>"]
    #[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV")]
    pub fn get_cooperative_matrix_properties_nv<
        R: DynamicArray<CooperativeMatrixPropertiesNV<'static>>,
    >(
        &self,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_cooperative_matrix_properties_nv(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html>"]
    #[doc(alias = "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")]
    pub fn get_supported_framebuffer_mixed_samples_combinations_nv<
        R: DynamicArray<FramebufferMixedSamplesCombinationNV<'static>>,
    >(
        &self,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html>"]
    #[doc(alias = "vkGetPhysicalDeviceSurfacePresentModes2EXT")]
    pub fn get_surface_present_modes2_ext<R: DynamicArray<PresentModeKHR>>(
        &self,
        p_surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_surface_present_modes2_ext(
                self,
                p_surface_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireDrmDisplayEXT.html>"]
    #[doc(alias = "vkAcquireDrmDisplayEXT")]
    pub fn acquire_drm_display_ext(&self, drm_fd: i32, display: &raw::DisplayKHR) -> Result<()> {
        unsafe {
            raw::acquire_drm_display_ext(self, drm_fd, display, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDrmDisplayEXT.html>"]
    #[doc(alias = "vkGetDrmDisplayEXT")]
    pub fn get_drm_display_ext(&self, drm_fd: i32, connector_id: u32) -> Result<DisplayKHR> {
        let vk_result = unsafe {
            raw::get_drm_display_ext(
                self,
                drm_fd,
                connector_id,
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { DisplayKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireWinrtDisplayNV.html>"]
    #[doc(alias = "vkAcquireWinrtDisplayNV")]
    pub fn acquire_winrt_display_nv(&self, display: &raw::DisplayKHR) -> Result<()> {
        unsafe { raw::acquire_winrt_display_nv(self, display, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetWinrtDisplayNV.html>"]
    #[doc(alias = "vkGetWinrtDisplayNV")]
    pub fn get_winrt_display_nv(&self, device_relative_id: u32) -> Result<DisplayKHR> {
        let vk_result = unsafe {
            raw::get_winrt_display_nv(self, device_relative_id, self.disp.get_command_dispatcher())
        };
        vk_result.map(|vk_result| unsafe { DisplayKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html>"]
    #[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
    pub fn get_direct_fbpresentation_support_ext(
        &self,
        queue_family_index: u32,
        dfb: &VoidPtr,
    ) -> bool {
        unsafe {
            raw::get_physical_device_direct_fbpresentation_support_ext(
                self,
                queue_family_index,
                dfb,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceScreenPresentationSupportQNX.html>"]
    #[doc(alias = "vkGetPhysicalDeviceScreenPresentationSupportQNX")]
    pub fn get_screen_presentation_support_qnx(
        &self,
        queue_family_index: u32,
        window: &VoidPtr,
    ) -> bool {
        unsafe {
            raw::get_physical_device_screen_presentation_support_qnx(
                self,
                queue_family_index,
                window,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html>"]
    #[doc(alias = "vkGetPhysicalDeviceOpticalFlowImageFormatsNV")]
    pub fn get_optical_flow_image_formats_nv<
        R: DynamicArray<OpticalFlowImageFormatPropertiesNV<'static>>,
    >(
        &self,
        p_optical_flow_image_format_info: &OpticalFlowImageFormatInfoNV,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_optical_flow_image_formats_nv(
                self,
                p_optical_flow_image_format_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR")]
    pub fn get_cooperative_matrix_properties_khr<
        R: DynamicArray<CooperativeMatrixPropertiesKHR<'static>>,
    >(
        &self,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_cooperative_matrix_properties_khr(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsKHR")]
    pub fn get_calibrateable_time_domains_khr<R: DynamicArray<TimeDomainKHR>>(&self) -> Result<R> {
        unsafe {
            raw::get_physical_device_calibrateable_time_domains_khr(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html>"]
    #[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")]
    pub fn get_calibrateable_time_domains_ext<R: DynamicArray<TimeDomainKHR>>(&self) -> Result<R> {
        unsafe {
            raw::get_physical_device_calibrateable_time_domains_ext(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
}
#[repr(C)]
#[derive(Clone)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDevice.html>"]
#[doc(alias = "VkDevice")]
pub struct Device<D: Dispatcher = DynamicDispatcher, A: Allocator = DefaultAllocator> {
    inner: <raw::Device as Handle>::InnerType,
    disp: D,
    alloc: A,
}
unsafe impl Alias<raw::Device> for Device {}
impl<D: Dispatcher, A: Allocator> Copy for Device<D, A>
where
    D: Copy,
    A: Copy,
{
}
impl<D: Dispatcher, A: Allocator> Deref for Device<D, A> {
    type Target = raw::Device;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl<D: Dispatcher, A: Allocator> Device<D, A> {
    pub unsafe fn from_inner(handle: raw::Device, disp: D, alloc: A) -> Self {
        Self {
            inner: handle.as_raw(),
            disp,
            alloc,
        }
    }
    pub fn get_dispatcher(&self) -> &D {
        &self.disp
    }
    pub fn get_allocator(&self) -> &A {
        &self.alloc
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html>"]
    #[doc(alias = "vkGetDeviceProcAddr")]
    pub fn get_proc_addr(&self, p_name: &CStr) -> FuncPtr {
        unsafe { raw::get_device_proc_addr(self, p_name, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDevice.html>"]
    #[doc(alias = "vkDestroyDevice")]
    pub unsafe fn destroy(&self) {
        unsafe {
            raw::destroy_device(
                Some(self),
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue.html>"]
    #[doc(alias = "vkGetDeviceQueue")]
    pub fn get_queue(&self, queue_family_index: u32, queue_index: u32) -> Queue<D, A> {
        let vk_result = unsafe {
            raw::get_device_queue(
                self,
                queue_family_index,
                queue_index,
                self.disp.get_command_dispatcher(),
            )
        };
        unsafe { Queue::from_inner(vk_result, self.disp.clone(), self.alloc.clone()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeviceWaitIdle.html>"]
    #[doc(alias = "vkDeviceWaitIdle")]
    pub fn wait_idle(&self) -> Result<()> {
        unsafe { raw::device_wait_idle(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateMemory.html>"]
    #[doc(alias = "vkAllocateMemory")]
    pub fn allocate_memory(&self, p_allocate_info: &MemoryAllocateInfo) -> Result<DeviceMemory> {
        let vk_result = unsafe {
            raw::allocate_memory(
                self,
                p_allocate_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { DeviceMemory::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeMemory.html>"]
    #[doc(alias = "vkFreeMemory")]
    pub fn free_memory(&self, memory: Option<&raw::DeviceMemory>) {
        unsafe {
            raw::free_memory(
                self,
                memory,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMapMemory.html>"]
    #[doc(alias = "vkMapMemory")]
    pub fn map_memory(
        &self,
        memory: &raw::DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
    ) -> Result<VoidPtr> {
        unsafe {
            raw::map_memory(
                self,
                memory,
                offset,
                size,
                flags,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory.html>"]
    #[doc(alias = "vkUnmapMemory")]
    pub fn unmap_memory(&self, memory: &raw::DeviceMemory) {
        unsafe { raw::unmap_memory(self, memory, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFlushMappedMemoryRanges.html>"]
    #[doc(alias = "vkFlushMappedMemoryRanges")]
    pub fn flush_mapped_memory_ranges<'a>(
        &self,
        p_memory_ranges: impl AsSlice<'a, MappedMemoryRange<'a>>,
    ) -> Result<()> {
        unsafe {
            raw::flush_mapped_memory_ranges(
                self,
                p_memory_ranges,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkInvalidateMappedMemoryRanges.html>"]
    #[doc(alias = "vkInvalidateMappedMemoryRanges")]
    pub fn invalidate_mapped_memory_ranges<'a>(
        &self,
        p_memory_ranges: impl AsSlice<'a, MappedMemoryRange<'a>>,
    ) -> Result<()> {
        unsafe {
            raw::invalidate_mapped_memory_ranges(
                self,
                p_memory_ranges,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryCommitment.html>"]
    #[doc(alias = "vkGetDeviceMemoryCommitment")]
    pub fn get_memory_commitment(&self, memory: &raw::DeviceMemory) -> DeviceSize {
        unsafe {
            raw::get_device_memory_commitment(self, memory, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory.html>"]
    #[doc(alias = "vkBindBufferMemory")]
    pub fn bind_buffer_memory(
        &self,
        buffer: &raw::Buffer,
        memory: &raw::DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result<()> {
        unsafe {
            raw::bind_buffer_memory(
                self,
                buffer,
                memory,
                memory_offset,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory.html>"]
    #[doc(alias = "vkBindImageMemory")]
    pub fn bind_image_memory(
        &self,
        image: &raw::Image,
        memory: &raw::DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result<()> {
        unsafe {
            raw::bind_image_memory(
                self,
                image,
                memory,
                memory_offset,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements.html>"]
    #[doc(alias = "vkGetBufferMemoryRequirements")]
    pub fn get_buffer_memory_requirements(&self, buffer: &raw::Buffer) -> MemoryRequirements {
        unsafe {
            raw::get_buffer_memory_requirements(self, buffer, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements.html>"]
    #[doc(alias = "vkGetImageMemoryRequirements")]
    pub fn get_image_memory_requirements(&self, image: &raw::Image) -> MemoryRequirements {
        unsafe {
            raw::get_image_memory_requirements(self, image, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements.html>"]
    #[doc(alias = "vkGetImageSparseMemoryRequirements")]
    pub fn get_image_sparse_memory_requirements<R: DynamicArray<SparseImageMemoryRequirements>>(
        &self,
        image: &raw::Image,
    ) -> R {
        unsafe {
            raw::get_image_sparse_memory_requirements(
                self,
                image,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateFence.html>"]
    #[doc(alias = "vkCreateFence")]
    pub fn create_fence(&self, p_create_info: &FenceCreateInfo) -> Result<Fence> {
        let vk_result = unsafe {
            raw::create_fence(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { Fence::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyFence.html>"]
    #[doc(alias = "vkDestroyFence")]
    pub unsafe fn destroy_fence(&self, fence: Option<&raw::Fence>) {
        unsafe {
            raw::destroy_fence(
                self,
                fence,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetFences.html>"]
    #[doc(alias = "vkResetFences")]
    pub fn reset_fences<'a, V2: Alias<raw::Fence> + 'a>(
        &self,
        p_fences: impl AsSlice<'a, V2>,
    ) -> Result<()> {
        unsafe { raw::reset_fences(self, p_fences, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceStatus.html>"]
    #[doc(alias = "vkGetFenceStatus")]
    pub fn get_fence_status(&self, fence: &raw::Fence) -> Result<Status> {
        unsafe { raw::get_fence_status(self, fence, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitForFences.html>"]
    #[doc(alias = "vkWaitForFences")]
    pub fn wait_for_fences<'a, V2: Alias<raw::Fence> + 'a>(
        &self,
        p_fences: impl AsSlice<'a, V2>,
        wait_all: impl Into<Bool32>,
        timeout: u64,
    ) -> Result<Status> {
        unsafe {
            raw::wait_for_fences(
                self,
                p_fences,
                wait_all,
                timeout,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSemaphore.html>"]
    #[doc(alias = "vkCreateSemaphore")]
    pub fn create_semaphore(&self, p_create_info: &SemaphoreCreateInfo) -> Result<Semaphore> {
        let vk_result = unsafe {
            raw::create_semaphore(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { Semaphore::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphore.html>"]
    #[doc(alias = "vkDestroySemaphore")]
    pub unsafe fn destroy_semaphore(&self, semaphore: Option<&raw::Semaphore>) {
        unsafe {
            raw::destroy_semaphore(
                self,
                semaphore,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateEvent.html>"]
    #[doc(alias = "vkCreateEvent")]
    pub fn create_event(&self, p_create_info: &EventCreateInfo) -> Result<Event> {
        let vk_result = unsafe {
            raw::create_event(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { Event::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyEvent.html>"]
    #[doc(alias = "vkDestroyEvent")]
    pub unsafe fn destroy_event(&self, event: Option<&raw::Event>) {
        unsafe {
            raw::destroy_event(
                self,
                event,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetEventStatus.html>"]
    #[doc(alias = "vkGetEventStatus")]
    pub fn get_event_status(&self, event: &raw::Event) -> Result<Status> {
        unsafe { raw::get_event_status(self, event, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetEvent.html>"]
    #[doc(alias = "vkSetEvent")]
    pub fn set_event(&self, event: &raw::Event) -> Result<()> {
        unsafe { raw::set_event(self, event, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetEvent.html>"]
    #[doc(alias = "vkResetEvent")]
    pub fn reset_event(&self, event: &raw::Event) -> Result<()> {
        unsafe { raw::reset_event(self, event, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateQueryPool.html>"]
    #[doc(alias = "vkCreateQueryPool")]
    pub fn create_query_pool(&self, p_create_info: &QueryPoolCreateInfo) -> Result<QueryPool> {
        let vk_result = unsafe {
            raw::create_query_pool(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { QueryPool::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyQueryPool.html>"]
    #[doc(alias = "vkDestroyQueryPool")]
    pub unsafe fn destroy_query_pool(&self, query_pool: Option<&raw::QueryPool>) {
        unsafe {
            raw::destroy_query_pool(
                self,
                query_pool,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueryPoolResults.html>"]
    #[doc(alias = "vkGetQueryPoolResults")]
    pub fn get_query_pool_results(
        &self,
        query_pool: &raw::QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        p_data: VoidPtr,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> Result<Status> {
        unsafe {
            raw::get_query_pool_results(
                self,
                query_pool,
                first_query,
                query_count,
                data_size,
                p_data,
                stride,
                flags,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBuffer.html>"]
    #[doc(alias = "vkCreateBuffer")]
    pub fn create_buffer(&self, p_create_info: &BufferCreateInfo) -> Result<Buffer> {
        let vk_result = unsafe {
            raw::create_buffer(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { Buffer::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBuffer.html>"]
    #[doc(alias = "vkDestroyBuffer")]
    pub unsafe fn destroy_buffer(&self, buffer: Option<&raw::Buffer>) {
        unsafe {
            raw::destroy_buffer(
                self,
                buffer,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBufferView.html>"]
    #[doc(alias = "vkCreateBufferView")]
    pub fn create_buffer_view(&self, p_create_info: &BufferViewCreateInfo) -> Result<BufferView> {
        let vk_result = unsafe {
            raw::create_buffer_view(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { BufferView::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferView.html>"]
    #[doc(alias = "vkDestroyBufferView")]
    pub unsafe fn destroy_buffer_view(&self, buffer_view: Option<&raw::BufferView>) {
        unsafe {
            raw::destroy_buffer_view(
                self,
                buffer_view,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImage.html>"]
    #[doc(alias = "vkCreateImage")]
    pub fn create_image(&self, p_create_info: &ImageCreateInfo) -> Result<Image> {
        let vk_result = unsafe {
            raw::create_image(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { Image::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImage.html>"]
    #[doc(alias = "vkDestroyImage")]
    pub unsafe fn destroy_image(&self, image: Option<&raw::Image>) {
        unsafe {
            raw::destroy_image(
                self,
                image,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout.html>"]
    #[doc(alias = "vkGetImageSubresourceLayout")]
    pub fn get_image_subresource_layout(
        &self,
        image: &raw::Image,
        p_subresource: &ImageSubresource,
    ) -> SubresourceLayout {
        unsafe {
            raw::get_image_subresource_layout(
                self,
                image,
                p_subresource,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImageView.html>"]
    #[doc(alias = "vkCreateImageView")]
    pub fn create_image_view(&self, p_create_info: &ImageViewCreateInfo) -> Result<ImageView> {
        let vk_result = unsafe {
            raw::create_image_view(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { ImageView::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImageView.html>"]
    #[doc(alias = "vkDestroyImageView")]
    pub unsafe fn destroy_image_view(&self, image_view: Option<&raw::ImageView>) {
        unsafe {
            raw::destroy_image_view(
                self,
                image_view,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateShaderModule.html>"]
    #[doc(alias = "vkCreateShaderModule")]
    pub fn create_shader_module(
        &self,
        p_create_info: &ShaderModuleCreateInfo,
    ) -> Result<ShaderModule> {
        let vk_result = unsafe {
            raw::create_shader_module(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { ShaderModule::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderModule.html>"]
    #[doc(alias = "vkDestroyShaderModule")]
    pub unsafe fn destroy_shader_module(&self, shader_module: Option<&raw::ShaderModule>) {
        unsafe {
            raw::destroy_shader_module(
                self,
                shader_module,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineCache.html>"]
    #[doc(alias = "vkCreatePipelineCache")]
    pub fn create_pipeline_cache(
        &self,
        p_create_info: &PipelineCacheCreateInfo,
    ) -> Result<PipelineCache> {
        let vk_result = unsafe {
            raw::create_pipeline_cache(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { PipelineCache::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineCache.html>"]
    #[doc(alias = "vkDestroyPipelineCache")]
    pub unsafe fn destroy_pipeline_cache(&self, pipeline_cache: Option<&raw::PipelineCache>) {
        unsafe {
            raw::destroy_pipeline_cache(
                self,
                pipeline_cache,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineCacheData.html>"]
    #[doc(alias = "vkGetPipelineCacheData")]
    pub fn get_pipeline_cache_data(
        &self,
        pipeline_cache: &raw::PipelineCache,
        p_data: VoidPtr,
    ) -> Result<usize> {
        unsafe {
            raw::get_pipeline_cache_data(
                self,
                pipeline_cache,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMergePipelineCaches.html>"]
    #[doc(alias = "vkMergePipelineCaches")]
    pub fn merge_pipeline_caches<'a, V3: Alias<raw::PipelineCache> + 'a>(
        &self,
        dst_cache: &raw::PipelineCache,
        p_src_caches: impl AsSlice<'a, V3>,
    ) -> Result<()> {
        unsafe {
            raw::merge_pipeline_caches(
                self,
                dst_cache,
                p_src_caches,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateGraphicsPipelines.html>"]
    #[doc(alias = "vkCreateGraphicsPipelines")]
    pub fn create_graphics_pipelines<'a, R: AdvancedDynamicArray<Pipeline, raw::Pipeline>>(
        &self,
        pipeline_cache: Option<&raw::PipelineCache>,
        p_create_infos: impl AsSlice<'a, GraphicsPipelineCreateInfo<'a>>,
    ) -> Result<(Status, R)> {
        let vk_result: Result<(Status, R::InnerArrayType)> = unsafe {
            raw::create_graphics_pipelines(
                self,
                pipeline_cache,
                p_create_infos,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|(status, vk_result)| {
            (
                status,
                vk_result
                    .into_iter()
                    .map(|el| unsafe { Pipeline::from_inner(el) })
                    .collect(),
            )
        })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateComputePipelines.html>"]
    #[doc(alias = "vkCreateComputePipelines")]
    pub fn create_compute_pipelines<'a, R: AdvancedDynamicArray<Pipeline, raw::Pipeline>>(
        &self,
        pipeline_cache: Option<&raw::PipelineCache>,
        p_create_infos: impl AsSlice<'a, ComputePipelineCreateInfo<'a>>,
    ) -> Result<(Status, R)> {
        let vk_result: Result<(Status, R::InnerArrayType)> = unsafe {
            raw::create_compute_pipelines(
                self,
                pipeline_cache,
                p_create_infos,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|(status, vk_result)| {
            (
                status,
                vk_result
                    .into_iter()
                    .map(|el| unsafe { Pipeline::from_inner(el) })
                    .collect(),
            )
        })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipeline.html>"]
    #[doc(alias = "vkDestroyPipeline")]
    pub unsafe fn destroy_pipeline(&self, pipeline: Option<&raw::Pipeline>) {
        unsafe {
            raw::destroy_pipeline(
                self,
                pipeline,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineLayout.html>"]
    #[doc(alias = "vkCreatePipelineLayout")]
    pub fn create_pipeline_layout(
        &self,
        p_create_info: &PipelineLayoutCreateInfo,
    ) -> Result<PipelineLayout> {
        let vk_result = unsafe {
            raw::create_pipeline_layout(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { PipelineLayout::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineLayout.html>"]
    #[doc(alias = "vkDestroyPipelineLayout")]
    pub unsafe fn destroy_pipeline_layout(&self, pipeline_layout: Option<&raw::PipelineLayout>) {
        unsafe {
            raw::destroy_pipeline_layout(
                self,
                pipeline_layout,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSampler.html>"]
    #[doc(alias = "vkCreateSampler")]
    pub fn create_sampler(&self, p_create_info: &SamplerCreateInfo) -> Result<Sampler> {
        let vk_result = unsafe {
            raw::create_sampler(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { Sampler::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySampler.html>"]
    #[doc(alias = "vkDestroySampler")]
    pub unsafe fn destroy_sampler(&self, sampler: Option<&raw::Sampler>) {
        unsafe {
            raw::destroy_sampler(
                self,
                sampler,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorSetLayout.html>"]
    #[doc(alias = "vkCreateDescriptorSetLayout")]
    pub fn create_descriptor_set_layout(
        &self,
        p_create_info: &DescriptorSetLayoutCreateInfo,
    ) -> Result<DescriptorSetLayout> {
        let vk_result = unsafe {
            raw::create_descriptor_set_layout(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { DescriptorSetLayout::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorSetLayout.html>"]
    #[doc(alias = "vkDestroyDescriptorSetLayout")]
    pub unsafe fn destroy_descriptor_set_layout(
        &self,
        descriptor_set_layout: Option<&raw::DescriptorSetLayout>,
    ) {
        unsafe {
            raw::destroy_descriptor_set_layout(
                self,
                descriptor_set_layout,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorPool.html>"]
    #[doc(alias = "vkCreateDescriptorPool")]
    pub fn create_descriptor_pool(
        &self,
        p_create_info: &DescriptorPoolCreateInfo,
    ) -> Result<DescriptorPool> {
        let vk_result = unsafe {
            raw::create_descriptor_pool(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { DescriptorPool::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorPool.html>"]
    #[doc(alias = "vkDestroyDescriptorPool")]
    pub unsafe fn destroy_descriptor_pool(&self, descriptor_pool: Option<&raw::DescriptorPool>) {
        unsafe {
            raw::destroy_descriptor_pool(
                self,
                descriptor_pool,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetDescriptorPool.html>"]
    #[doc(alias = "vkResetDescriptorPool")]
    pub fn reset_descriptor_pool(
        &self,
        descriptor_pool: &raw::DescriptorPool,
        flags: u32,
    ) -> Result<()> {
        unsafe {
            raw::reset_descriptor_pool(
                self,
                descriptor_pool,
                flags,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateDescriptorSets.html>"]
    #[doc(alias = "vkAllocateDescriptorSets")]
    pub fn allocate_descriptor_sets<R: AdvancedDynamicArray<DescriptorSet, raw::DescriptorSet>>(
        &self,
        p_allocate_info: &DescriptorSetAllocateInfo,
    ) -> Result<R> {
        let vk_result: Result<R::InnerArrayType> = unsafe {
            raw::allocate_descriptor_sets(self, p_allocate_info, self.disp.get_command_dispatcher())
        };
        vk_result.map(|vk_result| {
            vk_result
                .into_iter()
                .map(|el| unsafe { DescriptorSet::from_inner(el) })
                .collect()
        })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeDescriptorSets.html>"]
    #[doc(alias = "vkFreeDescriptorSets")]
    pub fn free_descriptor_sets<'a, V3: Alias<raw::DescriptorSet> + 'a>(
        &self,
        descriptor_pool: &raw::DescriptorPool,
        p_descriptor_sets: impl AsSlice<'a, V3>,
    ) -> Result<()> {
        unsafe {
            raw::free_descriptor_sets(
                self,
                descriptor_pool,
                p_descriptor_sets,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSets.html>"]
    #[doc(alias = "vkUpdateDescriptorSets")]
    pub fn update_descriptor_sets<'a>(
        &self,
        p_descriptor_writes: impl AsSlice<'a, WriteDescriptorSet<'a>>,
        p_descriptor_copies: impl AsSlice<'a, CopyDescriptorSet<'a>>,
    ) {
        unsafe {
            raw::update_descriptor_sets(
                self,
                p_descriptor_writes,
                p_descriptor_copies,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateFramebuffer.html>"]
    #[doc(alias = "vkCreateFramebuffer")]
    pub fn create_framebuffer(&self, p_create_info: &FramebufferCreateInfo) -> Result<Framebuffer> {
        let vk_result = unsafe {
            raw::create_framebuffer(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { Framebuffer::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyFramebuffer.html>"]
    #[doc(alias = "vkDestroyFramebuffer")]
    pub unsafe fn destroy_framebuffer(&self, framebuffer: Option<&raw::Framebuffer>) {
        unsafe {
            raw::destroy_framebuffer(
                self,
                framebuffer,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass.html>"]
    #[doc(alias = "vkCreateRenderPass")]
    pub fn create_render_pass(&self, p_create_info: &RenderPassCreateInfo) -> Result<RenderPass> {
        let vk_result = unsafe {
            raw::create_render_pass(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { RenderPass::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyRenderPass.html>"]
    #[doc(alias = "vkDestroyRenderPass")]
    pub unsafe fn destroy_render_pass(&self, render_pass: Option<&raw::RenderPass>) {
        unsafe {
            raw::destroy_render_pass(
                self,
                render_pass,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRenderAreaGranularity.html>"]
    #[doc(alias = "vkGetRenderAreaGranularity")]
    pub fn get_render_area_granularity(&self, render_pass: &raw::RenderPass) -> Extent2D {
        unsafe {
            raw::get_render_area_granularity(self, render_pass, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCommandPool.html>"]
    #[doc(alias = "vkCreateCommandPool")]
    pub fn create_command_pool(
        &self,
        p_create_info: &CommandPoolCreateInfo,
    ) -> Result<CommandPool> {
        let vk_result = unsafe {
            raw::create_command_pool(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { CommandPool::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCommandPool.html>"]
    #[doc(alias = "vkDestroyCommandPool")]
    pub unsafe fn destroy_command_pool(&self, command_pool: Option<&raw::CommandPool>) {
        unsafe {
            raw::destroy_command_pool(
                self,
                command_pool,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandPool.html>"]
    #[doc(alias = "vkResetCommandPool")]
    pub fn reset_command_pool(
        &self,
        command_pool: &raw::CommandPool,
        flags: CommandPoolResetFlags,
    ) -> Result<()> {
        unsafe {
            raw::reset_command_pool(
                self,
                command_pool,
                flags,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateCommandBuffers.html>"]
    #[doc(alias = "vkAllocateCommandBuffers")]
    pub fn allocate_command_buffers<
        R: AdvancedDynamicArray<CommandBuffer<D, A>, raw::CommandBuffer>,
    >(
        &self,
        p_allocate_info: &CommandBufferAllocateInfo,
    ) -> Result<R> {
        let vk_result: Result<R::InnerArrayType> = unsafe {
            raw::allocate_command_buffers(self, p_allocate_info, self.disp.get_command_dispatcher())
        };
        vk_result.map(|vk_result| {
            vk_result
                .into_iter()
                .map(|el| unsafe {
                    CommandBuffer::from_inner(el, self.disp.clone(), self.alloc.clone())
                })
                .collect()
        })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeCommandBuffers.html>"]
    #[doc(alias = "vkFreeCommandBuffers")]
    pub fn free_command_buffers<'a, V3: Alias<raw::CommandBuffer> + 'a>(
        &self,
        command_pool: &raw::CommandPool,
        p_command_buffers: impl AsSlice<'a, V3>,
    ) {
        unsafe {
            raw::free_command_buffers(
                self,
                command_pool,
                p_command_buffers,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2.html>"]
    #[doc(alias = "vkBindBufferMemory2")]
    pub fn bind_buffer_memory2<'a>(
        &self,
        p_bind_infos: impl AsSlice<'a, BindBufferMemoryInfo<'a>>,
    ) -> Result<()> {
        unsafe { raw::bind_buffer_memory2(self, p_bind_infos, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2KHR.html>"]
    #[doc(alias = "vkBindBufferMemory2KHR")]
    pub fn bind_buffer_memory2_khr<'a>(
        &self,
        p_bind_infos: impl AsSlice<'a, BindBufferMemoryInfo<'a>>,
    ) -> Result<()> {
        unsafe {
            raw::bind_buffer_memory2_khr(self, p_bind_infos, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2.html>"]
    #[doc(alias = "vkBindImageMemory2")]
    pub fn bind_image_memory2<'a>(
        &self,
        p_bind_infos: impl AsSlice<'a, BindImageMemoryInfo<'a>>,
    ) -> Result<()> {
        unsafe { raw::bind_image_memory2(self, p_bind_infos, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2KHR.html>"]
    #[doc(alias = "vkBindImageMemory2KHR")]
    pub fn bind_image_memory2_khr<'a>(
        &self,
        p_bind_infos: impl AsSlice<'a, BindImageMemoryInfo<'a>>,
    ) -> Result<()> {
        unsafe {
            raw::bind_image_memory2_khr(self, p_bind_infos, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html>"]
    #[doc(alias = "vkGetDeviceGroupPeerMemoryFeatures")]
    pub fn get_group_peer_memory_features(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> PeerMemoryFeatureFlags {
        unsafe {
            raw::get_device_group_peer_memory_features(
                self,
                heap_index,
                local_device_index,
                remote_device_index,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeaturesKHR.html>"]
    #[doc(alias = "vkGetDeviceGroupPeerMemoryFeaturesKHR")]
    pub fn get_group_peer_memory_features_khr(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> PeerMemoryFeatureFlags {
        unsafe {
            raw::get_device_group_peer_memory_features_khr(
                self,
                heap_index,
                local_device_index,
                remote_device_index,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2.html>"]
    #[doc(alias = "vkGetImageMemoryRequirements2")]
    pub fn get_image_memory_requirements2<S: StructureChainOut<MemoryRequirements2<'static>>>(
        &self,
        p_info: &ImageMemoryRequirementsInfo2,
    ) -> S {
        unsafe {
            raw::get_image_memory_requirements2(self, p_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2KHR.html>"]
    #[doc(alias = "vkGetImageMemoryRequirements2KHR")]
    pub fn get_image_memory_requirements2_khr<
        S: StructureChainOut<MemoryRequirements2<'static>>,
    >(
        &self,
        p_info: &ImageMemoryRequirementsInfo2,
    ) -> S {
        unsafe {
            raw::get_image_memory_requirements2_khr(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2.html>"]
    #[doc(alias = "vkGetBufferMemoryRequirements2")]
    pub fn get_buffer_memory_requirements2<S: StructureChainOut<MemoryRequirements2<'static>>>(
        &self,
        p_info: &BufferMemoryRequirementsInfo2,
    ) -> S {
        unsafe {
            raw::get_buffer_memory_requirements2(self, p_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2KHR.html>"]
    #[doc(alias = "vkGetBufferMemoryRequirements2KHR")]
    pub fn get_buffer_memory_requirements2_khr<
        S: StructureChainOut<MemoryRequirements2<'static>>,
    >(
        &self,
        p_info: &BufferMemoryRequirementsInfo2,
    ) -> S {
        unsafe {
            raw::get_buffer_memory_requirements2_khr(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2.html>"]
    #[doc(alias = "vkGetImageSparseMemoryRequirements2")]
    pub fn get_image_sparse_memory_requirements2<
        R: DynamicArray<SparseImageMemoryRequirements2<'static>>,
    >(
        &self,
        p_info: &ImageSparseMemoryRequirementsInfo2,
    ) -> R {
        unsafe {
            raw::get_image_sparse_memory_requirements2(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2KHR.html>"]
    #[doc(alias = "vkGetImageSparseMemoryRequirements2KHR")]
    pub fn get_image_sparse_memory_requirements2_khr<
        R: DynamicArray<SparseImageMemoryRequirements2<'static>>,
    >(
        &self,
        p_info: &ImageSparseMemoryRequirementsInfo2,
    ) -> R {
        unsafe {
            raw::get_image_sparse_memory_requirements2_khr(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPool.html>"]
    #[doc(alias = "vkTrimCommandPool")]
    pub fn trim_command_pool(&self, command_pool: &raw::CommandPool, flags: u32) {
        unsafe {
            raw::trim_command_pool(
                self,
                command_pool,
                flags,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPoolKHR.html>"]
    #[doc(alias = "vkTrimCommandPoolKHR")]
    pub fn trim_command_pool_khr(&self, command_pool: &raw::CommandPool, flags: u32) {
        unsafe {
            raw::trim_command_pool_khr(
                self,
                command_pool,
                flags,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue2.html>"]
    #[doc(alias = "vkGetDeviceQueue2")]
    pub fn get_queue2(&self, p_queue_info: &DeviceQueueInfo2) -> Queue<D, A> {
        let vk_result = unsafe {
            raw::get_device_queue2(self, p_queue_info, self.disp.get_command_dispatcher())
        };
        unsafe { Queue::from_inner(vk_result, self.disp.clone(), self.alloc.clone()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversion.html>"]
    #[doc(alias = "vkCreateSamplerYcbcrConversion")]
    pub fn create_sampler_ycbcr_conversion(
        &self,
        p_create_info: &SamplerYcbcrConversionCreateInfo,
    ) -> Result<SamplerYcbcrConversion> {
        let vk_result = unsafe {
            raw::create_sampler_ycbcr_conversion(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SamplerYcbcrConversion::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversionKHR.html>"]
    #[doc(alias = "vkCreateSamplerYcbcrConversionKHR")]
    pub fn create_sampler_ycbcr_conversion_khr(
        &self,
        p_create_info: &SamplerYcbcrConversionCreateInfo,
    ) -> Result<SamplerYcbcrConversion> {
        let vk_result = unsafe {
            raw::create_sampler_ycbcr_conversion_khr(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SamplerYcbcrConversion::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversion.html>"]
    #[doc(alias = "vkDestroySamplerYcbcrConversion")]
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        ycbcr_conversion: Option<&raw::SamplerYcbcrConversion>,
    ) {
        unsafe {
            raw::destroy_sampler_ycbcr_conversion(
                self,
                ycbcr_conversion,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversionKHR.html>"]
    #[doc(alias = "vkDestroySamplerYcbcrConversionKHR")]
    pub unsafe fn destroy_sampler_ycbcr_conversion_khr(
        &self,
        ycbcr_conversion: Option<&raw::SamplerYcbcrConversion>,
    ) {
        unsafe {
            raw::destroy_sampler_ycbcr_conversion_khr(
                self,
                ycbcr_conversion,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplate.html>"]
    #[doc(alias = "vkCreateDescriptorUpdateTemplate")]
    pub fn create_descriptor_update_template(
        &self,
        p_create_info: &DescriptorUpdateTemplateCreateInfo,
    ) -> Result<DescriptorUpdateTemplate> {
        let vk_result = unsafe {
            raw::create_descriptor_update_template(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { DescriptorUpdateTemplate::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplateKHR.html>"]
    #[doc(alias = "vkCreateDescriptorUpdateTemplateKHR")]
    pub fn create_descriptor_update_template_khr(
        &self,
        p_create_info: &DescriptorUpdateTemplateCreateInfo,
    ) -> Result<DescriptorUpdateTemplate> {
        let vk_result = unsafe {
            raw::create_descriptor_update_template_khr(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { DescriptorUpdateTemplate::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html>"]
    #[doc(alias = "vkDestroyDescriptorUpdateTemplate")]
    pub unsafe fn destroy_descriptor_update_template(
        &self,
        descriptor_update_template: Option<&raw::DescriptorUpdateTemplate>,
    ) {
        unsafe {
            raw::destroy_descriptor_update_template(
                self,
                descriptor_update_template,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorUpdateTemplateKHR.html>"]
    #[doc(alias = "vkDestroyDescriptorUpdateTemplateKHR")]
    pub unsafe fn destroy_descriptor_update_template_khr(
        &self,
        descriptor_update_template: Option<&raw::DescriptorUpdateTemplate>,
    ) {
        unsafe {
            raw::destroy_descriptor_update_template_khr(
                self,
                descriptor_update_template,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html>"]
    #[doc(alias = "vkUpdateDescriptorSetWithTemplate")]
    pub fn update_descriptor_set_with_template(
        &self,
        descriptor_set: &raw::DescriptorSet,
        descriptor_update_template: &raw::DescriptorUpdateTemplate,
        p_data: VoidPtr,
    ) {
        unsafe {
            raw::update_descriptor_set_with_template(
                self,
                descriptor_set,
                descriptor_update_template,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplateKHR.html>"]
    #[doc(alias = "vkUpdateDescriptorSetWithTemplateKHR")]
    pub fn update_descriptor_set_with_template_khr(
        &self,
        descriptor_set: &raw::DescriptorSet,
        descriptor_update_template: &raw::DescriptorUpdateTemplate,
        p_data: VoidPtr,
    ) {
        unsafe {
            raw::update_descriptor_set_with_template_khr(
                self,
                descriptor_set,
                descriptor_update_template,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupport.html>"]
    #[doc(alias = "vkGetDescriptorSetLayoutSupport")]
    pub fn get_descriptor_set_layout_support<
        S: StructureChainOut<DescriptorSetLayoutSupport<'static>>,
    >(
        &self,
        p_create_info: &DescriptorSetLayoutCreateInfo,
    ) -> S {
        unsafe {
            raw::get_descriptor_set_layout_support(
                self,
                p_create_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupportKHR.html>"]
    #[doc(alias = "vkGetDescriptorSetLayoutSupportKHR")]
    pub fn get_descriptor_set_layout_support_khr<
        S: StructureChainOut<DescriptorSetLayoutSupport<'static>>,
    >(
        &self,
        p_create_info: &DescriptorSetLayoutCreateInfo,
    ) -> S {
        unsafe {
            raw::get_descriptor_set_layout_support_khr(
                self,
                p_create_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2.html>"]
    #[doc(alias = "vkCreateRenderPass2")]
    pub fn create_render_pass2(&self, p_create_info: &RenderPassCreateInfo2) -> Result<RenderPass> {
        let vk_result = unsafe {
            raw::create_render_pass2(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { RenderPass::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2KHR.html>"]
    #[doc(alias = "vkCreateRenderPass2KHR")]
    pub fn create_render_pass2_khr(
        &self,
        p_create_info: &RenderPassCreateInfo2,
    ) -> Result<RenderPass> {
        let vk_result = unsafe {
            raw::create_render_pass2_khr(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { RenderPass::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetQueryPool.html>"]
    #[doc(alias = "vkResetQueryPool")]
    pub fn reset_query_pool(
        &self,
        query_pool: &raw::QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        unsafe {
            raw::reset_query_pool(
                self,
                query_pool,
                first_query,
                query_count,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetQueryPoolEXT.html>"]
    #[doc(alias = "vkResetQueryPoolEXT")]
    pub fn reset_query_pool_ext(
        &self,
        query_pool: &raw::QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        unsafe {
            raw::reset_query_pool_ext(
                self,
                query_pool,
                first_query,
                query_count,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValue.html>"]
    #[doc(alias = "vkGetSemaphoreCounterValue")]
    pub fn get_semaphore_counter_value(&self, semaphore: &raw::Semaphore) -> Result<u64> {
        unsafe {
            raw::get_semaphore_counter_value(self, semaphore, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValueKHR.html>"]
    #[doc(alias = "vkGetSemaphoreCounterValueKHR")]
    pub fn get_semaphore_counter_value_khr(&self, semaphore: &raw::Semaphore) -> Result<u64> {
        unsafe {
            raw::get_semaphore_counter_value_khr(
                self,
                semaphore,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphores.html>"]
    #[doc(alias = "vkWaitSemaphores")]
    pub fn wait_semaphores(&self, p_wait_info: &SemaphoreWaitInfo, timeout: u64) -> Result<Status> {
        unsafe {
            raw::wait_semaphores(
                self,
                p_wait_info,
                timeout,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphoresKHR.html>"]
    #[doc(alias = "vkWaitSemaphoresKHR")]
    pub fn wait_semaphores_khr(
        &self,
        p_wait_info: &SemaphoreWaitInfo,
        timeout: u64,
    ) -> Result<Status> {
        unsafe {
            raw::wait_semaphores_khr(
                self,
                p_wait_info,
                timeout,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphore.html>"]
    #[doc(alias = "vkSignalSemaphore")]
    pub fn signal_semaphore(&self, p_signal_info: &SemaphoreSignalInfo) -> Result<()> {
        unsafe { raw::signal_semaphore(self, p_signal_info, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphoreKHR.html>"]
    #[doc(alias = "vkSignalSemaphoreKHR")]
    pub fn signal_semaphore_khr(&self, p_signal_info: &SemaphoreSignalInfo) -> Result<()> {
        unsafe {
            raw::signal_semaphore_khr(self, p_signal_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddress.html>"]
    #[doc(alias = "vkGetBufferDeviceAddress")]
    pub fn get_buffer_address(&self, p_info: &BufferDeviceAddressInfo) -> DeviceAddress {
        unsafe { raw::get_buffer_device_address(self, p_info, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddressKHR.html>"]
    #[doc(alias = "vkGetBufferDeviceAddressKHR")]
    pub fn get_buffer_address_khr(&self, p_info: &BufferDeviceAddressInfo) -> DeviceAddress {
        unsafe {
            raw::get_buffer_device_address_khr(self, p_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddressEXT.html>"]
    #[doc(alias = "vkGetBufferDeviceAddressEXT")]
    pub fn get_buffer_address_ext(&self, p_info: &BufferDeviceAddressInfo) -> DeviceAddress {
        unsafe {
            raw::get_buffer_device_address_ext(self, p_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureAddress.html>"]
    #[doc(alias = "vkGetBufferOpaqueCaptureAddress")]
    pub fn get_buffer_opaque_capture_address(&self, p_info: &BufferDeviceAddressInfo) -> u64 {
        unsafe {
            raw::get_buffer_opaque_capture_address(self, p_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureAddressKHR.html>"]
    #[doc(alias = "vkGetBufferOpaqueCaptureAddressKHR")]
    pub fn get_buffer_opaque_capture_address_khr(&self, p_info: &BufferDeviceAddressInfo) -> u64 {
        unsafe {
            raw::get_buffer_opaque_capture_address_khr(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddress.html>"]
    #[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddress")]
    pub fn get_memory_opaque_capture_address(
        &self,
        p_info: &DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        unsafe {
            raw::get_device_memory_opaque_capture_address(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html>"]
    #[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddressKHR")]
    pub fn get_memory_opaque_capture_address_khr(
        &self,
        p_info: &DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        unsafe {
            raw::get_device_memory_opaque_capture_address_khr(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePrivateDataSlot.html>"]
    #[doc(alias = "vkCreatePrivateDataSlot")]
    pub fn create_private_data_slot(
        &self,
        p_create_info: &PrivateDataSlotCreateInfo,
    ) -> Result<PrivateDataSlot> {
        let vk_result = unsafe {
            raw::create_private_data_slot(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { PrivateDataSlot::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePrivateDataSlotEXT.html>"]
    #[doc(alias = "vkCreatePrivateDataSlotEXT")]
    pub fn create_private_data_slot_ext(
        &self,
        p_create_info: &PrivateDataSlotCreateInfo,
    ) -> Result<PrivateDataSlot> {
        let vk_result = unsafe {
            raw::create_private_data_slot_ext(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { PrivateDataSlot::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPrivateDataSlot.html>"]
    #[doc(alias = "vkDestroyPrivateDataSlot")]
    pub unsafe fn destroy_private_data_slot(
        &self,
        private_data_slot: Option<&raw::PrivateDataSlot>,
    ) {
        unsafe {
            raw::destroy_private_data_slot(
                self,
                private_data_slot,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPrivateDataSlotEXT.html>"]
    #[doc(alias = "vkDestroyPrivateDataSlotEXT")]
    pub unsafe fn destroy_private_data_slot_ext(
        &self,
        private_data_slot: Option<&raw::PrivateDataSlot>,
    ) {
        unsafe {
            raw::destroy_private_data_slot_ext(
                self,
                private_data_slot,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetPrivateData.html>"]
    #[doc(alias = "vkSetPrivateData")]
    pub fn set_private_data(
        &self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: &raw::PrivateDataSlot,
        data: u64,
    ) -> Result<()> {
        unsafe {
            raw::set_private_data(
                self,
                object_type,
                object_handle,
                private_data_slot,
                data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetPrivateDataEXT.html>"]
    #[doc(alias = "vkSetPrivateDataEXT")]
    pub fn set_private_data_ext(
        &self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: &raw::PrivateDataSlot,
        data: u64,
    ) -> Result<()> {
        unsafe {
            raw::set_private_data_ext(
                self,
                object_type,
                object_handle,
                private_data_slot,
                data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPrivateData.html>"]
    #[doc(alias = "vkGetPrivateData")]
    pub fn get_private_data(
        &self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: &raw::PrivateDataSlot,
    ) -> u64 {
        unsafe {
            raw::get_private_data(
                self,
                object_type,
                object_handle,
                private_data_slot,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPrivateDataEXT.html>"]
    #[doc(alias = "vkGetPrivateDataEXT")]
    pub fn get_private_data_ext(
        &self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: &raw::PrivateDataSlot,
    ) -> u64 {
        unsafe {
            raw::get_private_data_ext(
                self,
                object_type,
                object_handle,
                private_data_slot,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirements.html>"]
    #[doc(alias = "vkGetDeviceBufferMemoryRequirements")]
    pub fn get_device_buffer_memory_requirements<
        S: StructureChainOut<MemoryRequirements2<'static>>,
    >(
        &self,
        p_info: &DeviceBufferMemoryRequirements,
    ) -> S {
        unsafe {
            raw::get_device_buffer_memory_requirements(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirementsKHR.html>"]
    #[doc(alias = "vkGetDeviceBufferMemoryRequirementsKHR")]
    pub fn get_buffer_memory_requirements_khr<
        S: StructureChainOut<MemoryRequirements2<'static>>,
    >(
        &self,
        p_info: &DeviceBufferMemoryRequirements,
    ) -> S {
        unsafe {
            raw::get_device_buffer_memory_requirements_khr(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirements.html>"]
    #[doc(alias = "vkGetDeviceImageMemoryRequirements")]
    pub fn get_device_image_memory_requirements<
        S: StructureChainOut<MemoryRequirements2<'static>>,
    >(
        &self,
        p_info: &DeviceImageMemoryRequirements,
    ) -> S {
        unsafe {
            raw::get_device_image_memory_requirements(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirementsKHR.html>"]
    #[doc(alias = "vkGetDeviceImageMemoryRequirementsKHR")]
    pub fn get_image_memory_requirements_khr<S: StructureChainOut<MemoryRequirements2<'static>>>(
        &self,
        p_info: &DeviceImageMemoryRequirements,
    ) -> S {
        unsafe {
            raw::get_device_image_memory_requirements_khr(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSparseMemoryRequirements.html>"]
    #[doc(alias = "vkGetDeviceImageSparseMemoryRequirements")]
    pub fn get_device_image_sparse_memory_requirements<
        R: DynamicArray<SparseImageMemoryRequirements2<'static>>,
    >(
        &self,
        p_info: &DeviceImageMemoryRequirements,
    ) -> R {
        unsafe {
            raw::get_device_image_sparse_memory_requirements(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSparseMemoryRequirementsKHR.html>"]
    #[doc(alias = "vkGetDeviceImageSparseMemoryRequirementsKHR")]
    pub fn get_image_sparse_memory_requirements_khr<
        R: DynamicArray<SparseImageMemoryRequirements2<'static>>,
    >(
        &self,
        p_info: &DeviceImageMemoryRequirements,
    ) -> R {
        unsafe {
            raw::get_device_image_sparse_memory_requirements_khr(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html>"]
    #[doc(alias = "vkCreateSwapchainKHR")]
    pub fn create_swapchain_khr(
        &self,
        p_create_info: &SwapchainCreateInfoKHR,
    ) -> Result<SwapchainKHR> {
        let vk_result = unsafe {
            raw::create_swapchain_khr(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SwapchainKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html>"]
    #[doc(alias = "vkDestroySwapchainKHR")]
    pub unsafe fn destroy_swapchain_khr(&self, swapchain: Option<&raw::SwapchainKHR>) {
        unsafe {
            raw::destroy_swapchain_khr(
                self,
                swapchain,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html>"]
    #[doc(alias = "vkGetSwapchainImagesKHR")]
    pub fn get_swapchain_images_khr<R: AdvancedDynamicArray<Image, raw::Image>>(
        &self,
        swapchain: &raw::SwapchainKHR,
    ) -> Result<R> {
        let vk_result: Result<R::InnerArrayType> = unsafe {
            raw::get_swapchain_images_khr(self, swapchain, self.disp.get_command_dispatcher())
        };
        vk_result.map(|vk_result| {
            vk_result
                .into_iter()
                .map(|el| unsafe { Image::from_inner(el) })
                .collect()
        })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html>"]
    #[doc(alias = "vkAcquireNextImageKHR")]
    pub fn acquire_next_image_khr(
        &self,
        swapchain: &raw::SwapchainKHR,
        timeout: u64,
        semaphore: Option<&raw::Semaphore>,
        fence: Option<&raw::Fence>,
    ) -> Result<(Status, u32)> {
        unsafe {
            raw::acquire_next_image_khr(
                self,
                swapchain,
                timeout,
                semaphore,
                fence,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html>"]
    #[doc(alias = "vkGetDeviceGroupPresentCapabilitiesKHR")]
    pub fn get_group_present_capabilities_khr<
        S: StructureChainOut<DeviceGroupPresentCapabilitiesKHR<'static>>,
    >(
        &self,
    ) -> Result<S> {
        unsafe {
            raw::get_device_group_present_capabilities_khr(self, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html>"]
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
    pub fn get_group_surface_present_modes_khr(
        &self,
        surface: &raw::SurfaceKHR,
    ) -> Result<DeviceGroupPresentModeFlagsKHR> {
        unsafe {
            raw::get_device_group_surface_present_modes_khr(
                self,
                surface,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html>"]
    #[doc(alias = "vkAcquireNextImage2KHR")]
    pub fn acquire_next_image2_khr(
        &self,
        p_acquire_info: &AcquireNextImageInfoKHR,
    ) -> Result<(Status, u32)> {
        unsafe {
            raw::acquire_next_image2_khr(self, p_acquire_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSharedSwapchainsKHR.html>"]
    #[doc(alias = "vkCreateSharedSwapchainsKHR")]
    pub fn create_shared_swapchains_khr<
        'a,
        R: AdvancedDynamicArray<SwapchainKHR, raw::SwapchainKHR>,
    >(
        &self,
        p_create_infos: impl AsSlice<'a, SwapchainCreateInfoKHR<'a>>,
    ) -> Result<R> {
        let vk_result: Result<R::InnerArrayType> = unsafe {
            raw::create_shared_swapchains_khr(
                self,
                p_create_infos,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| {
            vk_result
                .into_iter()
                .map(|el| unsafe { SwapchainKHR::from_inner(el) })
                .collect()
        })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html>"]
    #[doc(alias = "vkDebugMarkerSetObjectTagEXT")]
    pub fn debug_marker_set_object_tag_ext(
        &self,
        p_tag_info: &DebugMarkerObjectTagInfoEXT,
    ) -> Result<()> {
        unsafe {
            raw::debug_marker_set_object_tag_ext(
                self,
                p_tag_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html>"]
    #[doc(alias = "vkDebugMarkerSetObjectNameEXT")]
    pub fn debug_marker_set_object_name_ext(
        &self,
        p_name_info: &DebugMarkerObjectNameInfoEXT,
    ) -> Result<()> {
        unsafe {
            raw::debug_marker_set_object_name_ext(
                self,
                p_name_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCuModuleNVX.html>"]
    #[doc(alias = "vkCreateCuModuleNVX")]
    pub fn create_cu_module_nvx(
        &self,
        p_create_info: &CuModuleCreateInfoNVX,
    ) -> Result<CuModuleNVX> {
        let vk_result = unsafe {
            raw::create_cu_module_nvx(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { CuModuleNVX::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCuFunctionNVX.html>"]
    #[doc(alias = "vkCreateCuFunctionNVX")]
    pub fn create_cu_function_nvx(
        &self,
        p_create_info: &CuFunctionCreateInfoNVX,
    ) -> Result<CuFunctionNVX> {
        let vk_result = unsafe {
            raw::create_cu_function_nvx(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { CuFunctionNVX::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCuModuleNVX.html>"]
    #[doc(alias = "vkDestroyCuModuleNVX")]
    pub unsafe fn destroy_cu_module_nvx(&self, module: &raw::CuModuleNVX) {
        unsafe {
            raw::destroy_cu_module_nvx(
                self,
                module,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCuFunctionNVX.html>"]
    #[doc(alias = "vkDestroyCuFunctionNVX")]
    pub unsafe fn destroy_cu_function_nvx(&self, function: &raw::CuFunctionNVX) {
        unsafe {
            raw::destroy_cu_function_nvx(
                self,
                function,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewHandleNVX.html>"]
    #[doc(alias = "vkGetImageViewHandleNVX")]
    pub fn get_image_view_handle_nvx(&self, p_info: &ImageViewHandleInfoNVX) -> u32 {
        unsafe { raw::get_image_view_handle_nvx(self, p_info, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewAddressNVX.html>"]
    #[doc(alias = "vkGetImageViewAddressNVX")]
    pub fn get_image_view_address_nvx<
        S: StructureChainOut<ImageViewAddressPropertiesNVX<'static>>,
    >(
        &self,
        image_view: &raw::ImageView,
    ) -> Result<S> {
        unsafe {
            raw::get_image_view_address_nvx(self, image_view, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderInfoAMD.html>"]
    #[doc(alias = "vkGetShaderInfoAMD")]
    pub fn get_shader_info_amd(
        &self,
        pipeline: &raw::Pipeline,
        shader_stage: ShaderStageFlags,
        info_type: ShaderInfoTypeAMD,
        p_info: VoidPtr,
    ) -> Result<usize> {
        unsafe {
            raw::get_shader_info_amd(
                self,
                pipeline,
                shader_stage,
                info_type,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleNV.html>"]
    #[doc(alias = "vkGetMemoryWin32HandleNV")]
    pub fn get_memory_win32_handle_nv(
        &self,
        memory: &raw::DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
    ) -> Result<VoidPtr> {
        unsafe {
            raw::get_memory_win32_handle_nv(
                self,
                memory,
                handle_type,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleKHR.html>"]
    #[doc(alias = "vkGetMemoryWin32HandleKHR")]
    pub fn get_memory_win32_handle_khr(
        &self,
        p_get_win32_handle_info: &MemoryGetWin32HandleInfoKHR,
    ) -> Result<VoidPtr> {
        unsafe {
            raw::get_memory_win32_handle_khr(
                self,
                p_get_win32_handle_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html>"]
    #[doc(alias = "vkGetMemoryWin32HandlePropertiesKHR")]
    pub fn get_memory_win32_handle_properties_khr<
        S: StructureChainOut<MemoryWin32HandlePropertiesKHR<'static>>,
    >(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: VoidPtr,
    ) -> Result<S> {
        unsafe {
            raw::get_memory_win32_handle_properties_khr(
                self,
                handle_type,
                handle,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdKHR.html>"]
    #[doc(alias = "vkGetMemoryFdKHR")]
    pub fn get_memory_fd_khr(&self, p_get_fd_info: &MemoryGetFdInfoKHR) -> Result<c_int> {
        unsafe { raw::get_memory_fd_khr(self, p_get_fd_info, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdPropertiesKHR.html>"]
    #[doc(alias = "vkGetMemoryFdPropertiesKHR")]
    pub fn get_memory_fd_properties_khr<S: StructureChainOut<MemoryFdPropertiesKHR<'static>>>(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        fd: c_int,
    ) -> Result<S> {
        unsafe {
            raw::get_memory_fd_properties_khr(
                self,
                handle_type,
                fd,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html>"]
    #[doc(alias = "vkImportSemaphoreWin32HandleKHR")]
    pub fn import_semaphore_win32_handle_khr(
        &self,
        p_import_semaphore_win32_handle_info: &ImportSemaphoreWin32HandleInfoKHR,
    ) -> Result<()> {
        unsafe {
            raw::import_semaphore_win32_handle_khr(
                self,
                p_import_semaphore_win32_handle_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html>"]
    #[doc(alias = "vkGetSemaphoreWin32HandleKHR")]
    pub fn get_semaphore_win32_handle_khr(
        &self,
        p_get_win32_handle_info: &SemaphoreGetWin32HandleInfoKHR,
    ) -> Result<VoidPtr> {
        unsafe {
            raw::get_semaphore_win32_handle_khr(
                self,
                p_get_win32_handle_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreFdKHR.html>"]
    #[doc(alias = "vkImportSemaphoreFdKHR")]
    pub fn import_semaphore_fd_khr(
        &self,
        p_import_semaphore_fd_info: &ImportSemaphoreFdInfoKHR,
    ) -> Result<()> {
        unsafe {
            raw::import_semaphore_fd_khr(
                self,
                p_import_semaphore_fd_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreFdKHR.html>"]
    #[doc(alias = "vkGetSemaphoreFdKHR")]
    pub fn get_semaphore_fd_khr(&self, p_get_fd_info: &SemaphoreGetFdInfoKHR) -> Result<c_int> {
        unsafe {
            raw::get_semaphore_fd_khr(self, p_get_fd_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDisplayPowerControlEXT.html>"]
    #[doc(alias = "vkDisplayPowerControlEXT")]
    pub fn display_power_control_ext(
        &self,
        display: &raw::DisplayKHR,
        p_display_power_info: &DisplayPowerInfoEXT,
    ) -> Result<()> {
        unsafe {
            raw::display_power_control_ext(
                self,
                display,
                p_display_power_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkRegisterDeviceEventEXT.html>"]
    #[doc(alias = "vkRegisterDeviceEventEXT")]
    pub fn register_event_ext(&self, p_device_event_info: &DeviceEventInfoEXT) -> Result<Fence> {
        let vk_result = unsafe {
            raw::register_device_event_ext(
                self,
                p_device_event_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { Fence::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkRegisterDisplayEventEXT.html>"]
    #[doc(alias = "vkRegisterDisplayEventEXT")]
    pub fn register_display_event_ext(
        &self,
        display: &raw::DisplayKHR,
        p_display_event_info: &DisplayEventInfoEXT,
    ) -> Result<Fence> {
        let vk_result = unsafe {
            raw::register_display_event_ext(
                self,
                display,
                p_display_event_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { Fence::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainCounterEXT.html>"]
    #[doc(alias = "vkGetSwapchainCounterEXT")]
    pub fn get_swapchain_counter_ext(
        &self,
        swapchain: &raw::SwapchainKHR,
        counter: SurfaceCounterFlagsEXT,
    ) -> Result<u64> {
        unsafe {
            raw::get_swapchain_counter_ext(
                self,
                swapchain,
                counter,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html>"]
    #[doc(alias = "vkGetRefreshCycleDurationGOOGLE")]
    pub fn get_refresh_cycle_duration_google(
        &self,
        swapchain: &raw::SwapchainKHR,
    ) -> Result<RefreshCycleDurationGOOGLE> {
        unsafe {
            raw::get_refresh_cycle_duration_google(
                self,
                swapchain,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html>"]
    #[doc(alias = "vkGetPastPresentationTimingGOOGLE")]
    pub fn get_past_presentation_timing_google<R: DynamicArray<PastPresentationTimingGOOGLE>>(
        &self,
        swapchain: &raw::SwapchainKHR,
    ) -> Result<R> {
        unsafe {
            raw::get_past_presentation_timing_google(
                self,
                swapchain,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetHdrMetadataEXT.html>"]
    #[doc(alias = "vkSetHdrMetadataEXT")]
    pub fn set_hdr_metadata_ext<'a, V2: Alias<raw::SwapchainKHR> + 'a>(
        &self,
        p_swapchains: impl AsSlice<'a, V2>,
        p_metadata: impl AsSlice<'a, HdrMetadataEXT<'a>>,
    ) {
        unsafe {
            raw::set_hdr_metadata_ext(
                self,
                p_swapchains,
                p_metadata,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainStatusKHR.html>"]
    #[doc(alias = "vkGetSwapchainStatusKHR")]
    pub fn get_swapchain_status_khr(&self, swapchain: &raw::SwapchainKHR) -> Result<Status> {
        unsafe {
            raw::get_swapchain_status_khr(self, swapchain, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceWin32HandleKHR.html>"]
    #[doc(alias = "vkImportFenceWin32HandleKHR")]
    pub fn import_fence_win32_handle_khr(
        &self,
        p_import_fence_win32_handle_info: &ImportFenceWin32HandleInfoKHR,
    ) -> Result<()> {
        unsafe {
            raw::import_fence_win32_handle_khr(
                self,
                p_import_fence_win32_handle_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceWin32HandleKHR.html>"]
    #[doc(alias = "vkGetFenceWin32HandleKHR")]
    pub fn get_fence_win32_handle_khr(
        &self,
        p_get_win32_handle_info: &FenceGetWin32HandleInfoKHR,
    ) -> Result<VoidPtr> {
        unsafe {
            raw::get_fence_win32_handle_khr(
                self,
                p_get_win32_handle_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceFdKHR.html>"]
    #[doc(alias = "vkImportFenceFdKHR")]
    pub fn import_fence_fd_khr(&self, p_import_fence_fd_info: &ImportFenceFdInfoKHR) -> Result<()> {
        unsafe {
            raw::import_fence_fd_khr(
                self,
                p_import_fence_fd_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceFdKHR.html>"]
    #[doc(alias = "vkGetFenceFdKHR")]
    pub fn get_fence_fd_khr(&self, p_get_fd_info: &FenceGetFdInfoKHR) -> Result<c_int> {
        unsafe { raw::get_fence_fd_khr(self, p_get_fd_info, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireProfilingLockKHR.html>"]
    #[doc(alias = "vkAcquireProfilingLockKHR")]
    pub fn acquire_profiling_lock_khr(&self, p_info: &AcquireProfilingLockInfoKHR) -> Result<()> {
        unsafe { raw::acquire_profiling_lock_khr(self, p_info, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseProfilingLockKHR.html>"]
    #[doc(alias = "vkReleaseProfilingLockKHR")]
    pub fn release_profiling_lock_khr(&self) {
        unsafe { raw::release_profiling_lock_khr(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html>"]
    #[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
    pub fn set_debug_utils_object_name_ext(
        &self,
        p_name_info: &DebugUtilsObjectNameInfoEXT,
    ) -> Result<()> {
        unsafe {
            raw::set_debug_utils_object_name_ext(
                self,
                p_name_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html>"]
    #[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
    pub fn set_debug_utils_object_tag_ext(
        &self,
        p_tag_info: &DebugUtilsObjectTagInfoEXT,
    ) -> Result<()> {
        unsafe {
            raw::set_debug_utils_object_tag_ext(
                self,
                p_tag_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html>"]
    #[doc(alias = "vkGetAndroidHardwareBufferPropertiesANDROID")]
    pub fn get_android_hardware_buffer_properties_android<
        S: StructureChainOut<AndroidHardwareBufferPropertiesANDROID<'static>>,
    >(
        &self,
        buffer: &AHardwareBuffer,
    ) -> Result<S> {
        unsafe {
            raw::get_android_hardware_buffer_properties_android(
                self,
                buffer,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html>"]
    #[doc(alias = "vkGetMemoryAndroidHardwareBufferANDROID")]
    pub fn get_memory_android_hardware_buffer_android(
        &self,
        p_info: &MemoryGetAndroidHardwareBufferInfoANDROID,
        p_buffer: &&AHardwareBuffer,
    ) -> Result<()> {
        unsafe {
            raw::get_memory_android_hardware_buffer_android(
                self,
                p_info,
                p_buffer,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateExecutionGraphPipelinesAMDX.html>"]
    #[doc(alias = "vkCreateExecutionGraphPipelinesAMDX")]
    pub fn create_execution_graph_pipelines_amdx<
        'a,
        R: AdvancedDynamicArray<Pipeline, raw::Pipeline>,
    >(
        &self,
        pipeline_cache: Option<&raw::PipelineCache>,
        p_create_infos: impl AsSlice<'a, ExecutionGraphPipelineCreateInfoAMDX<'a>>,
    ) -> Result<(Status, R)> {
        let vk_result: Result<(Status, R::InnerArrayType)> = unsafe {
            raw::create_execution_graph_pipelines_amdx(
                self,
                pipeline_cache,
                p_create_infos,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|(status, vk_result)| {
            (
                status,
                vk_result
                    .into_iter()
                    .map(|el| unsafe { Pipeline::from_inner(el) })
                    .collect(),
            )
        })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetExecutionGraphPipelineScratchSizeAMDX.html>"]
    #[doc(alias = "vkGetExecutionGraphPipelineScratchSizeAMDX")]
    pub fn get_execution_graph_pipeline_scratch_size_amdx<
        S: StructureChainOut<ExecutionGraphPipelineScratchSizeAMDX<'static>>,
    >(
        &self,
        execution_graph: &raw::Pipeline,
    ) -> Result<S> {
        unsafe {
            raw::get_execution_graph_pipeline_scratch_size_amdx(
                self,
                execution_graph,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetExecutionGraphPipelineNodeIndexAMDX.html>"]
    #[doc(alias = "vkGetExecutionGraphPipelineNodeIndexAMDX")]
    pub fn get_execution_graph_pipeline_node_index_amdx(
        &self,
        execution_graph: &raw::Pipeline,
        p_node_info: &PipelineShaderStageNodeCreateInfoAMDX,
    ) -> Result<u32> {
        unsafe {
            raw::get_execution_graph_pipeline_node_index_amdx(
                self,
                execution_graph,
                p_node_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureKHR.html>"]
    #[doc(alias = "vkCreateAccelerationStructureKHR")]
    pub fn create_acceleration_structure_khr(
        &self,
        p_create_info: &AccelerationStructureCreateInfoKHR,
    ) -> Result<AccelerationStructureKHR> {
        let vk_result = unsafe {
            raw::create_acceleration_structure_khr(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { AccelerationStructureKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureKHR.html>"]
    #[doc(alias = "vkDestroyAccelerationStructureKHR")]
    pub unsafe fn destroy_acceleration_structure_khr(
        &self,
        acceleration_structure: Option<&raw::AccelerationStructureKHR>,
    ) {
        unsafe {
            raw::destroy_acceleration_structure_khr(
                self,
                acceleration_structure,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBuildAccelerationStructuresKHR.html>"]
    #[doc(alias = "vkBuildAccelerationStructuresKHR")]
    pub fn build_acceleration_structures_khr<'a>(
        &self,
        deferred_operation: Option<&raw::DeferredOperationKHR>,
        p_infos: impl AsSlice<'a, AccelerationStructureBuildGeometryInfoKHR<'a>>,
        pp_build_range_infos: &&AccelerationStructureBuildRangeInfoKHR,
    ) -> Result<Status> {
        unsafe {
            raw::build_acceleration_structures_khr(
                self,
                deferred_operation,
                p_infos,
                pp_build_range_infos,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureKHR.html>"]
    #[doc(alias = "vkCopyAccelerationStructureKHR")]
    pub fn copy_acceleration_structure_khr(
        &self,
        deferred_operation: Option<&raw::DeferredOperationKHR>,
        p_info: &CopyAccelerationStructureInfoKHR,
    ) -> Result<Status> {
        unsafe {
            raw::copy_acceleration_structure_khr(
                self,
                deferred_operation,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureToMemoryKHR.html>"]
    #[doc(alias = "vkCopyAccelerationStructureToMemoryKHR")]
    pub fn copy_acceleration_structure_to_memory_khr(
        &self,
        deferred_operation: Option<&raw::DeferredOperationKHR>,
        p_info: &CopyAccelerationStructureToMemoryInfoKHR,
    ) -> Result<Status> {
        unsafe {
            raw::copy_acceleration_structure_to_memory_khr(
                self,
                deferred_operation,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToAccelerationStructureKHR.html>"]
    #[doc(alias = "vkCopyMemoryToAccelerationStructureKHR")]
    pub fn copy_memory_to_acceleration_structure_khr(
        &self,
        deferred_operation: Option<&raw::DeferredOperationKHR>,
        p_info: &CopyMemoryToAccelerationStructureInfoKHR,
    ) -> Result<Status> {
        unsafe {
            raw::copy_memory_to_acceleration_structure_khr(
                self,
                deferred_operation,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html>"]
    #[doc(alias = "vkWriteAccelerationStructuresPropertiesKHR")]
    pub fn write_acceleration_structures_properties_khr<
        'a,
        V2: Alias<raw::AccelerationStructureKHR> + 'a,
    >(
        &self,
        p_acceleration_structures: impl AsSlice<'a, V2>,
        query_type: QueryType,
        data_size: usize,
        p_data: VoidPtr,
        stride: usize,
    ) -> Result<()> {
        unsafe {
            raw::write_acceleration_structures_properties_khr(
                self,
                p_acceleration_structures,
                query_type,
                data_size,
                p_data,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html>"]
    #[doc(alias = "vkGetAccelerationStructureDeviceAddressKHR")]
    pub fn get_acceleration_structure_address_khr(
        &self,
        p_info: &AccelerationStructureDeviceAddressInfoKHR,
    ) -> DeviceAddress {
        unsafe {
            raw::get_acceleration_structure_device_address_khr(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html>"]
    #[doc(alias = "vkGetDeviceAccelerationStructureCompatibilityKHR")]
    pub fn get_acceleration_structure_compatibility_khr(
        &self,
        p_version_info: &AccelerationStructureVersionInfoKHR,
    ) -> AccelerationStructureCompatibilityKHR {
        unsafe {
            raw::get_device_acceleration_structure_compatibility_khr(
                self,
                p_version_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureBuildSizesKHR.html>"]
    #[doc(alias = "vkGetAccelerationStructureBuildSizesKHR")]
    pub fn get_acceleration_structure_build_sizes_khr<
        'a,
        S: StructureChainOut<AccelerationStructureBuildSizesInfoKHR<'static>>,
    >(
        &self,
        build_type: AccelerationStructureBuildTypeKHR,
        p_build_info: &AccelerationStructureBuildGeometryInfoKHR,
        p_max_primitive_counts: Option<impl AsSlice<'a, u32>>,
    ) -> S {
        unsafe {
            raw::get_acceleration_structure_build_sizes_khr(
                self,
                build_type,
                p_build_info,
                p_max_primitive_counts,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesKHR.html>"]
    #[doc(alias = "vkCreateRayTracingPipelinesKHR")]
    pub fn create_ray_tracing_pipelines_khr<
        'a,
        R: AdvancedDynamicArray<Pipeline, raw::Pipeline>,
    >(
        &self,
        deferred_operation: Option<&raw::DeferredOperationKHR>,
        pipeline_cache: Option<&raw::PipelineCache>,
        p_create_infos: impl AsSlice<'a, RayTracingPipelineCreateInfoKHR<'a>>,
    ) -> Result<(Status, R)> {
        let vk_result: Result<(Status, R::InnerArrayType)> = unsafe {
            raw::create_ray_tracing_pipelines_khr(
                self,
                deferred_operation,
                pipeline_cache,
                p_create_infos,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|(status, vk_result)| {
            (
                status,
                vk_result
                    .into_iter()
                    .map(|el| unsafe { Pipeline::from_inner(el) })
                    .collect(),
            )
        })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html>"]
    #[doc(alias = "vkGetRayTracingShaderGroupHandlesKHR")]
    pub fn get_ray_tracing_shader_group_handles_khr(
        &self,
        pipeline: &raw::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: VoidPtr,
    ) -> Result<()> {
        unsafe {
            raw::get_ray_tracing_shader_group_handles_khr(
                self,
                pipeline,
                first_group,
                group_count,
                data_size,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesNV.html>"]
    #[doc(alias = "vkGetRayTracingShaderGroupHandlesNV")]
    pub fn get_ray_tracing_shader_group_handles_nv(
        &self,
        pipeline: &raw::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: VoidPtr,
    ) -> Result<()> {
        unsafe {
            raw::get_ray_tracing_shader_group_handles_nv(
                self,
                pipeline,
                first_group,
                group_count,
                data_size,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html>"]
    #[doc(alias = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")]
    pub fn get_ray_tracing_capture_replay_shader_group_handles_khr(
        &self,
        pipeline: &raw::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: VoidPtr,
    ) -> Result<()> {
        unsafe {
            raw::get_ray_tracing_capture_replay_shader_group_handles_khr(
                self,
                pipeline,
                first_group,
                group_count,
                data_size,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html>"]
    #[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
    pub fn get_ray_tracing_shader_group_stack_size_khr(
        &self,
        pipeline: &raw::Pipeline,
        group: u32,
        group_shader: ShaderGroupShaderKHR,
    ) -> DeviceSize {
        unsafe {
            raw::get_ray_tracing_shader_group_stack_size_khr(
                self,
                pipeline,
                group,
                group_shader,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html>"]
    #[doc(alias = "vkGetImageDrmFormatModifierPropertiesEXT")]
    pub fn get_image_drm_format_modifier_properties_ext<
        S: StructureChainOut<ImageDrmFormatModifierPropertiesEXT<'static>>,
    >(
        &self,
        image: &raw::Image,
    ) -> Result<S> {
        unsafe {
            raw::get_image_drm_format_modifier_properties_ext(
                self,
                image,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateValidationCacheEXT.html>"]
    #[doc(alias = "vkCreateValidationCacheEXT")]
    pub fn create_validation_cache_ext(
        &self,
        p_create_info: &ValidationCacheCreateInfoEXT,
    ) -> Result<ValidationCacheEXT> {
        let vk_result = unsafe {
            raw::create_validation_cache_ext(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { ValidationCacheEXT::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyValidationCacheEXT.html>"]
    #[doc(alias = "vkDestroyValidationCacheEXT")]
    pub unsafe fn destroy_validation_cache_ext(
        &self,
        validation_cache: Option<&raw::ValidationCacheEXT>,
    ) {
        unsafe {
            raw::destroy_validation_cache_ext(
                self,
                validation_cache,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMergeValidationCachesEXT.html>"]
    #[doc(alias = "vkMergeValidationCachesEXT")]
    pub fn merge_validation_caches_ext<'a, V3: Alias<raw::ValidationCacheEXT> + 'a>(
        &self,
        dst_cache: &raw::ValidationCacheEXT,
        p_src_caches: impl AsSlice<'a, V3>,
    ) -> Result<()> {
        unsafe {
            raw::merge_validation_caches_ext(
                self,
                dst_cache,
                p_src_caches,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetValidationCacheDataEXT.html>"]
    #[doc(alias = "vkGetValidationCacheDataEXT")]
    pub fn get_validation_cache_data_ext(
        &self,
        validation_cache: &raw::ValidationCacheEXT,
        p_data: VoidPtr,
    ) -> Result<usize> {
        unsafe {
            raw::get_validation_cache_data_ext(
                self,
                validation_cache,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureNV.html>"]
    #[doc(alias = "vkCreateAccelerationStructureNV")]
    pub fn create_acceleration_structure_nv(
        &self,
        p_create_info: &AccelerationStructureCreateInfoNV,
    ) -> Result<AccelerationStructureNV> {
        let vk_result = unsafe {
            raw::create_acceleration_structure_nv(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { AccelerationStructureNV::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureNV.html>"]
    #[doc(alias = "vkDestroyAccelerationStructureNV")]
    pub unsafe fn destroy_acceleration_structure_nv(
        &self,
        acceleration_structure: Option<&raw::AccelerationStructureNV>,
    ) {
        unsafe {
            raw::destroy_acceleration_structure_nv(
                self,
                acceleration_structure,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html>"]
    #[doc(alias = "vkGetAccelerationStructureMemoryRequirementsNV")]
    pub fn get_acceleration_structure_memory_requirements_nv<
        S: StructureChainOut<MemoryRequirements2KHR<'static>>,
    >(
        &self,
        p_info: &AccelerationStructureMemoryRequirementsInfoNV,
    ) -> S {
        unsafe {
            raw::get_acceleration_structure_memory_requirements_nv(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindAccelerationStructureMemoryNV.html>"]
    #[doc(alias = "vkBindAccelerationStructureMemoryNV")]
    pub fn bind_acceleration_structure_memory_nv<'a>(
        &self,
        p_bind_infos: impl AsSlice<'a, BindAccelerationStructureMemoryInfoNV<'a>>,
    ) -> Result<()> {
        unsafe {
            raw::bind_acceleration_structure_memory_nv(
                self,
                p_bind_infos,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesNV.html>"]
    #[doc(alias = "vkCreateRayTracingPipelinesNV")]
    pub fn create_ray_tracing_pipelines_nv<'a, R: AdvancedDynamicArray<Pipeline, raw::Pipeline>>(
        &self,
        pipeline_cache: Option<&raw::PipelineCache>,
        p_create_infos: impl AsSlice<'a, RayTracingPipelineCreateInfoNV<'a>>,
    ) -> Result<(Status, R)> {
        let vk_result: Result<(Status, R::InnerArrayType)> = unsafe {
            raw::create_ray_tracing_pipelines_nv(
                self,
                pipeline_cache,
                p_create_infos,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|(status, vk_result)| {
            (
                status,
                vk_result
                    .into_iter()
                    .map(|el| unsafe { Pipeline::from_inner(el) })
                    .collect(),
            )
        })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureHandleNV.html>"]
    #[doc(alias = "vkGetAccelerationStructureHandleNV")]
    pub fn get_acceleration_structure_handle_nv(
        &self,
        acceleration_structure: &raw::AccelerationStructureNV,
        data_size: usize,
        p_data: VoidPtr,
    ) -> Result<()> {
        unsafe {
            raw::get_acceleration_structure_handle_nv(
                self,
                acceleration_structure,
                data_size,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCompileDeferredNV.html>"]
    #[doc(alias = "vkCompileDeferredNV")]
    pub fn compile_deferred_nv(&self, pipeline: &raw::Pipeline, shader: u32) -> Result<()> {
        unsafe {
            raw::compile_deferred_nv(self, pipeline, shader, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html>"]
    #[doc(alias = "vkGetMemoryHostPointerPropertiesEXT")]
    pub fn get_memory_host_pointer_properties_ext<
        S: StructureChainOut<MemoryHostPointerPropertiesEXT<'static>>,
    >(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        p_host_pointer: VoidPtr,
    ) -> Result<S> {
        unsafe {
            raw::get_memory_host_pointer_properties_ext(
                self,
                handle_type,
                p_host_pointer,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkInitializePerformanceApiINTEL.html>"]
    #[doc(alias = "vkInitializePerformanceApiINTEL")]
    pub fn initialize_performance_api_intel(
        &self,
        p_initialize_info: &InitializePerformanceApiInfoINTEL,
    ) -> Result<()> {
        unsafe {
            raw::initialize_performance_api_intel(
                self,
                p_initialize_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUninitializePerformanceApiINTEL.html>"]
    #[doc(alias = "vkUninitializePerformanceApiINTEL")]
    pub fn uninitialize_performance_api_intel(&self) {
        unsafe { raw::uninitialize_performance_api_intel(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html>"]
    #[doc(alias = "vkAcquirePerformanceConfigurationINTEL")]
    pub fn acquire_performance_configuration_intel(
        &self,
        p_acquire_info: &PerformanceConfigurationAcquireInfoINTEL,
    ) -> Result<PerformanceConfigurationINTEL> {
        let vk_result = unsafe {
            raw::acquire_performance_configuration_intel(
                self,
                p_acquire_info,
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { PerformanceConfigurationINTEL::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html>"]
    #[doc(alias = "vkReleasePerformanceConfigurationINTEL")]
    pub fn release_performance_configuration_intel(
        &self,
        configuration: Option<&raw::PerformanceConfigurationINTEL>,
    ) -> Result<()> {
        unsafe {
            raw::release_performance_configuration_intel(
                self,
                configuration,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPerformanceParameterINTEL.html>"]
    #[doc(alias = "vkGetPerformanceParameterINTEL")]
    pub fn get_performance_parameter_intel(
        &self,
        parameter: PerformanceParameterTypeINTEL,
    ) -> Result<PerformanceValueINTEL> {
        unsafe {
            raw::get_performance_parameter_intel(
                self,
                parameter,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetLocalDimmingAMD.html>"]
    #[doc(alias = "vkSetLocalDimmingAMD")]
    pub fn set_local_dimming_amd(
        &self,
        swap_chain: &raw::SwapchainKHR,
        local_dimming_enable: impl Into<Bool32>,
    ) {
        unsafe {
            raw::set_local_dimming_amd(
                self,
                swap_chain,
                local_dimming_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitForPresentKHR.html>"]
    #[doc(alias = "vkWaitForPresentKHR")]
    pub fn wait_for_present_khr(
        &self,
        swapchain: &raw::SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> Result<Status> {
        unsafe {
            raw::wait_for_present_khr(
                self,
                swapchain,
                present_id,
                timeout,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html>"]
    #[doc(alias = "vkAcquireFullScreenExclusiveModeEXT")]
    pub fn acquire_full_screen_exclusive_mode_ext(
        &self,
        swapchain: &raw::SwapchainKHR,
    ) -> Result<()> {
        unsafe {
            raw::acquire_full_screen_exclusive_mode_ext(
                self,
                swapchain,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html>"]
    #[doc(alias = "vkReleaseFullScreenExclusiveModeEXT")]
    pub fn release_full_screen_exclusive_mode_ext(
        &self,
        swapchain: &raw::SwapchainKHR,
    ) -> Result<()> {
        unsafe {
            raw::release_full_screen_exclusive_mode_ext(
                self,
                swapchain,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html>"]
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModes2EXT")]
    pub fn get_group_surface_present_modes2_ext(
        &self,
        p_surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<DeviceGroupPresentModeFlagsKHR> {
        unsafe {
            raw::get_device_group_surface_present_modes2_ext(
                self,
                p_surface_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDeferredOperationKHR.html>"]
    #[doc(alias = "vkCreateDeferredOperationKHR")]
    pub fn create_deferred_operation_khr(&self) -> Result<DeferredOperationKHR> {
        let vk_result = unsafe {
            raw::create_deferred_operation_khr(
                self,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { DeferredOperationKHR::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDeferredOperationKHR.html>"]
    #[doc(alias = "vkDestroyDeferredOperationKHR")]
    pub unsafe fn destroy_deferred_operation_khr(
        &self,
        operation: Option<&raw::DeferredOperationKHR>,
    ) {
        unsafe {
            raw::destroy_deferred_operation_khr(
                self,
                operation,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html>"]
    #[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
    pub fn get_deferred_operation_max_concurrency_khr(
        &self,
        operation: &raw::DeferredOperationKHR,
    ) -> u32 {
        unsafe {
            raw::get_deferred_operation_max_concurrency_khr(
                self,
                operation,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationResultKHR.html>"]
    #[doc(alias = "vkGetDeferredOperationResultKHR")]
    pub fn get_deferred_operation_result_khr(
        &self,
        operation: &raw::DeferredOperationKHR,
    ) -> Result<Status> {
        unsafe {
            raw::get_deferred_operation_result_khr(
                self,
                operation,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeferredOperationJoinKHR.html>"]
    #[doc(alias = "vkDeferredOperationJoinKHR")]
    pub fn deferred_operation_join_khr(
        &self,
        operation: &raw::DeferredOperationKHR,
    ) -> Result<Status> {
        unsafe {
            raw::deferred_operation_join_khr(self, operation, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html>"]
    #[doc(alias = "vkGetPipelineExecutablePropertiesKHR")]
    pub fn get_pipeline_executable_properties_khr<
        R: DynamicArray<PipelineExecutablePropertiesKHR<'static>>,
    >(
        &self,
        p_pipeline_info: &PipelineInfoKHR,
    ) -> Result<R> {
        unsafe {
            raw::get_pipeline_executable_properties_khr(
                self,
                p_pipeline_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html>"]
    #[doc(alias = "vkGetPipelineExecutableStatisticsKHR")]
    pub fn get_pipeline_executable_statistics_khr<
        R: DynamicArray<PipelineExecutableStatisticKHR<'static>>,
    >(
        &self,
        p_executable_info: &PipelineExecutableInfoKHR,
    ) -> Result<R> {
        unsafe {
            raw::get_pipeline_executable_statistics_khr(
                self,
                p_executable_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html>"]
    #[doc(alias = "vkGetPipelineExecutableInternalRepresentationsKHR")]
    pub fn get_pipeline_executable_internal_representations_khr<
        R: DynamicArray<PipelineExecutableInternalRepresentationKHR<'static>>,
    >(
        &self,
        p_executable_info: &PipelineExecutableInfoKHR,
    ) -> Result<R> {
        unsafe {
            raw::get_pipeline_executable_internal_representations_khr(
                self,
                p_executable_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToImageEXT.html>"]
    #[doc(alias = "vkCopyMemoryToImageEXT")]
    pub fn copy_memory_to_image_ext(
        &self,
        p_copy_memory_to_image_info: &CopyMemoryToImageInfoEXT,
    ) -> Result<()> {
        unsafe {
            raw::copy_memory_to_image_ext(
                self,
                p_copy_memory_to_image_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyImageToMemoryEXT.html>"]
    #[doc(alias = "vkCopyImageToMemoryEXT")]
    pub fn copy_image_to_memory_ext(
        &self,
        p_copy_image_to_memory_info: &CopyImageToMemoryInfoEXT,
    ) -> Result<()> {
        unsafe {
            raw::copy_image_to_memory_ext(
                self,
                p_copy_image_to_memory_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyImageToImageEXT.html>"]
    #[doc(alias = "vkCopyImageToImageEXT")]
    pub fn copy_image_to_image_ext(
        &self,
        p_copy_image_to_image_info: &CopyImageToImageInfoEXT,
    ) -> Result<()> {
        unsafe {
            raw::copy_image_to_image_ext(
                self,
                p_copy_image_to_image_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkTransitionImageLayoutEXT.html>"]
    #[doc(alias = "vkTransitionImageLayoutEXT")]
    pub fn transition_image_layout_ext<'a>(
        &self,
        p_transitions: impl AsSlice<'a, HostImageLayoutTransitionInfoEXT<'a>>,
    ) -> Result<()> {
        unsafe {
            raw::transition_image_layout_ext(
                self,
                p_transitions,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMapMemory2KHR.html>"]
    #[doc(alias = "vkMapMemory2KHR")]
    pub fn map_memory2_khr(&self, p_memory_map_info: &MemoryMapInfoKHR) -> Result<VoidPtr> {
        unsafe { raw::map_memory2_khr(self, p_memory_map_info, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory2KHR.html>"]
    #[doc(alias = "vkUnmapMemory2KHR")]
    pub fn unmap_memory2_khr(&self, p_memory_unmap_info: &MemoryUnmapInfoKHR) -> Result<()> {
        unsafe {
            raw::unmap_memory2_khr(
                self,
                p_memory_unmap_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseSwapchainImagesEXT.html>"]
    #[doc(alias = "vkReleaseSwapchainImagesEXT")]
    pub fn release_swapchain_images_ext(
        &self,
        p_release_info: &ReleaseSwapchainImagesInfoEXT,
    ) -> Result<()> {
        unsafe {
            raw::release_swapchain_images_ext(
                self,
                p_release_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html>"]
    #[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsNV")]
    pub fn get_generated_commands_memory_requirements_nv<
        S: StructureChainOut<MemoryRequirements2<'static>>,
    >(
        &self,
        p_info: &GeneratedCommandsMemoryRequirementsInfoNV,
    ) -> S {
        unsafe {
            raw::get_generated_commands_memory_requirements_nv(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateIndirectCommandsLayoutNV.html>"]
    #[doc(alias = "vkCreateIndirectCommandsLayoutNV")]
    pub fn create_indirect_commands_layout_nv(
        &self,
        p_create_info: &IndirectCommandsLayoutCreateInfoNV,
    ) -> Result<IndirectCommandsLayoutNV> {
        let vk_result = unsafe {
            raw::create_indirect_commands_layout_nv(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { IndirectCommandsLayoutNV::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyIndirectCommandsLayoutNV.html>"]
    #[doc(alias = "vkDestroyIndirectCommandsLayoutNV")]
    pub unsafe fn destroy_indirect_commands_layout_nv(
        &self,
        indirect_commands_layout: Option<&raw::IndirectCommandsLayoutNV>,
    ) {
        unsafe {
            raw::destroy_indirect_commands_layout_nv(
                self,
                indirect_commands_layout,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCudaModuleNV.html>"]
    #[doc(alias = "vkCreateCudaModuleNV")]
    pub fn create_cuda_module_nv(
        &self,
        p_create_info: &CudaModuleCreateInfoNV,
    ) -> Result<CudaModuleNV> {
        let vk_result = unsafe {
            raw::create_cuda_module_nv(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { CudaModuleNV::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetCudaModuleCacheNV.html>"]
    #[doc(alias = "vkGetCudaModuleCacheNV")]
    pub fn get_cuda_module_cache_nv(
        &self,
        module: &raw::CudaModuleNV,
        p_cache_data: VoidPtr,
    ) -> Result<usize> {
        unsafe {
            raw::get_cuda_module_cache_nv(
                self,
                module,
                p_cache_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCudaFunctionNV.html>"]
    #[doc(alias = "vkCreateCudaFunctionNV")]
    pub fn create_cuda_function_nv(
        &self,
        p_create_info: &CudaFunctionCreateInfoNV,
    ) -> Result<CudaFunctionNV> {
        let vk_result = unsafe {
            raw::create_cuda_function_nv(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { CudaFunctionNV::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCudaModuleNV.html>"]
    #[doc(alias = "vkDestroyCudaModuleNV")]
    pub unsafe fn destroy_cuda_module_nv(&self, module: &raw::CudaModuleNV) {
        unsafe {
            raw::destroy_cuda_module_nv(
                self,
                module,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCudaFunctionNV.html>"]
    #[doc(alias = "vkDestroyCudaFunctionNV")]
    pub unsafe fn destroy_cuda_function_nv(&self, function: &raw::CudaFunctionNV) {
        unsafe {
            raw::destroy_cuda_function_nv(
                self,
                function,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkExportMetalObjectsEXT.html>"]
    #[doc(alias = "vkExportMetalObjectsEXT")]
    pub fn export_metal_objects_ext<S: StructureChainOut<ExportMetalObjectsInfoEXT<'static>>>(
        &self,
    ) -> S {
        unsafe { raw::export_metal_objects_ext(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSizeEXT.html>"]
    #[doc(alias = "vkGetDescriptorSetLayoutSizeEXT")]
    pub fn get_descriptor_set_layout_size_ext(
        &self,
        layout: &raw::DescriptorSetLayout,
    ) -> DeviceSize {
        unsafe {
            raw::get_descriptor_set_layout_size_ext(
                self,
                layout,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutBindingOffsetEXT.html>"]
    #[doc(alias = "vkGetDescriptorSetLayoutBindingOffsetEXT")]
    pub fn get_descriptor_set_layout_binding_offset_ext(
        &self,
        layout: &raw::DescriptorSetLayout,
        binding: u32,
    ) -> DeviceSize {
        unsafe {
            raw::get_descriptor_set_layout_binding_offset_ext(
                self,
                layout,
                binding,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorEXT.html>"]
    #[doc(alias = "vkGetDescriptorEXT")]
    pub fn get_descriptor_ext(
        &self,
        p_descriptor_info: &DescriptorGetInfoEXT,
        data_size: usize,
        p_descriptor: VoidPtr,
    ) {
        unsafe {
            raw::get_descriptor_ext(
                self,
                p_descriptor_info,
                data_size,
                p_descriptor,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureDescriptorDataEXT.html>"]
    #[doc(alias = "vkGetBufferOpaqueCaptureDescriptorDataEXT")]
    pub fn get_buffer_opaque_capture_descriptor_data_ext(
        &self,
        p_info: &BufferCaptureDescriptorDataInfoEXT,
        p_data: VoidPtr,
    ) -> Result<()> {
        unsafe {
            raw::get_buffer_opaque_capture_descriptor_data_ext(
                self,
                p_info,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageOpaqueCaptureDescriptorDataEXT.html>"]
    #[doc(alias = "vkGetImageOpaqueCaptureDescriptorDataEXT")]
    pub fn get_image_opaque_capture_descriptor_data_ext(
        &self,
        p_info: &ImageCaptureDescriptorDataInfoEXT,
        p_data: VoidPtr,
    ) -> Result<()> {
        unsafe {
            raw::get_image_opaque_capture_descriptor_data_ext(
                self,
                p_info,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html>"]
    #[doc(alias = "vkGetImageViewOpaqueCaptureDescriptorDataEXT")]
    pub fn get_image_view_opaque_capture_descriptor_data_ext(
        &self,
        p_info: &ImageViewCaptureDescriptorDataInfoEXT,
        p_data: VoidPtr,
    ) -> Result<()> {
        unsafe {
            raw::get_image_view_opaque_capture_descriptor_data_ext(
                self,
                p_info,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html>"]
    #[doc(alias = "vkGetSamplerOpaqueCaptureDescriptorDataEXT")]
    pub fn get_sampler_opaque_capture_descriptor_data_ext(
        &self,
        p_info: &SamplerCaptureDescriptorDataInfoEXT,
        p_data: VoidPtr,
    ) -> Result<()> {
        unsafe {
            raw::get_sampler_opaque_capture_descriptor_data_ext(
                self,
                p_info,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html>"]
    #[doc(alias = "vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT")]
    pub fn get_acceleration_structure_opaque_capture_descriptor_data_ext(
        &self,
        p_info: &AccelerationStructureCaptureDescriptorDataInfoEXT,
        p_data: VoidPtr,
    ) -> Result<()> {
        unsafe {
            raw::get_acceleration_structure_opaque_capture_descriptor_data_ext(
                self,
                p_info,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandleFUCHSIA.html>"]
    #[doc(alias = "vkGetMemoryZirconHandleFUCHSIA")]
    pub fn get_memory_zircon_handle_fuchsia(
        &self,
        p_get_zircon_handle_info: &MemoryGetZirconHandleInfoFUCHSIA,
    ) -> Result<VoidPtr> {
        unsafe {
            raw::get_memory_zircon_handle_fuchsia(
                self,
                p_get_zircon_handle_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandlePropertiesFUCHSIA.html>"]
    #[doc(alias = "vkGetMemoryZirconHandlePropertiesFUCHSIA")]
    pub fn get_memory_zircon_handle_properties_fuchsia<
        S: StructureChainOut<MemoryZirconHandlePropertiesFUCHSIA<'static>>,
    >(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        zircon_handle: VoidPtr,
    ) -> Result<S> {
        unsafe {
            raw::get_memory_zircon_handle_properties_fuchsia(
                self,
                handle_type,
                zircon_handle,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreZirconHandleFUCHSIA.html>"]
    #[doc(alias = "vkImportSemaphoreZirconHandleFUCHSIA")]
    pub fn import_semaphore_zircon_handle_fuchsia(
        &self,
        p_import_semaphore_zircon_handle_info: &ImportSemaphoreZirconHandleInfoFUCHSIA,
    ) -> Result<()> {
        unsafe {
            raw::import_semaphore_zircon_handle_fuchsia(
                self,
                p_import_semaphore_zircon_handle_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreZirconHandleFUCHSIA.html>"]
    #[doc(alias = "vkGetSemaphoreZirconHandleFUCHSIA")]
    pub fn get_semaphore_zircon_handle_fuchsia(
        &self,
        p_get_zircon_handle_info: &SemaphoreGetZirconHandleInfoFUCHSIA,
    ) -> Result<VoidPtr> {
        unsafe {
            raw::get_semaphore_zircon_handle_fuchsia(
                self,
                p_get_zircon_handle_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBufferCollectionFUCHSIA.html>"]
    #[doc(alias = "vkCreateBufferCollectionFUCHSIA")]
    pub fn create_buffer_collection_fuchsia(
        &self,
        p_create_info: &BufferCollectionCreateInfoFUCHSIA,
    ) -> Result<BufferCollectionFUCHSIA> {
        let vk_result = unsafe {
            raw::create_buffer_collection_fuchsia(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { BufferCollectionFUCHSIA::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionImageConstraintsFUCHSIA.html>"]
    #[doc(alias = "vkSetBufferCollectionImageConstraintsFUCHSIA")]
    pub fn set_buffer_collection_image_constraints_fuchsia(
        &self,
        collection: &raw::BufferCollectionFUCHSIA,
        p_image_constraints_info: &ImageConstraintsInfoFUCHSIA,
    ) -> Result<()> {
        unsafe {
            raw::set_buffer_collection_image_constraints_fuchsia(
                self,
                collection,
                p_image_constraints_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionBufferConstraintsFUCHSIA.html>"]
    #[doc(alias = "vkSetBufferCollectionBufferConstraintsFUCHSIA")]
    pub fn set_buffer_collection_buffer_constraints_fuchsia(
        &self,
        collection: &raw::BufferCollectionFUCHSIA,
        p_buffer_constraints_info: &BufferConstraintsInfoFUCHSIA,
    ) -> Result<()> {
        unsafe {
            raw::set_buffer_collection_buffer_constraints_fuchsia(
                self,
                collection,
                p_buffer_constraints_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferCollectionFUCHSIA.html>"]
    #[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
    pub unsafe fn destroy_buffer_collection_fuchsia(
        &self,
        collection: &raw::BufferCollectionFUCHSIA,
    ) {
        unsafe {
            raw::destroy_buffer_collection_fuchsia(
                self,
                collection,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferCollectionPropertiesFUCHSIA.html>"]
    #[doc(alias = "vkGetBufferCollectionPropertiesFUCHSIA")]
    pub fn get_buffer_collection_properties_fuchsia<
        S: StructureChainOut<BufferCollectionPropertiesFUCHSIA<'static>>,
    >(
        &self,
        collection: &raw::BufferCollectionFUCHSIA,
    ) -> Result<S> {
        unsafe {
            raw::get_buffer_collection_properties_fuchsia(
                self,
                collection,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html>"]
    #[doc(alias = "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")]
    pub fn get_subpass_shading_max_workgroup_size_huawei<R: DynamicArray<Extent2D>>(
        &self,
        renderpass: &raw::RenderPass,
    ) -> Result<R> {
        unsafe {
            raw::get_device_subpass_shading_max_workgroup_size_huawei(
                self,
                renderpass,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryRemoteAddressNV.html>"]
    #[doc(alias = "vkGetMemoryRemoteAddressNV")]
    pub fn get_memory_remote_address_nv(
        &self,
        p_memory_get_remote_address_info: &MemoryGetRemoteAddressInfoNV,
    ) -> Result<RemoteAddressNV> {
        unsafe {
            raw::get_memory_remote_address_nv(
                self,
                p_memory_get_remote_address_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelinePropertiesEXT.html>"]
    #[doc(alias = "vkGetPipelinePropertiesEXT")]
    pub fn get_pipeline_properties_ext(
        &self,
        p_pipeline_info: &PipelineInfoEXT,
    ) -> Result<BaseOutStructure<'static>> {
        unsafe {
            raw::get_pipeline_properties_ext(
                self,
                p_pipeline_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMicromapEXT.html>"]
    #[doc(alias = "vkCreateMicromapEXT")]
    pub fn create_micromap_ext(
        &self,
        p_create_info: &MicromapCreateInfoEXT,
    ) -> Result<MicromapEXT> {
        let vk_result = unsafe {
            raw::create_micromap_ext(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { MicromapEXT::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyMicromapEXT.html>"]
    #[doc(alias = "vkDestroyMicromapEXT")]
    pub unsafe fn destroy_micromap_ext(&self, micromap: Option<&raw::MicromapEXT>) {
        unsafe {
            raw::destroy_micromap_ext(
                self,
                micromap,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBuildMicromapsEXT.html>"]
    #[doc(alias = "vkBuildMicromapsEXT")]
    pub fn build_micromaps_ext<'a>(
        &self,
        deferred_operation: Option<&raw::DeferredOperationKHR>,
        p_infos: impl AsSlice<'a, MicromapBuildInfoEXT<'a>>,
    ) -> Result<Status> {
        unsafe {
            raw::build_micromaps_ext(
                self,
                deferred_operation,
                p_infos,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMicromapEXT.html>"]
    #[doc(alias = "vkCopyMicromapEXT")]
    pub fn copy_micromap_ext(
        &self,
        deferred_operation: Option<&raw::DeferredOperationKHR>,
        p_info: &CopyMicromapInfoEXT,
    ) -> Result<Status> {
        unsafe {
            raw::copy_micromap_ext(
                self,
                deferred_operation,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMicromapToMemoryEXT.html>"]
    #[doc(alias = "vkCopyMicromapToMemoryEXT")]
    pub fn copy_micromap_to_memory_ext(
        &self,
        deferred_operation: Option<&raw::DeferredOperationKHR>,
        p_info: &CopyMicromapToMemoryInfoEXT,
    ) -> Result<Status> {
        unsafe {
            raw::copy_micromap_to_memory_ext(
                self,
                deferred_operation,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToMicromapEXT.html>"]
    #[doc(alias = "vkCopyMemoryToMicromapEXT")]
    pub fn copy_memory_to_micromap_ext(
        &self,
        deferred_operation: Option<&raw::DeferredOperationKHR>,
        p_info: &CopyMemoryToMicromapInfoEXT,
    ) -> Result<Status> {
        unsafe {
            raw::copy_memory_to_micromap_ext(
                self,
                deferred_operation,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWriteMicromapsPropertiesEXT.html>"]
    #[doc(alias = "vkWriteMicromapsPropertiesEXT")]
    pub fn write_micromaps_properties_ext<'a, V2: Alias<raw::MicromapEXT> + 'a>(
        &self,
        p_micromaps: impl AsSlice<'a, V2>,
        query_type: QueryType,
        data_size: usize,
        p_data: VoidPtr,
        stride: usize,
    ) -> Result<()> {
        unsafe {
            raw::write_micromaps_properties_ext(
                self,
                p_micromaps,
                query_type,
                data_size,
                p_data,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMicromapCompatibilityEXT.html>"]
    #[doc(alias = "vkGetDeviceMicromapCompatibilityEXT")]
    pub fn get_micromap_compatibility_ext(
        &self,
        p_version_info: &MicromapVersionInfoEXT,
    ) -> AccelerationStructureCompatibilityKHR {
        unsafe {
            raw::get_device_micromap_compatibility_ext(
                self,
                p_version_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMicromapBuildSizesEXT.html>"]
    #[doc(alias = "vkGetMicromapBuildSizesEXT")]
    pub fn get_micromap_build_sizes_ext<
        S: StructureChainOut<MicromapBuildSizesInfoEXT<'static>>,
    >(
        &self,
        build_type: AccelerationStructureBuildTypeKHR,
        p_build_info: &MicromapBuildInfoEXT,
    ) -> S {
        unsafe {
            raw::get_micromap_build_sizes_ext(
                self,
                build_type,
                p_build_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDeviceMemoryPriorityEXT.html>"]
    #[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
    pub fn set_memory_priority_ext(&self, memory: &raw::DeviceMemory, priority: f32) {
        unsafe {
            raw::set_device_memory_priority_ext(
                self,
                memory,
                priority,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html>"]
    #[doc(alias = "vkGetDescriptorSetLayoutHostMappingInfoVALVE")]
    pub fn get_descriptor_set_layout_host_mapping_info_valve<
        S: StructureChainOut<DescriptorSetLayoutHostMappingInfoVALVE<'static>>,
    >(
        &self,
        p_binding_reference: &DescriptorSetBindingReferenceVALVE,
    ) -> S {
        unsafe {
            raw::get_descriptor_set_layout_host_mapping_info_valve(
                self,
                p_binding_reference,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetHostMappingVALVE.html>"]
    #[doc(alias = "vkGetDescriptorSetHostMappingVALVE")]
    pub fn get_descriptor_set_host_mapping_valve(
        &self,
        descriptor_set: &raw::DescriptorSet,
    ) -> VoidPtr {
        unsafe {
            raw::get_descriptor_set_host_mapping_valve(
                self,
                descriptor_set,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineIndirectMemoryRequirementsNV.html>"]
    #[doc(alias = "vkGetPipelineIndirectMemoryRequirementsNV")]
    pub fn get_pipeline_indirect_memory_requirements_nv<
        S: StructureChainOut<MemoryRequirements2<'static>>,
    >(
        &self,
        p_create_info: &ComputePipelineCreateInfo,
    ) -> S {
        unsafe {
            raw::get_pipeline_indirect_memory_requirements_nv(
                self,
                p_create_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineIndirectDeviceAddressNV.html>"]
    #[doc(alias = "vkGetPipelineIndirectDeviceAddressNV")]
    pub fn get_pipeline_indirect_address_nv(
        &self,
        p_info: &PipelineIndirectDeviceAddressInfoNV,
    ) -> DeviceAddress {
        unsafe {
            raw::get_pipeline_indirect_device_address_nv(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderModuleIdentifierEXT.html>"]
    #[doc(alias = "vkGetShaderModuleIdentifierEXT")]
    pub fn get_shader_module_identifier_ext<
        S: StructureChainOut<ShaderModuleIdentifierEXT<'static>>,
    >(
        &self,
        shader_module: &raw::ShaderModule,
    ) -> S {
        unsafe {
            raw::get_shader_module_identifier_ext(
                self,
                shader_module,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderModuleCreateInfoIdentifierEXT.html>"]
    #[doc(alias = "vkGetShaderModuleCreateInfoIdentifierEXT")]
    pub fn get_shader_module_create_info_identifier_ext<
        S: StructureChainOut<ShaderModuleIdentifierEXT<'static>>,
    >(
        &self,
        p_create_info: &ShaderModuleCreateInfo,
    ) -> S {
        unsafe {
            raw::get_shader_module_create_info_identifier_ext(
                self,
                p_create_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateOpticalFlowSessionNV.html>"]
    #[doc(alias = "vkCreateOpticalFlowSessionNV")]
    pub fn create_optical_flow_session_nv(
        &self,
        p_create_info: &OpticalFlowSessionCreateInfoNV,
    ) -> Result<OpticalFlowSessionNV> {
        let vk_result = unsafe {
            raw::create_optical_flow_session_nv(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { OpticalFlowSessionNV::from_inner(vk_result) })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyOpticalFlowSessionNV.html>"]
    #[doc(alias = "vkDestroyOpticalFlowSessionNV")]
    pub unsafe fn destroy_optical_flow_session_nv(&self, session: &raw::OpticalFlowSessionNV) {
        unsafe {
            raw::destroy_optical_flow_session_nv(
                self,
                session,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindOpticalFlowSessionImageNV.html>"]
    #[doc(alias = "vkBindOpticalFlowSessionImageNV")]
    pub fn bind_optical_flow_session_image_nv(
        &self,
        session: &raw::OpticalFlowSessionNV,
        binding_point: OpticalFlowSessionBindingPointNV,
        view: Option<&raw::ImageView>,
        layout: ImageLayout,
    ) -> Result<()> {
        unsafe {
            raw::bind_optical_flow_session_image_nv(
                self,
                session,
                binding_point,
                view,
                layout,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRenderingAreaGranularityKHR.html>"]
    #[doc(alias = "vkGetRenderingAreaGranularityKHR")]
    pub fn get_rendering_area_granularity_khr(
        &self,
        p_rendering_area_info: &RenderingAreaInfoKHR,
    ) -> Extent2D {
        unsafe {
            raw::get_rendering_area_granularity_khr(
                self,
                p_rendering_area_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSubresourceLayoutKHR.html>"]
    #[doc(alias = "vkGetDeviceImageSubresourceLayoutKHR")]
    pub fn get_image_subresource_layout_khr<
        S: StructureChainOut<SubresourceLayout2KHR<'static>>,
    >(
        &self,
        p_info: &DeviceImageSubresourceInfoKHR,
    ) -> S {
        unsafe {
            raw::get_device_image_subresource_layout_khr(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout2KHR.html>"]
    #[doc(alias = "vkGetImageSubresourceLayout2KHR")]
    pub fn get_image_subresource_layout2_khr<
        S: StructureChainOut<SubresourceLayout2KHR<'static>>,
    >(
        &self,
        image: &raw::Image,
        p_subresource: &ImageSubresource2KHR,
    ) -> S {
        unsafe {
            raw::get_image_subresource_layout2_khr(
                self,
                image,
                p_subresource,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout2EXT.html>"]
    #[doc(alias = "vkGetImageSubresourceLayout2EXT")]
    pub fn get_image_subresource_layout2_ext<
        S: StructureChainOut<SubresourceLayout2KHR<'static>>,
    >(
        &self,
        image: &raw::Image,
        p_subresource: &ImageSubresource2KHR,
    ) -> S {
        unsafe {
            raw::get_image_subresource_layout2_ext(
                self,
                image,
                p_subresource,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateShadersEXT.html>"]
    #[doc(alias = "vkCreateShadersEXT")]
    pub fn create_shaders_ext<'a, R: AdvancedDynamicArray<ShaderEXT, raw::ShaderEXT>>(
        &self,
        p_create_infos: impl AsSlice<'a, ShaderCreateInfoEXT<'a>>,
    ) -> Result<(Status, R)> {
        let vk_result: Result<(Status, R::InnerArrayType)> = unsafe {
            raw::create_shaders_ext(
                self,
                p_create_infos,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|(status, vk_result)| {
            (
                status,
                vk_result
                    .into_iter()
                    .map(|el| unsafe { ShaderEXT::from_inner(el) })
                    .collect(),
            )
        })
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderEXT.html>"]
    #[doc(alias = "vkDestroyShaderEXT")]
    pub unsafe fn destroy_shader_ext(&self, shader: Option<&raw::ShaderEXT>) {
        unsafe {
            raw::destroy_shader_ext(
                self,
                shader,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderBinaryDataEXT.html>"]
    #[doc(alias = "vkGetShaderBinaryDataEXT")]
    pub fn get_shader_binary_data_ext(
        &self,
        shader: &raw::ShaderEXT,
        p_data: VoidPtr,
    ) -> Result<usize> {
        unsafe {
            raw::get_shader_binary_data_ext(
                self,
                shader,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFramebufferTilePropertiesQCOM.html>"]
    #[doc(alias = "vkGetFramebufferTilePropertiesQCOM")]
    pub fn get_framebuffer_tile_properties_qcom<R: DynamicArray<TilePropertiesQCOM<'static>>>(
        &self,
        framebuffer: &raw::Framebuffer,
    ) -> Result<R> {
        unsafe {
            raw::get_framebuffer_tile_properties_qcom(
                self,
                framebuffer,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDynamicRenderingTilePropertiesQCOM.html>"]
    #[doc(alias = "vkGetDynamicRenderingTilePropertiesQCOM")]
    pub fn get_dynamic_rendering_tile_properties_qcom<
        S: StructureChainOut<TilePropertiesQCOM<'static>>,
    >(
        &self,
        p_rendering_info: &RenderingInfo,
    ) -> Result<S> {
        unsafe {
            raw::get_dynamic_rendering_tile_properties_qcom(
                self,
                p_rendering_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetLatencySleepModeNV.html>"]
    #[doc(alias = "vkSetLatencySleepModeNV")]
    pub fn set_latency_sleep_mode_nv(
        &self,
        swapchain: &raw::SwapchainKHR,
        p_sleep_mode_info: &LatencySleepModeInfoNV,
    ) -> Result<()> {
        unsafe {
            raw::set_latency_sleep_mode_nv(
                self,
                swapchain,
                p_sleep_mode_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkLatencySleepNV.html>"]
    #[doc(alias = "vkLatencySleepNV")]
    pub fn latency_sleep_nv(
        &self,
        swapchain: &raw::SwapchainKHR,
        p_sleep_info: &LatencySleepInfoNV,
    ) -> Result<()> {
        unsafe {
            raw::latency_sleep_nv(
                self,
                swapchain,
                p_sleep_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetLatencyMarkerNV.html>"]
    #[doc(alias = "vkSetLatencyMarkerNV")]
    pub fn set_latency_marker_nv(
        &self,
        swapchain: &raw::SwapchainKHR,
        p_latency_marker_info: &SetLatencyMarkerInfoNV,
    ) {
        unsafe {
            raw::set_latency_marker_nv(
                self,
                swapchain,
                p_latency_marker_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetLatencyTimingsNV.html>"]
    #[doc(alias = "vkGetLatencyTimingsNV")]
    pub fn get_latency_timings_nv<S: StructureChainOut<GetLatencyMarkerInfoNV<'static>>>(
        &self,
        swapchain: &raw::SwapchainKHR,
    ) -> S {
        unsafe { raw::get_latency_timings_nv(self, swapchain, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetScreenBufferPropertiesQNX.html>"]
    #[doc(alias = "vkGetScreenBufferPropertiesQNX")]
    pub fn get_screen_buffer_properties_qnx<
        S: StructureChainOut<ScreenBufferPropertiesQNX<'static>>,
    >(
        &self,
        buffer: &VoidPtr,
    ) -> Result<S> {
        unsafe {
            raw::get_screen_buffer_properties_qnx(self, buffer, self.disp.get_command_dispatcher())
        }
    }
}
#[repr(C)]
#[derive(Clone)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueue.html>"]
#[doc(alias = "VkQueue")]
pub struct Queue<D: Dispatcher = DynamicDispatcher, A: Allocator = DefaultAllocator> {
    inner: <raw::Queue as Handle>::InnerType,
    disp: D,
    alloc: A,
}
unsafe impl Alias<raw::Queue> for Queue {}
impl<D: Dispatcher, A: Allocator> Copy for Queue<D, A>
where
    D: Copy,
    A: Copy,
{
}
impl<D: Dispatcher, A: Allocator> Deref for Queue<D, A> {
    type Target = raw::Queue;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl<D: Dispatcher, A: Allocator> Queue<D, A> {
    pub unsafe fn from_inner(handle: raw::Queue, disp: D, alloc: A) -> Self {
        Self {
            inner: handle.as_raw(),
            disp,
            alloc,
        }
    }
    pub fn get_dispatcher(&self) -> &D {
        &self.disp
    }
    pub fn get_allocator(&self) -> &A {
        &self.alloc
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit.html>"]
    #[doc(alias = "vkQueueSubmit")]
    pub fn submit<'a>(
        &self,
        p_submits: impl AsSlice<'a, SubmitInfo<'a>>,
        fence: Option<&raw::Fence>,
    ) -> Result<()> {
        unsafe { raw::queue_submit(self, p_submits, fence, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueWaitIdle.html>"]
    #[doc(alias = "vkQueueWaitIdle")]
    pub fn wait_idle(&self) -> Result<()> {
        unsafe { raw::queue_wait_idle(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueBindSparse.html>"]
    #[doc(alias = "vkQueueBindSparse")]
    pub fn bind_sparse<'a>(
        &self,
        p_bind_info: impl AsSlice<'a, BindSparseInfo<'a>>,
        fence: Option<&raw::Fence>,
    ) -> Result<()> {
        unsafe {
            raw::queue_bind_sparse(self, p_bind_info, fence, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2.html>"]
    #[doc(alias = "vkQueueSubmit2")]
    pub fn submit2<'a>(
        &self,
        p_submits: impl AsSlice<'a, SubmitInfo2<'a>>,
        fence: Option<&raw::Fence>,
    ) -> Result<()> {
        unsafe { raw::queue_submit2(self, p_submits, fence, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2KHR.html>"]
    #[doc(alias = "vkQueueSubmit2KHR")]
    pub fn submit2_khr<'a>(
        &self,
        p_submits: impl AsSlice<'a, SubmitInfo2<'a>>,
        fence: Option<&raw::Fence>,
    ) -> Result<()> {
        unsafe {
            raw::queue_submit2_khr(self, p_submits, fence, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html>"]
    #[doc(alias = "vkQueuePresentKHR")]
    pub fn present_khr(&self, p_present_info: &PresentInfoKHR) -> Result<Status> {
        unsafe { raw::queue_present_khr(self, p_present_info, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html>"]
    #[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
    pub fn begin_debug_utils_label_ext(&self, p_label_info: &DebugUtilsLabelEXT) {
        unsafe {
            raw::queue_begin_debug_utils_label_ext(
                self,
                p_label_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html>"]
    #[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
    pub fn end_debug_utils_label_ext(&self) {
        unsafe { raw::queue_end_debug_utils_label_ext(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html>"]
    #[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
    pub fn insert_debug_utils_label_ext(&self, p_label_info: &DebugUtilsLabelEXT) {
        unsafe {
            raw::queue_insert_debug_utils_label_ext(
                self,
                p_label_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointDataNV.html>"]
    #[doc(alias = "vkGetQueueCheckpointDataNV")]
    pub fn get_checkpoint_data_nv<R: DynamicArray<CheckpointDataNV<'static>>>(&self) -> R {
        unsafe { raw::get_queue_checkpoint_data_nv(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html>"]
    #[doc(alias = "vkQueueSetPerformanceConfigurationINTEL")]
    pub fn set_performance_configuration_intel(
        &self,
        configuration: &raw::PerformanceConfigurationINTEL,
    ) -> Result<()> {
        unsafe {
            raw::queue_set_performance_configuration_intel(
                self,
                configuration,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointData2NV.html>"]
    #[doc(alias = "vkGetQueueCheckpointData2NV")]
    pub fn get_checkpoint_data2_nv<R: DynamicArray<CheckpointData2NV<'static>>>(&self) -> R {
        unsafe { raw::get_queue_checkpoint_data2_nv(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueNotifyOutOfBandNV.html>"]
    #[doc(alias = "vkQueueNotifyOutOfBandNV")]
    pub fn notify_out_of_band_nv(&self, p_queue_type_info: &OutOfBandQueueTypeInfoNV) {
        unsafe {
            raw::queue_notify_out_of_band_nv(
                self,
                p_queue_type_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemory.html>"]
#[doc(alias = "VkDeviceMemory")]
pub struct DeviceMemory {
    inner: <raw::DeviceMemory as Handle>::InnerType,
}
unsafe impl Alias<raw::DeviceMemory> for DeviceMemory {}
impl Deref for DeviceMemory {
    type Target = raw::DeviceMemory;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl DeviceMemory {
    pub fn from_inner(handle: raw::DeviceMemory) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFence.html>"]
#[doc(alias = "VkFence")]
pub struct Fence {
    inner: <raw::Fence as Handle>::InnerType,
}
unsafe impl Alias<raw::Fence> for Fence {}
impl Deref for Fence {
    type Target = raw::Fence;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl Fence {
    pub fn from_inner(handle: raw::Fence) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphore.html>"]
#[doc(alias = "VkSemaphore")]
pub struct Semaphore {
    inner: <raw::Semaphore as Handle>::InnerType,
}
unsafe impl Alias<raw::Semaphore> for Semaphore {}
impl Deref for Semaphore {
    type Target = raw::Semaphore;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl Semaphore {
    pub fn from_inner(handle: raw::Semaphore) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkEvent.html>"]
#[doc(alias = "VkEvent")]
pub struct Event {
    inner: <raw::Event as Handle>::InnerType,
}
unsafe impl Alias<raw::Event> for Event {}
impl Deref for Event {
    type Target = raw::Event;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl Event {
    pub fn from_inner(handle: raw::Event) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPool.html>"]
#[doc(alias = "VkQueryPool")]
pub struct QueryPool {
    inner: <raw::QueryPool as Handle>::InnerType,
}
unsafe impl Alias<raw::QueryPool> for QueryPool {}
impl Deref for QueryPool {
    type Target = raw::QueryPool;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl QueryPool {
    pub fn from_inner(handle: raw::QueryPool) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuffer.html>"]
#[doc(alias = "VkBuffer")]
pub struct Buffer {
    inner: <raw::Buffer as Handle>::InnerType,
}
unsafe impl Alias<raw::Buffer> for Buffer {}
impl Deref for Buffer {
    type Target = raw::Buffer;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl Buffer {
    pub fn from_inner(handle: raw::Buffer) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferView.html>"]
#[doc(alias = "VkBufferView")]
pub struct BufferView {
    inner: <raw::BufferView as Handle>::InnerType,
}
unsafe impl Alias<raw::BufferView> for BufferView {}
impl Deref for BufferView {
    type Target = raw::BufferView;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl BufferView {
    pub fn from_inner(handle: raw::BufferView) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImage.html>"]
#[doc(alias = "VkImage")]
pub struct Image {
    inner: <raw::Image as Handle>::InnerType,
}
unsafe impl Alias<raw::Image> for Image {}
impl Deref for Image {
    type Target = raw::Image;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl Image {
    pub fn from_inner(handle: raw::Image) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageView.html>"]
#[doc(alias = "VkImageView")]
pub struct ImageView {
    inner: <raw::ImageView as Handle>::InnerType,
}
unsafe impl Alias<raw::ImageView> for ImageView {}
impl Deref for ImageView {
    type Target = raw::ImageView;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl ImageView {
    pub fn from_inner(handle: raw::ImageView) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderModule.html>"]
#[doc(alias = "VkShaderModule")]
pub struct ShaderModule {
    inner: <raw::ShaderModule as Handle>::InnerType,
}
unsafe impl Alias<raw::ShaderModule> for ShaderModule {}
impl Deref for ShaderModule {
    type Target = raw::ShaderModule;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl ShaderModule {
    pub fn from_inner(handle: raw::ShaderModule) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCache.html>"]
#[doc(alias = "VkPipelineCache")]
pub struct PipelineCache {
    inner: <raw::PipelineCache as Handle>::InnerType,
}
unsafe impl Alias<raw::PipelineCache> for PipelineCache {}
impl Deref for PipelineCache {
    type Target = raw::PipelineCache;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl PipelineCache {
    pub fn from_inner(handle: raw::PipelineCache) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipeline.html>"]
#[doc(alias = "VkPipeline")]
pub struct Pipeline {
    inner: <raw::Pipeline as Handle>::InnerType,
}
unsafe impl Alias<raw::Pipeline> for Pipeline {}
impl Deref for Pipeline {
    type Target = raw::Pipeline;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl Pipeline {
    pub fn from_inner(handle: raw::Pipeline) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineLayout.html>"]
#[doc(alias = "VkPipelineLayout")]
pub struct PipelineLayout {
    inner: <raw::PipelineLayout as Handle>::InnerType,
}
unsafe impl Alias<raw::PipelineLayout> for PipelineLayout {}
impl Deref for PipelineLayout {
    type Target = raw::PipelineLayout;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl PipelineLayout {
    pub fn from_inner(handle: raw::PipelineLayout) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampler.html>"]
#[doc(alias = "VkSampler")]
pub struct Sampler {
    inner: <raw::Sampler as Handle>::InnerType,
}
unsafe impl Alias<raw::Sampler> for Sampler {}
impl Deref for Sampler {
    type Target = raw::Sampler;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl Sampler {
    pub fn from_inner(handle: raw::Sampler) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorPool.html>"]
#[doc(alias = "VkDescriptorPool")]
pub struct DescriptorPool {
    inner: <raw::DescriptorPool as Handle>::InnerType,
}
unsafe impl Alias<raw::DescriptorPool> for DescriptorPool {}
impl Deref for DescriptorPool {
    type Target = raw::DescriptorPool;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl DescriptorPool {
    pub fn from_inner(handle: raw::DescriptorPool) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSet.html>"]
#[doc(alias = "VkDescriptorSet")]
pub struct DescriptorSet {
    inner: <raw::DescriptorSet as Handle>::InnerType,
}
unsafe impl Alias<raw::DescriptorSet> for DescriptorSet {}
impl Deref for DescriptorSet {
    type Target = raw::DescriptorSet;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl DescriptorSet {
    pub fn from_inner(handle: raw::DescriptorSet) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayout.html>"]
#[doc(alias = "VkDescriptorSetLayout")]
pub struct DescriptorSetLayout {
    inner: <raw::DescriptorSetLayout as Handle>::InnerType,
}
unsafe impl Alias<raw::DescriptorSetLayout> for DescriptorSetLayout {}
impl Deref for DescriptorSetLayout {
    type Target = raw::DescriptorSetLayout;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl DescriptorSetLayout {
    pub fn from_inner(handle: raw::DescriptorSetLayout) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebuffer.html>"]
#[doc(alias = "VkFramebuffer")]
pub struct Framebuffer {
    inner: <raw::Framebuffer as Handle>::InnerType,
}
unsafe impl Alias<raw::Framebuffer> for Framebuffer {}
impl Deref for Framebuffer {
    type Target = raw::Framebuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl Framebuffer {
    pub fn from_inner(handle: raw::Framebuffer) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPass.html>"]
#[doc(alias = "VkRenderPass")]
pub struct RenderPass {
    inner: <raw::RenderPass as Handle>::InnerType,
}
unsafe impl Alias<raw::RenderPass> for RenderPass {}
impl Deref for RenderPass {
    type Target = raw::RenderPass;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl RenderPass {
    pub fn from_inner(handle: raw::RenderPass) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPool.html>"]
#[doc(alias = "VkCommandPool")]
pub struct CommandPool {
    inner: <raw::CommandPool as Handle>::InnerType,
}
unsafe impl Alias<raw::CommandPool> for CommandPool {}
impl Deref for CommandPool {
    type Target = raw::CommandPool;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl CommandPool {
    pub fn from_inner(handle: raw::CommandPool) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBuffer.html>"]
#[doc(alias = "VkCommandBuffer")]
pub struct CommandBuffer<D: Dispatcher = DynamicDispatcher, A: Allocator = DefaultAllocator> {
    inner: <raw::CommandBuffer as Handle>::InnerType,
    disp: D,
    alloc: A,
}
unsafe impl Alias<raw::CommandBuffer> for CommandBuffer {}
impl<D: Dispatcher, A: Allocator> Copy for CommandBuffer<D, A>
where
    D: Copy,
    A: Copy,
{
}
impl<D: Dispatcher, A: Allocator> Deref for CommandBuffer<D, A> {
    type Target = raw::CommandBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl<D: Dispatcher, A: Allocator> CommandBuffer<D, A> {
    pub unsafe fn from_inner(handle: raw::CommandBuffer, disp: D, alloc: A) -> Self {
        Self {
            inner: handle.as_raw(),
            disp,
            alloc,
        }
    }
    pub fn get_dispatcher(&self) -> &D {
        &self.disp
    }
    pub fn get_allocator(&self) -> &A {
        &self.alloc
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBeginCommandBuffer.html>"]
    #[doc(alias = "vkBeginCommandBuffer")]
    pub fn begin(&self, p_begin_info: &CommandBufferBeginInfo) -> Result<()> {
        unsafe { raw::begin_command_buffer(self, p_begin_info, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEndCommandBuffer.html>"]
    #[doc(alias = "vkEndCommandBuffer")]
    pub fn end(&self) -> Result<()> {
        unsafe { raw::end_command_buffer(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandBuffer.html>"]
    #[doc(alias = "vkResetCommandBuffer")]
    pub fn reset(&self, flags: CommandBufferResetFlags) -> Result<()> {
        unsafe { raw::reset_command_buffer(self, flags, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipeline.html>"]
    #[doc(alias = "vkCmdBindPipeline")]
    pub fn bind_pipeline(&self, pipeline_bind_point: PipelineBindPoint, pipeline: &raw::Pipeline) {
        unsafe {
            raw::cmd_bind_pipeline(
                self,
                pipeline_bind_point,
                pipeline,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewport.html>"]
    #[doc(alias = "vkCmdSetViewport")]
    pub fn set_viewport<'a>(&self, first_viewport: u32, p_viewports: impl AsSlice<'a, Viewport>) {
        unsafe {
            raw::cmd_set_viewport(
                self,
                first_viewport,
                p_viewports,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissor.html>"]
    #[doc(alias = "vkCmdSetScissor")]
    pub fn set_scissor<'a>(&self, first_scissor: u32, p_scissors: impl AsSlice<'a, Rect2D>) {
        unsafe {
            raw::cmd_set_scissor(
                self,
                first_scissor,
                p_scissors,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineWidth.html>"]
    #[doc(alias = "vkCmdSetLineWidth")]
    pub fn set_line_width(&self, line_width: f32) {
        unsafe { raw::cmd_set_line_width(self, line_width, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBias.html>"]
    #[doc(alias = "vkCmdSetDepthBias")]
    pub fn set_depth_bias(
        &self,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) {
        unsafe {
            raw::cmd_set_depth_bias(
                self,
                depth_bias_constant_factor,
                depth_bias_clamp,
                depth_bias_slope_factor,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetBlendConstants.html>"]
    #[doc(alias = "vkCmdSetBlendConstants")]
    pub fn set_blend_constants(&self, blend_constants: [f32; 4u16 as _]) {
        unsafe {
            raw::cmd_set_blend_constants(self, blend_constants, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBounds.html>"]
    #[doc(alias = "vkCmdSetDepthBounds")]
    pub fn set_depth_bounds(&self, min_depth_bounds: f32, max_depth_bounds: f32) {
        unsafe {
            raw::cmd_set_depth_bounds(
                self,
                min_depth_bounds,
                max_depth_bounds,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilCompareMask.html>"]
    #[doc(alias = "vkCmdSetStencilCompareMask")]
    pub fn set_stencil_compare_mask(&self, face_mask: StencilFaceFlags, compare_mask: u32) {
        unsafe {
            raw::cmd_set_stencil_compare_mask(
                self,
                face_mask,
                compare_mask,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilWriteMask.html>"]
    #[doc(alias = "vkCmdSetStencilWriteMask")]
    pub fn set_stencil_write_mask(&self, face_mask: StencilFaceFlags, write_mask: u32) {
        unsafe {
            raw::cmd_set_stencil_write_mask(
                self,
                face_mask,
                write_mask,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilReference.html>"]
    #[doc(alias = "vkCmdSetStencilReference")]
    pub fn set_stencil_reference(&self, face_mask: StencilFaceFlags, reference: u32) {
        unsafe {
            raw::cmd_set_stencil_reference(
                self,
                face_mask,
                reference,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorSets.html>"]
    #[doc(alias = "vkCmdBindDescriptorSets")]
    pub fn bind_descriptor_sets<'a, V5: Alias<raw::DescriptorSet> + 'a>(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: &raw::PipelineLayout,
        first_set: u32,
        p_descriptor_sets: impl AsSlice<'a, V5>,
        p_dynamic_offsets: impl AsSlice<'a, u32>,
    ) {
        unsafe {
            raw::cmd_bind_descriptor_sets(
                self,
                pipeline_bind_point,
                layout,
                first_set,
                p_descriptor_sets,
                p_dynamic_offsets,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindIndexBuffer.html>"]
    #[doc(alias = "vkCmdBindIndexBuffer")]
    pub fn bind_index_buffer(
        &self,
        buffer: Option<&raw::Buffer>,
        offset: DeviceSize,
        index_type: IndexType,
    ) {
        unsafe {
            raw::cmd_bind_index_buffer(
                self,
                buffer,
                offset,
                index_type,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers.html>"]
    #[doc(alias = "vkCmdBindVertexBuffers")]
    pub fn bind_vertex_buffers<'a, V3: Alias<raw::Buffer> + 'a>(
        &self,
        first_binding: u32,
        p_buffers: impl AsSlice<'a, V3>,
        p_offsets: impl AsSlice<'a, DeviceSize>,
    ) {
        unsafe {
            raw::cmd_bind_vertex_buffers(
                self,
                first_binding,
                p_buffers,
                p_offsets,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDraw.html>"]
    #[doc(alias = "vkCmdDraw")]
    pub fn draw(
        &self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            raw::cmd_draw(
                self,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexed.html>"]
    #[doc(alias = "vkCmdDrawIndexed")]
    pub fn draw_indexed(
        &self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        unsafe {
            raw::cmd_draw_indexed(
                self,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirect.html>"]
    #[doc(alias = "vkCmdDrawIndirect")]
    pub fn draw_indirect(
        &self,
        buffer: &raw::Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            raw::cmd_draw_indirect(
                self,
                buffer,
                offset,
                draw_count,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirect.html>"]
    #[doc(alias = "vkCmdDrawIndexedIndirect")]
    pub fn draw_indexed_indirect(
        &self,
        buffer: &raw::Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            raw::cmd_draw_indexed_indirect(
                self,
                buffer,
                offset,
                draw_count,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatch.html>"]
    #[doc(alias = "vkCmdDispatch")]
    pub fn dispatch(&self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        unsafe {
            raw::cmd_dispatch(
                self,
                group_count_x,
                group_count_y,
                group_count_z,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchIndirect.html>"]
    #[doc(alias = "vkCmdDispatchIndirect")]
    pub fn dispatch_indirect(&self, buffer: &raw::Buffer, offset: DeviceSize) {
        unsafe {
            raw::cmd_dispatch_indirect(self, buffer, offset, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer.html>"]
    #[doc(alias = "vkCmdCopyBuffer")]
    pub fn copy_buffer<'a>(
        &self,
        src_buffer: &raw::Buffer,
        dst_buffer: &raw::Buffer,
        p_regions: impl AsSlice<'a, BufferCopy>,
    ) {
        unsafe {
            raw::cmd_copy_buffer(
                self,
                src_buffer,
                dst_buffer,
                p_regions,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage.html>"]
    #[doc(alias = "vkCmdCopyImage")]
    pub fn copy_image<'a>(
        &self,
        src_image: &raw::Image,
        src_image_layout: ImageLayout,
        dst_image: &raw::Image,
        dst_image_layout: ImageLayout,
        p_regions: impl AsSlice<'a, ImageCopy>,
    ) {
        unsafe {
            raw::cmd_copy_image(
                self,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                p_regions,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage.html>"]
    #[doc(alias = "vkCmdBlitImage")]
    pub fn blit_image<'a>(
        &self,
        src_image: &raw::Image,
        src_image_layout: ImageLayout,
        dst_image: &raw::Image,
        dst_image_layout: ImageLayout,
        p_regions: impl AsSlice<'a, ImageBlit>,
        filter: Filter,
    ) {
        unsafe {
            raw::cmd_blit_image(
                self,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                p_regions,
                filter,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage.html>"]
    #[doc(alias = "vkCmdCopyBufferToImage")]
    pub fn copy_buffer_to_image<'a>(
        &self,
        src_buffer: &raw::Buffer,
        dst_image: &raw::Image,
        dst_image_layout: ImageLayout,
        p_regions: impl AsSlice<'a, BufferImageCopy>,
    ) {
        unsafe {
            raw::cmd_copy_buffer_to_image(
                self,
                src_buffer,
                dst_image,
                dst_image_layout,
                p_regions,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer.html>"]
    #[doc(alias = "vkCmdCopyImageToBuffer")]
    pub fn copy_image_to_buffer<'a>(
        &self,
        src_image: &raw::Image,
        src_image_layout: ImageLayout,
        dst_buffer: &raw::Buffer,
        p_regions: impl AsSlice<'a, BufferImageCopy>,
    ) {
        unsafe {
            raw::cmd_copy_image_to_buffer(
                self,
                src_image,
                src_image_layout,
                dst_buffer,
                p_regions,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdUpdateBuffer.html>"]
    #[doc(alias = "vkCmdUpdateBuffer")]
    pub fn update_buffer(
        &self,
        dst_buffer: &raw::Buffer,
        dst_offset: DeviceSize,
        data_size: DeviceSize,
        p_data: VoidPtr,
    ) {
        unsafe {
            raw::cmd_update_buffer(
                self,
                dst_buffer,
                dst_offset,
                data_size,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdFillBuffer.html>"]
    #[doc(alias = "vkCmdFillBuffer")]
    pub fn fill_buffer(
        &self,
        dst_buffer: &raw::Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: u32,
    ) {
        unsafe {
            raw::cmd_fill_buffer(
                self,
                dst_buffer,
                dst_offset,
                size,
                data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearColorImage.html>"]
    #[doc(alias = "vkCmdClearColorImage")]
    pub fn clear_color_image<'a>(
        &self,
        image: &raw::Image,
        image_layout: ImageLayout,
        p_color: &ClearColorValue,
        p_ranges: impl AsSlice<'a, ImageSubresourceRange>,
    ) {
        unsafe {
            raw::cmd_clear_color_image(
                self,
                image,
                image_layout,
                p_color,
                p_ranges,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearDepthStencilImage.html>"]
    #[doc(alias = "vkCmdClearDepthStencilImage")]
    pub fn clear_depth_stencil_image<'a>(
        &self,
        image: &raw::Image,
        image_layout: ImageLayout,
        p_depth_stencil: &ClearDepthStencilValue,
        p_ranges: impl AsSlice<'a, ImageSubresourceRange>,
    ) {
        unsafe {
            raw::cmd_clear_depth_stencil_image(
                self,
                image,
                image_layout,
                p_depth_stencil,
                p_ranges,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearAttachments.html>"]
    #[doc(alias = "vkCmdClearAttachments")]
    pub fn clear_attachments<'a>(
        &self,
        p_attachments: impl AsSlice<'a, ClearAttachment>,
        p_rects: impl AsSlice<'a, ClearRect>,
    ) {
        unsafe {
            raw::cmd_clear_attachments(
                self,
                p_attachments,
                p_rects,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage.html>"]
    #[doc(alias = "vkCmdResolveImage")]
    pub fn resolve_image<'a>(
        &self,
        src_image: &raw::Image,
        src_image_layout: ImageLayout,
        dst_image: &raw::Image,
        dst_image_layout: ImageLayout,
        p_regions: impl AsSlice<'a, ImageResolve>,
    ) {
        unsafe {
            raw::cmd_resolve_image(
                self,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                p_regions,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent.html>"]
    #[doc(alias = "vkCmdSetEvent")]
    pub fn set_event(&self, event: &raw::Event, stage_mask: PipelineStageFlags) {
        unsafe { raw::cmd_set_event(self, event, stage_mask, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent.html>"]
    #[doc(alias = "vkCmdResetEvent")]
    pub fn reset_event(&self, event: &raw::Event, stage_mask: PipelineStageFlags) {
        unsafe { raw::cmd_reset_event(self, event, stage_mask, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents.html>"]
    #[doc(alias = "vkCmdWaitEvents")]
    pub fn wait_events<'a, V2: Alias<raw::Event> + 'a>(
        &self,
        p_events: impl AsSlice<'a, V2>,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        p_memory_barriers: impl AsSlice<'a, MemoryBarrier<'a>>,
        p_buffer_memory_barriers: impl AsSlice<'a, BufferMemoryBarrier<'a>>,
        p_image_memory_barriers: impl AsSlice<'a, ImageMemoryBarrier<'a>>,
    ) {
        unsafe {
            raw::cmd_wait_events(
                self,
                p_events,
                src_stage_mask,
                dst_stage_mask,
                p_memory_barriers,
                p_buffer_memory_barriers,
                p_image_memory_barriers,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier.html>"]
    #[doc(alias = "vkCmdPipelineBarrier")]
    pub fn pipeline_barrier<'a>(
        &self,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        p_memory_barriers: impl AsSlice<'a, MemoryBarrier<'a>>,
        p_buffer_memory_barriers: impl AsSlice<'a, BufferMemoryBarrier<'a>>,
        p_image_memory_barriers: impl AsSlice<'a, ImageMemoryBarrier<'a>>,
    ) {
        unsafe {
            raw::cmd_pipeline_barrier(
                self,
                src_stage_mask,
                dst_stage_mask,
                dependency_flags,
                p_memory_barriers,
                p_buffer_memory_barriers,
                p_image_memory_barriers,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQuery.html>"]
    #[doc(alias = "vkCmdBeginQuery")]
    pub fn begin_query(&self, query_pool: &raw::QueryPool, query: u32, flags: QueryControlFlags) {
        unsafe {
            raw::cmd_begin_query(
                self,
                query_pool,
                query,
                flags,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQuery.html>"]
    #[doc(alias = "vkCmdEndQuery")]
    pub fn end_query(&self, query_pool: &raw::QueryPool, query: u32) {
        unsafe { raw::cmd_end_query(self, query_pool, query, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetQueryPool.html>"]
    #[doc(alias = "vkCmdResetQueryPool")]
    pub fn reset_query_pool(
        &self,
        query_pool: &raw::QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        unsafe {
            raw::cmd_reset_query_pool(
                self,
                query_pool,
                first_query,
                query_count,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp.html>"]
    #[doc(alias = "vkCmdWriteTimestamp")]
    pub fn write_timestamp(
        &self,
        pipeline_stage: PipelineStageFlags,
        query_pool: &raw::QueryPool,
        query: u32,
    ) {
        unsafe {
            raw::cmd_write_timestamp(
                self,
                pipeline_stage,
                query_pool,
                query,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyQueryPoolResults.html>"]
    #[doc(alias = "vkCmdCopyQueryPoolResults")]
    pub fn copy_query_pool_results(
        &self,
        query_pool: &raw::QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: &raw::Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) {
        unsafe {
            raw::cmd_copy_query_pool_results(
                self,
                query_pool,
                first_query,
                query_count,
                dst_buffer,
                dst_offset,
                stride,
                flags,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants.html>"]
    #[doc(alias = "vkCmdPushConstants")]
    pub fn push_constants(
        &self,
        layout: &raw::PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        size: u32,
        p_values: VoidPtr,
    ) {
        unsafe {
            raw::cmd_push_constants(
                self,
                layout,
                stage_flags,
                offset,
                size,
                p_values,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass.html>"]
    #[doc(alias = "vkCmdBeginRenderPass")]
    pub fn begin_render_pass(
        &self,
        p_render_pass_begin: &RenderPassBeginInfo,
        contents: SubpassContents,
    ) {
        unsafe {
            raw::cmd_begin_render_pass(
                self,
                p_render_pass_begin,
                contents,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass.html>"]
    #[doc(alias = "vkCmdNextSubpass")]
    pub fn next_subpass(&self, contents: SubpassContents) {
        unsafe { raw::cmd_next_subpass(self, contents, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass.html>"]
    #[doc(alias = "vkCmdEndRenderPass")]
    pub fn end_render_pass(&self) {
        unsafe { raw::cmd_end_render_pass(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteCommands.html>"]
    #[doc(alias = "vkCmdExecuteCommands")]
    pub fn execute_commands<'a, V2: Alias<raw::CommandBuffer> + 'a>(
        &self,
        p_command_buffers: impl AsSlice<'a, V2>,
    ) {
        unsafe {
            raw::cmd_execute_commands(self, p_command_buffers, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMask.html>"]
    #[doc(alias = "vkCmdSetDeviceMask")]
    pub fn set_device_mask(&self, device_mask: u32) {
        unsafe { raw::cmd_set_device_mask(self, device_mask, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMaskKHR.html>"]
    #[doc(alias = "vkCmdSetDeviceMaskKHR")]
    pub fn set_device_mask_khr(&self, device_mask: u32) {
        unsafe {
            raw::cmd_set_device_mask_khr(self, device_mask, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchBase.html>"]
    #[doc(alias = "vkCmdDispatchBase")]
    pub fn dispatch_base(
        &self,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe {
            raw::cmd_dispatch_base(
                self,
                base_group_x,
                base_group_y,
                base_group_z,
                group_count_x,
                group_count_y,
                group_count_z,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchBaseKHR.html>"]
    #[doc(alias = "vkCmdDispatchBaseKHR")]
    pub fn dispatch_base_khr(
        &self,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe {
            raw::cmd_dispatch_base_khr(
                self,
                base_group_x,
                base_group_y,
                base_group_z,
                group_count_x,
                group_count_y,
                group_count_z,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCount.html>"]
    #[doc(alias = "vkCmdDrawIndirectCount")]
    pub fn draw_indirect_count(
        &self,
        buffer: &raw::Buffer,
        offset: DeviceSize,
        count_buffer: &raw::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            raw::cmd_draw_indirect_count(
                self,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCountKHR.html>"]
    #[doc(alias = "vkCmdDrawIndirectCountKHR")]
    pub fn draw_indirect_count_khr(
        &self,
        buffer: &raw::Buffer,
        offset: DeviceSize,
        count_buffer: &raw::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            raw::cmd_draw_indirect_count_khr(
                self,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCountAMD.html>"]
    #[doc(alias = "vkCmdDrawIndirectCountAMD")]
    pub fn draw_indirect_count_amd(
        &self,
        buffer: &raw::Buffer,
        offset: DeviceSize,
        count_buffer: &raw::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            raw::cmd_draw_indirect_count_amd(
                self,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCount.html>"]
    #[doc(alias = "vkCmdDrawIndexedIndirectCount")]
    pub fn draw_indexed_indirect_count(
        &self,
        buffer: &raw::Buffer,
        offset: DeviceSize,
        count_buffer: &raw::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            raw::cmd_draw_indexed_indirect_count(
                self,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCountKHR.html>"]
    #[doc(alias = "vkCmdDrawIndexedIndirectCountKHR")]
    pub fn draw_indexed_indirect_count_khr(
        &self,
        buffer: &raw::Buffer,
        offset: DeviceSize,
        count_buffer: &raw::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            raw::cmd_draw_indexed_indirect_count_khr(
                self,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCountAMD.html>"]
    #[doc(alias = "vkCmdDrawIndexedIndirectCountAMD")]
    pub fn draw_indexed_indirect_count_amd(
        &self,
        buffer: &raw::Buffer,
        offset: DeviceSize,
        count_buffer: &raw::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            raw::cmd_draw_indexed_indirect_count_amd(
                self,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2.html>"]
    #[doc(alias = "vkCmdBeginRenderPass2")]
    pub fn begin_render_pass2(
        &self,
        p_render_pass_begin: &RenderPassBeginInfo,
        p_subpass_begin_info: &SubpassBeginInfo,
    ) {
        unsafe {
            raw::cmd_begin_render_pass2(
                self,
                p_render_pass_begin,
                p_subpass_begin_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2KHR.html>"]
    #[doc(alias = "vkCmdBeginRenderPass2KHR")]
    pub fn begin_render_pass2_khr(
        &self,
        p_render_pass_begin: &RenderPassBeginInfo,
        p_subpass_begin_info: &SubpassBeginInfo,
    ) {
        unsafe {
            raw::cmd_begin_render_pass2_khr(
                self,
                p_render_pass_begin,
                p_subpass_begin_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2.html>"]
    #[doc(alias = "vkCmdNextSubpass2")]
    pub fn next_subpass2(
        &self,
        p_subpass_begin_info: &SubpassBeginInfo,
        p_subpass_end_info: &SubpassEndInfo,
    ) {
        unsafe {
            raw::cmd_next_subpass2(
                self,
                p_subpass_begin_info,
                p_subpass_end_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2KHR.html>"]
    #[doc(alias = "vkCmdNextSubpass2KHR")]
    pub fn next_subpass2_khr(
        &self,
        p_subpass_begin_info: &SubpassBeginInfo,
        p_subpass_end_info: &SubpassEndInfo,
    ) {
        unsafe {
            raw::cmd_next_subpass2_khr(
                self,
                p_subpass_begin_info,
                p_subpass_end_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2.html>"]
    #[doc(alias = "vkCmdEndRenderPass2")]
    pub fn end_render_pass2(&self, p_subpass_end_info: &SubpassEndInfo) {
        unsafe {
            raw::cmd_end_render_pass2(self, p_subpass_end_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2KHR.html>"]
    #[doc(alias = "vkCmdEndRenderPass2KHR")]
    pub fn end_render_pass2_khr(&self, p_subpass_end_info: &SubpassEndInfo) {
        unsafe {
            raw::cmd_end_render_pass2_khr(
                self,
                p_subpass_end_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2.html>"]
    #[doc(alias = "vkCmdSetEvent2")]
    pub fn set_event2(&self, event: &raw::Event, p_dependency_info: &DependencyInfo) {
        unsafe {
            raw::cmd_set_event2(
                self,
                event,
                p_dependency_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2KHR.html>"]
    #[doc(alias = "vkCmdSetEvent2KHR")]
    pub fn set_event2_khr(&self, event: &raw::Event, p_dependency_info: &DependencyInfo) {
        unsafe {
            raw::cmd_set_event2_khr(
                self,
                event,
                p_dependency_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2.html>"]
    #[doc(alias = "vkCmdResetEvent2")]
    pub fn reset_event2(&self, event: &raw::Event, stage_mask: PipelineStageFlags2) {
        unsafe {
            raw::cmd_reset_event2(self, event, stage_mask, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2KHR.html>"]
    #[doc(alias = "vkCmdResetEvent2KHR")]
    pub fn reset_event2_khr(&self, event: &raw::Event, stage_mask: PipelineStageFlags2) {
        unsafe {
            raw::cmd_reset_event2_khr(self, event, stage_mask, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2.html>"]
    #[doc(alias = "vkCmdWaitEvents2")]
    pub fn wait_events2<'a, V2: Alias<raw::Event> + 'a>(
        &self,
        p_events: impl AsSlice<'a, V2>,
        p_dependency_infos: impl AsSlice<'a, DependencyInfo<'a>>,
    ) {
        unsafe {
            raw::cmd_wait_events2(
                self,
                p_events,
                p_dependency_infos,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2KHR.html>"]
    #[doc(alias = "vkCmdWaitEvents2KHR")]
    pub fn wait_events2_khr<'a, V2: Alias<raw::Event> + 'a>(
        &self,
        p_events: impl AsSlice<'a, V2>,
        p_dependency_infos: impl AsSlice<'a, DependencyInfo<'a>>,
    ) {
        unsafe {
            raw::cmd_wait_events2_khr(
                self,
                p_events,
                p_dependency_infos,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2.html>"]
    #[doc(alias = "vkCmdPipelineBarrier2")]
    pub fn pipeline_barrier2(&self, p_dependency_info: &DependencyInfo) {
        unsafe {
            raw::cmd_pipeline_barrier2(self, p_dependency_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2KHR.html>"]
    #[doc(alias = "vkCmdPipelineBarrier2KHR")]
    pub fn pipeline_barrier2_khr(&self, p_dependency_info: &DependencyInfo) {
        unsafe {
            raw::cmd_pipeline_barrier2_khr(
                self,
                p_dependency_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2.html>"]
    #[doc(alias = "vkCmdWriteTimestamp2")]
    pub fn write_timestamp2(
        &self,
        stage: PipelineStageFlags2,
        query_pool: &raw::QueryPool,
        query: u32,
    ) {
        unsafe {
            raw::cmd_write_timestamp2(
                self,
                stage,
                query_pool,
                query,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2KHR.html>"]
    #[doc(alias = "vkCmdWriteTimestamp2KHR")]
    pub fn write_timestamp2_khr(
        &self,
        stage: PipelineStageFlags2,
        query_pool: &raw::QueryPool,
        query: u32,
    ) {
        unsafe {
            raw::cmd_write_timestamp2_khr(
                self,
                stage,
                query_pool,
                query,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer2.html>"]
    #[doc(alias = "vkCmdCopyBuffer2")]
    pub fn copy_buffer2(&self, p_copy_buffer_info: &CopyBufferInfo2) {
        unsafe {
            raw::cmd_copy_buffer2(self, p_copy_buffer_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer2KHR.html>"]
    #[doc(alias = "vkCmdCopyBuffer2KHR")]
    pub fn copy_buffer2_khr(&self, p_copy_buffer_info: &CopyBufferInfo2) {
        unsafe {
            raw::cmd_copy_buffer2_khr(self, p_copy_buffer_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage2.html>"]
    #[doc(alias = "vkCmdCopyImage2")]
    pub fn copy_image2(&self, p_copy_image_info: &CopyImageInfo2) {
        unsafe { raw::cmd_copy_image2(self, p_copy_image_info, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage2KHR.html>"]
    #[doc(alias = "vkCmdCopyImage2KHR")]
    pub fn copy_image2_khr(&self, p_copy_image_info: &CopyImageInfo2) {
        unsafe {
            raw::cmd_copy_image2_khr(self, p_copy_image_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2.html>"]
    #[doc(alias = "vkCmdCopyBufferToImage2")]
    pub fn copy_buffer_to_image2(&self, p_copy_buffer_to_image_info: &CopyBufferToImageInfo2) {
        unsafe {
            raw::cmd_copy_buffer_to_image2(
                self,
                p_copy_buffer_to_image_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2KHR.html>"]
    #[doc(alias = "vkCmdCopyBufferToImage2KHR")]
    pub fn copy_buffer_to_image2_khr(&self, p_copy_buffer_to_image_info: &CopyBufferToImageInfo2) {
        unsafe {
            raw::cmd_copy_buffer_to_image2_khr(
                self,
                p_copy_buffer_to_image_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2.html>"]
    #[doc(alias = "vkCmdCopyImageToBuffer2")]
    pub fn copy_image_to_buffer2(&self, p_copy_image_to_buffer_info: &CopyImageToBufferInfo2) {
        unsafe {
            raw::cmd_copy_image_to_buffer2(
                self,
                p_copy_image_to_buffer_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2KHR.html>"]
    #[doc(alias = "vkCmdCopyImageToBuffer2KHR")]
    pub fn copy_image_to_buffer2_khr(&self, p_copy_image_to_buffer_info: &CopyImageToBufferInfo2) {
        unsafe {
            raw::cmd_copy_image_to_buffer2_khr(
                self,
                p_copy_image_to_buffer_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage2.html>"]
    #[doc(alias = "vkCmdBlitImage2")]
    pub fn blit_image2(&self, p_blit_image_info: &BlitImageInfo2) {
        unsafe { raw::cmd_blit_image2(self, p_blit_image_info, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage2KHR.html>"]
    #[doc(alias = "vkCmdBlitImage2KHR")]
    pub fn blit_image2_khr(&self, p_blit_image_info: &BlitImageInfo2) {
        unsafe {
            raw::cmd_blit_image2_khr(self, p_blit_image_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage2.html>"]
    #[doc(alias = "vkCmdResolveImage2")]
    pub fn resolve_image2(&self, p_resolve_image_info: &ResolveImageInfo2) {
        unsafe {
            raw::cmd_resolve_image2(
                self,
                p_resolve_image_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage2KHR.html>"]
    #[doc(alias = "vkCmdResolveImage2KHR")]
    pub fn resolve_image2_khr(&self, p_resolve_image_info: &ResolveImageInfo2) {
        unsafe {
            raw::cmd_resolve_image2_khr(
                self,
                p_resolve_image_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRendering.html>"]
    #[doc(alias = "vkCmdBeginRendering")]
    pub fn begin_rendering(&self, p_rendering_info: &RenderingInfo) {
        unsafe {
            raw::cmd_begin_rendering(self, p_rendering_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderingKHR.html>"]
    #[doc(alias = "vkCmdBeginRenderingKHR")]
    pub fn begin_rendering_khr(&self, p_rendering_info: &RenderingInfo) {
        unsafe {
            raw::cmd_begin_rendering_khr(self, p_rendering_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRendering.html>"]
    #[doc(alias = "vkCmdEndRendering")]
    pub fn end_rendering(&self) {
        unsafe { raw::cmd_end_rendering(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderingKHR.html>"]
    #[doc(alias = "vkCmdEndRenderingKHR")]
    pub fn end_rendering_khr(&self) {
        unsafe { raw::cmd_end_rendering_khr(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullMode.html>"]
    #[doc(alias = "vkCmdSetCullMode")]
    pub fn set_cull_mode(&self, cull_mode: CullModeFlags) {
        unsafe { raw::cmd_set_cull_mode(self, cull_mode, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullModeEXT.html>"]
    #[doc(alias = "vkCmdSetCullModeEXT")]
    pub fn set_cull_mode_ext(&self, cull_mode: CullModeFlags) {
        unsafe { raw::cmd_set_cull_mode_ext(self, cull_mode, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFace.html>"]
    #[doc(alias = "vkCmdSetFrontFace")]
    pub fn set_front_face(&self, front_face: FrontFace) {
        unsafe { raw::cmd_set_front_face(self, front_face, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFaceEXT.html>"]
    #[doc(alias = "vkCmdSetFrontFaceEXT")]
    pub fn set_front_face_ext(&self, front_face: FrontFace) {
        unsafe { raw::cmd_set_front_face_ext(self, front_face, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopology.html>"]
    #[doc(alias = "vkCmdSetPrimitiveTopology")]
    pub fn set_primitive_topology(&self, primitive_topology: PrimitiveTopology) {
        unsafe {
            raw::cmd_set_primitive_topology(
                self,
                primitive_topology,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopologyEXT.html>"]
    #[doc(alias = "vkCmdSetPrimitiveTopologyEXT")]
    pub fn set_primitive_topology_ext(&self, primitive_topology: PrimitiveTopology) {
        unsafe {
            raw::cmd_set_primitive_topology_ext(
                self,
                primitive_topology,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCount.html>"]
    #[doc(alias = "vkCmdSetViewportWithCount")]
    pub fn set_viewport_with_count<'a>(&self, p_viewports: impl AsSlice<'a, Viewport>) {
        unsafe {
            raw::cmd_set_viewport_with_count(self, p_viewports, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCountEXT.html>"]
    #[doc(alias = "vkCmdSetViewportWithCountEXT")]
    pub fn set_viewport_with_count_ext<'a>(&self, p_viewports: impl AsSlice<'a, Viewport>) {
        unsafe {
            raw::cmd_set_viewport_with_count_ext(
                self,
                p_viewports,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCount.html>"]
    #[doc(alias = "vkCmdSetScissorWithCount")]
    pub fn set_scissor_with_count<'a>(&self, p_scissors: impl AsSlice<'a, Rect2D>) {
        unsafe {
            raw::cmd_set_scissor_with_count(self, p_scissors, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCountEXT.html>"]
    #[doc(alias = "vkCmdSetScissorWithCountEXT")]
    pub fn set_scissor_with_count_ext<'a>(&self, p_scissors: impl AsSlice<'a, Rect2D>) {
        unsafe {
            raw::cmd_set_scissor_with_count_ext(
                self,
                p_scissors,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2.html>"]
    #[doc(alias = "vkCmdBindVertexBuffers2")]
    pub fn bind_vertex_buffers2<'a, V3: Alias<raw::Buffer> + 'a>(
        &self,
        first_binding: u32,
        p_buffers: impl AsSlice<'a, V3>,
        p_offsets: impl AsSlice<'a, DeviceSize>,
        p_sizes: Option<impl AsSlice<'a, DeviceSize>>,
        p_strides: Option<impl AsSlice<'a, DeviceSize>>,
    ) {
        unsafe {
            raw::cmd_bind_vertex_buffers2(
                self,
                first_binding,
                p_buffers,
                p_offsets,
                p_sizes,
                p_strides,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2EXT.html>"]
    #[doc(alias = "vkCmdBindVertexBuffers2EXT")]
    pub fn bind_vertex_buffers2_ext<'a, V3: Alias<raw::Buffer> + 'a>(
        &self,
        first_binding: u32,
        p_buffers: impl AsSlice<'a, V3>,
        p_offsets: impl AsSlice<'a, DeviceSize>,
        p_sizes: Option<impl AsSlice<'a, DeviceSize>>,
        p_strides: Option<impl AsSlice<'a, DeviceSize>>,
    ) {
        unsafe {
            raw::cmd_bind_vertex_buffers2_ext(
                self,
                first_binding,
                p_buffers,
                p_offsets,
                p_sizes,
                p_strides,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnable.html>"]
    #[doc(alias = "vkCmdSetDepthTestEnable")]
    pub fn set_depth_test_enable(&self, depth_test_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_test_enable(
                self,
                depth_test_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnableEXT.html>"]
    #[doc(alias = "vkCmdSetDepthTestEnableEXT")]
    pub fn set_depth_test_enable_ext(&self, depth_test_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_test_enable_ext(
                self,
                depth_test_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnable.html>"]
    #[doc(alias = "vkCmdSetDepthWriteEnable")]
    pub fn set_depth_write_enable(&self, depth_write_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_write_enable(
                self,
                depth_write_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnableEXT.html>"]
    #[doc(alias = "vkCmdSetDepthWriteEnableEXT")]
    pub fn set_depth_write_enable_ext(&self, depth_write_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_write_enable_ext(
                self,
                depth_write_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOp.html>"]
    #[doc(alias = "vkCmdSetDepthCompareOp")]
    pub fn set_depth_compare_op(&self, depth_compare_op: CompareOp) {
        unsafe {
            raw::cmd_set_depth_compare_op(
                self,
                depth_compare_op,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOpEXT.html>"]
    #[doc(alias = "vkCmdSetDepthCompareOpEXT")]
    pub fn set_depth_compare_op_ext(&self, depth_compare_op: CompareOp) {
        unsafe {
            raw::cmd_set_depth_compare_op_ext(
                self,
                depth_compare_op,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnable.html>"]
    #[doc(alias = "vkCmdSetDepthBoundsTestEnable")]
    pub fn set_depth_bounds_test_enable(&self, depth_bounds_test_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_bounds_test_enable(
                self,
                depth_bounds_test_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnableEXT.html>"]
    #[doc(alias = "vkCmdSetDepthBoundsTestEnableEXT")]
    pub fn set_depth_bounds_test_enable_ext(&self, depth_bounds_test_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_bounds_test_enable_ext(
                self,
                depth_bounds_test_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilTestEnable.html>"]
    #[doc(alias = "vkCmdSetStencilTestEnable")]
    pub fn set_stencil_test_enable(&self, stencil_test_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_stencil_test_enable(
                self,
                stencil_test_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilTestEnableEXT.html>"]
    #[doc(alias = "vkCmdSetStencilTestEnableEXT")]
    pub fn set_stencil_test_enable_ext(&self, stencil_test_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_stencil_test_enable_ext(
                self,
                stencil_test_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilOp.html>"]
    #[doc(alias = "vkCmdSetStencilOp")]
    pub fn set_stencil_op(
        &self,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) {
        unsafe {
            raw::cmd_set_stencil_op(
                self,
                face_mask,
                fail_op,
                pass_op,
                depth_fail_op,
                compare_op,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilOpEXT.html>"]
    #[doc(alias = "vkCmdSetStencilOpEXT")]
    pub fn set_stencil_op_ext(
        &self,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) {
        unsafe {
            raw::cmd_set_stencil_op_ext(
                self,
                face_mask,
                fail_op,
                pass_op,
                depth_fail_op,
                compare_op,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnable.html>"]
    #[doc(alias = "vkCmdSetRasterizerDiscardEnable")]
    pub fn set_rasterizer_discard_enable(&self, rasterizer_discard_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_rasterizer_discard_enable(
                self,
                rasterizer_discard_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnableEXT.html>"]
    #[doc(alias = "vkCmdSetRasterizerDiscardEnableEXT")]
    pub fn set_rasterizer_discard_enable_ext(&self, rasterizer_discard_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_rasterizer_discard_enable_ext(
                self,
                rasterizer_discard_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnable.html>"]
    #[doc(alias = "vkCmdSetDepthBiasEnable")]
    pub fn set_depth_bias_enable(&self, depth_bias_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_bias_enable(
                self,
                depth_bias_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnableEXT.html>"]
    #[doc(alias = "vkCmdSetDepthBiasEnableEXT")]
    pub fn set_depth_bias_enable_ext(&self, depth_bias_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_bias_enable_ext(
                self,
                depth_bias_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnable.html>"]
    #[doc(alias = "vkCmdSetPrimitiveRestartEnable")]
    pub fn set_primitive_restart_enable(&self, primitive_restart_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_primitive_restart_enable(
                self,
                primitive_restart_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnableEXT.html>"]
    #[doc(alias = "vkCmdSetPrimitiveRestartEnableEXT")]
    pub fn set_primitive_restart_enable_ext(&self, primitive_restart_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_primitive_restart_enable_ext(
                self,
                primitive_restart_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerBeginEXT.html>"]
    #[doc(alias = "vkCmdDebugMarkerBeginEXT")]
    pub fn debug_marker_begin_ext(&self, p_marker_info: &DebugMarkerMarkerInfoEXT) {
        unsafe {
            raw::cmd_debug_marker_begin_ext(self, p_marker_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerEndEXT.html>"]
    #[doc(alias = "vkCmdDebugMarkerEndEXT")]
    pub fn debug_marker_end_ext(&self) {
        unsafe { raw::cmd_debug_marker_end_ext(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerInsertEXT.html>"]
    #[doc(alias = "vkCmdDebugMarkerInsertEXT")]
    pub fn debug_marker_insert_ext(&self, p_marker_info: &DebugMarkerMarkerInfoEXT) {
        unsafe {
            raw::cmd_debug_marker_insert_ext(
                self,
                p_marker_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindTransformFeedbackBuffersEXT.html>"]
    #[doc(alias = "vkCmdBindTransformFeedbackBuffersEXT")]
    pub fn bind_transform_feedback_buffers_ext<'a, V3: Alias<raw::Buffer> + 'a>(
        &self,
        first_binding: u32,
        p_buffers: impl AsSlice<'a, V3>,
        p_offsets: impl AsSlice<'a, DeviceSize>,
        p_sizes: Option<impl AsSlice<'a, DeviceSize>>,
    ) {
        unsafe {
            raw::cmd_bind_transform_feedback_buffers_ext(
                self,
                first_binding,
                p_buffers,
                p_offsets,
                p_sizes,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html>"]
    #[doc(alias = "vkCmdBeginTransformFeedbackEXT")]
    pub fn begin_transform_feedback_ext<'a, V3: Alias<raw::Buffer> + 'a>(
        &self,
        first_counter_buffer: u32,
        p_counter_buffers: impl AsSlice<'a, V3>,
        p_counter_buffer_offsets: Option<impl AsSlice<'a, DeviceSize>>,
    ) {
        unsafe {
            raw::cmd_begin_transform_feedback_ext(
                self,
                first_counter_buffer,
                p_counter_buffers,
                p_counter_buffer_offsets,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndTransformFeedbackEXT.html>"]
    #[doc(alias = "vkCmdEndTransformFeedbackEXT")]
    pub fn end_transform_feedback_ext<'a, V3: Alias<raw::Buffer> + 'a>(
        &self,
        first_counter_buffer: u32,
        p_counter_buffers: impl AsSlice<'a, V3>,
        p_counter_buffer_offsets: Option<impl AsSlice<'a, DeviceSize>>,
    ) {
        unsafe {
            raw::cmd_end_transform_feedback_ext(
                self,
                first_counter_buffer,
                p_counter_buffers,
                p_counter_buffer_offsets,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQueryIndexedEXT.html>"]
    #[doc(alias = "vkCmdBeginQueryIndexedEXT")]
    pub fn begin_query_indexed_ext(
        &self,
        query_pool: &raw::QueryPool,
        query: u32,
        flags: QueryControlFlags,
        index: u32,
    ) {
        unsafe {
            raw::cmd_begin_query_indexed_ext(
                self,
                query_pool,
                query,
                flags,
                index,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQueryIndexedEXT.html>"]
    #[doc(alias = "vkCmdEndQueryIndexedEXT")]
    pub fn end_query_indexed_ext(&self, query_pool: &raw::QueryPool, query: u32, index: u32) {
        unsafe {
            raw::cmd_end_query_indexed_ext(
                self,
                query_pool,
                query,
                index,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectByteCountEXT.html>"]
    #[doc(alias = "vkCmdDrawIndirectByteCountEXT")]
    pub fn draw_indirect_byte_count_ext(
        &self,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: &raw::Buffer,
        counter_buffer_offset: DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        unsafe {
            raw::cmd_draw_indirect_byte_count_ext(
                self,
                instance_count,
                first_instance,
                counter_buffer,
                counter_buffer_offset,
                counter_offset,
                vertex_stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCuLaunchKernelNVX.html>"]
    #[doc(alias = "vkCmdCuLaunchKernelNVX")]
    pub fn cu_launch_kernel_nvx(&self, p_launch_info: &CuLaunchInfoNVX) {
        unsafe {
            raw::cmd_cu_launch_kernel_nvx(self, p_launch_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetKHR.html>"]
    #[doc(alias = "vkCmdPushDescriptorSetKHR")]
    pub fn push_descriptor_set_khr<'a>(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: &raw::PipelineLayout,
        set: u32,
        p_descriptor_writes: impl AsSlice<'a, WriteDescriptorSet<'a>>,
    ) {
        unsafe {
            raw::cmd_push_descriptor_set_khr(
                self,
                pipeline_bind_point,
                layout,
                set,
                p_descriptor_writes,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html>"]
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplateKHR")]
    pub fn push_descriptor_set_with_template_khr(
        &self,
        descriptor_update_template: &raw::DescriptorUpdateTemplate,
        layout: &raw::PipelineLayout,
        set: u32,
        p_data: VoidPtr,
    ) {
        unsafe {
            raw::cmd_push_descriptor_set_with_template_khr(
                self,
                descriptor_update_template,
                layout,
                set,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html>"]
    #[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
    pub fn begin_conditional_rendering_ext(
        &self,
        p_conditional_rendering_begin: &ConditionalRenderingBeginInfoEXT,
    ) {
        unsafe {
            raw::cmd_begin_conditional_rendering_ext(
                self,
                p_conditional_rendering_begin,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndConditionalRenderingEXT.html>"]
    #[doc(alias = "vkCmdEndConditionalRenderingEXT")]
    pub fn end_conditional_rendering_ext(&self) {
        unsafe { raw::cmd_end_conditional_rendering_ext(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingNV.html>"]
    #[doc(alias = "vkCmdSetViewportWScalingNV")]
    pub fn set_viewport_wscaling_nv<'a>(
        &self,
        first_viewport: u32,
        p_viewport_wscalings: impl AsSlice<'a, ViewportWScalingNV>,
    ) {
        unsafe {
            raw::cmd_set_viewport_wscaling_nv(
                self,
                first_viewport,
                p_viewport_wscalings,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEXT.html>"]
    #[doc(alias = "vkCmdSetDiscardRectangleEXT")]
    pub fn set_discard_rectangle_ext<'a>(
        &self,
        first_discard_rectangle: u32,
        p_discard_rectangles: impl AsSlice<'a, Rect2D>,
    ) {
        unsafe {
            raw::cmd_set_discard_rectangle_ext(
                self,
                first_discard_rectangle,
                p_discard_rectangles,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEnableEXT.html>"]
    #[doc(alias = "vkCmdSetDiscardRectangleEnableEXT")]
    pub fn set_discard_rectangle_enable_ext(&self, discard_rectangle_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_discard_rectangle_enable_ext(
                self,
                discard_rectangle_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleModeEXT.html>"]
    #[doc(alias = "vkCmdSetDiscardRectangleModeEXT")]
    pub fn set_discard_rectangle_mode_ext(&self, discard_rectangle_mode: DiscardRectangleModeEXT) {
        unsafe {
            raw::cmd_set_discard_rectangle_mode_ext(
                self,
                discard_rectangle_mode,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html>"]
    #[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
    pub fn begin_debug_utils_label_ext(&self, p_label_info: &DebugUtilsLabelEXT) {
        unsafe {
            raw::cmd_begin_debug_utils_label_ext(
                self,
                p_label_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html>"]
    #[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
    pub fn end_debug_utils_label_ext(&self) {
        unsafe { raw::cmd_end_debug_utils_label_ext(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html>"]
    #[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
    pub fn insert_debug_utils_label_ext(&self, p_label_info: &DebugUtilsLabelEXT) {
        unsafe {
            raw::cmd_insert_debug_utils_label_ext(
                self,
                p_label_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdInitializeGraphScratchMemoryAMDX.html>"]
    #[doc(alias = "vkCmdInitializeGraphScratchMemoryAMDX")]
    pub fn initialize_graph_scratch_memory_amdx(&self, scratch: DeviceAddress) {
        unsafe {
            raw::cmd_initialize_graph_scratch_memory_amdx(
                self,
                scratch,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchGraphAMDX.html>"]
    #[doc(alias = "vkCmdDispatchGraphAMDX")]
    pub fn dispatch_graph_amdx(
        &self,
        scratch: DeviceAddress,
        p_count_info: &DispatchGraphCountInfoAMDX,
    ) {
        unsafe {
            raw::cmd_dispatch_graph_amdx(
                self,
                scratch,
                p_count_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchGraphIndirectAMDX.html>"]
    #[doc(alias = "vkCmdDispatchGraphIndirectAMDX")]
    pub fn dispatch_graph_indirect_amdx(
        &self,
        scratch: DeviceAddress,
        p_count_info: &DispatchGraphCountInfoAMDX,
    ) {
        unsafe {
            raw::cmd_dispatch_graph_indirect_amdx(
                self,
                scratch,
                p_count_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchGraphIndirectCountAMDX.html>"]
    #[doc(alias = "vkCmdDispatchGraphIndirectCountAMDX")]
    pub fn dispatch_graph_indirect_count_amdx(
        &self,
        scratch: DeviceAddress,
        count_info: DeviceAddress,
    ) {
        unsafe {
            raw::cmd_dispatch_graph_indirect_count_amdx(
                self,
                scratch,
                count_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEXT.html>"]
    #[doc(alias = "vkCmdSetSampleLocationsEXT")]
    pub fn set_sample_locations_ext(&self, p_sample_locations_info: &SampleLocationsInfoEXT) {
        unsafe {
            raw::cmd_set_sample_locations_ext(
                self,
                p_sample_locations_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructuresKHR.html>"]
    #[doc(alias = "vkCmdBuildAccelerationStructuresKHR")]
    pub fn build_acceleration_structures_khr<'a>(
        &self,
        p_infos: impl AsSlice<'a, AccelerationStructureBuildGeometryInfoKHR<'a>>,
        pp_build_range_infos: &&AccelerationStructureBuildRangeInfoKHR,
    ) {
        unsafe {
            raw::cmd_build_acceleration_structures_khr(
                self,
                p_infos,
                pp_build_range_infos,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructuresIndirectKHR.html>"]
    #[doc(alias = "vkCmdBuildAccelerationStructuresIndirectKHR")]
    pub fn build_acceleration_structures_indirect_khr<'a>(
        &self,
        p_infos: impl AsSlice<'a, AccelerationStructureBuildGeometryInfoKHR<'a>>,
        p_indirect_device_addresses: impl AsSlice<'a, DeviceAddress>,
        p_indirect_strides: impl AsSlice<'a, u32>,
        pp_max_primitive_counts: &&u32,
    ) {
        unsafe {
            raw::cmd_build_acceleration_structures_indirect_khr(
                self,
                p_infos,
                p_indirect_device_addresses,
                p_indirect_strides,
                pp_max_primitive_counts,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html>"]
    #[doc(alias = "vkCmdCopyAccelerationStructureKHR")]
    pub fn copy_acceleration_structure_khr(&self, p_info: &CopyAccelerationStructureInfoKHR) {
        unsafe {
            raw::cmd_copy_acceleration_structure_khr(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html>"]
    #[doc(alias = "vkCmdCopyAccelerationStructureToMemoryKHR")]
    pub fn copy_acceleration_structure_to_memory_khr(
        &self,
        p_info: &CopyAccelerationStructureToMemoryInfoKHR,
    ) {
        unsafe {
            raw::cmd_copy_acceleration_structure_to_memory_khr(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html>"]
    #[doc(alias = "vkCmdCopyMemoryToAccelerationStructureKHR")]
    pub fn copy_memory_to_acceleration_structure_khr(
        &self,
        p_info: &CopyMemoryToAccelerationStructureInfoKHR,
    ) {
        unsafe {
            raw::cmd_copy_memory_to_acceleration_structure_khr(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html>"]
    #[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesKHR")]
    pub fn write_acceleration_structures_properties_khr<
        'a,
        V2: Alias<raw::AccelerationStructureKHR> + 'a,
    >(
        &self,
        p_acceleration_structures: impl AsSlice<'a, V2>,
        query_type: QueryType,
        query_pool: &raw::QueryPool,
        first_query: u32,
    ) {
        unsafe {
            raw::cmd_write_acceleration_structures_properties_khr(
                self,
                p_acceleration_structures,
                query_type,
                query_pool,
                first_query,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysKHR.html>"]
    #[doc(alias = "vkCmdTraceRaysKHR")]
    pub fn trace_rays_khr(
        &self,
        p_raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        unsafe {
            raw::cmd_trace_rays_khr(
                self,
                p_raygen_shader_binding_table,
                p_miss_shader_binding_table,
                p_hit_shader_binding_table,
                p_callable_shader_binding_table,
                width,
                height,
                depth,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirectKHR.html>"]
    #[doc(alias = "vkCmdTraceRaysIndirectKHR")]
    pub fn trace_rays_indirect_khr(
        &self,
        p_raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        indirect_device_address: DeviceAddress,
    ) {
        unsafe {
            raw::cmd_trace_rays_indirect_khr(
                self,
                p_raygen_shader_binding_table,
                p_miss_shader_binding_table,
                p_hit_shader_binding_table,
                p_callable_shader_binding_table,
                indirect_device_address,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html>"]
    #[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
    pub fn set_ray_tracing_pipeline_stack_size_khr(&self, pipeline_stack_size: u32) {
        unsafe {
            raw::cmd_set_ray_tracing_pipeline_stack_size_khr(
                self,
                pipeline_stack_size,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindShadingRateImageNV.html>"]
    #[doc(alias = "vkCmdBindShadingRateImageNV")]
    pub fn bind_shading_rate_image_nv(
        &self,
        image_view: Option<&raw::ImageView>,
        image_layout: ImageLayout,
    ) {
        unsafe {
            raw::cmd_bind_shading_rate_image_nv(
                self,
                image_view,
                image_layout,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportShadingRatePaletteNV.html>"]
    #[doc(alias = "vkCmdSetViewportShadingRatePaletteNV")]
    pub fn set_viewport_shading_rate_palette_nv<'a>(
        &self,
        first_viewport: u32,
        p_shading_rate_palettes: impl AsSlice<'a, ShadingRatePaletteNV<'a>>,
    ) {
        unsafe {
            raw::cmd_set_viewport_shading_rate_palette_nv(
                self,
                first_viewport,
                p_shading_rate_palettes,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoarseSampleOrderNV.html>"]
    #[doc(alias = "vkCmdSetCoarseSampleOrderNV")]
    pub fn set_coarse_sample_order_nv<'a>(
        &self,
        sample_order_type: CoarseSampleOrderTypeNV,
        p_custom_sample_orders: impl AsSlice<'a, CoarseSampleOrderCustomNV<'a>>,
    ) {
        unsafe {
            raw::cmd_set_coarse_sample_order_nv(
                self,
                sample_order_type,
                p_custom_sample_orders,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructureNV.html>"]
    #[doc(alias = "vkCmdBuildAccelerationStructureNV")]
    pub fn build_acceleration_structure_nv(
        &self,
        p_info: &AccelerationStructureInfoNV,
        instance_data: Option<&raw::Buffer>,
        instance_offset: DeviceSize,
        update: impl Into<Bool32>,
        dst: &raw::AccelerationStructureNV,
        src: Option<&raw::AccelerationStructureNV>,
        scratch: &raw::Buffer,
        scratch_offset: DeviceSize,
    ) {
        unsafe {
            raw::cmd_build_acceleration_structure_nv(
                self,
                p_info,
                instance_data,
                instance_offset,
                update,
                dst,
                src,
                scratch,
                scratch_offset,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureNV.html>"]
    #[doc(alias = "vkCmdCopyAccelerationStructureNV")]
    pub fn copy_acceleration_structure_nv(
        &self,
        dst: &raw::AccelerationStructureNV,
        src: &raw::AccelerationStructureNV,
        mode: CopyAccelerationStructureModeKHR,
    ) {
        unsafe {
            raw::cmd_copy_acceleration_structure_nv(
                self,
                dst,
                src,
                mode,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysNV.html>"]
    #[doc(alias = "vkCmdTraceRaysNV")]
    pub fn trace_rays_nv(
        &self,
        raygen_shader_binding_table_buffer: &raw::Buffer,
        raygen_shader_binding_offset: DeviceSize,
        miss_shader_binding_table_buffer: Option<&raw::Buffer>,
        miss_shader_binding_offset: DeviceSize,
        miss_shader_binding_stride: DeviceSize,
        hit_shader_binding_table_buffer: Option<&raw::Buffer>,
        hit_shader_binding_offset: DeviceSize,
        hit_shader_binding_stride: DeviceSize,
        callable_shader_binding_table_buffer: Option<&raw::Buffer>,
        callable_shader_binding_offset: DeviceSize,
        callable_shader_binding_stride: DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        unsafe {
            raw::cmd_trace_rays_nv(
                self,
                raygen_shader_binding_table_buffer,
                raygen_shader_binding_offset,
                miss_shader_binding_table_buffer,
                miss_shader_binding_offset,
                miss_shader_binding_stride,
                hit_shader_binding_table_buffer,
                hit_shader_binding_offset,
                hit_shader_binding_stride,
                callable_shader_binding_table_buffer,
                callable_shader_binding_offset,
                callable_shader_binding_stride,
                width,
                height,
                depth,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html>"]
    #[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesNV")]
    pub fn write_acceleration_structures_properties_nv<
        'a,
        V2: Alias<raw::AccelerationStructureNV> + 'a,
    >(
        &self,
        p_acceleration_structures: impl AsSlice<'a, V2>,
        query_type: QueryType,
        query_pool: &raw::QueryPool,
        first_query: u32,
    ) {
        unsafe {
            raw::cmd_write_acceleration_structures_properties_nv(
                self,
                p_acceleration_structures,
                query_type,
                query_pool,
                first_query,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarkerAMD.html>"]
    #[doc(alias = "vkCmdWriteBufferMarkerAMD")]
    pub fn write_buffer_marker_amd(
        &self,
        pipeline_stage: PipelineStageFlags,
        dst_buffer: &raw::Buffer,
        dst_offset: DeviceSize,
        marker: u32,
    ) {
        unsafe {
            raw::cmd_write_buffer_marker_amd(
                self,
                pipeline_stage,
                dst_buffer,
                dst_offset,
                marker,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksNV.html>"]
    #[doc(alias = "vkCmdDrawMeshTasksNV")]
    pub fn draw_mesh_tasks_nv(&self, task_count: u32, first_task: u32) {
        unsafe {
            raw::cmd_draw_mesh_tasks_nv(
                self,
                task_count,
                first_task,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html>"]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
    pub fn draw_mesh_tasks_indirect_nv(
        &self,
        buffer: &raw::Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            raw::cmd_draw_mesh_tasks_indirect_nv(
                self,
                buffer,
                offset,
                draw_count,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html>"]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectCountNV")]
    pub fn draw_mesh_tasks_indirect_count_nv(
        &self,
        buffer: &raw::Buffer,
        offset: DeviceSize,
        count_buffer: &raw::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            raw::cmd_draw_mesh_tasks_indirect_count_nv(
                self,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorEnableNV.html>"]
    #[doc(alias = "vkCmdSetExclusiveScissorEnableNV")]
    pub fn set_exclusive_scissor_enable_nv<'a>(
        &self,
        first_exclusive_scissor: u32,
        p_exclusive_scissor_enables: impl AsSlice<'a, Bool32>,
    ) {
        unsafe {
            raw::cmd_set_exclusive_scissor_enable_nv(
                self,
                first_exclusive_scissor,
                p_exclusive_scissor_enables,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorNV.html>"]
    #[doc(alias = "vkCmdSetExclusiveScissorNV")]
    pub fn set_exclusive_scissor_nv<'a>(
        &self,
        first_exclusive_scissor: u32,
        p_exclusive_scissors: impl AsSlice<'a, Rect2D>,
    ) {
        unsafe {
            raw::cmd_set_exclusive_scissor_nv(
                self,
                first_exclusive_scissor,
                p_exclusive_scissors,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCheckpointNV.html>"]
    #[doc(alias = "vkCmdSetCheckpointNV")]
    pub fn set_checkpoint_nv(&self, p_checkpoint_marker: VoidPtr) {
        unsafe {
            raw::cmd_set_checkpoint_nv(
                self,
                p_checkpoint_marker,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html>"]
    #[doc(alias = "vkCmdSetPerformanceMarkerINTEL")]
    pub fn set_performance_marker_intel(
        &self,
        p_marker_info: &PerformanceMarkerInfoINTEL,
    ) -> Result<()> {
        unsafe {
            raw::cmd_set_performance_marker_intel(
                self,
                p_marker_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html>"]
    #[doc(alias = "vkCmdSetPerformanceStreamMarkerINTEL")]
    pub fn set_performance_stream_marker_intel(
        &self,
        p_marker_info: &PerformanceStreamMarkerInfoINTEL,
    ) -> Result<()> {
        unsafe {
            raw::cmd_set_performance_stream_marker_intel(
                self,
                p_marker_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html>"]
    #[doc(alias = "vkCmdSetPerformanceOverrideINTEL")]
    pub fn set_performance_override_intel(
        &self,
        p_override_info: &PerformanceOverrideInfoINTEL,
    ) -> Result<()> {
        unsafe {
            raw::cmd_set_performance_override_intel(
                self,
                p_override_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateKHR.html>"]
    #[doc(alias = "vkCmdSetFragmentShadingRateKHR")]
    pub fn set_fragment_shading_rate_khr(
        &self,
        p_fragment_size: &Extent2D,
        combiner_ops: [FragmentShadingRateCombinerOpKHR; 2u16 as _],
    ) {
        unsafe {
            raw::cmd_set_fragment_shading_rate_khr(
                self,
                p_fragment_size,
                combiner_ops,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRenderingAttachmentLocationsKHR.html>"]
    #[doc(alias = "vkCmdSetRenderingAttachmentLocationsKHR")]
    pub fn set_rendering_attachment_locations_khr(
        &self,
        p_location_info: &RenderingAttachmentLocationInfoKHR,
    ) {
        unsafe {
            raw::cmd_set_rendering_attachment_locations_khr(
                self,
                p_location_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRenderingInputAttachmentIndicesKHR.html>"]
    #[doc(alias = "vkCmdSetRenderingInputAttachmentIndicesKHR")]
    pub fn set_rendering_input_attachment_indices_khr(
        &self,
        p_input_attachment_index_info: &RenderingInputAttachmentIndexInfoKHR,
    ) {
        unsafe {
            raw::cmd_set_rendering_input_attachment_indices_khr(
                self,
                p_input_attachment_index_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPreprocessGeneratedCommandsNV.html>"]
    #[doc(alias = "vkCmdPreprocessGeneratedCommandsNV")]
    pub fn preprocess_generated_commands_nv(
        &self,
        p_generated_commands_info: &GeneratedCommandsInfoNV,
    ) {
        unsafe {
            raw::cmd_preprocess_generated_commands_nv(
                self,
                p_generated_commands_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteGeneratedCommandsNV.html>"]
    #[doc(alias = "vkCmdExecuteGeneratedCommandsNV")]
    pub fn execute_generated_commands_nv(
        &self,
        is_preprocessed: impl Into<Bool32>,
        p_generated_commands_info: &GeneratedCommandsInfoNV,
    ) {
        unsafe {
            raw::cmd_execute_generated_commands_nv(
                self,
                is_preprocessed,
                p_generated_commands_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipelineShaderGroupNV.html>"]
    #[doc(alias = "vkCmdBindPipelineShaderGroupNV")]
    pub fn bind_pipeline_shader_group_nv(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: &raw::Pipeline,
        group_index: u32,
    ) {
        unsafe {
            raw::cmd_bind_pipeline_shader_group_nv(
                self,
                pipeline_bind_point,
                pipeline,
                group_index,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBias2EXT.html>"]
    #[doc(alias = "vkCmdSetDepthBias2EXT")]
    pub fn set_depth_bias2_ext(&self, p_depth_bias_info: &DepthBiasInfoEXT) {
        unsafe {
            raw::cmd_set_depth_bias2_ext(
                self,
                p_depth_bias_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCudaLaunchKernelNV.html>"]
    #[doc(alias = "vkCmdCudaLaunchKernelNV")]
    pub fn cuda_launch_kernel_nv(&self, p_launch_info: &CudaLaunchInfoNV) {
        unsafe {
            raw::cmd_cuda_launch_kernel_nv(self, p_launch_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarker2AMD.html>"]
    #[doc(alias = "vkCmdWriteBufferMarker2AMD")]
    pub fn write_buffer_marker2_amd(
        &self,
        stage: PipelineStageFlags2,
        dst_buffer: &raw::Buffer,
        dst_offset: DeviceSize,
        marker: u32,
    ) {
        unsafe {
            raw::cmd_write_buffer_marker2_amd(
                self,
                stage,
                dst_buffer,
                dst_offset,
                marker,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBuffersEXT.html>"]
    #[doc(alias = "vkCmdBindDescriptorBuffersEXT")]
    pub fn bind_descriptor_buffers_ext<'a>(
        &self,
        p_binding_infos: impl AsSlice<'a, DescriptorBufferBindingInfoEXT<'a>>,
    ) {
        unsafe {
            raw::cmd_bind_descriptor_buffers_ext(
                self,
                p_binding_infos,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDescriptorBufferOffsetsEXT.html>"]
    #[doc(alias = "vkCmdSetDescriptorBufferOffsetsEXT")]
    pub fn set_descriptor_buffer_offsets_ext<'a>(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: &raw::PipelineLayout,
        first_set: u32,
        p_buffer_indices: impl AsSlice<'a, u32>,
        p_offsets: impl AsSlice<'a, DeviceSize>,
    ) {
        unsafe {
            raw::cmd_set_descriptor_buffer_offsets_ext(
                self,
                pipeline_bind_point,
                layout,
                first_set,
                p_buffer_indices,
                p_offsets,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html>"]
    #[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplersEXT")]
    pub fn bind_descriptor_buffer_embedded_samplers_ext(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: &raw::PipelineLayout,
        set: u32,
    ) {
        unsafe {
            raw::cmd_bind_descriptor_buffer_embedded_samplers_ext(
                self,
                pipeline_bind_point,
                layout,
                set,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateEnumNV.html>"]
    #[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
    pub fn set_fragment_shading_rate_enum_nv(
        &self,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: [FragmentShadingRateCombinerOpKHR; 2u16 as _],
    ) {
        unsafe {
            raw::cmd_set_fragment_shading_rate_enum_nv(
                self,
                shading_rate,
                combiner_ops,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksEXT.html>"]
    #[doc(alias = "vkCmdDrawMeshTasksEXT")]
    pub fn draw_mesh_tasks_ext(&self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        unsafe {
            raw::cmd_draw_mesh_tasks_ext(
                self,
                group_count_x,
                group_count_y,
                group_count_z,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectEXT.html>"]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectEXT")]
    pub fn draw_mesh_tasks_indirect_ext(
        &self,
        buffer: &raw::Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            raw::cmd_draw_mesh_tasks_indirect_ext(
                self,
                buffer,
                offset,
                draw_count,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountEXT.html>"]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectCountEXT")]
    pub fn draw_mesh_tasks_indirect_count_ext(
        &self,
        buffer: &raw::Buffer,
        offset: DeviceSize,
        count_buffer: &raw::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            raw::cmd_draw_mesh_tasks_indirect_count_ext(
                self,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetVertexInputEXT.html>"]
    #[doc(alias = "vkCmdSetVertexInputEXT")]
    pub fn set_vertex_input_ext<'a>(
        &self,
        p_vertex_binding_descriptions: impl AsSlice<'a, VertexInputBindingDescription2EXT<'a>>,
        p_vertex_attribute_descriptions: impl AsSlice<'a, VertexInputAttributeDescription2EXT<'a>>,
    ) {
        unsafe {
            raw::cmd_set_vertex_input_ext(
                self,
                p_vertex_binding_descriptions,
                p_vertex_attribute_descriptions,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSubpassShadingHUAWEI.html>"]
    #[doc(alias = "vkCmdSubpassShadingHUAWEI")]
    pub fn subpass_shading_huawei(&self) {
        unsafe { raw::cmd_subpass_shading_huawei(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindInvocationMaskHUAWEI.html>"]
    #[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
    pub fn bind_invocation_mask_huawei(
        &self,
        image_view: Option<&raw::ImageView>,
        image_layout: ImageLayout,
    ) {
        unsafe {
            raw::cmd_bind_invocation_mask_huawei(
                self,
                image_view,
                image_layout,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPatchControlPointsEXT.html>"]
    #[doc(alias = "vkCmdSetPatchControlPointsEXT")]
    pub fn set_patch_control_points_ext(&self, patch_control_points: u32) {
        unsafe {
            raw::cmd_set_patch_control_points_ext(
                self,
                patch_control_points,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEXT.html>"]
    #[doc(alias = "vkCmdSetLogicOpEXT")]
    pub fn set_logic_op_ext(&self, logic_op: LogicOp) {
        unsafe { raw::cmd_set_logic_op_ext(self, logic_op, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteEnableEXT.html>"]
    #[doc(alias = "vkCmdSetColorWriteEnableEXT")]
    pub fn set_color_write_enable_ext<'a>(&self, p_color_write_enables: impl AsSlice<'a, Bool32>) {
        unsafe {
            raw::cmd_set_color_write_enable_ext(
                self,
                p_color_write_enables,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirect2KHR.html>"]
    #[doc(alias = "vkCmdTraceRaysIndirect2KHR")]
    pub fn trace_rays_indirect2_khr(&self, indirect_device_address: DeviceAddress) {
        unsafe {
            raw::cmd_trace_rays_indirect2_khr(
                self,
                indirect_device_address,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiEXT.html>"]
    #[doc(alias = "vkCmdDrawMultiEXT")]
    pub fn draw_multi_ext<'a>(
        &self,
        p_vertex_info: impl AsSlice<'a, MultiDrawInfoEXT>,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    ) {
        unsafe {
            raw::cmd_draw_multi_ext(
                self,
                p_vertex_info,
                instance_count,
                first_instance,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiIndexedEXT.html>"]
    #[doc(alias = "vkCmdDrawMultiIndexedEXT")]
    pub fn draw_multi_indexed_ext<'a>(
        &self,
        p_index_info: impl AsSlice<'a, MultiDrawIndexedInfoEXT>,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        p_vertex_offset: Option<&i32>,
    ) {
        unsafe {
            raw::cmd_draw_multi_indexed_ext(
                self,
                p_index_info,
                instance_count,
                first_instance,
                stride,
                p_vertex_offset,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildMicromapsEXT.html>"]
    #[doc(alias = "vkCmdBuildMicromapsEXT")]
    pub fn build_micromaps_ext<'a>(&self, p_infos: impl AsSlice<'a, MicromapBuildInfoEXT<'a>>) {
        unsafe { raw::cmd_build_micromaps_ext(self, p_infos, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMicromapEXT.html>"]
    #[doc(alias = "vkCmdCopyMicromapEXT")]
    pub fn copy_micromap_ext(&self, p_info: &CopyMicromapInfoEXT) {
        unsafe { raw::cmd_copy_micromap_ext(self, p_info, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMicromapToMemoryEXT.html>"]
    #[doc(alias = "vkCmdCopyMicromapToMemoryEXT")]
    pub fn copy_micromap_to_memory_ext(&self, p_info: &CopyMicromapToMemoryInfoEXT) {
        unsafe {
            raw::cmd_copy_micromap_to_memory_ext(self, p_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToMicromapEXT.html>"]
    #[doc(alias = "vkCmdCopyMemoryToMicromapEXT")]
    pub fn copy_memory_to_micromap_ext(&self, p_info: &CopyMemoryToMicromapInfoEXT) {
        unsafe {
            raw::cmd_copy_memory_to_micromap_ext(self, p_info, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteMicromapsPropertiesEXT.html>"]
    #[doc(alias = "vkCmdWriteMicromapsPropertiesEXT")]
    pub fn write_micromaps_properties_ext<'a, V2: Alias<raw::MicromapEXT> + 'a>(
        &self,
        p_micromaps: impl AsSlice<'a, V2>,
        query_type: QueryType,
        query_pool: &raw::QueryPool,
        first_query: u32,
    ) {
        unsafe {
            raw::cmd_write_micromaps_properties_ext(
                self,
                p_micromaps,
                query_type,
                query_pool,
                first_query,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawClusterHUAWEI.html>"]
    #[doc(alias = "vkCmdDrawClusterHUAWEI")]
    pub fn draw_cluster_huawei(&self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        unsafe {
            raw::cmd_draw_cluster_huawei(
                self,
                group_count_x,
                group_count_y,
                group_count_z,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawClusterIndirectHUAWEI.html>"]
    #[doc(alias = "vkCmdDrawClusterIndirectHUAWEI")]
    pub fn draw_cluster_indirect_huawei(&self, buffer: &raw::Buffer, offset: DeviceSize) {
        unsafe {
            raw::cmd_draw_cluster_indirect_huawei(
                self,
                buffer,
                offset,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryIndirectNV.html>"]
    #[doc(alias = "vkCmdCopyMemoryIndirectNV")]
    pub fn copy_memory_indirect_nv(
        &self,
        copy_buffer_address: DeviceAddress,
        copy_count: u32,
        stride: u32,
    ) {
        unsafe {
            raw::cmd_copy_memory_indirect_nv(
                self,
                copy_buffer_address,
                copy_count,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToImageIndirectNV.html>"]
    #[doc(alias = "vkCmdCopyMemoryToImageIndirectNV")]
    pub fn copy_memory_to_image_indirect_nv<'a>(
        &self,
        copy_buffer_address: DeviceAddress,
        stride: u32,
        dst_image: &raw::Image,
        dst_image_layout: ImageLayout,
        p_image_subresources: impl AsSlice<'a, ImageSubresourceLayers>,
    ) {
        unsafe {
            raw::cmd_copy_memory_to_image_indirect_nv(
                self,
                copy_buffer_address,
                stride,
                dst_image,
                dst_image_layout,
                p_image_subresources,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDecompressMemoryNV.html>"]
    #[doc(alias = "vkCmdDecompressMemoryNV")]
    pub fn decompress_memory_nv<'a>(
        &self,
        p_decompress_memory_regions: impl AsSlice<'a, DecompressMemoryRegionNV>,
    ) {
        unsafe {
            raw::cmd_decompress_memory_nv(
                self,
                p_decompress_memory_regions,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDecompressMemoryIndirectCountNV.html>"]
    #[doc(alias = "vkCmdDecompressMemoryIndirectCountNV")]
    pub fn decompress_memory_indirect_count_nv(
        &self,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        stride: u32,
    ) {
        unsafe {
            raw::cmd_decompress_memory_indirect_count_nv(
                self,
                indirect_commands_address,
                indirect_commands_count_address,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdUpdatePipelineIndirectBufferNV.html>"]
    #[doc(alias = "vkCmdUpdatePipelineIndirectBufferNV")]
    pub fn update_pipeline_indirect_buffer_nv(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: &raw::Pipeline,
    ) {
        unsafe {
            raw::cmd_update_pipeline_indirect_buffer_nv(
                self,
                pipeline_bind_point,
                pipeline,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClampEnableEXT.html>"]
    #[doc(alias = "vkCmdSetDepthClampEnableEXT")]
    pub fn set_depth_clamp_enable_ext(&self, depth_clamp_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_clamp_enable_ext(
                self,
                depth_clamp_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPolygonModeEXT.html>"]
    #[doc(alias = "vkCmdSetPolygonModeEXT")]
    pub fn set_polygon_mode_ext(&self, polygon_mode: PolygonMode) {
        unsafe {
            raw::cmd_set_polygon_mode_ext(self, polygon_mode, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationSamplesEXT.html>"]
    #[doc(alias = "vkCmdSetRasterizationSamplesEXT")]
    pub fn set_rasterization_samples_ext(&self, rasterization_samples: SampleCountFlags) {
        unsafe {
            raw::cmd_set_rasterization_samples_ext(
                self,
                rasterization_samples,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleMaskEXT.html>"]
    #[doc(alias = "vkCmdSetSampleMaskEXT")]
    pub fn set_sample_mask_ext<'a>(
        &self,
        samples: SampleCountFlags,
        p_sample_mask: impl AsSlice<'a, SampleMask>,
    ) {
        unsafe {
            raw::cmd_set_sample_mask_ext(
                self,
                samples,
                p_sample_mask,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToCoverageEnableEXT.html>"]
    #[doc(alias = "vkCmdSetAlphaToCoverageEnableEXT")]
    pub fn set_alpha_to_coverage_enable_ext(&self, alpha_to_coverage_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_alpha_to_coverage_enable_ext(
                self,
                alpha_to_coverage_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToOneEnableEXT.html>"]
    #[doc(alias = "vkCmdSetAlphaToOneEnableEXT")]
    pub fn set_alpha_to_one_enable_ext(&self, alpha_to_one_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_alpha_to_one_enable_ext(
                self,
                alpha_to_one_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEnableEXT.html>"]
    #[doc(alias = "vkCmdSetLogicOpEnableEXT")]
    pub fn set_logic_op_enable_ext(&self, logic_op_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_logic_op_enable_ext(
                self,
                logic_op_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEnableEXT.html>"]
    #[doc(alias = "vkCmdSetColorBlendEnableEXT")]
    pub fn set_color_blend_enable_ext<'a>(
        &self,
        first_attachment: u32,
        p_color_blend_enables: impl AsSlice<'a, Bool32>,
    ) {
        unsafe {
            raw::cmd_set_color_blend_enable_ext(
                self,
                first_attachment,
                p_color_blend_enables,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEquationEXT.html>"]
    #[doc(alias = "vkCmdSetColorBlendEquationEXT")]
    pub fn set_color_blend_equation_ext<'a>(
        &self,
        first_attachment: u32,
        p_color_blend_equations: impl AsSlice<'a, ColorBlendEquationEXT>,
    ) {
        unsafe {
            raw::cmd_set_color_blend_equation_ext(
                self,
                first_attachment,
                p_color_blend_equations,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteMaskEXT.html>"]
    #[doc(alias = "vkCmdSetColorWriteMaskEXT")]
    pub fn set_color_write_mask_ext<'a>(
        &self,
        first_attachment: u32,
        p_color_write_masks: impl AsSlice<'a, ColorComponentFlags>,
    ) {
        unsafe {
            raw::cmd_set_color_write_mask_ext(
                self,
                first_attachment,
                p_color_write_masks,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetTessellationDomainOriginEXT.html>"]
    #[doc(alias = "vkCmdSetTessellationDomainOriginEXT")]
    pub fn set_tessellation_domain_origin_ext(&self, domain_origin: TessellationDomainOrigin) {
        unsafe {
            raw::cmd_set_tessellation_domain_origin_ext(
                self,
                domain_origin,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationStreamEXT.html>"]
    #[doc(alias = "vkCmdSetRasterizationStreamEXT")]
    pub fn set_rasterization_stream_ext(&self, rasterization_stream: u32) {
        unsafe {
            raw::cmd_set_rasterization_stream_ext(
                self,
                rasterization_stream,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetConservativeRasterizationModeEXT.html>"]
    #[doc(alias = "vkCmdSetConservativeRasterizationModeEXT")]
    pub fn set_conservative_rasterization_mode_ext(
        &self,
        conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    ) {
        unsafe {
            raw::cmd_set_conservative_rasterization_mode_ext(
                self,
                conservative_rasterization_mode,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html>"]
    #[doc(alias = "vkCmdSetExtraPrimitiveOverestimationSizeEXT")]
    pub fn set_extra_primitive_overestimation_size_ext(
        &self,
        extra_primitive_overestimation_size: f32,
    ) {
        unsafe {
            raw::cmd_set_extra_primitive_overestimation_size_ext(
                self,
                extra_primitive_overestimation_size,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipEnableEXT.html>"]
    #[doc(alias = "vkCmdSetDepthClipEnableEXT")]
    pub fn set_depth_clip_enable_ext(&self, depth_clip_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_clip_enable_ext(
                self,
                depth_clip_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEnableEXT.html>"]
    #[doc(alias = "vkCmdSetSampleLocationsEnableEXT")]
    pub fn set_sample_locations_enable_ext(&self, sample_locations_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_sample_locations_enable_ext(
                self,
                sample_locations_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendAdvancedEXT.html>"]
    #[doc(alias = "vkCmdSetColorBlendAdvancedEXT")]
    pub fn set_color_blend_advanced_ext<'a>(
        &self,
        first_attachment: u32,
        p_color_blend_advanced: impl AsSlice<'a, ColorBlendAdvancedEXT>,
    ) {
        unsafe {
            raw::cmd_set_color_blend_advanced_ext(
                self,
                first_attachment,
                p_color_blend_advanced,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetProvokingVertexModeEXT.html>"]
    #[doc(alias = "vkCmdSetProvokingVertexModeEXT")]
    pub fn set_provoking_vertex_mode_ext(&self, provoking_vertex_mode: ProvokingVertexModeEXT) {
        unsafe {
            raw::cmd_set_provoking_vertex_mode_ext(
                self,
                provoking_vertex_mode,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineRasterizationModeEXT.html>"]
    #[doc(alias = "vkCmdSetLineRasterizationModeEXT")]
    pub fn set_line_rasterization_mode_ext(
        &self,
        line_rasterization_mode: LineRasterizationModeEXT,
    ) {
        unsafe {
            raw::cmd_set_line_rasterization_mode_ext(
                self,
                line_rasterization_mode,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEnableEXT.html>"]
    #[doc(alias = "vkCmdSetLineStippleEnableEXT")]
    pub fn set_line_stipple_enable_ext(&self, stippled_line_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_line_stipple_enable_ext(
                self,
                stippled_line_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipNegativeOneToOneEXT.html>"]
    #[doc(alias = "vkCmdSetDepthClipNegativeOneToOneEXT")]
    pub fn set_depth_clip_negative_one_to_one_ext(&self, negative_one_to_one: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_clip_negative_one_to_one_ext(
                self,
                negative_one_to_one,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingEnableNV.html>"]
    #[doc(alias = "vkCmdSetViewportWScalingEnableNV")]
    pub fn set_viewport_wscaling_enable_nv(&self, viewport_wscaling_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_viewport_wscaling_enable_nv(
                self,
                viewport_wscaling_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportSwizzleNV.html>"]
    #[doc(alias = "vkCmdSetViewportSwizzleNV")]
    pub fn set_viewport_swizzle_nv<'a>(
        &self,
        first_viewport: u32,
        p_viewport_swizzles: impl AsSlice<'a, ViewportSwizzleNV>,
    ) {
        unsafe {
            raw::cmd_set_viewport_swizzle_nv(
                self,
                first_viewport,
                p_viewport_swizzles,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorEnableNV.html>"]
    #[doc(alias = "vkCmdSetCoverageToColorEnableNV")]
    pub fn set_coverage_to_color_enable_nv(&self, coverage_to_color_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_coverage_to_color_enable_nv(
                self,
                coverage_to_color_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorLocationNV.html>"]
    #[doc(alias = "vkCmdSetCoverageToColorLocationNV")]
    pub fn set_coverage_to_color_location_nv(&self, coverage_to_color_location: u32) {
        unsafe {
            raw::cmd_set_coverage_to_color_location_nv(
                self,
                coverage_to_color_location,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationModeNV.html>"]
    #[doc(alias = "vkCmdSetCoverageModulationModeNV")]
    pub fn set_coverage_modulation_mode_nv(
        &self,
        coverage_modulation_mode: CoverageModulationModeNV,
    ) {
        unsafe {
            raw::cmd_set_coverage_modulation_mode_nv(
                self,
                coverage_modulation_mode,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableEnableNV.html>"]
    #[doc(alias = "vkCmdSetCoverageModulationTableEnableNV")]
    pub fn set_coverage_modulation_table_enable_nv(
        &self,
        coverage_modulation_table_enable: impl Into<Bool32>,
    ) {
        unsafe {
            raw::cmd_set_coverage_modulation_table_enable_nv(
                self,
                coverage_modulation_table_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableNV.html>"]
    #[doc(alias = "vkCmdSetCoverageModulationTableNV")]
    pub fn set_coverage_modulation_table_nv<'a>(
        &self,
        p_coverage_modulation_table: impl AsSlice<'a, f32>,
    ) {
        unsafe {
            raw::cmd_set_coverage_modulation_table_nv(
                self,
                p_coverage_modulation_table,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetShadingRateImageEnableNV.html>"]
    #[doc(alias = "vkCmdSetShadingRateImageEnableNV")]
    pub fn set_shading_rate_image_enable_nv(&self, shading_rate_image_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_shading_rate_image_enable_nv(
                self,
                shading_rate_image_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRepresentativeFragmentTestEnableNV.html>"]
    #[doc(alias = "vkCmdSetRepresentativeFragmentTestEnableNV")]
    pub fn set_representative_fragment_test_enable_nv(
        &self,
        representative_fragment_test_enable: impl Into<Bool32>,
    ) {
        unsafe {
            raw::cmd_set_representative_fragment_test_enable_nv(
                self,
                representative_fragment_test_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageReductionModeNV.html>"]
    #[doc(alias = "vkCmdSetCoverageReductionModeNV")]
    pub fn set_coverage_reduction_mode_nv(&self, coverage_reduction_mode: CoverageReductionModeNV) {
        unsafe {
            raw::cmd_set_coverage_reduction_mode_nv(
                self,
                coverage_reduction_mode,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdOpticalFlowExecuteNV.html>"]
    #[doc(alias = "vkCmdOpticalFlowExecuteNV")]
    pub fn optical_flow_execute_nv(
        &self,
        session: &raw::OpticalFlowSessionNV,
        p_execute_info: &OpticalFlowExecuteInfoNV,
    ) {
        unsafe {
            raw::cmd_optical_flow_execute_nv(
                self,
                session,
                p_execute_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindIndexBuffer2KHR.html>"]
    #[doc(alias = "vkCmdBindIndexBuffer2KHR")]
    pub fn bind_index_buffer2_khr(
        &self,
        buffer: Option<&raw::Buffer>,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ) {
        unsafe {
            raw::cmd_bind_index_buffer2_khr(
                self,
                buffer,
                offset,
                size,
                index_type,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindShadersEXT.html>"]
    #[doc(alias = "vkCmdBindShadersEXT")]
    pub fn bind_shaders_ext<'a, V3: Alias<raw::ShaderEXT> + 'a>(
        &self,
        p_stages: impl AsSlice<'a, ShaderStageFlags>,
        p_shaders: impl AsSlice<'a, V3>,
    ) {
        unsafe {
            raw::cmd_bind_shaders_ext(
                self,
                p_stages,
                p_shaders,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetAttachmentFeedbackLoopEnableEXT.html>"]
    #[doc(alias = "vkCmdSetAttachmentFeedbackLoopEnableEXT")]
    pub fn set_attachment_feedback_loop_enable_ext(&self, aspect_mask: ImageAspectFlags) {
        unsafe {
            raw::cmd_set_attachment_feedback_loop_enable_ext(
                self,
                aspect_mask,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleKHR.html>"]
    #[doc(alias = "vkCmdSetLineStippleKHR")]
    pub fn set_line_stipple_khr(&self, line_stipple_factor: u32, line_stipple_pattern: u16) {
        unsafe {
            raw::cmd_set_line_stipple_khr(
                self,
                line_stipple_factor,
                line_stipple_pattern,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEXT.html>"]
    #[doc(alias = "vkCmdSetLineStippleEXT")]
    pub fn set_line_stipple_ext(&self, line_stipple_factor: u32, line_stipple_pattern: u16) {
        unsafe {
            raw::cmd_set_line_stipple_ext(
                self,
                line_stipple_factor,
                line_stipple_pattern,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorSets2KHR.html>"]
    #[doc(alias = "vkCmdBindDescriptorSets2KHR")]
    pub fn bind_descriptor_sets2_khr(
        &self,
        p_bind_descriptor_sets_info: &BindDescriptorSetsInfoKHR,
    ) {
        unsafe {
            raw::cmd_bind_descriptor_sets2_khr(
                self,
                p_bind_descriptor_sets_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants2KHR.html>"]
    #[doc(alias = "vkCmdPushConstants2KHR")]
    pub fn push_constants2_khr(&self, p_push_constants_info: &PushConstantsInfoKHR) {
        unsafe {
            raw::cmd_push_constants2_khr(
                self,
                p_push_constants_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSet2KHR.html>"]
    #[doc(alias = "vkCmdPushDescriptorSet2KHR")]
    pub fn push_descriptor_set2_khr(&self, p_push_descriptor_set_info: &PushDescriptorSetInfoKHR) {
        unsafe {
            raw::cmd_push_descriptor_set2_khr(
                self,
                p_push_descriptor_set_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetWithTemplate2KHR.html>"]
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplate2KHR")]
    pub fn push_descriptor_set_with_template2_khr(
        &self,
        p_push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfoKHR,
    ) {
        unsafe {
            raw::cmd_push_descriptor_set_with_template2_khr(
                self,
                p_push_descriptor_set_with_template_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDescriptorBufferOffsets2EXT.html>"]
    #[doc(alias = "vkCmdSetDescriptorBufferOffsets2EXT")]
    pub fn set_descriptor_buffer_offsets2_ext(
        &self,
        p_set_descriptor_buffer_offsets_info: &SetDescriptorBufferOffsetsInfoEXT,
    ) {
        unsafe {
            raw::cmd_set_descriptor_buffer_offsets2_ext(
                self,
                p_set_descriptor_buffer_offsets_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html>"]
    #[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplers2EXT")]
    pub fn bind_descriptor_buffer_embedded_samplers2_ext(
        &self,
        p_bind_descriptor_buffer_embedded_samplers_info : & BindDescriptorBufferEmbeddedSamplersInfoEXT,
    ) {
        unsafe {
            raw::cmd_bind_descriptor_buffer_embedded_samplers2_ext(
                self,
                p_bind_descriptor_buffer_embedded_samplers_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversion.html>"]
#[doc(alias = "VkSamplerYcbcrConversion")]
pub struct SamplerYcbcrConversion {
    inner: <raw::SamplerYcbcrConversion as Handle>::InnerType,
}
unsafe impl Alias<raw::SamplerYcbcrConversion> for SamplerYcbcrConversion {}
impl Deref for SamplerYcbcrConversion {
    type Target = raw::SamplerYcbcrConversion;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl SamplerYcbcrConversion {
    pub fn from_inner(handle: raw::SamplerYcbcrConversion) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionKHR.html>"]
#[doc(alias = "VkSamplerYcbcrConversionKHR")]
pub type SamplerYcbcrConversionKHR = raw::SamplerYcbcrConversion;
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplate.html>"]
#[doc(alias = "VkDescriptorUpdateTemplate")]
pub struct DescriptorUpdateTemplate {
    inner: <raw::DescriptorUpdateTemplate as Handle>::InnerType,
}
unsafe impl Alias<raw::DescriptorUpdateTemplate> for DescriptorUpdateTemplate {}
impl Deref for DescriptorUpdateTemplate {
    type Target = raw::DescriptorUpdateTemplate;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl DescriptorUpdateTemplate {
    pub fn from_inner(handle: raw::DescriptorUpdateTemplate) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateKHR.html>"]
#[doc(alias = "VkDescriptorUpdateTemplateKHR")]
pub type DescriptorUpdateTemplateKHR = raw::DescriptorUpdateTemplate;
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlot.html>"]
#[doc(alias = "VkPrivateDataSlot")]
pub struct PrivateDataSlot {
    inner: <raw::PrivateDataSlot as Handle>::InnerType,
}
unsafe impl Alias<raw::PrivateDataSlot> for PrivateDataSlot {}
impl Deref for PrivateDataSlot {
    type Target = raw::PrivateDataSlot;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl PrivateDataSlot {
    pub fn from_inner(handle: raw::PrivateDataSlot) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlotEXT.html>"]
#[doc(alias = "VkPrivateDataSlotEXT")]
pub type PrivateDataSlotEXT = raw::PrivateDataSlot;
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceKHR.html>"]
#[doc(alias = "VkSurfaceKHR")]
pub struct SurfaceKHR {
    inner: <raw::SurfaceKHR as Handle>::InnerType,
}
unsafe impl Alias<raw::SurfaceKHR> for SurfaceKHR {}
impl Deref for SurfaceKHR {
    type Target = raw::SurfaceKHR;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl SurfaceKHR {
    pub fn from_inner(handle: raw::SurfaceKHR) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainKHR.html>"]
#[doc(alias = "VkSwapchainKHR")]
pub struct SwapchainKHR {
    inner: <raw::SwapchainKHR as Handle>::InnerType,
}
unsafe impl Alias<raw::SwapchainKHR> for SwapchainKHR {}
impl Deref for SwapchainKHR {
    type Target = raw::SwapchainKHR;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl SwapchainKHR {
    pub fn from_inner(handle: raw::SwapchainKHR) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayKHR.html>"]
#[doc(alias = "VkDisplayKHR")]
pub struct DisplayKHR {
    inner: <raw::DisplayKHR as Handle>::InnerType,
}
unsafe impl Alias<raw::DisplayKHR> for DisplayKHR {}
impl Deref for DisplayKHR {
    type Target = raw::DisplayKHR;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl DisplayKHR {
    pub fn from_inner(handle: raw::DisplayKHR) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeKHR.html>"]
#[doc(alias = "VkDisplayModeKHR")]
pub struct DisplayModeKHR {
    inner: <raw::DisplayModeKHR as Handle>::InnerType,
}
unsafe impl Alias<raw::DisplayModeKHR> for DisplayModeKHR {}
impl Deref for DisplayModeKHR {
    type Target = raw::DisplayModeKHR;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl DisplayModeKHR {
    pub fn from_inner(handle: raw::DisplayModeKHR) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugReportCallbackEXT.html>"]
#[doc(alias = "VkDebugReportCallbackEXT")]
pub struct DebugReportCallbackEXT {
    inner: <raw::DebugReportCallbackEXT as Handle>::InnerType,
}
unsafe impl Alias<raw::DebugReportCallbackEXT> for DebugReportCallbackEXT {}
impl Deref for DebugReportCallbackEXT {
    type Target = raw::DebugReportCallbackEXT;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl DebugReportCallbackEXT {
    pub fn from_inner(handle: raw::DebugReportCallbackEXT) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCuModuleNVX.html>"]
#[doc(alias = "VkCuModuleNVX")]
pub struct CuModuleNVX {
    inner: <raw::CuModuleNVX as Handle>::InnerType,
}
unsafe impl Alias<raw::CuModuleNVX> for CuModuleNVX {}
impl Deref for CuModuleNVX {
    type Target = raw::CuModuleNVX;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl CuModuleNVX {
    pub fn from_inner(handle: raw::CuModuleNVX) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCuFunctionNVX.html>"]
#[doc(alias = "VkCuFunctionNVX")]
pub struct CuFunctionNVX {
    inner: <raw::CuFunctionNVX as Handle>::InnerType,
}
unsafe impl Alias<raw::CuFunctionNVX> for CuFunctionNVX {}
impl Deref for CuFunctionNVX {
    type Target = raw::CuFunctionNVX;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl CuFunctionNVX {
    pub fn from_inner(handle: raw::CuFunctionNVX) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerEXT.html>"]
#[doc(alias = "VkDebugUtilsMessengerEXT")]
pub struct DebugUtilsMessengerEXT {
    inner: <raw::DebugUtilsMessengerEXT as Handle>::InnerType,
}
unsafe impl Alias<raw::DebugUtilsMessengerEXT> for DebugUtilsMessengerEXT {}
impl Deref for DebugUtilsMessengerEXT {
    type Target = raw::DebugUtilsMessengerEXT;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl DebugUtilsMessengerEXT {
    pub fn from_inner(handle: raw::DebugUtilsMessengerEXT) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureKHR.html>"]
#[doc(alias = "VkAccelerationStructureKHR")]
pub struct AccelerationStructureKHR {
    inner: <raw::AccelerationStructureKHR as Handle>::InnerType,
}
unsafe impl Alias<raw::AccelerationStructureKHR> for AccelerationStructureKHR {}
impl Deref for AccelerationStructureKHR {
    type Target = raw::AccelerationStructureKHR;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl AccelerationStructureKHR {
    pub fn from_inner(handle: raw::AccelerationStructureKHR) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCacheEXT.html>"]
#[doc(alias = "VkValidationCacheEXT")]
pub struct ValidationCacheEXT {
    inner: <raw::ValidationCacheEXT as Handle>::InnerType,
}
unsafe impl Alias<raw::ValidationCacheEXT> for ValidationCacheEXT {}
impl Deref for ValidationCacheEXT {
    type Target = raw::ValidationCacheEXT;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl ValidationCacheEXT {
    pub fn from_inner(handle: raw::ValidationCacheEXT) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureNV.html>"]
#[doc(alias = "VkAccelerationStructureNV")]
pub struct AccelerationStructureNV {
    inner: <raw::AccelerationStructureNV as Handle>::InnerType,
}
unsafe impl Alias<raw::AccelerationStructureNV> for AccelerationStructureNV {}
impl Deref for AccelerationStructureNV {
    type Target = raw::AccelerationStructureNV;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl AccelerationStructureNV {
    pub fn from_inner(handle: raw::AccelerationStructureNV) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationINTEL.html>"]
#[doc(alias = "VkPerformanceConfigurationINTEL")]
pub struct PerformanceConfigurationINTEL {
    inner: <raw::PerformanceConfigurationINTEL as Handle>::InnerType,
}
unsafe impl Alias<raw::PerformanceConfigurationINTEL> for PerformanceConfigurationINTEL {}
impl Deref for PerformanceConfigurationINTEL {
    type Target = raw::PerformanceConfigurationINTEL;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl PerformanceConfigurationINTEL {
    pub fn from_inner(handle: raw::PerformanceConfigurationINTEL) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeferredOperationKHR.html>"]
#[doc(alias = "VkDeferredOperationKHR")]
pub struct DeferredOperationKHR {
    inner: <raw::DeferredOperationKHR as Handle>::InnerType,
}
unsafe impl Alias<raw::DeferredOperationKHR> for DeferredOperationKHR {}
impl Deref for DeferredOperationKHR {
    type Target = raw::DeferredOperationKHR;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl DeferredOperationKHR {
    pub fn from_inner(handle: raw::DeferredOperationKHR) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutNV.html>"]
#[doc(alias = "VkIndirectCommandsLayoutNV")]
pub struct IndirectCommandsLayoutNV {
    inner: <raw::IndirectCommandsLayoutNV as Handle>::InnerType,
}
unsafe impl Alias<raw::IndirectCommandsLayoutNV> for IndirectCommandsLayoutNV {}
impl Deref for IndirectCommandsLayoutNV {
    type Target = raw::IndirectCommandsLayoutNV;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl IndirectCommandsLayoutNV {
    pub fn from_inner(handle: raw::IndirectCommandsLayoutNV) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCudaModuleNV.html>"]
#[doc(alias = "VkCudaModuleNV")]
pub struct CudaModuleNV {
    inner: <raw::CudaModuleNV as Handle>::InnerType,
}
unsafe impl Alias<raw::CudaModuleNV> for CudaModuleNV {}
impl Deref for CudaModuleNV {
    type Target = raw::CudaModuleNV;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl CudaModuleNV {
    pub fn from_inner(handle: raw::CudaModuleNV) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCudaFunctionNV.html>"]
#[doc(alias = "VkCudaFunctionNV")]
pub struct CudaFunctionNV {
    inner: <raw::CudaFunctionNV as Handle>::InnerType,
}
unsafe impl Alias<raw::CudaFunctionNV> for CudaFunctionNV {}
impl Deref for CudaFunctionNV {
    type Target = raw::CudaFunctionNV;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl CudaFunctionNV {
    pub fn from_inner(handle: raw::CudaFunctionNV) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionFUCHSIA.html>"]
#[doc(alias = "VkBufferCollectionFUCHSIA")]
pub struct BufferCollectionFUCHSIA {
    inner: <raw::BufferCollectionFUCHSIA as Handle>::InnerType,
}
unsafe impl Alias<raw::BufferCollectionFUCHSIA> for BufferCollectionFUCHSIA {}
impl Deref for BufferCollectionFUCHSIA {
    type Target = raw::BufferCollectionFUCHSIA;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl BufferCollectionFUCHSIA {
    pub fn from_inner(handle: raw::BufferCollectionFUCHSIA) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMicromapEXT.html>"]
#[doc(alias = "VkMicromapEXT")]
pub struct MicromapEXT {
    inner: <raw::MicromapEXT as Handle>::InnerType,
}
unsafe impl Alias<raw::MicromapEXT> for MicromapEXT {}
impl Deref for MicromapEXT {
    type Target = raw::MicromapEXT;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl MicromapEXT {
    pub fn from_inner(handle: raw::MicromapEXT) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionNV.html>"]
#[doc(alias = "VkOpticalFlowSessionNV")]
pub struct OpticalFlowSessionNV {
    inner: <raw::OpticalFlowSessionNV as Handle>::InnerType,
}
unsafe impl Alias<raw::OpticalFlowSessionNV> for OpticalFlowSessionNV {}
impl Deref for OpticalFlowSessionNV {
    type Target = raw::OpticalFlowSessionNV;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl OpticalFlowSessionNV {
    pub fn from_inner(handle: raw::OpticalFlowSessionNV) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderEXT.html>"]
#[doc(alias = "VkShaderEXT")]
pub struct ShaderEXT {
    inner: <raw::ShaderEXT as Handle>::InnerType,
}
unsafe impl Alias<raw::ShaderEXT> for ShaderEXT {}
impl Deref for ShaderEXT {
    type Target = raw::ShaderEXT;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl ShaderEXT {
    pub fn from_inner(handle: raw::ShaderEXT) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
