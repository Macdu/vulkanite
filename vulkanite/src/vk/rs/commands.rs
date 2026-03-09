#![allow(unused_unsafe)]
#[allow(unused_imports)]
use crate::StructureChainOut;
use crate::{
    vk::*, AdvancedDynamicArray, Alias, AsSlice, BaseAllocator, DefaultAllocator, Dispatcher,
    DynamicArray, DynamicDispatcher, Handle,
};
#[allow(unused_imports)]
use std::{
    ffi::{c_int, CStr},
    ops::Deref,
};
#[derive(Clone)]
pub struct Entry<D: Dispatcher = DynamicDispatcher, A: BaseAllocator = DefaultAllocator> {
    disp: D,
    alloc: A,
}
impl<D: Dispatcher, A: BaseAllocator> Copy for Entry<D, A>
where
    D: Copy,
    A: Copy,
{
}
impl<D: Dispatcher, A: BaseAllocator> Entry<D, A> {
    pub fn new(disp: D, alloc: A) -> Self {
        Self { disp, alloc }
    }
    #[inline(always)]
    pub fn get_dispatcher(&self) -> &D {
        &self.disp
    }
    #[inline(always)]
    pub fn get_allocator(&self) -> &A {
        &self.alloc
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateInstance.html>"]
    #[doc(alias = "vkCreateInstance")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html>"]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceLayerProperties.html>"]
    #[doc(alias = "vkEnumerateInstanceLayerProperties")]
    pub fn enumerate_instance_layer_properties<R: DynamicArray<LayerProperties>>(
        &self,
    ) -> Result<R> {
        unsafe { raw::enumerate_instance_layer_properties(self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "version_1_1")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceVersion.html>"]
    #[doc(alias = "vkEnumerateInstanceVersion")]
    #[inline]
    pub fn enumerate_instance_version(&self) -> Result<u32> {
        unsafe { raw::enumerate_instance_version(self.disp.get_command_dispatcher()) }
    }
}
#[repr(C)]
#[derive(Clone)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkInstance.html>"]
#[doc(alias = "VkInstance")]
pub struct Instance<D: Dispatcher = DynamicDispatcher, A: BaseAllocator = DefaultAllocator> {
    inner: <raw::Instance as Handle>::InnerType,
    disp: D,
    alloc: A,
}
unsafe impl Alias<raw::Instance> for Instance {}
impl<D: Dispatcher, A: BaseAllocator> Copy for Instance<D, A>
where
    D: Copy,
    A: Copy,
{
}
impl<D: Dispatcher, A: BaseAllocator> Deref for Instance<D, A> {
    type Target = raw::Instance;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl<D: Dispatcher, A: BaseAllocator> Instance<D, A> {
    pub unsafe fn from_inner(handle: raw::Instance, disp: D, alloc: A) -> Self {
        Self {
            inner: handle.as_raw(),
            disp,
            alloc,
        }
    }
    #[inline(always)]
    pub fn get_dispatcher(&self) -> &D {
        &self.disp
    }
    #[inline(always)]
    pub fn get_allocator(&self) -> &A {
        &self.alloc
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyInstance.html>"]
    #[doc(alias = "vkDestroyInstance")]
    #[inline]
    pub unsafe fn destroy(&self) {
        unsafe {
            raw::destroy_instance(
                Some(self),
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDevices.html>"]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html>"]
    #[doc(alias = "vkGetInstanceProcAddr")]
    #[inline]
    pub fn get_proc_addr(&self, p_name: &CStr) -> FuncPtr {
        unsafe {
            raw::get_instance_proc_addr(Some(self), p_name, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_device_group_creation", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html>"]
    #[doc(alias = "vkEnumeratePhysicalDeviceGroups")]
    pub fn enumerate_physical_device_groups<
        R: DynamicArray<PhysicalDeviceGroupProperties<'static>>,
    >(
        &self,
    ) -> Result<R> {
        unsafe { raw::enumerate_physical_device_groups(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(feature = "ext_device_group_creation", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroupsKHR.html>"]
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
    #[cfg(feature = "ext_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySurfaceKHR.html>"]
    #[doc(alias = "vkDestroySurfaceKHR")]
    #[inline]
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
    #[cfg(feature = "ext_display")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayPlaneSurfaceKHR.html>"]
    #[doc(alias = "vkCreateDisplayPlaneSurfaceKHR")]
    #[inline]
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
    #[cfg(feature = "ext_xlib_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXlibSurfaceKHR.html>"]
    #[doc(alias = "vkCreateXlibSurfaceKHR")]
    #[inline]
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
    #[cfg(feature = "ext_xcb_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXcbSurfaceKHR.html>"]
    #[doc(alias = "vkCreateXcbSurfaceKHR")]
    #[inline]
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
    #[cfg(feature = "ext_wayland_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWaylandSurfaceKHR.html>"]
    #[doc(alias = "vkCreateWaylandSurfaceKHR")]
    #[inline]
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
    #[cfg(feature = "ext_android_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAndroidSurfaceKHR.html>"]
    #[doc(alias = "vkCreateAndroidSurfaceKHR")]
    #[inline]
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
    #[cfg(feature = "ext_win32_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWin32SurfaceKHR.html>"]
    #[doc(alias = "vkCreateWin32SurfaceKHR")]
    #[inline]
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
    #[cfg(feature = "ext_debug_report")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugReportCallbackEXT.html>"]
    #[doc(alias = "vkCreateDebugReportCallbackEXT")]
    #[inline]
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
    #[cfg(feature = "ext_debug_report")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugReportCallbackEXT.html>"]
    #[doc(alias = "vkDestroyDebugReportCallbackEXT")]
    #[inline]
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
    #[cfg(feature = "ext_debug_report")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugReportMessageEXT.html>"]
    #[doc(alias = "vkDebugReportMessageEXT")]
    #[inline]
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
    #[cfg(feature = "ext_stream_descriptor_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateStreamDescriptorSurfaceGGP.html>"]
    #[doc(alias = "vkCreateStreamDescriptorSurfaceGGP")]
    #[inline]
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
    #[cfg(feature = "ext_vi_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateViSurfaceNN.html>"]
    #[doc(alias = "vkCreateViSurfaceNN")]
    #[inline]
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
    #[cfg(feature = "ext_ios_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIOSSurfaceMVK.html>"]
    #[doc(alias = "vkCreateIOSSurfaceMVK")]
    #[inline]
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
    #[cfg(feature = "ext_macos_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMacOSSurfaceMVK.html>"]
    #[doc(alias = "vkCreateMacOSSurfaceMVK")]
    #[inline]
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
    #[cfg(feature = "ext_debug_utils")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugUtilsMessengerEXT.html>"]
    #[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
    #[inline]
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
    #[cfg(feature = "ext_debug_utils")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugUtilsMessengerEXT.html>"]
    #[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
    #[inline]
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
    #[cfg(feature = "ext_debug_utils")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSubmitDebugUtilsMessageEXT.html>"]
    #[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
    #[inline]
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
    #[cfg(feature = "ext_imagepipe_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImagePipeSurfaceFUCHSIA.html>"]
    #[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
    #[inline]
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
    #[cfg(feature = "ext_metal_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMetalSurfaceEXT.html>"]
    #[doc(alias = "vkCreateMetalSurfaceEXT")]
    #[inline]
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
    #[cfg(feature = "ext_headless_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateHeadlessSurfaceEXT.html>"]
    #[doc(alias = "vkCreateHeadlessSurfaceEXT")]
    #[inline]
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
    #[cfg(feature = "ext_directfb_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDirectFBSurfaceEXT.html>"]
    #[doc(alias = "vkCreateDirectFBSurfaceEXT")]
    #[inline]
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
    #[cfg(feature = "ext_screen_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateScreenSurfaceQNX.html>"]
    #[doc(alias = "vkCreateScreenSurfaceQNX")]
    #[inline]
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
    #[cfg(feature = "ext_ohos_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSurfaceOHOS.html>"]
    #[doc(alias = "vkCreateSurfaceOHOS")]
    #[inline]
    pub fn create_surface_ohos(&self, p_create_info: &SurfaceCreateInfoOHOS) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_surface_ohos(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { SurfaceKHR::from_inner(vk_result) })
    }
    #[cfg(feature = "ext_ubm_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateUbmSurfaceSEC.html>"]
    #[doc(alias = "vkCreateUbmSurfaceSEC")]
    #[inline]
    pub fn create_ubm_surface_sec(
        &self,
        p_create_info: &UbmSurfaceCreateInfoSEC,
    ) -> Result<SurfaceKHR> {
        let vk_result = unsafe {
            raw::create_ubm_surface_sec(
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice.html>"]
#[doc(alias = "VkPhysicalDevice")]
pub struct PhysicalDevice<D: Dispatcher = DynamicDispatcher, A: BaseAllocator = DefaultAllocator> {
    inner: <raw::PhysicalDevice as Handle>::InnerType,
    disp: D,
    alloc: A,
}
unsafe impl Alias<raw::PhysicalDevice> for PhysicalDevice {}
impl<D: Dispatcher, A: BaseAllocator> Copy for PhysicalDevice<D, A>
where
    D: Copy,
    A: Copy,
{
}
impl<D: Dispatcher, A: BaseAllocator> Deref for PhysicalDevice<D, A> {
    type Target = raw::PhysicalDevice;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl<D: Dispatcher, A: BaseAllocator> PhysicalDevice<D, A> {
    pub unsafe fn from_inner(handle: raw::PhysicalDevice, disp: D, alloc: A) -> Self {
        Self {
            inner: handle.as_raw(),
            disp,
            alloc,
        }
    }
    #[inline(always)]
    pub fn get_dispatcher(&self) -> &D {
        &self.disp
    }
    #[inline(always)]
    pub fn get_allocator(&self) -> &A {
        &self.alloc
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures.html>"]
    #[doc(alias = "vkGetPhysicalDeviceFeatures")]
    #[inline]
    pub fn get_features(&self) -> PhysicalDeviceFeatures {
        unsafe { raw::get_physical_device_features(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties.html>"]
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties")]
    #[inline]
    pub fn get_format_properties(&self, format: Format) -> FormatProperties {
        unsafe {
            raw::get_physical_device_format_properties(
                self,
                format,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties.html>"]
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties.html>"]
    #[doc(alias = "vkGetPhysicalDeviceProperties")]
    #[inline]
    pub fn get_properties(&self) -> PhysicalDeviceProperties {
        unsafe { raw::get_physical_device_properties(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties.html>"]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties")]
    pub fn get_queue_family_properties<R: DynamicArray<QueueFamilyProperties>>(&self) -> R {
        unsafe {
            raw::get_physical_device_queue_family_properties(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties.html>"]
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties")]
    #[inline]
    pub fn get_memory_properties(&self) -> PhysicalDeviceMemoryProperties {
        unsafe {
            raw::get_physical_device_memory_properties(self, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDevice.html>"]
    #[doc(alias = "vkCreateDevice")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceExtensionProperties.html>"]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceLayerProperties.html>"]
    #[doc(alias = "vkEnumerateDeviceLayerProperties")]
    pub fn enumerate_device_layer_properties<R: DynamicArray<LayerProperties>>(&self) -> Result<R> {
        unsafe { raw::enumerate_device_layer_properties(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties.html>"]
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
    #[cfg(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2.html>"]
    #[doc(alias = "vkGetPhysicalDeviceFeatures2")]
    pub fn get_features2<S: StructureChainOut<PhysicalDeviceFeatures2<'static>>>(&self) -> S {
        unsafe { raw::get_physical_device_features2(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2KHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceFeatures2KHR")]
    pub fn get_features2_khr<S: StructureChainOut<PhysicalDeviceFeatures2<'static>>>(&self) -> S {
        unsafe { raw::get_physical_device_features2_khr(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2.html>"]
    #[doc(alias = "vkGetPhysicalDeviceProperties2")]
    pub fn get_properties2<S: StructureChainOut<PhysicalDeviceProperties2<'static>>>(&self) -> S {
        unsafe { raw::get_physical_device_properties2(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2KHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceProperties2KHR")]
    pub fn get_properties2_khr<S: StructureChainOut<PhysicalDeviceProperties2<'static>>>(
        &self,
    ) -> S {
        unsafe {
            raw::get_physical_device_properties2_khr(self, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2.html>"]
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
    #[cfg(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2KHR.html>"]
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
    #[cfg(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2.html>"]
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
    #[cfg(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2KHR.html>"]
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
    #[cfg(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2.html>"]
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
    #[cfg(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html>"]
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
    #[cfg(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2.html>"]
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
    #[cfg(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2KHR.html>"]
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
    #[cfg(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2.html>"]
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
    #[cfg(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html>"]
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
    #[cfg(any(feature = "ext_external_memory_capabilities", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferProperties.html>"]
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
    #[cfg(any(feature = "ext_external_memory_capabilities", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html>"]
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
    #[cfg(any(feature = "ext_external_fence_capabilities", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFenceProperties.html>"]
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
    #[cfg(any(feature = "ext_external_fence_capabilities", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFencePropertiesKHR.html>"]
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
    #[cfg(any(
        feature = "ext_external_semaphore_capabilities",
        feature = "version_1_1"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphoreProperties.html>"]
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
    #[cfg(any(
        feature = "ext_external_semaphore_capabilities",
        feature = "version_1_1"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html>"]
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
    #[cfg(any(feature = "ext_tooling_info", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolProperties.html>"]
    #[doc(alias = "vkGetPhysicalDeviceToolProperties")]
    pub fn get_tool_properties<R: DynamicArray<PhysicalDeviceToolProperties<'static>>>(
        &self,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_tool_properties(self, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_tooling_info", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolPropertiesEXT.html>"]
    #[doc(alias = "vkGetPhysicalDeviceToolPropertiesEXT")]
    pub fn get_tool_properties_ext<R: DynamicArray<PhysicalDeviceToolProperties<'static>>>(
        &self,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_tool_properties_ext(self, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceSupportKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
    #[inline]
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
    #[cfg(feature = "ext_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
    #[inline]
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
    #[cfg(feature = "ext_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormatsKHR.html>"]
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
    #[cfg(feature = "ext_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModesKHR.html>"]
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
    #[cfg(any(
        all(feature = "ext_swapchain", feature = "version_1_1"),
        all(feature = "ext_device_group", feature = "ext_surface")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDevicePresentRectanglesKHR.html>"]
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
    #[cfg(feature = "ext_display")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPropertiesKHR.html>"]
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
    #[cfg(feature = "ext_display")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html>"]
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
    #[cfg(feature = "ext_display")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneSupportedDisplaysKHR.html>"]
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
    #[cfg(feature = "ext_display")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModePropertiesKHR.html>"]
    #[doc(alias = "vkGetDisplayModePropertiesKHR")]
    pub fn get_display_mode_properties_khr<R: DynamicArray<DisplayModePropertiesKHR<'static>>>(
        &self,
        display: &raw::DisplayKHR,
    ) -> Result<R> {
        unsafe {
            raw::get_display_mode_properties_khr(self, display, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_display")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayModeKHR.html>"]
    #[doc(alias = "vkCreateDisplayModeKHR")]
    #[inline]
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
    #[cfg(feature = "ext_display")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilitiesKHR.html>"]
    #[doc(alias = "vkGetDisplayPlaneCapabilitiesKHR")]
    #[inline]
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
    #[cfg(feature = "ext_xlib_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXlibPresentationSupportKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
    #[inline]
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
    #[cfg(feature = "ext_xcb_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXcbPresentationSupportKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceXcbPresentationSupportKHR")]
    #[inline]
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
    #[cfg(feature = "ext_wayland_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
    #[inline]
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
    #[cfg(feature = "ext_win32_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWin32PresentationSupportKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceWin32PresentationSupportKHR")]
    #[inline]
    pub fn get_win32_presentation_support_khr(&self, queue_family_index: u32) -> bool {
        unsafe {
            raw::get_physical_device_win32_presentation_support_khr(
                self,
                queue_family_index,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_external_memory_capabilities")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html>"]
    #[doc(alias = "vkGetPhysicalDeviceExternalImageFormatPropertiesNV")]
    #[inline]
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
    #[cfg(feature = "ext_direct_mode_display")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseDisplayEXT.html>"]
    #[doc(alias = "vkReleaseDisplayEXT")]
    #[inline]
    pub fn release_display_ext(&self, display: &raw::DisplayKHR) -> Result<()> {
        unsafe { raw::release_display_ext(self, display, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_acquire_xlib_display")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireXlibDisplayEXT.html>"]
    #[doc(alias = "vkAcquireXlibDisplayEXT")]
    #[inline]
    pub fn acquire_xlib_display_ext(&self, dpy: &VoidPtr, display: &raw::DisplayKHR) -> Result<()> {
        unsafe {
            raw::acquire_xlib_display_ext(self, dpy, display, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_acquire_xlib_display")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRandROutputDisplayEXT.html>"]
    #[doc(alias = "vkGetRandROutputDisplayEXT")]
    #[inline]
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
    #[cfg(feature = "ext_display_surface_counter")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html>"]
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
    #[cfg(feature = "ext_performance_query")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")]
    #[inline]
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
    #[cfg(feature = "ext_get_surface_capabilities2")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html>"]
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
    #[cfg(feature = "ext_get_surface_capabilities2")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormats2KHR.html>"]
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
    #[cfg(feature = "ext_get_display_properties2")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayProperties2KHR.html>"]
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
    #[cfg(feature = "ext_get_display_properties2")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html>"]
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
    #[cfg(feature = "ext_get_display_properties2")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModeProperties2KHR.html>"]
    #[doc(alias = "vkGetDisplayModeProperties2KHR")]
    pub fn get_display_mode_properties2_khr<R: DynamicArray<DisplayModeProperties2KHR<'static>>>(
        &self,
        display: &raw::DisplayKHR,
    ) -> Result<R> {
        unsafe {
            raw::get_display_mode_properties2_khr(self, display, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_get_display_properties2")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilities2KHR.html>"]
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
    #[cfg(feature = "ext_descriptor_heap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDescriptorSizeEXT.html>"]
    #[doc(alias = "vkGetPhysicalDeviceDescriptorSizeEXT")]
    #[inline]
    pub fn get_descriptor_size_ext(&self, descriptor_type: DescriptorType) -> DeviceSize {
        unsafe {
            raw::get_physical_device_descriptor_size_ext(
                self,
                descriptor_type,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_sample_locations")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMultisamplePropertiesEXT.html>"]
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
    #[cfg(feature = "ext_fragment_shading_rate")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFragmentShadingRatesKHR.html>"]
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
    #[cfg(feature = "ext_cooperative_matrix")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html>"]
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
    #[cfg(feature = "ext_coverage_reduction_mode")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html>"]
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
    #[cfg(feature = "ext_full_screen_exclusive")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModes2EXT.html>"]
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
    #[cfg(feature = "ext_acquire_drm_display")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireDrmDisplayEXT.html>"]
    #[doc(alias = "vkAcquireDrmDisplayEXT")]
    #[inline]
    pub fn acquire_drm_display_ext(&self, drm_fd: i32, display: &raw::DisplayKHR) -> Result<()> {
        unsafe {
            raw::acquire_drm_display_ext(self, drm_fd, display, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_acquire_drm_display")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDrmDisplayEXT.html>"]
    #[doc(alias = "vkGetDrmDisplayEXT")]
    #[inline]
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
    #[cfg(feature = "ext_acquire_winrt_display")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireWinrtDisplayNV.html>"]
    #[doc(alias = "vkAcquireWinrtDisplayNV")]
    #[inline]
    pub fn acquire_winrt_display_nv(&self, display: &raw::DisplayKHR) -> Result<()> {
        unsafe { raw::acquire_winrt_display_nv(self, display, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_acquire_winrt_display")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetWinrtDisplayNV.html>"]
    #[doc(alias = "vkGetWinrtDisplayNV")]
    #[inline]
    pub fn get_winrt_display_nv(&self, device_relative_id: u32) -> Result<DisplayKHR> {
        let vk_result = unsafe {
            raw::get_winrt_display_nv(self, device_relative_id, self.disp.get_command_dispatcher())
        };
        vk_result.map(|vk_result| unsafe { DisplayKHR::from_inner(vk_result) })
    }
    #[cfg(feature = "ext_directfb_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html>"]
    #[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
    #[inline]
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
    #[cfg(feature = "ext_screen_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceScreenPresentationSupportQNX.html>"]
    #[doc(alias = "vkGetPhysicalDeviceScreenPresentationSupportQNX")]
    #[inline]
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
    #[cfg(feature = "ext_tensors")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalTensorPropertiesARM.html>"]
    #[doc(alias = "vkGetPhysicalDeviceExternalTensorPropertiesARM")]
    pub fn get_external_tensor_properties_arm<
        S: StructureChainOut<ExternalTensorPropertiesARM<'static>>,
    >(
        &self,
        p_external_tensor_info: &PhysicalDeviceExternalTensorInfoARM,
    ) -> S {
        unsafe {
            raw::get_physical_device_external_tensor_properties_arm(
                self,
                p_external_tensor_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_optical_flow")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html>"]
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
    #[cfg(feature = "ext_cooperative_vector")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeVectorPropertiesNV.html>"]
    #[doc(alias = "vkGetPhysicalDeviceCooperativeVectorPropertiesNV")]
    pub fn get_cooperative_vector_properties_nv<
        R: DynamicArray<CooperativeVectorPropertiesNV<'static>>,
    >(
        &self,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_cooperative_vector_properties_nv(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_cooperative_matrix")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html>"]
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
    #[cfg(feature = "ext_data_graph")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM.html>"]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM")]
    pub fn get_queue_family_data_graph_properties_arm<
        R: DynamicArray<QueueFamilyDataGraphPropertiesARM<'static>>,
    >(
        &self,
        queue_family_index: u32,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_queue_family_data_graph_properties_arm(
                self,
                queue_family_index,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_data_graph")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM.html>"]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM")]
    pub fn get_queue_family_data_graph_processing_engine_properties_arm<
        S: StructureChainOut<QueueFamilyDataGraphProcessingEnginePropertiesARM<'static>>,
    >(
        &self,
        p_queue_family_data_graph_processing_engine_info : & PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM,
    ) -> S {
        unsafe {
            raw::get_physical_device_queue_family_data_graph_processing_engine_properties_arm(
                self,
                p_queue_family_data_graph_processing_engine_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_calibrated_timestamps")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html>"]
    #[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsKHR")]
    pub fn get_calibrateable_time_domains_khr<R: DynamicArray<TimeDomainKHR>>(&self) -> Result<R> {
        unsafe {
            raw::get_physical_device_calibrateable_time_domains_khr(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_calibrated_timestamps")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html>"]
    #[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")]
    pub fn get_calibrateable_time_domains_ext<R: DynamicArray<TimeDomainKHR>>(&self) -> Result<R> {
        unsafe {
            raw::get_physical_device_calibrateable_time_domains_ext(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_cooperative_matrix2")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV.html>"]
    #[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV")]
    pub fn get_cooperative_matrix_flexible_dimensions_properties_nv<
        R: DynamicArray<CooperativeMatrixFlexibleDimensionsPropertiesNV<'static>>,
    >(
        &self,
    ) -> Result<R> {
        unsafe {
            raw::get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_shader_instrumentation")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM.html>"]
    #[doc(alias = "vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM")]
    pub fn enumerate_shader_instrumentation_metrics_arm<
        R: DynamicArray<ShaderInstrumentationMetricDescriptionARM<'static>>,
    >(
        &self,
    ) -> Result<R> {
        unsafe {
            raw::enumerate_physical_device_shader_instrumentation_metrics_arm(
                self,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_ubm_surface")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceUbmPresentationSupportSEC.html>"]
    #[doc(alias = "vkGetPhysicalDeviceUbmPresentationSupportSEC")]
    #[inline]
    pub fn get_ubm_presentation_support_sec(
        &self,
        queue_family_index: u32,
        device: &VoidPtr,
    ) -> bool {
        unsafe {
            raw::get_physical_device_ubm_presentation_support_sec(
                self,
                queue_family_index,
                device,
                self.disp.get_command_dispatcher(),
            )
        }
    }
}
#[repr(C)]
#[derive(Clone)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDevice.html>"]
#[doc(alias = "VkDevice")]
pub struct Device<D: Dispatcher = DynamicDispatcher, A: BaseAllocator = DefaultAllocator> {
    inner: <raw::Device as Handle>::InnerType,
    disp: D,
    alloc: A,
}
unsafe impl Alias<raw::Device> for Device {}
impl<D: Dispatcher, A: BaseAllocator> Copy for Device<D, A>
where
    D: Copy,
    A: Copy,
{
}
impl<D: Dispatcher, A: BaseAllocator> Deref for Device<D, A> {
    type Target = raw::Device;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl<D: Dispatcher, A: BaseAllocator> Device<D, A> {
    pub unsafe fn from_inner(handle: raw::Device, disp: D, alloc: A) -> Self {
        Self {
            inner: handle.as_raw(),
            disp,
            alloc,
        }
    }
    #[inline(always)]
    pub fn get_dispatcher(&self) -> &D {
        &self.disp
    }
    #[inline(always)]
    pub fn get_allocator(&self) -> &A {
        &self.alloc
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceProcAddr.html>"]
    #[doc(alias = "vkGetDeviceProcAddr")]
    #[inline]
    pub fn get_proc_addr(&self, p_name: &CStr) -> FuncPtr {
        unsafe { raw::get_device_proc_addr(self, p_name, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDevice.html>"]
    #[doc(alias = "vkDestroyDevice")]
    #[inline]
    pub unsafe fn destroy(&self) {
        unsafe {
            raw::destroy_device(
                Some(self),
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue.html>"]
    #[doc(alias = "vkGetDeviceQueue")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDeviceWaitIdle.html>"]
    #[doc(alias = "vkDeviceWaitIdle")]
    #[inline]
    pub fn wait_idle(&self) -> Result<()> {
        unsafe { raw::device_wait_idle(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateMemory.html>"]
    #[doc(alias = "vkAllocateMemory")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeMemory.html>"]
    #[doc(alias = "vkFreeMemory")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory.html>"]
    #[doc(alias = "vkMapMemory")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory.html>"]
    #[doc(alias = "vkUnmapMemory")]
    #[inline]
    pub fn unmap_memory(&self, memory: &raw::DeviceMemory) {
        unsafe { raw::unmap_memory(self, memory, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html>"]
    #[doc(alias = "vkFlushMappedMemoryRanges")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkInvalidateMappedMemoryRanges.html>"]
    #[doc(alias = "vkInvalidateMappedMemoryRanges")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryCommitment.html>"]
    #[doc(alias = "vkGetDeviceMemoryCommitment")]
    #[inline]
    pub fn get_memory_commitment(&self, memory: &raw::DeviceMemory) -> DeviceSize {
        unsafe {
            raw::get_device_memory_commitment(self, memory, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory.html>"]
    #[doc(alias = "vkBindBufferMemory")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory.html>"]
    #[doc(alias = "vkBindImageMemory")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements.html>"]
    #[doc(alias = "vkGetBufferMemoryRequirements")]
    #[inline]
    pub fn get_buffer_memory_requirements(&self, buffer: &raw::Buffer) -> MemoryRequirements {
        unsafe {
            raw::get_buffer_memory_requirements(self, buffer, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements.html>"]
    #[doc(alias = "vkGetImageMemoryRequirements")]
    #[inline]
    pub fn get_image_memory_requirements(&self, image: &raw::Image) -> MemoryRequirements {
        unsafe {
            raw::get_image_memory_requirements(self, image, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements.html>"]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFence.html>"]
    #[doc(alias = "vkCreateFence")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFence.html>"]
    #[doc(alias = "vkDestroyFence")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetFences.html>"]
    #[doc(alias = "vkResetFences")]
    #[inline]
    pub fn reset_fences<'a, V2: Alias<raw::Fence> + 'a>(
        &self,
        p_fences: impl AsSlice<'a, V2>,
    ) -> Result<()> {
        unsafe { raw::reset_fences(self, p_fences, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceStatus.html>"]
    #[doc(alias = "vkGetFenceStatus")]
    #[inline]
    pub fn get_fence_status(&self, fence: &raw::Fence) -> Result<Status> {
        unsafe { raw::get_fence_status(self, fence, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForFences.html>"]
    #[doc(alias = "vkWaitForFences")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSemaphore.html>"]
    #[doc(alias = "vkCreateSemaphore")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySemaphore.html>"]
    #[doc(alias = "vkDestroySemaphore")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateQueryPool.html>"]
    #[doc(alias = "vkCreateQueryPool")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyQueryPool.html>"]
    #[doc(alias = "vkDestroyQueryPool")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueryPoolResults.html>"]
    #[doc(alias = "vkGetQueryPoolResults")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBuffer.html>"]
    #[doc(alias = "vkCreateBuffer")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBuffer.html>"]
    #[doc(alias = "vkDestroyBuffer")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImage.html>"]
    #[doc(alias = "vkCreateImage")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImage.html>"]
    #[doc(alias = "vkDestroyImage")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout.html>"]
    #[doc(alias = "vkGetImageSubresourceLayout")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImageView.html>"]
    #[doc(alias = "vkCreateImageView")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImageView.html>"]
    #[doc(alias = "vkDestroyImageView")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCommandPool.html>"]
    #[doc(alias = "vkCreateCommandPool")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCommandPool.html>"]
    #[doc(alias = "vkDestroyCommandPool")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandPool.html>"]
    #[doc(alias = "vkResetCommandPool")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateCommandBuffers.html>"]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeCommandBuffers.html>"]
    #[doc(alias = "vkFreeCommandBuffers")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateEvent.html>"]
    #[doc(alias = "vkCreateEvent")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyEvent.html>"]
    #[doc(alias = "vkDestroyEvent")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEventStatus.html>"]
    #[doc(alias = "vkGetEventStatus")]
    #[inline]
    pub fn get_event_status(&self, event: &raw::Event) -> Result<Status> {
        unsafe { raw::get_event_status(self, event, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetEvent.html>"]
    #[doc(alias = "vkSetEvent")]
    #[inline]
    pub fn set_event(&self, event: &raw::Event) -> Result<()> {
        unsafe { raw::set_event(self, event, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetEvent.html>"]
    #[doc(alias = "vkResetEvent")]
    #[inline]
    pub fn reset_event(&self, event: &raw::Event) -> Result<()> {
        unsafe { raw::reset_event(self, event, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferView.html>"]
    #[doc(alias = "vkCreateBufferView")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferView.html>"]
    #[doc(alias = "vkDestroyBufferView")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderModule.html>"]
    #[doc(alias = "vkCreateShaderModule")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderModule.html>"]
    #[doc(alias = "vkDestroyShaderModule")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineCache.html>"]
    #[doc(alias = "vkCreatePipelineCache")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineCache.html>"]
    #[doc(alias = "vkDestroyPipelineCache")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineCacheData.html>"]
    #[doc(alias = "vkGetPipelineCacheData")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkMergePipelineCaches.html>"]
    #[doc(alias = "vkMergePipelineCaches")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateComputePipelines.html>"]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipeline.html>"]
    #[doc(alias = "vkDestroyPipeline")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineLayout.html>"]
    #[doc(alias = "vkCreatePipelineLayout")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineLayout.html>"]
    #[doc(alias = "vkDestroyPipelineLayout")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSampler.html>"]
    #[doc(alias = "vkCreateSampler")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySampler.html>"]
    #[doc(alias = "vkDestroySampler")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorSetLayout.html>"]
    #[doc(alias = "vkCreateDescriptorSetLayout")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorSetLayout.html>"]
    #[doc(alias = "vkDestroyDescriptorSetLayout")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorPool.html>"]
    #[doc(alias = "vkCreateDescriptorPool")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorPool.html>"]
    #[doc(alias = "vkDestroyDescriptorPool")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetDescriptorPool.html>"]
    #[doc(alias = "vkResetDescriptorPool")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateDescriptorSets.html>"]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeDescriptorSets.html>"]
    #[doc(alias = "vkFreeDescriptorSets")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSets.html>"]
    #[doc(alias = "vkUpdateDescriptorSets")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGraphicsPipelines.html>"]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFramebuffer.html>"]
    #[doc(alias = "vkCreateFramebuffer")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFramebuffer.html>"]
    #[doc(alias = "vkDestroyFramebuffer")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass.html>"]
    #[doc(alias = "vkCreateRenderPass")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyRenderPass.html>"]
    #[doc(alias = "vkDestroyRenderPass")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderAreaGranularity.html>"]
    #[doc(alias = "vkGetRenderAreaGranularity")]
    #[inline]
    pub fn get_render_area_granularity(&self, render_pass: &raw::RenderPass) -> Extent2D {
        unsafe {
            raw::get_render_area_granularity(self, render_pass, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_bind_memory2", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2.html>"]
    #[doc(alias = "vkBindBufferMemory2")]
    #[inline]
    pub fn bind_buffer_memory2<'a>(
        &self,
        p_bind_infos: impl AsSlice<'a, BindBufferMemoryInfo<'a>>,
    ) -> Result<()> {
        unsafe { raw::bind_buffer_memory2(self, p_bind_infos, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(feature = "ext_bind_memory2", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2KHR.html>"]
    #[doc(alias = "vkBindBufferMemory2KHR")]
    #[inline]
    pub fn bind_buffer_memory2_khr<'a>(
        &self,
        p_bind_infos: impl AsSlice<'a, BindBufferMemoryInfo<'a>>,
    ) -> Result<()> {
        unsafe {
            raw::bind_buffer_memory2_khr(self, p_bind_infos, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_bind_memory2", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2.html>"]
    #[doc(alias = "vkBindImageMemory2")]
    #[inline]
    pub fn bind_image_memory2<'a>(
        &self,
        p_bind_infos: impl AsSlice<'a, BindImageMemoryInfo<'a>>,
    ) -> Result<()> {
        unsafe { raw::bind_image_memory2(self, p_bind_infos, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(feature = "ext_bind_memory2", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2KHR.html>"]
    #[doc(alias = "vkBindImageMemory2KHR")]
    #[inline]
    pub fn bind_image_memory2_khr<'a>(
        &self,
        p_bind_infos: impl AsSlice<'a, BindImageMemoryInfo<'a>>,
    ) -> Result<()> {
        unsafe {
            raw::bind_image_memory2_khr(self, p_bind_infos, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_device_group", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeatures.html>"]
    #[doc(alias = "vkGetDeviceGroupPeerMemoryFeatures")]
    #[inline]
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
    #[cfg(any(feature = "ext_device_group", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeaturesKHR.html>"]
    #[doc(alias = "vkGetDeviceGroupPeerMemoryFeaturesKHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_get_memory_requirements2", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2.html>"]
    #[doc(alias = "vkGetImageMemoryRequirements2")]
    pub fn get_image_memory_requirements2<S: StructureChainOut<MemoryRequirements2<'static>>>(
        &self,
        p_info: &ImageMemoryRequirementsInfo2,
    ) -> S {
        unsafe {
            raw::get_image_memory_requirements2(self, p_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_get_memory_requirements2", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2KHR.html>"]
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
    #[cfg(any(feature = "ext_get_memory_requirements2", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2.html>"]
    #[doc(alias = "vkGetBufferMemoryRequirements2")]
    pub fn get_buffer_memory_requirements2<S: StructureChainOut<MemoryRequirements2<'static>>>(
        &self,
        p_info: &BufferMemoryRequirementsInfo2,
    ) -> S {
        unsafe {
            raw::get_buffer_memory_requirements2(self, p_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_get_memory_requirements2", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2KHR.html>"]
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
    #[cfg(any(feature = "ext_get_memory_requirements2", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2.html>"]
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
    #[cfg(any(feature = "ext_get_memory_requirements2", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2KHR.html>"]
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
    #[cfg(any(feature = "ext_maintenance1", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPool.html>"]
    #[doc(alias = "vkTrimCommandPool")]
    #[inline]
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
    #[cfg(any(feature = "ext_maintenance1", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPoolKHR.html>"]
    #[doc(alias = "vkTrimCommandPoolKHR")]
    #[inline]
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
    #[cfg(feature = "version_1_1")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue2.html>"]
    #[doc(alias = "vkGetDeviceQueue2")]
    #[inline]
    pub fn get_queue2(&self, p_queue_info: &DeviceQueueInfo2) -> Queue<D, A> {
        let vk_result = unsafe {
            raw::get_device_queue2(self, p_queue_info, self.disp.get_command_dispatcher())
        };
        unsafe { Queue::from_inner(vk_result, self.disp.clone(), self.alloc.clone()) }
    }
    #[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplate.html>"]
    #[doc(alias = "vkCreateDescriptorUpdateTemplate")]
    #[inline]
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
    #[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplateKHR.html>"]
    #[doc(alias = "vkCreateDescriptorUpdateTemplateKHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html>"]
    #[doc(alias = "vkDestroyDescriptorUpdateTemplate")]
    #[inline]
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
    #[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplateKHR.html>"]
    #[doc(alias = "vkDestroyDescriptorUpdateTemplateKHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html>"]
    #[doc(alias = "vkUpdateDescriptorSetWithTemplate")]
    #[inline]
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
    #[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplateKHR.html>"]
    #[doc(alias = "vkUpdateDescriptorSetWithTemplateKHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_maintenance3", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupport.html>"]
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
    #[cfg(any(feature = "ext_maintenance3", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupportKHR.html>"]
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
    #[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversion.html>"]
    #[doc(alias = "vkCreateSamplerYcbcrConversion")]
    #[inline]
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
    #[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversionKHR.html>"]
    #[doc(alias = "vkCreateSamplerYcbcrConversionKHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversion.html>"]
    #[doc(alias = "vkDestroySamplerYcbcrConversion")]
    #[inline]
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
    #[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversionKHR.html>"]
    #[doc(alias = "vkDestroySamplerYcbcrConversionKHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_host_query_reset", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPool.html>"]
    #[doc(alias = "vkResetQueryPool")]
    #[inline]
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
    #[cfg(any(feature = "ext_host_query_reset", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPoolEXT.html>"]
    #[doc(alias = "vkResetQueryPoolEXT")]
    #[inline]
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
    #[cfg(any(feature = "ext_timeline_semaphore", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValue.html>"]
    #[doc(alias = "vkGetSemaphoreCounterValue")]
    #[inline]
    pub fn get_semaphore_counter_value(&self, semaphore: &raw::Semaphore) -> Result<u64> {
        unsafe {
            raw::get_semaphore_counter_value(self, semaphore, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_timeline_semaphore", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValueKHR.html>"]
    #[doc(alias = "vkGetSemaphoreCounterValueKHR")]
    #[inline]
    pub fn get_semaphore_counter_value_khr(&self, semaphore: &raw::Semaphore) -> Result<u64> {
        unsafe {
            raw::get_semaphore_counter_value_khr(
                self,
                semaphore,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_timeline_semaphore", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphores.html>"]
    #[doc(alias = "vkWaitSemaphores")]
    #[inline]
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
    #[cfg(any(feature = "ext_timeline_semaphore", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphoresKHR.html>"]
    #[doc(alias = "vkWaitSemaphoresKHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_timeline_semaphore", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphore.html>"]
    #[doc(alias = "vkSignalSemaphore")]
    #[inline]
    pub fn signal_semaphore(&self, p_signal_info: &SemaphoreSignalInfo) -> Result<()> {
        unsafe { raw::signal_semaphore(self, p_signal_info, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(feature = "ext_timeline_semaphore", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphoreKHR.html>"]
    #[doc(alias = "vkSignalSemaphoreKHR")]
    #[inline]
    pub fn signal_semaphore_khr(&self, p_signal_info: &SemaphoreSignalInfo) -> Result<()> {
        unsafe {
            raw::signal_semaphore_khr(self, p_signal_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_buffer_device_address", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html>"]
    #[doc(alias = "vkGetBufferDeviceAddress")]
    #[inline]
    pub fn get_buffer_address(&self, p_info: &BufferDeviceAddressInfo) -> DeviceAddress {
        unsafe { raw::get_buffer_device_address(self, p_info, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(feature = "ext_buffer_device_address", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressKHR.html>"]
    #[doc(alias = "vkGetBufferDeviceAddressKHR")]
    #[inline]
    pub fn get_buffer_address_khr(&self, p_info: &BufferDeviceAddressInfo) -> DeviceAddress {
        unsafe {
            raw::get_buffer_device_address_khr(self, p_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_buffer_device_address", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressEXT.html>"]
    #[doc(alias = "vkGetBufferDeviceAddressEXT")]
    #[inline]
    pub fn get_buffer_address_ext(&self, p_info: &BufferDeviceAddressInfo) -> DeviceAddress {
        unsafe {
            raw::get_buffer_device_address_ext(self, p_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_buffer_device_address", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddress.html>"]
    #[doc(alias = "vkGetBufferOpaqueCaptureAddress")]
    #[inline]
    pub fn get_buffer_opaque_capture_address(&self, p_info: &BufferDeviceAddressInfo) -> u64 {
        unsafe {
            raw::get_buffer_opaque_capture_address(self, p_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_buffer_device_address", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddressKHR.html>"]
    #[doc(alias = "vkGetBufferOpaqueCaptureAddressKHR")]
    #[inline]
    pub fn get_buffer_opaque_capture_address_khr(&self, p_info: &BufferDeviceAddressInfo) -> u64 {
        unsafe {
            raw::get_buffer_opaque_capture_address_khr(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_buffer_device_address", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddress.html>"]
    #[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddress")]
    #[inline]
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
    #[cfg(any(feature = "ext_buffer_device_address", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html>"]
    #[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddressKHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_create_renderpass2", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2.html>"]
    #[doc(alias = "vkCreateRenderPass2")]
    #[inline]
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
    #[cfg(any(feature = "ext_create_renderpass2", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2KHR.html>"]
    #[doc(alias = "vkCreateRenderPass2KHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlot.html>"]
    #[doc(alias = "vkCreatePrivateDataSlot")]
    #[inline]
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
    #[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlotEXT.html>"]
    #[doc(alias = "vkCreatePrivateDataSlotEXT")]
    #[inline]
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
    #[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlot.html>"]
    #[doc(alias = "vkDestroyPrivateDataSlot")]
    #[inline]
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
    #[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlotEXT.html>"]
    #[doc(alias = "vkDestroyPrivateDataSlotEXT")]
    #[inline]
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
    #[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateData.html>"]
    #[doc(alias = "vkSetPrivateData")]
    #[inline]
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
    #[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateDataEXT.html>"]
    #[doc(alias = "vkSetPrivateDataEXT")]
    #[inline]
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
    #[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateData.html>"]
    #[doc(alias = "vkGetPrivateData")]
    #[inline]
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
    #[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateDataEXT.html>"]
    #[doc(alias = "vkGetPrivateDataEXT")]
    #[inline]
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
    #[cfg(any(feature = "ext_maintenance4", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirements.html>"]
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
    #[cfg(any(feature = "ext_maintenance4", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirementsKHR.html>"]
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
    #[cfg(any(feature = "ext_maintenance4", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirements.html>"]
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
    #[cfg(any(feature = "ext_maintenance4", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirementsKHR.html>"]
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
    #[cfg(any(feature = "ext_maintenance4", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirements.html>"]
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
    #[cfg(any(feature = "ext_maintenance4", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirementsKHR.html>"]
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
    #[cfg(any(feature = "ext_map_memory2", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2.html>"]
    #[doc(alias = "vkMapMemory2")]
    #[inline]
    pub fn map_memory2(&self, p_memory_map_info: &MemoryMapInfo) -> Result<VoidPtr> {
        unsafe { raw::map_memory2(self, p_memory_map_info, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(feature = "ext_map_memory2", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2KHR.html>"]
    #[doc(alias = "vkMapMemory2KHR")]
    #[inline]
    pub fn map_memory2_khr(&self, p_memory_map_info: &MemoryMapInfo) -> Result<VoidPtr> {
        unsafe { raw::map_memory2_khr(self, p_memory_map_info, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(feature = "ext_map_memory2", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2.html>"]
    #[doc(alias = "vkUnmapMemory2")]
    #[inline]
    pub fn unmap_memory2(&self, p_memory_unmap_info: &MemoryUnmapInfo) -> Result<()> {
        unsafe {
            raw::unmap_memory2(
                self,
                p_memory_unmap_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_map_memory2", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2KHR.html>"]
    #[doc(alias = "vkUnmapMemory2KHR")]
    #[inline]
    pub fn unmap_memory2_khr(&self, p_memory_unmap_info: &MemoryUnmapInfo) -> Result<()> {
        unsafe {
            raw::unmap_memory2_khr(
                self,
                p_memory_unmap_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_maintenance5", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayout.html>"]
    #[doc(alias = "vkGetDeviceImageSubresourceLayout")]
    pub fn get_device_image_subresource_layout<
        S: StructureChainOut<SubresourceLayout2<'static>>,
    >(
        &self,
        p_info: &DeviceImageSubresourceInfo,
    ) -> S {
        unsafe {
            raw::get_device_image_subresource_layout(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_maintenance5", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayoutKHR.html>"]
    #[doc(alias = "vkGetDeviceImageSubresourceLayoutKHR")]
    pub fn get_image_subresource_layout_khr<S: StructureChainOut<SubresourceLayout2<'static>>>(
        &self,
        p_info: &DeviceImageSubresourceInfo,
    ) -> S {
        unsafe {
            raw::get_device_image_subresource_layout_khr(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_host_image_copy",
        feature = "ext_image_compression_control",
        feature = "ext_maintenance5",
        feature = "version_1_4"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html>"]
    #[doc(alias = "vkGetImageSubresourceLayout2")]
    pub fn get_image_subresource_layout2<S: StructureChainOut<SubresourceLayout2<'static>>>(
        &self,
        image: &raw::Image,
        p_subresource: &ImageSubresource2,
    ) -> S {
        unsafe {
            raw::get_image_subresource_layout2(
                self,
                image,
                p_subresource,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_host_image_copy",
        feature = "ext_image_compression_control",
        feature = "ext_maintenance5",
        feature = "version_1_4"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2KHR.html>"]
    #[doc(alias = "vkGetImageSubresourceLayout2KHR")]
    pub fn get_image_subresource_layout2_khr<S: StructureChainOut<SubresourceLayout2<'static>>>(
        &self,
        image: &raw::Image,
        p_subresource: &ImageSubresource2,
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
    #[cfg(any(
        feature = "ext_host_image_copy",
        feature = "ext_image_compression_control",
        feature = "ext_maintenance5",
        feature = "version_1_4"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2EXT.html>"]
    #[doc(alias = "vkGetImageSubresourceLayout2EXT")]
    pub fn get_image_subresource_layout2_ext<S: StructureChainOut<SubresourceLayout2<'static>>>(
        &self,
        image: &raw::Image,
        p_subresource: &ImageSubresource2,
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
    #[cfg(any(feature = "ext_host_image_copy", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImage.html>"]
    #[doc(alias = "vkCopyMemoryToImage")]
    #[inline]
    pub fn copy_memory_to_image(
        &self,
        p_copy_memory_to_image_info: &CopyMemoryToImageInfo,
    ) -> Result<()> {
        unsafe {
            raw::copy_memory_to_image(
                self,
                p_copy_memory_to_image_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_host_image_copy", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImageEXT.html>"]
    #[doc(alias = "vkCopyMemoryToImageEXT")]
    #[inline]
    pub fn copy_memory_to_image_ext(
        &self,
        p_copy_memory_to_image_info: &CopyMemoryToImageInfo,
    ) -> Result<()> {
        unsafe {
            raw::copy_memory_to_image_ext(
                self,
                p_copy_memory_to_image_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_host_image_copy", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemory.html>"]
    #[doc(alias = "vkCopyImageToMemory")]
    #[inline]
    pub fn copy_image_to_memory(
        &self,
        p_copy_image_to_memory_info: &CopyImageToMemoryInfo,
    ) -> Result<()> {
        unsafe {
            raw::copy_image_to_memory(
                self,
                p_copy_image_to_memory_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_host_image_copy", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemoryEXT.html>"]
    #[doc(alias = "vkCopyImageToMemoryEXT")]
    #[inline]
    pub fn copy_image_to_memory_ext(
        &self,
        p_copy_image_to_memory_info: &CopyImageToMemoryInfo,
    ) -> Result<()> {
        unsafe {
            raw::copy_image_to_memory_ext(
                self,
                p_copy_image_to_memory_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_host_image_copy", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImage.html>"]
    #[doc(alias = "vkCopyImageToImage")]
    #[inline]
    pub fn copy_image_to_image(
        &self,
        p_copy_image_to_image_info: &CopyImageToImageInfo,
    ) -> Result<()> {
        unsafe {
            raw::copy_image_to_image(
                self,
                p_copy_image_to_image_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_host_image_copy", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImageEXT.html>"]
    #[doc(alias = "vkCopyImageToImageEXT")]
    #[inline]
    pub fn copy_image_to_image_ext(
        &self,
        p_copy_image_to_image_info: &CopyImageToImageInfo,
    ) -> Result<()> {
        unsafe {
            raw::copy_image_to_image_ext(
                self,
                p_copy_image_to_image_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_host_image_copy", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayout.html>"]
    #[doc(alias = "vkTransitionImageLayout")]
    #[inline]
    pub fn transition_image_layout<'a>(
        &self,
        p_transitions: impl AsSlice<'a, HostImageLayoutTransitionInfo<'a>>,
    ) -> Result<()> {
        unsafe {
            raw::transition_image_layout(self, p_transitions, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_host_image_copy", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayoutEXT.html>"]
    #[doc(alias = "vkTransitionImageLayoutEXT")]
    #[inline]
    pub fn transition_image_layout_ext<'a>(
        &self,
        p_transitions: impl AsSlice<'a, HostImageLayoutTransitionInfo<'a>>,
    ) -> Result<()> {
        unsafe {
            raw::transition_image_layout_ext(
                self,
                p_transitions,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_maintenance5", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularity.html>"]
    #[doc(alias = "vkGetRenderingAreaGranularity")]
    #[inline]
    pub fn get_rendering_area_granularity(
        &self,
        p_rendering_area_info: &RenderingAreaInfo,
    ) -> Extent2D {
        unsafe {
            raw::get_rendering_area_granularity(
                self,
                p_rendering_area_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_maintenance5", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularityKHR.html>"]
    #[doc(alias = "vkGetRenderingAreaGranularityKHR")]
    #[inline]
    pub fn get_rendering_area_granularity_khr(
        &self,
        p_rendering_area_info: &RenderingAreaInfo,
    ) -> Extent2D {
        unsafe {
            raw::get_rendering_area_granularity_khr(
                self,
                p_rendering_area_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_swapchain")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSwapchainKHR.html>"]
    #[doc(alias = "vkCreateSwapchainKHR")]
    #[inline]
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
    #[cfg(feature = "ext_swapchain")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySwapchainKHR.html>"]
    #[doc(alias = "vkDestroySwapchainKHR")]
    #[inline]
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
    #[cfg(feature = "ext_swapchain")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainImagesKHR.html>"]
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
    #[cfg(feature = "ext_swapchain")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImageKHR.html>"]
    #[doc(alias = "vkAcquireNextImageKHR")]
    #[inline]
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
    #[cfg(any(
        all(feature = "ext_swapchain", feature = "version_1_1"),
        all(feature = "ext_device_group", feature = "ext_surface")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPresentCapabilitiesKHR.html>"]
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
    #[cfg(any(
        all(feature = "ext_swapchain", feature = "version_1_1"),
        all(feature = "ext_device_group", feature = "ext_surface")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModesKHR.html>"]
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
    #[inline]
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
    #[cfg(any(
        all(feature = "ext_swapchain", feature = "version_1_1"),
        all(feature = "ext_device_group", feature = "ext_swapchain")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImage2KHR.html>"]
    #[doc(alias = "vkAcquireNextImage2KHR")]
    #[inline]
    pub fn acquire_next_image2_khr(
        &self,
        p_acquire_info: &AcquireNextImageInfoKHR,
    ) -> Result<(Status, u32)> {
        unsafe {
            raw::acquire_next_image2_khr(self, p_acquire_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_display_swapchain")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSharedSwapchainsKHR.html>"]
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
    #[cfg(feature = "ext_debug_marker")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectTagEXT.html>"]
    #[doc(alias = "vkDebugMarkerSetObjectTagEXT")]
    #[inline]
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
    #[cfg(feature = "ext_debug_marker")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectNameEXT.html>"]
    #[doc(alias = "vkDebugMarkerSetObjectNameEXT")]
    #[inline]
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
    #[cfg(feature = "ext_binary_import")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuModuleNVX.html>"]
    #[doc(alias = "vkCreateCuModuleNVX")]
    #[inline]
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
    #[cfg(feature = "ext_binary_import")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuFunctionNVX.html>"]
    #[doc(alias = "vkCreateCuFunctionNVX")]
    #[inline]
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
    #[cfg(feature = "ext_binary_import")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuModuleNVX.html>"]
    #[doc(alias = "vkDestroyCuModuleNVX")]
    #[inline]
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
    #[cfg(feature = "ext_binary_import")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuFunctionNVX.html>"]
    #[doc(alias = "vkDestroyCuFunctionNVX")]
    #[inline]
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
    #[cfg(feature = "ext_image_view_handle")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandleNVX.html>"]
    #[doc(alias = "vkGetImageViewHandleNVX")]
    #[inline]
    pub fn get_image_view_handle_nvx(&self, p_info: &ImageViewHandleInfoNVX) -> u32 {
        unsafe { raw::get_image_view_handle_nvx(self, p_info, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_image_view_handle")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandle64NVX.html>"]
    #[doc(alias = "vkGetImageViewHandle64NVX")]
    #[inline]
    pub fn get_image_view_handle64_nvx(&self, p_info: &ImageViewHandleInfoNVX) -> u64 {
        unsafe {
            raw::get_image_view_handle64_nvx(self, p_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_image_view_handle")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewAddressNVX.html>"]
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
    #[cfg(feature = "ext_image_view_handle")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceCombinedImageSamplerIndexNVX.html>"]
    #[doc(alias = "vkGetDeviceCombinedImageSamplerIndexNVX")]
    #[inline]
    pub fn get_combined_image_sampler_index_nvx(
        &self,
        image_view_index: u64,
        sampler_index: u64,
    ) -> u64 {
        unsafe {
            raw::get_device_combined_image_sampler_index_nvx(
                self,
                image_view_index,
                sampler_index,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_shader_info")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInfoAMD.html>"]
    #[doc(alias = "vkGetShaderInfoAMD")]
    #[inline]
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
    #[cfg(feature = "ext_external_memory_win32")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleNV.html>"]
    #[doc(alias = "vkGetMemoryWin32HandleNV")]
    #[inline]
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
    #[cfg(feature = "ext_external_memory_win32")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleKHR.html>"]
    #[doc(alias = "vkGetMemoryWin32HandleKHR")]
    #[inline]
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
    #[cfg(feature = "ext_external_memory_win32")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandlePropertiesKHR.html>"]
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
    #[cfg(feature = "ext_external_memory_fd")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdKHR.html>"]
    #[doc(alias = "vkGetMemoryFdKHR")]
    #[inline]
    pub fn get_memory_fd_khr(&self, p_get_fd_info: &MemoryGetFdInfoKHR) -> Result<c_int> {
        unsafe { raw::get_memory_fd_khr(self, p_get_fd_info, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_external_memory_fd")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdPropertiesKHR.html>"]
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
    #[cfg(feature = "ext_external_semaphore_win32")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreWin32HandleKHR.html>"]
    #[doc(alias = "vkImportSemaphoreWin32HandleKHR")]
    #[inline]
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
    #[cfg(feature = "ext_external_semaphore_win32")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreWin32HandleKHR.html>"]
    #[doc(alias = "vkGetSemaphoreWin32HandleKHR")]
    #[inline]
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
    #[cfg(feature = "ext_external_semaphore_fd")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreFdKHR.html>"]
    #[doc(alias = "vkImportSemaphoreFdKHR")]
    #[inline]
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
    #[cfg(feature = "ext_external_semaphore_fd")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreFdKHR.html>"]
    #[doc(alias = "vkGetSemaphoreFdKHR")]
    #[inline]
    pub fn get_semaphore_fd_khr(&self, p_get_fd_info: &SemaphoreGetFdInfoKHR) -> Result<c_int> {
        unsafe {
            raw::get_semaphore_fd_khr(self, p_get_fd_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_display_control")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDisplayPowerControlEXT.html>"]
    #[doc(alias = "vkDisplayPowerControlEXT")]
    #[inline]
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
    #[cfg(feature = "ext_display_control")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDeviceEventEXT.html>"]
    #[doc(alias = "vkRegisterDeviceEventEXT")]
    #[inline]
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
    #[cfg(feature = "ext_display_control")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDisplayEventEXT.html>"]
    #[doc(alias = "vkRegisterDisplayEventEXT")]
    #[inline]
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
    #[cfg(feature = "ext_display_control")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainCounterEXT.html>"]
    #[doc(alias = "vkGetSwapchainCounterEXT")]
    #[inline]
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
    #[cfg(feature = "ext_display_timing")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRefreshCycleDurationGOOGLE.html>"]
    #[doc(alias = "vkGetRefreshCycleDurationGOOGLE")]
    #[inline]
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
    #[cfg(feature = "ext_display_timing")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingGOOGLE.html>"]
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
    #[cfg(feature = "ext_hdr_metadata")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetHdrMetadataEXT.html>"]
    #[doc(alias = "vkSetHdrMetadataEXT")]
    #[inline]
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
    #[cfg(feature = "ext_shared_presentable_image")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainStatusKHR.html>"]
    #[doc(alias = "vkGetSwapchainStatusKHR")]
    #[inline]
    pub fn get_swapchain_status_khr(&self, swapchain: &raw::SwapchainKHR) -> Result<Status> {
        unsafe {
            raw::get_swapchain_status_khr(self, swapchain, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_external_fence_win32")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceWin32HandleKHR.html>"]
    #[doc(alias = "vkImportFenceWin32HandleKHR")]
    #[inline]
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
    #[cfg(feature = "ext_external_fence_win32")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceWin32HandleKHR.html>"]
    #[doc(alias = "vkGetFenceWin32HandleKHR")]
    #[inline]
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
    #[cfg(feature = "ext_external_fence_fd")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceFdKHR.html>"]
    #[doc(alias = "vkImportFenceFdKHR")]
    #[inline]
    pub fn import_fence_fd_khr(&self, p_import_fence_fd_info: &ImportFenceFdInfoKHR) -> Result<()> {
        unsafe {
            raw::import_fence_fd_khr(
                self,
                p_import_fence_fd_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_external_fence_fd")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceFdKHR.html>"]
    #[doc(alias = "vkGetFenceFdKHR")]
    #[inline]
    pub fn get_fence_fd_khr(&self, p_get_fd_info: &FenceGetFdInfoKHR) -> Result<c_int> {
        unsafe { raw::get_fence_fd_khr(self, p_get_fd_info, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_performance_query")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireProfilingLockKHR.html>"]
    #[doc(alias = "vkAcquireProfilingLockKHR")]
    #[inline]
    pub fn acquire_profiling_lock_khr(&self, p_info: &AcquireProfilingLockInfoKHR) -> Result<()> {
        unsafe { raw::acquire_profiling_lock_khr(self, p_info, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_performance_query")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseProfilingLockKHR.html>"]
    #[doc(alias = "vkReleaseProfilingLockKHR")]
    #[inline]
    pub fn release_profiling_lock_khr(&self) {
        unsafe { raw::release_profiling_lock_khr(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_debug_utils")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectNameEXT.html>"]
    #[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
    #[inline]
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
    #[cfg(feature = "ext_debug_utils")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectTagEXT.html>"]
    #[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
    #[inline]
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
    #[cfg(feature = "ext_external_memory_android_hardware_buffer")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAndroidHardwareBufferPropertiesANDROID.html>"]
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
    #[cfg(feature = "ext_external_memory_android_hardware_buffer")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryAndroidHardwareBufferANDROID.html>"]
    #[doc(alias = "vkGetMemoryAndroidHardwareBufferANDROID")]
    #[inline]
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
    #[cfg(feature = "ext_shader_enqueue")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExecutionGraphPipelinesAMDX.html>"]
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
    #[cfg(feature = "ext_shader_enqueue")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineScratchSizeAMDX.html>"]
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
    #[cfg(feature = "ext_shader_enqueue")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineNodeIndexAMDX.html>"]
    #[doc(alias = "vkGetExecutionGraphPipelineNodeIndexAMDX")]
    #[inline]
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
    #[cfg(feature = "ext_descriptor_heap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteSamplerDescriptorsEXT.html>"]
    #[doc(alias = "vkWriteSamplerDescriptorsEXT")]
    #[inline]
    pub fn write_sampler_descriptors_ext<'a>(
        &self,
        p_samplers: impl AsSlice<'a, SamplerCreateInfo<'a>>,
        p_descriptors: impl AsSlice<'a, HostAddressRangeEXT<'a>>,
    ) -> Result<()> {
        unsafe {
            raw::write_sampler_descriptors_ext(
                self,
                p_samplers,
                p_descriptors,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_descriptor_heap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteResourceDescriptorsEXT.html>"]
    #[doc(alias = "vkWriteResourceDescriptorsEXT")]
    #[inline]
    pub fn write_resource_descriptors_ext<'a>(
        &self,
        p_resources: impl AsSlice<'a, ResourceDescriptorInfoEXT<'a>>,
        p_descriptors: impl AsSlice<'a, HostAddressRangeEXT<'a>>,
    ) -> Result<()> {
        unsafe {
            raw::write_resource_descriptors_ext(
                self,
                p_resources,
                p_descriptors,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_descriptor_heap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDataEXT.html>"]
    #[doc(alias = "vkGetImageOpaqueCaptureDataEXT")]
    pub fn get_image_opaque_capture_data_ext<
        'a,
        R: DynamicArray<HostAddressRangeEXT<'static>>,
        V2: Alias<raw::Image> + 'a,
    >(
        &self,
        p_images: impl AsSlice<'a, V2>,
    ) -> Result<R> {
        unsafe {
            raw::get_image_opaque_capture_data_ext(
                self,
                p_images,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(all(feature = "ext_descriptor_heap", feature = "ext_custom_border_color"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterCustomBorderColorEXT.html>"]
    #[doc(alias = "vkRegisterCustomBorderColorEXT")]
    #[inline]
    pub fn register_custom_border_color_ext(
        &self,
        p_border_color: &SamplerCustomBorderColorCreateInfoEXT,
        request_index: impl Into<Bool32>,
    ) -> Result<u32> {
        unsafe {
            raw::register_custom_border_color_ext(
                self,
                p_border_color,
                request_index,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(all(feature = "ext_descriptor_heap", feature = "ext_custom_border_color"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUnregisterCustomBorderColorEXT.html>"]
    #[doc(alias = "vkUnregisterCustomBorderColorEXT")]
    #[inline]
    pub fn unregister_custom_border_color_ext(&self, index: u32) {
        unsafe {
            raw::unregister_custom_border_color_ext(self, index, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(all(feature = "ext_descriptor_heap", feature = "ext_tensors"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDataARM.html>"]
    #[doc(alias = "vkGetTensorOpaqueCaptureDataARM")]
    pub fn get_tensor_opaque_capture_data_arm<
        'a,
        R: DynamicArray<HostAddressRangeEXT<'static>>,
        V2: Alias<raw::TensorARM> + 'a,
    >(
        &self,
        p_tensors: impl AsSlice<'a, V2>,
    ) -> Result<R> {
        unsafe {
            raw::get_tensor_opaque_capture_data_arm(
                self,
                p_tensors,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureKHR.html>"]
    #[doc(alias = "vkCreateAccelerationStructureKHR")]
    #[inline]
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
    #[cfg(feature = "ext_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureKHR.html>"]
    #[doc(alias = "vkDestroyAccelerationStructureKHR")]
    #[inline]
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
    #[cfg(feature = "ext_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildAccelerationStructuresKHR.html>"]
    #[doc(alias = "vkBuildAccelerationStructuresKHR")]
    #[inline]
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
    #[cfg(feature = "ext_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureKHR.html>"]
    #[doc(alias = "vkCopyAccelerationStructureKHR")]
    #[inline]
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
    #[cfg(feature = "ext_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureToMemoryKHR.html>"]
    #[doc(alias = "vkCopyAccelerationStructureToMemoryKHR")]
    #[inline]
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
    #[cfg(feature = "ext_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToAccelerationStructureKHR.html>"]
    #[doc(alias = "vkCopyMemoryToAccelerationStructureKHR")]
    #[inline]
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
    #[cfg(feature = "ext_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteAccelerationStructuresPropertiesKHR.html>"]
    #[doc(alias = "vkWriteAccelerationStructuresPropertiesKHR")]
    #[inline]
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
    #[cfg(feature = "ext_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureDeviceAddressKHR.html>"]
    #[doc(alias = "vkGetAccelerationStructureDeviceAddressKHR")]
    #[inline]
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
    #[cfg(feature = "ext_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceAccelerationStructureCompatibilityKHR.html>"]
    #[doc(alias = "vkGetDeviceAccelerationStructureCompatibilityKHR")]
    #[inline]
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
    #[cfg(feature = "ext_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureBuildSizesKHR.html>"]
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
    #[cfg(feature = "ext_ray_tracing_pipeline")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesKHR.html>"]
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
    #[cfg(any(feature = "ext_ray_tracing_pipeline", feature = "ext_ray_tracing"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesKHR.html>"]
    #[doc(alias = "vkGetRayTracingShaderGroupHandlesKHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_ray_tracing_pipeline", feature = "ext_ray_tracing"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesNV.html>"]
    #[doc(alias = "vkGetRayTracingShaderGroupHandlesNV")]
    #[inline]
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
    #[cfg(feature = "ext_ray_tracing_pipeline")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html>"]
    #[doc(alias = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")]
    #[inline]
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
    #[cfg(feature = "ext_ray_tracing_pipeline")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupStackSizeKHR.html>"]
    #[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
    #[inline]
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
    #[cfg(feature = "ext_image_drm_format_modifier")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageDrmFormatModifierPropertiesEXT.html>"]
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
    #[cfg(feature = "ext_validation_cache")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateValidationCacheEXT.html>"]
    #[doc(alias = "vkCreateValidationCacheEXT")]
    #[inline]
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
    #[cfg(feature = "ext_validation_cache")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyValidationCacheEXT.html>"]
    #[doc(alias = "vkDestroyValidationCacheEXT")]
    #[inline]
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
    #[cfg(feature = "ext_validation_cache")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkMergeValidationCachesEXT.html>"]
    #[doc(alias = "vkMergeValidationCachesEXT")]
    #[inline]
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
    #[cfg(feature = "ext_validation_cache")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetValidationCacheDataEXT.html>"]
    #[doc(alias = "vkGetValidationCacheDataEXT")]
    #[inline]
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
    #[cfg(feature = "ext_ray_tracing")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureNV.html>"]
    #[doc(alias = "vkCreateAccelerationStructureNV")]
    #[inline]
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
    #[cfg(feature = "ext_ray_tracing")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureNV.html>"]
    #[doc(alias = "vkDestroyAccelerationStructureNV")]
    #[inline]
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
    #[cfg(feature = "ext_ray_tracing")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureMemoryRequirementsNV.html>"]
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
    #[cfg(feature = "ext_ray_tracing")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindAccelerationStructureMemoryNV.html>"]
    #[doc(alias = "vkBindAccelerationStructureMemoryNV")]
    #[inline]
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
    #[cfg(feature = "ext_ray_tracing")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesNV.html>"]
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
    #[cfg(feature = "ext_ray_tracing")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureHandleNV.html>"]
    #[doc(alias = "vkGetAccelerationStructureHandleNV")]
    #[inline]
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
    #[cfg(feature = "ext_ray_tracing")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCompileDeferredNV.html>"]
    #[doc(alias = "vkCompileDeferredNV")]
    #[inline]
    pub fn compile_deferred_nv(&self, pipeline: &raw::Pipeline, shader: u32) -> Result<()> {
        unsafe {
            raw::compile_deferred_nv(self, pipeline, shader, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_external_memory_host")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryHostPointerPropertiesEXT.html>"]
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
    #[cfg(feature = "ext_present_timing")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetSwapchainPresentTimingQueueSizeEXT.html>"]
    #[doc(alias = "vkSetSwapchainPresentTimingQueueSizeEXT")]
    #[inline]
    pub fn set_swapchain_present_timing_queue_size_ext(
        &self,
        swapchain: &raw::SwapchainKHR,
        size: u32,
    ) -> Result<Status> {
        unsafe {
            raw::set_swapchain_present_timing_queue_size_ext(
                self,
                swapchain,
                size,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_present_timing")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingEXT.html>"]
    #[doc(alias = "vkGetPastPresentationTimingEXT")]
    pub fn get_past_presentation_timing_ext<
        S: StructureChainOut<PastPresentationTimingPropertiesEXT<'static>>,
    >(
        &self,
        p_past_presentation_timing_info: &PastPresentationTimingInfoEXT,
    ) -> Result<S> {
        unsafe {
            raw::get_past_presentation_timing_ext(
                self,
                p_past_presentation_timing_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_performance_query")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkInitializePerformanceApiINTEL.html>"]
    #[doc(alias = "vkInitializePerformanceApiINTEL")]
    #[inline]
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
    #[cfg(feature = "ext_performance_query")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUninitializePerformanceApiINTEL.html>"]
    #[doc(alias = "vkUninitializePerformanceApiINTEL")]
    #[inline]
    pub fn uninitialize_performance_api_intel(&self) {
        unsafe { raw::uninitialize_performance_api_intel(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_performance_query")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquirePerformanceConfigurationINTEL.html>"]
    #[doc(alias = "vkAcquirePerformanceConfigurationINTEL")]
    #[inline]
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
    #[cfg(feature = "ext_performance_query")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkReleasePerformanceConfigurationINTEL.html>"]
    #[doc(alias = "vkReleasePerformanceConfigurationINTEL")]
    #[inline]
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
    #[cfg(feature = "ext_performance_query")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPerformanceParameterINTEL.html>"]
    #[doc(alias = "vkGetPerformanceParameterINTEL")]
    #[inline]
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
    #[cfg(feature = "ext_display_native_hdr")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLocalDimmingAMD.html>"]
    #[doc(alias = "vkSetLocalDimmingAMD")]
    #[inline]
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
    #[cfg(feature = "ext_present_wait")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresentKHR.html>"]
    #[doc(alias = "vkWaitForPresentKHR")]
    #[inline]
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
    #[cfg(feature = "ext_full_screen_exclusive")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireFullScreenExclusiveModeEXT.html>"]
    #[doc(alias = "vkAcquireFullScreenExclusiveModeEXT")]
    #[inline]
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
    #[cfg(feature = "ext_full_screen_exclusive")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseFullScreenExclusiveModeEXT.html>"]
    #[doc(alias = "vkReleaseFullScreenExclusiveModeEXT")]
    #[inline]
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
    #[cfg(all(
        feature = "ext_full_screen_exclusive",
        any(feature = "ext_device_group", feature = "version_1_1")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModes2EXT.html>"]
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModes2EXT")]
    #[inline]
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
    #[cfg(feature = "ext_deferred_host_operations")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDeferredOperationKHR.html>"]
    #[doc(alias = "vkCreateDeferredOperationKHR")]
    #[inline]
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
    #[cfg(feature = "ext_deferred_host_operations")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDeferredOperationKHR.html>"]
    #[doc(alias = "vkDestroyDeferredOperationKHR")]
    #[inline]
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
    #[cfg(feature = "ext_deferred_host_operations")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationMaxConcurrencyKHR.html>"]
    #[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
    #[inline]
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
    #[cfg(feature = "ext_deferred_host_operations")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationResultKHR.html>"]
    #[doc(alias = "vkGetDeferredOperationResultKHR")]
    #[inline]
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
    #[cfg(feature = "ext_deferred_host_operations")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDeferredOperationJoinKHR.html>"]
    #[doc(alias = "vkDeferredOperationJoinKHR")]
    #[inline]
    pub fn deferred_operation_join_khr(
        &self,
        operation: &raw::DeferredOperationKHR,
    ) -> Result<Status> {
        unsafe {
            raw::deferred_operation_join_khr(self, operation, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_pipeline_executable_properties")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutablePropertiesKHR.html>"]
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
    #[cfg(feature = "ext_pipeline_executable_properties")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableStatisticsKHR.html>"]
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
    #[cfg(feature = "ext_pipeline_executable_properties")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableInternalRepresentationsKHR.html>"]
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
    #[cfg(feature = "ext_device_generated_commands")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsNV.html>"]
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
    #[cfg(feature = "ext_device_generated_commands")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutNV.html>"]
    #[doc(alias = "vkCreateIndirectCommandsLayoutNV")]
    #[inline]
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
    #[cfg(feature = "ext_device_generated_commands")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutNV.html>"]
    #[doc(alias = "vkDestroyIndirectCommandsLayoutNV")]
    #[inline]
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
    #[cfg(feature = "ext_cuda_kernel_launch")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaModuleNV.html>"]
    #[doc(alias = "vkCreateCudaModuleNV")]
    #[inline]
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
    #[cfg(feature = "ext_cuda_kernel_launch")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCudaModuleCacheNV.html>"]
    #[doc(alias = "vkGetCudaModuleCacheNV")]
    #[inline]
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
    #[cfg(feature = "ext_cuda_kernel_launch")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaFunctionNV.html>"]
    #[doc(alias = "vkCreateCudaFunctionNV")]
    #[inline]
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
    #[cfg(feature = "ext_cuda_kernel_launch")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaModuleNV.html>"]
    #[doc(alias = "vkDestroyCudaModuleNV")]
    #[inline]
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
    #[cfg(feature = "ext_cuda_kernel_launch")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaFunctionNV.html>"]
    #[doc(alias = "vkDestroyCudaFunctionNV")]
    #[inline]
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
    #[cfg(feature = "ext_metal_objects")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkExportMetalObjectsEXT.html>"]
    #[doc(alias = "vkExportMetalObjectsEXT")]
    pub fn export_metal_objects_ext<S: StructureChainOut<ExportMetalObjectsInfoEXT<'static>>>(
        &self,
    ) -> S {
        unsafe { raw::export_metal_objects_ext(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_descriptor_buffer")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSizeEXT.html>"]
    #[doc(alias = "vkGetDescriptorSetLayoutSizeEXT")]
    #[inline]
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
    #[cfg(feature = "ext_descriptor_buffer")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutBindingOffsetEXT.html>"]
    #[doc(alias = "vkGetDescriptorSetLayoutBindingOffsetEXT")]
    #[inline]
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
    #[cfg(feature = "ext_descriptor_buffer")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorEXT.html>"]
    #[doc(alias = "vkGetDescriptorEXT")]
    #[inline]
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
    #[cfg(feature = "ext_descriptor_buffer")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureDescriptorDataEXT.html>"]
    #[doc(alias = "vkGetBufferOpaqueCaptureDescriptorDataEXT")]
    #[inline]
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
    #[cfg(feature = "ext_descriptor_buffer")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDescriptorDataEXT.html>"]
    #[doc(alias = "vkGetImageOpaqueCaptureDescriptorDataEXT")]
    #[inline]
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
    #[cfg(feature = "ext_descriptor_buffer")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html>"]
    #[doc(alias = "vkGetImageViewOpaqueCaptureDescriptorDataEXT")]
    #[inline]
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
    #[cfg(feature = "ext_descriptor_buffer")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html>"]
    #[doc(alias = "vkGetSamplerOpaqueCaptureDescriptorDataEXT")]
    #[inline]
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
    #[cfg(all(
        feature = "ext_descriptor_buffer",
        any(feature = "ext_acceleration_structure", feature = "ext_ray_tracing")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html>"]
    #[doc(alias = "vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT")]
    #[inline]
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
    #[cfg(feature = "ext_fuchsia_external_memory")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandleFUCHSIA.html>"]
    #[doc(alias = "vkGetMemoryZirconHandleFUCHSIA")]
    #[inline]
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
    #[cfg(feature = "ext_fuchsia_external_memory")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandlePropertiesFUCHSIA.html>"]
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
    #[cfg(feature = "ext_fuchsia_external_semaphore")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreZirconHandleFUCHSIA.html>"]
    #[doc(alias = "vkImportSemaphoreZirconHandleFUCHSIA")]
    #[inline]
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
    #[cfg(feature = "ext_fuchsia_external_semaphore")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreZirconHandleFUCHSIA.html>"]
    #[doc(alias = "vkGetSemaphoreZirconHandleFUCHSIA")]
    #[inline]
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
    #[cfg(feature = "ext_buffer_collection")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferCollectionFUCHSIA.html>"]
    #[doc(alias = "vkCreateBufferCollectionFUCHSIA")]
    #[inline]
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
    #[cfg(feature = "ext_buffer_collection")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionImageConstraintsFUCHSIA.html>"]
    #[doc(alias = "vkSetBufferCollectionImageConstraintsFUCHSIA")]
    #[inline]
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
    #[cfg(feature = "ext_buffer_collection")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionBufferConstraintsFUCHSIA.html>"]
    #[doc(alias = "vkSetBufferCollectionBufferConstraintsFUCHSIA")]
    #[inline]
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
    #[cfg(feature = "ext_buffer_collection")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferCollectionFUCHSIA.html>"]
    #[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
    #[inline]
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
    #[cfg(feature = "ext_buffer_collection")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferCollectionPropertiesFUCHSIA.html>"]
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
    #[cfg(feature = "ext_subpass_shading")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html>"]
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
    #[cfg(feature = "ext_external_memory_rdma")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryRemoteAddressNV.html>"]
    #[doc(alias = "vkGetMemoryRemoteAddressNV")]
    #[inline]
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
    #[cfg(feature = "ext_pipeline_properties")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelinePropertiesEXT.html>"]
    #[doc(alias = "vkGetPipelinePropertiesEXT")]
    #[inline]
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
    #[cfg(feature = "ext_opacity_micromap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMicromapEXT.html>"]
    #[doc(alias = "vkCreateMicromapEXT")]
    #[inline]
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
    #[cfg(feature = "ext_opacity_micromap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyMicromapEXT.html>"]
    #[doc(alias = "vkDestroyMicromapEXT")]
    #[inline]
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
    #[cfg(feature = "ext_opacity_micromap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildMicromapsEXT.html>"]
    #[doc(alias = "vkBuildMicromapsEXT")]
    #[inline]
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
    #[cfg(feature = "ext_opacity_micromap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapEXT.html>"]
    #[doc(alias = "vkCopyMicromapEXT")]
    #[inline]
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
    #[cfg(feature = "ext_opacity_micromap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapToMemoryEXT.html>"]
    #[doc(alias = "vkCopyMicromapToMemoryEXT")]
    #[inline]
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
    #[cfg(feature = "ext_opacity_micromap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToMicromapEXT.html>"]
    #[doc(alias = "vkCopyMemoryToMicromapEXT")]
    #[inline]
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
    #[cfg(feature = "ext_opacity_micromap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteMicromapsPropertiesEXT.html>"]
    #[doc(alias = "vkWriteMicromapsPropertiesEXT")]
    #[inline]
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
    #[cfg(feature = "ext_opacity_micromap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMicromapCompatibilityEXT.html>"]
    #[doc(alias = "vkGetDeviceMicromapCompatibilityEXT")]
    #[inline]
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
    #[cfg(feature = "ext_opacity_micromap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMicromapBuildSizesEXT.html>"]
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
    #[cfg(feature = "ext_pageable_device_local_memory")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDeviceMemoryPriorityEXT.html>"]
    #[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
    #[inline]
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
    #[cfg(feature = "ext_descriptor_set_host_mapping")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html>"]
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
    #[cfg(feature = "ext_descriptor_set_host_mapping")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetHostMappingVALVE.html>"]
    #[doc(alias = "vkGetDescriptorSetHostMappingVALVE")]
    #[inline]
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
    #[cfg(feature = "ext_device_generated_commands_compute")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectMemoryRequirementsNV.html>"]
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
    #[cfg(feature = "ext_device_generated_commands_compute")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectDeviceAddressNV.html>"]
    #[doc(alias = "vkGetPipelineIndirectDeviceAddressNV")]
    #[inline]
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
    #[cfg(feature = "ext_ohos_external_memory")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetNativeBufferPropertiesOHOS.html>"]
    #[doc(alias = "vkGetNativeBufferPropertiesOHOS")]
    pub fn get_native_buffer_properties_ohos<
        S: StructureChainOut<NativeBufferPropertiesOHOS<'static>>,
    >(
        &self,
        buffer: &OH_NativeBuffer,
    ) -> Result<S> {
        unsafe {
            raw::get_native_buffer_properties_ohos(self, buffer, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_ohos_external_memory")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryNativeBufferOHOS.html>"]
    #[doc(alias = "vkGetMemoryNativeBufferOHOS")]
    #[inline]
    pub fn get_memory_native_buffer_ohos(
        &self,
        p_info: &MemoryGetNativeBufferInfoOHOS,
        p_buffer: &&OH_NativeBuffer,
    ) -> Result<()> {
        unsafe {
            raw::get_memory_native_buffer_ohos(
                self,
                p_info,
                p_buffer,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_tensors")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorARM.html>"]
    #[doc(alias = "vkCreateTensorARM")]
    #[inline]
    pub fn create_tensor_arm(&self, p_create_info: &TensorCreateInfoARM) -> Result<TensorARM> {
        let vk_result = unsafe {
            raw::create_tensor_arm(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { TensorARM::from_inner(vk_result) })
    }
    #[cfg(feature = "ext_tensors")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorARM.html>"]
    #[doc(alias = "vkDestroyTensorARM")]
    #[inline]
    pub unsafe fn destroy_tensor_arm(&self, tensor: Option<&raw::TensorARM>) {
        unsafe {
            raw::destroy_tensor_arm(
                self,
                tensor,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_tensors")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorViewARM.html>"]
    #[doc(alias = "vkCreateTensorViewARM")]
    #[inline]
    pub fn create_tensor_view_arm(
        &self,
        p_create_info: &TensorViewCreateInfoARM,
    ) -> Result<TensorViewARM> {
        let vk_result = unsafe {
            raw::create_tensor_view_arm(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { TensorViewARM::from_inner(vk_result) })
    }
    #[cfg(feature = "ext_tensors")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorViewARM.html>"]
    #[doc(alias = "vkDestroyTensorViewARM")]
    #[inline]
    pub unsafe fn destroy_tensor_view_arm(&self, tensor_view: Option<&raw::TensorViewARM>) {
        unsafe {
            raw::destroy_tensor_view_arm(
                self,
                tensor_view,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_tensors")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorMemoryRequirementsARM.html>"]
    #[doc(alias = "vkGetTensorMemoryRequirementsARM")]
    pub fn get_tensor_memory_requirements_arm<
        S: StructureChainOut<MemoryRequirements2<'static>>,
    >(
        &self,
        p_info: &TensorMemoryRequirementsInfoARM,
    ) -> S {
        unsafe {
            raw::get_tensor_memory_requirements_arm(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_tensors")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindTensorMemoryARM.html>"]
    #[doc(alias = "vkBindTensorMemoryARM")]
    #[inline]
    pub fn bind_tensor_memory_arm<'a>(
        &self,
        p_bind_infos: impl AsSlice<'a, BindTensorMemoryInfoARM<'a>>,
    ) -> Result<()> {
        unsafe {
            raw::bind_tensor_memory_arm(self, p_bind_infos, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_tensors")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceTensorMemoryRequirementsARM.html>"]
    #[doc(alias = "vkGetDeviceTensorMemoryRequirementsARM")]
    pub fn get_device_tensor_memory_requirements_arm<
        S: StructureChainOut<MemoryRequirements2<'static>>,
    >(
        &self,
        p_info: &DeviceTensorMemoryRequirementsARM,
    ) -> S {
        unsafe {
            raw::get_device_tensor_memory_requirements_arm(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(all(feature = "ext_tensors", feature = "ext_descriptor_buffer"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDescriptorDataARM.html>"]
    #[doc(alias = "vkGetTensorOpaqueCaptureDescriptorDataARM")]
    #[inline]
    pub fn get_tensor_opaque_capture_descriptor_data_arm(
        &self,
        p_info: &TensorCaptureDescriptorDataInfoARM,
        p_data: VoidPtr,
    ) -> Result<()> {
        unsafe {
            raw::get_tensor_opaque_capture_descriptor_data_arm(
                self,
                p_info,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(all(feature = "ext_tensors", feature = "ext_descriptor_buffer"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorViewOpaqueCaptureDescriptorDataARM.html>"]
    #[doc(alias = "vkGetTensorViewOpaqueCaptureDescriptorDataARM")]
    #[inline]
    pub fn get_tensor_view_opaque_capture_descriptor_data_arm(
        &self,
        p_info: &TensorViewCaptureDescriptorDataInfoARM,
        p_data: VoidPtr,
    ) -> Result<()> {
        unsafe {
            raw::get_tensor_view_opaque_capture_descriptor_data_arm(
                self,
                p_info,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_shader_module_identifier")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleIdentifierEXT.html>"]
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
    #[cfg(feature = "ext_shader_module_identifier")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleCreateInfoIdentifierEXT.html>"]
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
    #[cfg(feature = "ext_optical_flow")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateOpticalFlowSessionNV.html>"]
    #[doc(alias = "vkCreateOpticalFlowSessionNV")]
    #[inline]
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
    #[cfg(feature = "ext_optical_flow")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyOpticalFlowSessionNV.html>"]
    #[doc(alias = "vkDestroyOpticalFlowSessionNV")]
    #[inline]
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
    #[cfg(feature = "ext_optical_flow")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindOpticalFlowSessionImageNV.html>"]
    #[doc(alias = "vkBindOpticalFlowSessionImageNV")]
    #[inline]
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
    #[cfg(feature = "ext_anti_lag")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAntiLagUpdateAMD.html>"]
    #[doc(alias = "vkAntiLagUpdateAMD")]
    #[inline]
    pub fn anti_lag_update_amd(&self, p_data: &AntiLagDataAMD) {
        unsafe { raw::anti_lag_update_amd(self, p_data, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_present_wait2")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresent2KHR.html>"]
    #[doc(alias = "vkWaitForPresent2KHR")]
    #[inline]
    pub fn wait_for_present2_khr(
        &self,
        swapchain: &raw::SwapchainKHR,
        p_present_wait2_info: &PresentWait2InfoKHR,
    ) -> Result<Status> {
        unsafe {
            raw::wait_for_present2_khr(
                self,
                swapchain,
                p_present_wait2_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_shader_object")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShadersEXT.html>"]
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
    #[cfg(feature = "ext_shader_object")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderEXT.html>"]
    #[doc(alias = "vkDestroyShaderEXT")]
    #[inline]
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
    #[cfg(feature = "ext_shader_object")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderBinaryDataEXT.html>"]
    #[doc(alias = "vkGetShaderBinaryDataEXT")]
    #[inline]
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
    #[cfg(feature = "ext_pipeline_binary")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineBinariesKHR.html>"]
    #[doc(alias = "vkCreatePipelineBinariesKHR")]
    pub fn create_pipeline_binaries_khr<
        S: StructureChainOut<PipelineBinaryHandlesInfoKHR<'static>>,
    >(
        &self,
        p_create_info: &PipelineBinaryCreateInfoKHR,
    ) -> Result<(Status, S)> {
        unsafe {
            raw::create_pipeline_binaries_khr(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_pipeline_binary")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineBinaryKHR.html>"]
    #[doc(alias = "vkDestroyPipelineBinaryKHR")]
    #[inline]
    pub unsafe fn destroy_pipeline_binary_khr(
        &self,
        pipeline_binary: Option<&raw::PipelineBinaryKHR>,
    ) {
        unsafe {
            raw::destroy_pipeline_binary_khr(
                self,
                pipeline_binary,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_pipeline_binary")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineKeyKHR.html>"]
    #[doc(alias = "vkGetPipelineKeyKHR")]
    pub fn get_pipeline_key_khr<S: StructureChainOut<PipelineBinaryKeyKHR<'static>>>(
        &self,
        p_pipeline_create_info: Option<&PipelineCreateInfoKHR>,
    ) -> Result<S> {
        unsafe {
            raw::get_pipeline_key_khr(
                self,
                p_pipeline_create_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_pipeline_binary")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseCapturedPipelineDataKHR.html>"]
    #[doc(alias = "vkReleaseCapturedPipelineDataKHR")]
    #[inline]
    pub fn release_captured_pipeline_data_khr(
        &self,
        p_info: &ReleaseCapturedPipelineDataInfoKHR,
    ) -> Result<()> {
        unsafe {
            raw::release_captured_pipeline_data_khr(
                self,
                p_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_tile_properties")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFramebufferTilePropertiesQCOM.html>"]
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
    #[cfg(feature = "ext_tile_properties")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDynamicRenderingTilePropertiesQCOM.html>"]
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
    #[cfg(feature = "ext_swapchain_maintenance1")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesKHR.html>"]
    #[doc(alias = "vkReleaseSwapchainImagesKHR")]
    #[inline]
    pub fn release_swapchain_images_khr(
        &self,
        p_release_info: &ReleaseSwapchainImagesInfoKHR,
    ) -> Result<()> {
        unsafe {
            raw::release_swapchain_images_khr(
                self,
                p_release_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_swapchain_maintenance1")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesEXT.html>"]
    #[doc(alias = "vkReleaseSwapchainImagesEXT")]
    #[inline]
    pub fn release_swapchain_images_ext(
        &self,
        p_release_info: &ReleaseSwapchainImagesInfoKHR,
    ) -> Result<()> {
        unsafe {
            raw::release_swapchain_images_ext(
                self,
                p_release_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_cooperative_vector")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkConvertCooperativeVectorMatrixNV.html>"]
    #[doc(alias = "vkConvertCooperativeVectorMatrixNV")]
    #[inline]
    pub fn convert_cooperative_vector_matrix_nv(
        &self,
        p_info: &ConvertCooperativeVectorMatrixInfoNV,
    ) -> Result<()> {
        unsafe {
            raw::convert_cooperative_vector_matrix_nv(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_low_latency2")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeNV.html>"]
    #[doc(alias = "vkSetLatencySleepModeNV")]
    #[inline]
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
    #[cfg(feature = "ext_low_latency2")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepNV.html>"]
    #[doc(alias = "vkLatencySleepNV")]
    #[inline]
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
    #[cfg(feature = "ext_low_latency2")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerNV.html>"]
    #[doc(alias = "vkSetLatencyMarkerNV")]
    #[inline]
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
    #[cfg(feature = "ext_low_latency2")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsNV.html>"]
    #[doc(alias = "vkGetLatencyTimingsNV")]
    pub fn get_latency_timings_nv<S: StructureChainOut<GetLatencyMarkerInfoNV<'static>>>(
        &self,
        swapchain: &raw::SwapchainKHR,
    ) -> S {
        unsafe { raw::get_latency_timings_nv(self, swapchain, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_data_graph")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelinesARM.html>"]
    #[doc(alias = "vkCreateDataGraphPipelinesARM")]
    pub fn create_data_graph_pipelines_arm<'a, R: AdvancedDynamicArray<Pipeline, raw::Pipeline>>(
        &self,
        deferred_operation: Option<&raw::DeferredOperationKHR>,
        pipeline_cache: Option<&raw::PipelineCache>,
        p_create_infos: impl AsSlice<'a, DataGraphPipelineCreateInfoARM<'a>>,
    ) -> Result<(Status, R)> {
        let vk_result: Result<(Status, R::InnerArrayType)> = unsafe {
            raw::create_data_graph_pipelines_arm(
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
    #[cfg(feature = "ext_data_graph")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelineSessionARM.html>"]
    #[doc(alias = "vkCreateDataGraphPipelineSessionARM")]
    #[inline]
    pub fn create_data_graph_pipeline_session_arm(
        &self,
        p_create_info: &DataGraphPipelineSessionCreateInfoARM,
    ) -> Result<DataGraphPipelineSessionARM> {
        let vk_result = unsafe {
            raw::create_data_graph_pipeline_session_arm(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { DataGraphPipelineSessionARM::from_inner(vk_result) })
    }
    #[cfg(feature = "ext_data_graph")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionBindPointRequirementsARM.html>"]
    #[doc(alias = "vkGetDataGraphPipelineSessionBindPointRequirementsARM")]
    pub fn get_data_graph_pipeline_session_bind_point_requirements_arm<
        R: DynamicArray<DataGraphPipelineSessionBindPointRequirementARM<'static>>,
    >(
        &self,
        p_info: &DataGraphPipelineSessionBindPointRequirementsInfoARM,
    ) -> Result<R> {
        unsafe {
            raw::get_data_graph_pipeline_session_bind_point_requirements_arm(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_data_graph")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionMemoryRequirementsARM.html>"]
    #[doc(alias = "vkGetDataGraphPipelineSessionMemoryRequirementsARM")]
    pub fn get_data_graph_pipeline_session_memory_requirements_arm<
        S: StructureChainOut<MemoryRequirements2<'static>>,
    >(
        &self,
        p_info: &DataGraphPipelineSessionMemoryRequirementsInfoARM,
    ) -> S {
        unsafe {
            raw::get_data_graph_pipeline_session_memory_requirements_arm(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_data_graph")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindDataGraphPipelineSessionMemoryARM.html>"]
    #[doc(alias = "vkBindDataGraphPipelineSessionMemoryARM")]
    #[inline]
    pub fn bind_data_graph_pipeline_session_memory_arm<'a>(
        &self,
        p_bind_infos: impl AsSlice<'a, BindDataGraphPipelineSessionMemoryInfoARM<'a>>,
    ) -> Result<()> {
        unsafe {
            raw::bind_data_graph_pipeline_session_memory_arm(
                self,
                p_bind_infos,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_data_graph")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDataGraphPipelineSessionARM.html>"]
    #[doc(alias = "vkDestroyDataGraphPipelineSessionARM")]
    #[inline]
    pub unsafe fn destroy_data_graph_pipeline_session_arm(
        &self,
        session: &raw::DataGraphPipelineSessionARM,
    ) {
        unsafe {
            raw::destroy_data_graph_pipeline_session_arm(
                self,
                session,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_data_graph")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineAvailablePropertiesARM.html>"]
    #[doc(alias = "vkGetDataGraphPipelineAvailablePropertiesARM")]
    pub fn get_data_graph_pipeline_available_properties_arm<
        R: DynamicArray<DataGraphPipelinePropertyARM>,
    >(
        &self,
        p_pipeline_info: &DataGraphPipelineInfoARM,
    ) -> Result<R> {
        unsafe {
            raw::get_data_graph_pipeline_available_properties_arm(
                self,
                p_pipeline_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_data_graph")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelinePropertiesARM.html>"]
    #[doc(alias = "vkGetDataGraphPipelinePropertiesARM")]
    pub fn get_data_graph_pipeline_properties_arm<
        R: DynamicArray<DataGraphPipelinePropertyQueryResultARM<'static>>,
    >(
        &self,
        p_pipeline_info: &DataGraphPipelineInfoARM,
        properties_count: u32,
    ) -> Result<R> {
        unsafe {
            raw::get_data_graph_pipeline_properties_arm(
                self,
                p_pipeline_info,
                properties_count,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_external_memory_screen_buffer")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetScreenBufferPropertiesQNX.html>"]
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
    #[cfg(feature = "ext_external_compute_queue")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExternalComputeQueueNV.html>"]
    #[doc(alias = "vkCreateExternalComputeQueueNV")]
    #[inline]
    pub fn create_external_compute_queue_nv(
        &self,
        p_create_info: &ExternalComputeQueueCreateInfoNV,
    ) -> Result<ExternalComputeQueueNV<D, A>> {
        let vk_result = unsafe {
            raw::create_external_compute_queue_nv(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe {
            ExternalComputeQueueNV::from_inner(vk_result, self.disp.clone(), self.alloc.clone())
        })
    }
    #[cfg(feature = "ext_external_compute_queue")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyExternalComputeQueueNV.html>"]
    #[doc(alias = "vkDestroyExternalComputeQueueNV")]
    #[inline]
    pub unsafe fn destroy_external_compute_queue_nv(
        &self,
        external_queue: &raw::ExternalComputeQueueNV,
    ) {
        unsafe {
            raw::destroy_external_compute_queue_nv(
                self,
                external_queue,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_cluster_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetClusterAccelerationStructureBuildSizesNV.html>"]
    #[doc(alias = "vkGetClusterAccelerationStructureBuildSizesNV")]
    pub fn get_cluster_acceleration_structure_build_sizes_nv<
        S: StructureChainOut<AccelerationStructureBuildSizesInfoKHR<'static>>,
    >(
        &self,
        p_info: &ClusterAccelerationStructureInputInfoNV,
    ) -> S {
        unsafe {
            raw::get_cluster_acceleration_structure_build_sizes_nv(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_partitioned_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPartitionedAccelerationStructuresBuildSizesNV.html>"]
    #[doc(alias = "vkGetPartitionedAccelerationStructuresBuildSizesNV")]
    pub fn get_partitioned_acceleration_structures_build_sizes_nv<
        S: StructureChainOut<AccelerationStructureBuildSizesInfoKHR<'static>>,
    >(
        &self,
        p_info: &PartitionedAccelerationStructureInstancesInputNV,
    ) -> S {
        unsafe {
            raw::get_partitioned_acceleration_structures_build_sizes_nv(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_device_generated_commands")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsEXT.html>"]
    #[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsEXT")]
    pub fn get_generated_commands_memory_requirements_ext<
        S: StructureChainOut<MemoryRequirements2<'static>>,
    >(
        &self,
        p_info: &GeneratedCommandsMemoryRequirementsInfoEXT,
    ) -> S {
        unsafe {
            raw::get_generated_commands_memory_requirements_ext(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_device_generated_commands")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutEXT.html>"]
    #[doc(alias = "vkCreateIndirectCommandsLayoutEXT")]
    #[inline]
    pub fn create_indirect_commands_layout_ext(
        &self,
        p_create_info: &IndirectCommandsLayoutCreateInfoEXT,
    ) -> Result<IndirectCommandsLayoutEXT> {
        let vk_result = unsafe {
            raw::create_indirect_commands_layout_ext(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { IndirectCommandsLayoutEXT::from_inner(vk_result) })
    }
    #[cfg(feature = "ext_device_generated_commands")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutEXT.html>"]
    #[doc(alias = "vkDestroyIndirectCommandsLayoutEXT")]
    #[inline]
    pub unsafe fn destroy_indirect_commands_layout_ext(
        &self,
        indirect_commands_layout: Option<&raw::IndirectCommandsLayoutEXT>,
    ) {
        unsafe {
            raw::destroy_indirect_commands_layout_ext(
                self,
                indirect_commands_layout,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_device_generated_commands")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectExecutionSetEXT.html>"]
    #[doc(alias = "vkCreateIndirectExecutionSetEXT")]
    #[inline]
    pub fn create_indirect_execution_set_ext(
        &self,
        p_create_info: &IndirectExecutionSetCreateInfoEXT,
    ) -> Result<IndirectExecutionSetEXT> {
        let vk_result = unsafe {
            raw::create_indirect_execution_set_ext(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { IndirectExecutionSetEXT::from_inner(vk_result) })
    }
    #[cfg(feature = "ext_device_generated_commands")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectExecutionSetEXT.html>"]
    #[doc(alias = "vkDestroyIndirectExecutionSetEXT")]
    #[inline]
    pub unsafe fn destroy_indirect_execution_set_ext(
        &self,
        indirect_execution_set: Option<&raw::IndirectExecutionSetEXT>,
    ) {
        unsafe {
            raw::destroy_indirect_execution_set_ext(
                self,
                indirect_execution_set,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_device_generated_commands")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetPipelineEXT.html>"]
    #[doc(alias = "vkUpdateIndirectExecutionSetPipelineEXT")]
    #[inline]
    pub fn update_indirect_execution_set_pipeline_ext<'a>(
        &self,
        indirect_execution_set: &raw::IndirectExecutionSetEXT,
        p_execution_set_writes: impl AsSlice<'a, WriteIndirectExecutionSetPipelineEXT<'a>>,
    ) {
        unsafe {
            raw::update_indirect_execution_set_pipeline_ext(
                self,
                indirect_execution_set,
                p_execution_set_writes,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_device_generated_commands")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetShaderEXT.html>"]
    #[doc(alias = "vkUpdateIndirectExecutionSetShaderEXT")]
    #[inline]
    pub fn update_indirect_execution_set_shader_ext<'a>(
        &self,
        indirect_execution_set: &raw::IndirectExecutionSetEXT,
        p_execution_set_writes: impl AsSlice<'a, WriteIndirectExecutionSetShaderEXT<'a>>,
    ) {
        unsafe {
            raw::update_indirect_execution_set_shader_ext(
                self,
                indirect_execution_set,
                p_execution_set_writes,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_external_memory_metal")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandleEXT.html>"]
    #[doc(alias = "vkGetMemoryMetalHandleEXT")]
    #[inline]
    pub fn get_memory_metal_handle_ext(
        &self,
        p_get_metal_handle_info: &MemoryGetMetalHandleInfoEXT,
    ) -> Result<VoidPtr> {
        unsafe {
            raw::get_memory_metal_handle_ext(
                self,
                p_get_metal_handle_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_external_memory_metal")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandlePropertiesEXT.html>"]
    #[doc(alias = "vkGetMemoryMetalHandlePropertiesEXT")]
    pub fn get_memory_metal_handle_properties_ext<
        S: StructureChainOut<MemoryMetalHandlePropertiesEXT<'static>>,
    >(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        p_handle: VoidPtr,
    ) -> Result<S> {
        unsafe {
            raw::get_memory_metal_handle_properties_ext(
                self,
                handle_type,
                p_handle,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_shader_instrumentation")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderInstrumentationARM.html>"]
    #[doc(alias = "vkCreateShaderInstrumentationARM")]
    #[inline]
    pub fn create_shader_instrumentation_arm(
        &self,
        p_create_info: &ShaderInstrumentationCreateInfoARM,
    ) -> Result<ShaderInstrumentationARM> {
        let vk_result = unsafe {
            raw::create_shader_instrumentation_arm(
                self,
                p_create_info,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        };
        vk_result.map(|vk_result| unsafe { ShaderInstrumentationARM::from_inner(vk_result) })
    }
    #[cfg(feature = "ext_shader_instrumentation")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderInstrumentationARM.html>"]
    #[doc(alias = "vkDestroyShaderInstrumentationARM")]
    #[inline]
    pub unsafe fn destroy_shader_instrumentation_arm(
        &self,
        instrumentation: Option<&raw::ShaderInstrumentationARM>,
    ) {
        unsafe {
            raw::destroy_shader_instrumentation_arm(
                self,
                instrumentation,
                self.alloc.get_allocation_callbacks().as_ref(),
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_shader_instrumentation")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInstrumentationValuesARM.html>"]
    #[doc(alias = "vkGetShaderInstrumentationValuesARM")]
    #[inline]
    pub fn get_shader_instrumentation_values_arm(
        &self,
        instrumentation: &raw::ShaderInstrumentationARM,
        p_metric_values: VoidPtr,
        flags: u32,
    ) -> Result<u32> {
        unsafe {
            raw::get_shader_instrumentation_values_arm(
                self,
                instrumentation,
                p_metric_values,
                flags,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_shader_instrumentation")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkClearShaderInstrumentationMetricsARM.html>"]
    #[doc(alias = "vkClearShaderInstrumentationMetricsARM")]
    #[inline]
    pub fn clear_shader_instrumentation_metrics_arm(
        &self,
        instrumentation: &raw::ShaderInstrumentationARM,
    ) {
        unsafe {
            raw::clear_shader_instrumentation_metrics_arm(
                self,
                instrumentation,
                self.disp.get_command_dispatcher(),
            )
        }
    }
}
#[repr(C)]
#[derive(Clone)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueue.html>"]
#[doc(alias = "VkQueue")]
pub struct Queue<D: Dispatcher = DynamicDispatcher, A: BaseAllocator = DefaultAllocator> {
    inner: <raw::Queue as Handle>::InnerType,
    disp: D,
    alloc: A,
}
unsafe impl Alias<raw::Queue> for Queue {}
impl<D: Dispatcher, A: BaseAllocator> Copy for Queue<D, A>
where
    D: Copy,
    A: Copy,
{
}
impl<D: Dispatcher, A: BaseAllocator> Deref for Queue<D, A> {
    type Target = raw::Queue;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl<D: Dispatcher, A: BaseAllocator> Queue<D, A> {
    pub unsafe fn from_inner(handle: raw::Queue, disp: D, alloc: A) -> Self {
        Self {
            inner: handle.as_raw(),
            disp,
            alloc,
        }
    }
    #[inline(always)]
    pub fn get_dispatcher(&self) -> &D {
        &self.disp
    }
    #[inline(always)]
    pub fn get_allocator(&self) -> &A {
        &self.alloc
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit.html>"]
    #[doc(alias = "vkQueueSubmit")]
    #[inline]
    pub fn submit<'a>(
        &self,
        p_submits: impl AsSlice<'a, SubmitInfo<'a>>,
        fence: Option<&raw::Fence>,
    ) -> Result<()> {
        unsafe { raw::queue_submit(self, p_submits, fence, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueWaitIdle.html>"]
    #[doc(alias = "vkQueueWaitIdle")]
    #[inline]
    pub fn wait_idle(&self) -> Result<()> {
        unsafe { raw::queue_wait_idle(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBindSparse.html>"]
    #[doc(alias = "vkQueueBindSparse")]
    #[inline]
    pub fn bind_sparse<'a>(
        &self,
        p_bind_info: impl AsSlice<'a, BindSparseInfo<'a>>,
        fence: Option<&raw::Fence>,
    ) -> Result<()> {
        unsafe {
            raw::queue_bind_sparse(self, p_bind_info, fence, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html>"]
    #[doc(alias = "vkQueueSubmit2")]
    #[inline]
    pub fn submit2<'a>(
        &self,
        p_submits: impl AsSlice<'a, SubmitInfo2<'a>>,
        fence: Option<&raw::Fence>,
    ) -> Result<()> {
        unsafe { raw::queue_submit2(self, p_submits, fence, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2KHR.html>"]
    #[doc(alias = "vkQueueSubmit2KHR")]
    #[inline]
    pub fn submit2_khr<'a>(
        &self,
        p_submits: impl AsSlice<'a, SubmitInfo2<'a>>,
        fence: Option<&raw::Fence>,
    ) -> Result<()> {
        unsafe {
            raw::queue_submit2_khr(self, p_submits, fence, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_swapchain")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueuePresentKHR.html>"]
    #[doc(alias = "vkQueuePresentKHR")]
    #[inline]
    pub fn present_khr(&self, p_present_info: &PresentInfoKHR) -> Result<Status> {
        unsafe { raw::queue_present_khr(self, p_present_info, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_debug_utils")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBeginDebugUtilsLabelEXT.html>"]
    #[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
    #[inline]
    pub fn begin_debug_utils_label_ext(&self, p_label_info: &DebugUtilsLabelEXT) {
        unsafe {
            raw::queue_begin_debug_utils_label_ext(
                self,
                p_label_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_debug_utils")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueEndDebugUtilsLabelEXT.html>"]
    #[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
    #[inline]
    pub fn end_debug_utils_label_ext(&self) {
        unsafe { raw::queue_end_debug_utils_label_ext(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_debug_utils")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueInsertDebugUtilsLabelEXT.html>"]
    #[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
    #[inline]
    pub fn insert_debug_utils_label_ext(&self, p_label_info: &DebugUtilsLabelEXT) {
        unsafe {
            raw::queue_insert_debug_utils_label_ext(
                self,
                p_label_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_device_diagnostic_checkpoints")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointDataNV.html>"]
    #[doc(alias = "vkGetQueueCheckpointDataNV")]
    pub fn get_checkpoint_data_nv<R: DynamicArray<CheckpointDataNV<'static>>>(&self) -> R {
        unsafe { raw::get_queue_checkpoint_data_nv(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(all(
        feature = "ext_device_diagnostic_checkpoints",
        any(feature = "version_1_3", feature = "ext_synchronization2")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointData2NV.html>"]
    #[doc(alias = "vkGetQueueCheckpointData2NV")]
    pub fn get_checkpoint_data2_nv<R: DynamicArray<CheckpointData2NV<'static>>>(&self) -> R {
        unsafe { raw::get_queue_checkpoint_data2_nv(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_performance_query")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerformanceConfigurationINTEL.html>"]
    #[doc(alias = "vkQueueSetPerformanceConfigurationINTEL")]
    #[inline]
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
    #[cfg(feature = "ext_low_latency2")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandNV.html>"]
    #[doc(alias = "vkQueueNotifyOutOfBandNV")]
    #[inline]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemory.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFence.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphore.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPool.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBuffer.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImage.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageView.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPool.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBuffer.html>"]
#[doc(alias = "VkCommandBuffer")]
pub struct CommandBuffer<D: Dispatcher = DynamicDispatcher, A: BaseAllocator = DefaultAllocator> {
    inner: <raw::CommandBuffer as Handle>::InnerType,
    disp: D,
    alloc: A,
}
unsafe impl Alias<raw::CommandBuffer> for CommandBuffer {}
impl<D: Dispatcher, A: BaseAllocator> Copy for CommandBuffer<D, A>
where
    D: Copy,
    A: Copy,
{
}
impl<D: Dispatcher, A: BaseAllocator> Deref for CommandBuffer<D, A> {
    type Target = raw::CommandBuffer;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
impl<D: Dispatcher, A: BaseAllocator> CommandBuffer<D, A> {
    pub unsafe fn from_inner(handle: raw::CommandBuffer, disp: D, alloc: A) -> Self {
        Self {
            inner: handle.as_raw(),
            disp,
            alloc,
        }
    }
    #[inline(always)]
    pub fn get_dispatcher(&self) -> &D {
        &self.disp
    }
    #[inline(always)]
    pub fn get_allocator(&self) -> &A {
        &self.alloc
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBeginCommandBuffer.html>"]
    #[doc(alias = "vkBeginCommandBuffer")]
    #[inline]
    pub fn begin(&self, p_begin_info: &CommandBufferBeginInfo) -> Result<()> {
        unsafe { raw::begin_command_buffer(self, p_begin_info, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEndCommandBuffer.html>"]
    #[doc(alias = "vkEndCommandBuffer")]
    #[inline]
    pub fn end(&self) -> Result<()> {
        unsafe { raw::end_command_buffer(self, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandBuffer.html>"]
    #[doc(alias = "vkResetCommandBuffer")]
    #[inline]
    pub fn reset(&self, flags: CommandBufferResetFlags) -> Result<()> {
        unsafe { raw::reset_command_buffer(self, flags, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer.html>"]
    #[doc(alias = "vkCmdCopyBuffer")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage.html>"]
    #[doc(alias = "vkCmdCopyImage")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage.html>"]
    #[doc(alias = "vkCmdCopyBufferToImage")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer.html>"]
    #[doc(alias = "vkCmdCopyImageToBuffer")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateBuffer.html>"]
    #[doc(alias = "vkCmdUpdateBuffer")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillBuffer.html>"]
    #[doc(alias = "vkCmdFillBuffer")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier.html>"]
    #[doc(alias = "vkCmdPipelineBarrier")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQuery.html>"]
    #[doc(alias = "vkCmdBeginQuery")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQuery.html>"]
    #[doc(alias = "vkCmdEndQuery")]
    #[inline]
    pub fn end_query(&self, query_pool: &raw::QueryPool, query: u32) {
        unsafe { raw::cmd_end_query(self, query_pool, query, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetQueryPool.html>"]
    #[doc(alias = "vkCmdResetQueryPool")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp.html>"]
    #[doc(alias = "vkCmdWriteTimestamp")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResults.html>"]
    #[doc(alias = "vkCmdCopyQueryPoolResults")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteCommands.html>"]
    #[doc(alias = "vkCmdExecuteCommands")]
    #[inline]
    pub fn execute_commands<'a, V2: Alias<raw::CommandBuffer> + 'a>(
        &self,
        p_command_buffers: impl AsSlice<'a, V2>,
    ) {
        unsafe {
            raw::cmd_execute_commands(self, p_command_buffers, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipeline.html>"]
    #[doc(alias = "vkCmdBindPipeline")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets.html>"]
    #[doc(alias = "vkCmdBindDescriptorSets")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearColorImage.html>"]
    #[doc(alias = "vkCmdClearColorImage")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatch.html>"]
    #[doc(alias = "vkCmdDispatch")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect.html>"]
    #[doc(alias = "vkCmdDispatchIndirect")]
    #[inline]
    pub fn dispatch_indirect(&self, buffer: &raw::Buffer, offset: DeviceSize) {
        unsafe {
            raw::cmd_dispatch_indirect(self, buffer, offset, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent.html>"]
    #[doc(alias = "vkCmdSetEvent")]
    #[inline]
    pub fn set_event(&self, event: &raw::Event, stage_mask: PipelineStageFlags) {
        unsafe { raw::cmd_set_event(self, event, stage_mask, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent.html>"]
    #[doc(alias = "vkCmdResetEvent")]
    #[inline]
    pub fn reset_event(&self, event: &raw::Event, stage_mask: PipelineStageFlags) {
        unsafe { raw::cmd_reset_event(self, event, stage_mask, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents.html>"]
    #[doc(alias = "vkCmdWaitEvents")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants.html>"]
    #[doc(alias = "vkCmdPushConstants")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewport.html>"]
    #[doc(alias = "vkCmdSetViewport")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissor.html>"]
    #[doc(alias = "vkCmdSetScissor")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineWidth.html>"]
    #[doc(alias = "vkCmdSetLineWidth")]
    #[inline]
    pub fn set_line_width(&self, line_width: f32) {
        unsafe { raw::cmd_set_line_width(self, line_width, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias.html>"]
    #[doc(alias = "vkCmdSetDepthBias")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetBlendConstants.html>"]
    #[doc(alias = "vkCmdSetBlendConstants")]
    #[inline]
    pub fn set_blend_constants(&self, blend_constants: [f32; 4u16 as _]) {
        unsafe {
            raw::cmd_set_blend_constants(self, blend_constants, self.disp.get_command_dispatcher())
        }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBounds.html>"]
    #[doc(alias = "vkCmdSetDepthBounds")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilCompareMask.html>"]
    #[doc(alias = "vkCmdSetStencilCompareMask")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilWriteMask.html>"]
    #[doc(alias = "vkCmdSetStencilWriteMask")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilReference.html>"]
    #[doc(alias = "vkCmdSetStencilReference")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer.html>"]
    #[doc(alias = "vkCmdBindIndexBuffer")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers.html>"]
    #[doc(alias = "vkCmdBindVertexBuffers")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDraw.html>"]
    #[doc(alias = "vkCmdDraw")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexed.html>"]
    #[doc(alias = "vkCmdDrawIndexed")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect.html>"]
    #[doc(alias = "vkCmdDrawIndirect")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect.html>"]
    #[doc(alias = "vkCmdDrawIndexedIndirect")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage.html>"]
    #[doc(alias = "vkCmdBlitImage")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearDepthStencilImage.html>"]
    #[doc(alias = "vkCmdClearDepthStencilImage")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearAttachments.html>"]
    #[doc(alias = "vkCmdClearAttachments")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage.html>"]
    #[doc(alias = "vkCmdResolveImage")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass.html>"]
    #[doc(alias = "vkCmdBeginRenderPass")]
    #[inline]
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
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass.html>"]
    #[doc(alias = "vkCmdNextSubpass")]
    #[inline]
    pub fn next_subpass(&self, contents: SubpassContents) {
        unsafe { raw::cmd_next_subpass(self, contents, self.disp.get_command_dispatcher()) }
    }
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass.html>"]
    #[doc(alias = "vkCmdEndRenderPass")]
    #[inline]
    pub fn end_render_pass(&self) {
        unsafe { raw::cmd_end_render_pass(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(feature = "ext_device_group", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMask.html>"]
    #[doc(alias = "vkCmdSetDeviceMask")]
    #[inline]
    pub fn set_device_mask(&self, device_mask: u32) {
        unsafe { raw::cmd_set_device_mask(self, device_mask, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(feature = "ext_device_group", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMaskKHR.html>"]
    #[doc(alias = "vkCmdSetDeviceMaskKHR")]
    #[inline]
    pub fn set_device_mask_khr(&self, device_mask: u32) {
        unsafe {
            raw::cmd_set_device_mask_khr(self, device_mask, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_device_group", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBase.html>"]
    #[doc(alias = "vkCmdDispatchBase")]
    #[inline]
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
    #[cfg(any(feature = "ext_device_group", feature = "version_1_1"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBaseKHR.html>"]
    #[doc(alias = "vkCmdDispatchBaseKHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_draw_indirect_count", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html>"]
    #[doc(alias = "vkCmdDrawIndirectCount")]
    #[inline]
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
    #[cfg(any(feature = "ext_draw_indirect_count", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountKHR.html>"]
    #[doc(alias = "vkCmdDrawIndirectCountKHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_draw_indirect_count", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountAMD.html>"]
    #[doc(alias = "vkCmdDrawIndirectCountAMD")]
    #[inline]
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
    #[cfg(any(feature = "ext_draw_indirect_count", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html>"]
    #[doc(alias = "vkCmdDrawIndexedIndirectCount")]
    #[inline]
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
    #[cfg(any(feature = "ext_draw_indirect_count", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountKHR.html>"]
    #[doc(alias = "vkCmdDrawIndexedIndirectCountKHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_draw_indirect_count", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountAMD.html>"]
    #[doc(alias = "vkCmdDrawIndexedIndirectCountAMD")]
    #[inline]
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
    #[cfg(any(feature = "ext_create_renderpass2", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2.html>"]
    #[doc(alias = "vkCmdBeginRenderPass2")]
    #[inline]
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
    #[cfg(any(feature = "ext_create_renderpass2", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2KHR.html>"]
    #[doc(alias = "vkCmdBeginRenderPass2KHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_create_renderpass2", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2.html>"]
    #[doc(alias = "vkCmdNextSubpass2")]
    #[inline]
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
    #[cfg(any(feature = "ext_create_renderpass2", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2KHR.html>"]
    #[doc(alias = "vkCmdNextSubpass2KHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_create_renderpass2", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2.html>"]
    #[doc(alias = "vkCmdEndRenderPass2")]
    #[inline]
    pub fn end_render_pass2(&self, p_subpass_end_info: &SubpassEndInfo) {
        unsafe {
            raw::cmd_end_render_pass2(self, p_subpass_end_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_create_renderpass2", feature = "version_1_2"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2KHR.html>"]
    #[doc(alias = "vkCmdEndRenderPass2KHR")]
    #[inline]
    pub fn end_render_pass2_khr(&self, p_subpass_end_info: &SubpassEndInfo) {
        unsafe {
            raw::cmd_end_render_pass2_khr(
                self,
                p_subpass_end_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2.html>"]
    #[doc(alias = "vkCmdPipelineBarrier2")]
    #[inline]
    pub fn pipeline_barrier2(&self, p_dependency_info: &DependencyInfo) {
        unsafe {
            raw::cmd_pipeline_barrier2(self, p_dependency_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2KHR.html>"]
    #[doc(alias = "vkCmdPipelineBarrier2KHR")]
    #[inline]
    pub fn pipeline_barrier2_khr(&self, p_dependency_info: &DependencyInfo) {
        unsafe {
            raw::cmd_pipeline_barrier2_khr(
                self,
                p_dependency_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2.html>"]
    #[doc(alias = "vkCmdWriteTimestamp2")]
    #[inline]
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
    #[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2KHR.html>"]
    #[doc(alias = "vkCmdWriteTimestamp2KHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2.html>"]
    #[doc(alias = "vkCmdCopyBuffer2")]
    #[inline]
    pub fn copy_buffer2(&self, p_copy_buffer_info: &CopyBufferInfo2) {
        unsafe {
            raw::cmd_copy_buffer2(self, p_copy_buffer_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2KHR.html>"]
    #[doc(alias = "vkCmdCopyBuffer2KHR")]
    #[inline]
    pub fn copy_buffer2_khr(&self, p_copy_buffer_info: &CopyBufferInfo2) {
        unsafe {
            raw::cmd_copy_buffer2_khr(self, p_copy_buffer_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2.html>"]
    #[doc(alias = "vkCmdCopyImage2")]
    #[inline]
    pub fn copy_image2(&self, p_copy_image_info: &CopyImageInfo2) {
        unsafe { raw::cmd_copy_image2(self, p_copy_image_info, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2KHR.html>"]
    #[doc(alias = "vkCmdCopyImage2KHR")]
    #[inline]
    pub fn copy_image2_khr(&self, p_copy_image_info: &CopyImageInfo2) {
        unsafe {
            raw::cmd_copy_image2_khr(self, p_copy_image_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2.html>"]
    #[doc(alias = "vkCmdCopyBufferToImage2")]
    #[inline]
    pub fn copy_buffer_to_image2(&self, p_copy_buffer_to_image_info: &CopyBufferToImageInfo2) {
        unsafe {
            raw::cmd_copy_buffer_to_image2(
                self,
                p_copy_buffer_to_image_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2KHR.html>"]
    #[doc(alias = "vkCmdCopyBufferToImage2KHR")]
    #[inline]
    pub fn copy_buffer_to_image2_khr(&self, p_copy_buffer_to_image_info: &CopyBufferToImageInfo2) {
        unsafe {
            raw::cmd_copy_buffer_to_image2_khr(
                self,
                p_copy_buffer_to_image_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2.html>"]
    #[doc(alias = "vkCmdCopyImageToBuffer2")]
    #[inline]
    pub fn copy_image_to_buffer2(&self, p_copy_image_to_buffer_info: &CopyImageToBufferInfo2) {
        unsafe {
            raw::cmd_copy_image_to_buffer2(
                self,
                p_copy_image_to_buffer_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2KHR.html>"]
    #[doc(alias = "vkCmdCopyImageToBuffer2KHR")]
    #[inline]
    pub fn copy_image_to_buffer2_khr(&self, p_copy_image_to_buffer_info: &CopyImageToBufferInfo2) {
        unsafe {
            raw::cmd_copy_image_to_buffer2_khr(
                self,
                p_copy_image_to_buffer_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2.html>"]
    #[doc(alias = "vkCmdSetEvent2")]
    #[inline]
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
    #[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2KHR.html>"]
    #[doc(alias = "vkCmdSetEvent2KHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2.html>"]
    #[doc(alias = "vkCmdResetEvent2")]
    #[inline]
    pub fn reset_event2(&self, event: &raw::Event, stage_mask: PipelineStageFlags2) {
        unsafe {
            raw::cmd_reset_event2(self, event, stage_mask, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2KHR.html>"]
    #[doc(alias = "vkCmdResetEvent2KHR")]
    #[inline]
    pub fn reset_event2_khr(&self, event: &raw::Event, stage_mask: PipelineStageFlags2) {
        unsafe {
            raw::cmd_reset_event2_khr(self, event, stage_mask, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2.html>"]
    #[doc(alias = "vkCmdWaitEvents2")]
    #[inline]
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
    #[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2KHR.html>"]
    #[doc(alias = "vkCmdWaitEvents2KHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html>"]
    #[doc(alias = "vkCmdBlitImage2")]
    #[inline]
    pub fn blit_image2(&self, p_blit_image_info: &BlitImageInfo2) {
        unsafe { raw::cmd_blit_image2(self, p_blit_image_info, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2KHR.html>"]
    #[doc(alias = "vkCmdBlitImage2KHR")]
    #[inline]
    pub fn blit_image2_khr(&self, p_blit_image_info: &BlitImageInfo2) {
        unsafe {
            raw::cmd_blit_image2_khr(self, p_blit_image_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2.html>"]
    #[doc(alias = "vkCmdResolveImage2")]
    #[inline]
    pub fn resolve_image2(&self, p_resolve_image_info: &ResolveImageInfo2) {
        unsafe {
            raw::cmd_resolve_image2(
                self,
                p_resolve_image_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2KHR.html>"]
    #[doc(alias = "vkCmdResolveImage2KHR")]
    #[inline]
    pub fn resolve_image2_khr(&self, p_resolve_image_info: &ResolveImageInfo2) {
        unsafe {
            raw::cmd_resolve_image2_khr(
                self,
                p_resolve_image_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_dynamic_rendering", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRendering.html>"]
    #[doc(alias = "vkCmdBeginRendering")]
    #[inline]
    pub fn begin_rendering(&self, p_rendering_info: &RenderingInfo) {
        unsafe {
            raw::cmd_begin_rendering(self, p_rendering_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_dynamic_rendering", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderingKHR.html>"]
    #[doc(alias = "vkCmdBeginRenderingKHR")]
    #[inline]
    pub fn begin_rendering_khr(&self, p_rendering_info: &RenderingInfo) {
        unsafe {
            raw::cmd_begin_rendering_khr(self, p_rendering_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_dynamic_rendering", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering.html>"]
    #[doc(alias = "vkCmdEndRendering")]
    #[inline]
    pub fn end_rendering(&self) {
        unsafe { raw::cmd_end_rendering(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(feature = "ext_dynamic_rendering", feature = "version_1_3"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderingKHR.html>"]
    #[doc(alias = "vkCmdEndRenderingKHR")]
    #[inline]
    pub fn end_rendering_khr(&self) {
        unsafe { raw::cmd_end_rendering_khr(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullMode.html>"]
    #[doc(alias = "vkCmdSetCullMode")]
    #[inline]
    pub fn set_cull_mode(&self, cull_mode: CullModeFlags) {
        unsafe { raw::cmd_set_cull_mode(self, cull_mode, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullModeEXT.html>"]
    #[doc(alias = "vkCmdSetCullModeEXT")]
    #[inline]
    pub fn set_cull_mode_ext(&self, cull_mode: CullModeFlags) {
        unsafe { raw::cmd_set_cull_mode_ext(self, cull_mode, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFace.html>"]
    #[doc(alias = "vkCmdSetFrontFace")]
    #[inline]
    pub fn set_front_face(&self, front_face: FrontFace) {
        unsafe { raw::cmd_set_front_face(self, front_face, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFaceEXT.html>"]
    #[doc(alias = "vkCmdSetFrontFaceEXT")]
    #[inline]
    pub fn set_front_face_ext(&self, front_face: FrontFace) {
        unsafe { raw::cmd_set_front_face_ext(self, front_face, self.disp.get_command_dispatcher()) }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopology.html>"]
    #[doc(alias = "vkCmdSetPrimitiveTopology")]
    #[inline]
    pub fn set_primitive_topology(&self, primitive_topology: PrimitiveTopology) {
        unsafe {
            raw::cmd_set_primitive_topology(
                self,
                primitive_topology,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopologyEXT.html>"]
    #[doc(alias = "vkCmdSetPrimitiveTopologyEXT")]
    #[inline]
    pub fn set_primitive_topology_ext(&self, primitive_topology: PrimitiveTopology) {
        unsafe {
            raw::cmd_set_primitive_topology_ext(
                self,
                primitive_topology,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html>"]
    #[doc(alias = "vkCmdSetViewportWithCount")]
    #[inline]
    pub fn set_viewport_with_count<'a>(&self, p_viewports: impl AsSlice<'a, Viewport>) {
        unsafe {
            raw::cmd_set_viewport_with_count(self, p_viewports, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCountEXT.html>"]
    #[doc(alias = "vkCmdSetViewportWithCountEXT")]
    #[inline]
    pub fn set_viewport_with_count_ext<'a>(&self, p_viewports: impl AsSlice<'a, Viewport>) {
        unsafe {
            raw::cmd_set_viewport_with_count_ext(
                self,
                p_viewports,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html>"]
    #[doc(alias = "vkCmdSetScissorWithCount")]
    #[inline]
    pub fn set_scissor_with_count<'a>(&self, p_scissors: impl AsSlice<'a, Rect2D>) {
        unsafe {
            raw::cmd_set_scissor_with_count(self, p_scissors, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCountEXT.html>"]
    #[doc(alias = "vkCmdSetScissorWithCountEXT")]
    #[inline]
    pub fn set_scissor_with_count_ext<'a>(&self, p_scissors: impl AsSlice<'a, Rect2D>) {
        unsafe {
            raw::cmd_set_scissor_with_count_ext(
                self,
                p_scissors,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html>"]
    #[doc(alias = "vkCmdBindVertexBuffers2")]
    #[inline]
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
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2EXT.html>"]
    #[doc(alias = "vkCmdBindVertexBuffers2EXT")]
    #[inline]
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
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnable.html>"]
    #[doc(alias = "vkCmdSetDepthTestEnable")]
    #[inline]
    pub fn set_depth_test_enable(&self, depth_test_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_test_enable(
                self,
                depth_test_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnableEXT.html>"]
    #[doc(alias = "vkCmdSetDepthTestEnableEXT")]
    #[inline]
    pub fn set_depth_test_enable_ext(&self, depth_test_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_test_enable_ext(
                self,
                depth_test_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnable.html>"]
    #[doc(alias = "vkCmdSetDepthWriteEnable")]
    #[inline]
    pub fn set_depth_write_enable(&self, depth_write_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_write_enable(
                self,
                depth_write_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnableEXT.html>"]
    #[doc(alias = "vkCmdSetDepthWriteEnableEXT")]
    #[inline]
    pub fn set_depth_write_enable_ext(&self, depth_write_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_write_enable_ext(
                self,
                depth_write_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOp.html>"]
    #[doc(alias = "vkCmdSetDepthCompareOp")]
    #[inline]
    pub fn set_depth_compare_op(&self, depth_compare_op: CompareOp) {
        unsafe {
            raw::cmd_set_depth_compare_op(
                self,
                depth_compare_op,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOpEXT.html>"]
    #[doc(alias = "vkCmdSetDepthCompareOpEXT")]
    #[inline]
    pub fn set_depth_compare_op_ext(&self, depth_compare_op: CompareOp) {
        unsafe {
            raw::cmd_set_depth_compare_op_ext(
                self,
                depth_compare_op,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnable.html>"]
    #[doc(alias = "vkCmdSetDepthBoundsTestEnable")]
    #[inline]
    pub fn set_depth_bounds_test_enable(&self, depth_bounds_test_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_bounds_test_enable(
                self,
                depth_bounds_test_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnableEXT.html>"]
    #[doc(alias = "vkCmdSetDepthBoundsTestEnableEXT")]
    #[inline]
    pub fn set_depth_bounds_test_enable_ext(&self, depth_bounds_test_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_bounds_test_enable_ext(
                self,
                depth_bounds_test_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnable.html>"]
    #[doc(alias = "vkCmdSetStencilTestEnable")]
    #[inline]
    pub fn set_stencil_test_enable(&self, stencil_test_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_stencil_test_enable(
                self,
                stencil_test_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnableEXT.html>"]
    #[doc(alias = "vkCmdSetStencilTestEnableEXT")]
    #[inline]
    pub fn set_stencil_test_enable_ext(&self, stencil_test_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_stencil_test_enable_ext(
                self,
                stencil_test_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOp.html>"]
    #[doc(alias = "vkCmdSetStencilOp")]
    #[inline]
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
    #[cfg(any(
        feature = "ext_extended_dynamic_state",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOpEXT.html>"]
    #[doc(alias = "vkCmdSetStencilOpEXT")]
    #[inline]
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
    #[cfg(any(
        feature = "ext_extended_dynamic_state2",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnable.html>"]
    #[doc(alias = "vkCmdSetRasterizerDiscardEnable")]
    #[inline]
    pub fn set_rasterizer_discard_enable(&self, rasterizer_discard_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_rasterizer_discard_enable(
                self,
                rasterizer_discard_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state2",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnableEXT.html>"]
    #[doc(alias = "vkCmdSetRasterizerDiscardEnableEXT")]
    #[inline]
    pub fn set_rasterizer_discard_enable_ext(&self, rasterizer_discard_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_rasterizer_discard_enable_ext(
                self,
                rasterizer_discard_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state2",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnable.html>"]
    #[doc(alias = "vkCmdSetDepthBiasEnable")]
    #[inline]
    pub fn set_depth_bias_enable(&self, depth_bias_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_bias_enable(
                self,
                depth_bias_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state2",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnableEXT.html>"]
    #[doc(alias = "vkCmdSetDepthBiasEnableEXT")]
    #[inline]
    pub fn set_depth_bias_enable_ext(&self, depth_bias_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_bias_enable_ext(
                self,
                depth_bias_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state2",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnable.html>"]
    #[doc(alias = "vkCmdSetPrimitiveRestartEnable")]
    #[inline]
    pub fn set_primitive_restart_enable(&self, primitive_restart_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_primitive_restart_enable(
                self,
                primitive_restart_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_extended_dynamic_state2",
        feature = "ext_shader_object",
        feature = "version_1_3"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnableEXT.html>"]
    #[doc(alias = "vkCmdSetPrimitiveRestartEnableEXT")]
    #[inline]
    pub fn set_primitive_restart_enable_ext(&self, primitive_restart_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_primitive_restart_enable_ext(
                self,
                primitive_restart_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_push_descriptor", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet.html>"]
    #[doc(alias = "vkCmdPushDescriptorSet")]
    #[inline]
    pub fn push_descriptor_set<'a>(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: &raw::PipelineLayout,
        set: u32,
        p_descriptor_writes: impl AsSlice<'a, WriteDescriptorSet<'a>>,
    ) {
        unsafe {
            raw::cmd_push_descriptor_set(
                self,
                pipeline_bind_point,
                layout,
                set,
                p_descriptor_writes,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_push_descriptor", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetKHR.html>"]
    #[doc(alias = "vkCmdPushDescriptorSetKHR")]
    #[inline]
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
    #[cfg(any(
        all(
            feature = "ext_push_descriptor",
            any(feature = "version_1_1", feature = "ext_descriptor_update_template")
        ),
        all(
            feature = "ext_descriptor_update_template",
            feature = "ext_push_descriptor"
        ),
        feature = "version_1_4"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate.html>"]
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplate")]
    #[inline]
    pub fn push_descriptor_set_with_template(
        &self,
        descriptor_update_template: &raw::DescriptorUpdateTemplate,
        layout: &raw::PipelineLayout,
        set: u32,
        p_data: VoidPtr,
    ) {
        unsafe {
            raw::cmd_push_descriptor_set_with_template(
                self,
                descriptor_update_template,
                layout,
                set,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        all(
            feature = "ext_push_descriptor",
            any(feature = "version_1_1", feature = "ext_descriptor_update_template")
        ),
        all(
            feature = "ext_descriptor_update_template",
            feature = "ext_push_descriptor"
        ),
        feature = "version_1_4"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplateKHR.html>"]
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplateKHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_maintenance6", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html>"]
    #[doc(alias = "vkCmdBindDescriptorSets2")]
    #[inline]
    pub fn bind_descriptor_sets2(&self, p_bind_descriptor_sets_info: &BindDescriptorSetsInfo) {
        unsafe {
            raw::cmd_bind_descriptor_sets2(
                self,
                p_bind_descriptor_sets_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_maintenance6", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2KHR.html>"]
    #[doc(alias = "vkCmdBindDescriptorSets2KHR")]
    #[inline]
    pub fn bind_descriptor_sets2_khr(&self, p_bind_descriptor_sets_info: &BindDescriptorSetsInfo) {
        unsafe {
            raw::cmd_bind_descriptor_sets2_khr(
                self,
                p_bind_descriptor_sets_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_maintenance6", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2.html>"]
    #[doc(alias = "vkCmdPushConstants2")]
    #[inline]
    pub fn push_constants2(&self, p_push_constants_info: &PushConstantsInfo) {
        unsafe {
            raw::cmd_push_constants2(
                self,
                p_push_constants_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_maintenance6", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2KHR.html>"]
    #[doc(alias = "vkCmdPushConstants2KHR")]
    #[inline]
    pub fn push_constants2_khr(&self, p_push_constants_info: &PushConstantsInfo) {
        unsafe {
            raw::cmd_push_constants2_khr(
                self,
                p_push_constants_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        all(feature = "ext_maintenance6", feature = "ext_push_descriptor"),
        feature = "version_1_4"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html>"]
    #[doc(alias = "vkCmdPushDescriptorSet2")]
    #[inline]
    pub fn push_descriptor_set2(&self, p_push_descriptor_set_info: &PushDescriptorSetInfo) {
        unsafe {
            raw::cmd_push_descriptor_set2(
                self,
                p_push_descriptor_set_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        all(feature = "ext_maintenance6", feature = "ext_push_descriptor"),
        feature = "version_1_4"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2KHR.html>"]
    #[doc(alias = "vkCmdPushDescriptorSet2KHR")]
    #[inline]
    pub fn push_descriptor_set2_khr(&self, p_push_descriptor_set_info: &PushDescriptorSetInfo) {
        unsafe {
            raw::cmd_push_descriptor_set2_khr(
                self,
                p_push_descriptor_set_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        all(feature = "ext_maintenance6", feature = "ext_push_descriptor"),
        feature = "version_1_4"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2.html>"]
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplate2")]
    #[inline]
    pub fn push_descriptor_set_with_template2(
        &self,
        p_push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo,
    ) {
        unsafe {
            raw::cmd_push_descriptor_set_with_template2(
                self,
                p_push_descriptor_set_with_template_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        all(feature = "ext_maintenance6", feature = "ext_push_descriptor"),
        feature = "version_1_4"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2KHR.html>"]
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplate2KHR")]
    #[inline]
    pub fn push_descriptor_set_with_template2_khr(
        &self,
        p_push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo,
    ) {
        unsafe {
            raw::cmd_push_descriptor_set_with_template2_khr(
                self,
                p_push_descriptor_set_with_template_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_line_rasterization", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html>"]
    #[doc(alias = "vkCmdSetLineStipple")]
    #[inline]
    pub fn set_line_stipple(&self, line_stipple_factor: u32, line_stipple_pattern: u16) {
        unsafe {
            raw::cmd_set_line_stipple(
                self,
                line_stipple_factor,
                line_stipple_pattern,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_line_rasterization", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleKHR.html>"]
    #[doc(alias = "vkCmdSetLineStippleKHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_line_rasterization", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEXT.html>"]
    #[doc(alias = "vkCmdSetLineStippleEXT")]
    #[inline]
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
    #[cfg(any(feature = "ext_maintenance5", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2.html>"]
    #[doc(alias = "vkCmdBindIndexBuffer2")]
    #[inline]
    pub fn bind_index_buffer2(
        &self,
        buffer: Option<&raw::Buffer>,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ) {
        unsafe {
            raw::cmd_bind_index_buffer2(
                self,
                buffer,
                offset,
                size,
                index_type,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_maintenance5", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2KHR.html>"]
    #[doc(alias = "vkCmdBindIndexBuffer2KHR")]
    #[inline]
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
    #[cfg(any(feature = "ext_dynamic_rendering_local_read", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html>"]
    #[doc(alias = "vkCmdSetRenderingAttachmentLocations")]
    #[inline]
    pub fn set_rendering_attachment_locations(
        &self,
        p_location_info: &RenderingAttachmentLocationInfo,
    ) {
        unsafe {
            raw::cmd_set_rendering_attachment_locations(
                self,
                p_location_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_dynamic_rendering_local_read", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocationsKHR.html>"]
    #[doc(alias = "vkCmdSetRenderingAttachmentLocationsKHR")]
    #[inline]
    pub fn set_rendering_attachment_locations_khr(
        &self,
        p_location_info: &RenderingAttachmentLocationInfo,
    ) {
        unsafe {
            raw::cmd_set_rendering_attachment_locations_khr(
                self,
                p_location_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_dynamic_rendering_local_read", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html>"]
    #[doc(alias = "vkCmdSetRenderingInputAttachmentIndices")]
    #[inline]
    pub fn set_rendering_input_attachment_indices(
        &self,
        p_input_attachment_index_info: &RenderingInputAttachmentIndexInfo,
    ) {
        unsafe {
            raw::cmd_set_rendering_input_attachment_indices(
                self,
                p_input_attachment_index_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_dynamic_rendering_local_read", feature = "version_1_4"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndicesKHR.html>"]
    #[doc(alias = "vkCmdSetRenderingInputAttachmentIndicesKHR")]
    #[inline]
    pub fn set_rendering_input_attachment_indices_khr(
        &self,
        p_input_attachment_index_info: &RenderingInputAttachmentIndexInfo,
    ) {
        unsafe {
            raw::cmd_set_rendering_input_attachment_indices_khr(
                self,
                p_input_attachment_index_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_debug_marker")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerBeginEXT.html>"]
    #[doc(alias = "vkCmdDebugMarkerBeginEXT")]
    #[inline]
    pub fn debug_marker_begin_ext(&self, p_marker_info: &DebugMarkerMarkerInfoEXT) {
        unsafe {
            raw::cmd_debug_marker_begin_ext(self, p_marker_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_debug_marker")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerEndEXT.html>"]
    #[doc(alias = "vkCmdDebugMarkerEndEXT")]
    #[inline]
    pub fn debug_marker_end_ext(&self) {
        unsafe { raw::cmd_debug_marker_end_ext(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_debug_marker")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerInsertEXT.html>"]
    #[doc(alias = "vkCmdDebugMarkerInsertEXT")]
    #[inline]
    pub fn debug_marker_insert_ext(&self, p_marker_info: &DebugMarkerMarkerInfoEXT) {
        unsafe {
            raw::cmd_debug_marker_insert_ext(
                self,
                p_marker_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_transform_feedback")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffersEXT.html>"]
    #[doc(alias = "vkCmdBindTransformFeedbackBuffersEXT")]
    #[inline]
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
    #[cfg(feature = "ext_transform_feedback")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedbackEXT.html>"]
    #[doc(alias = "vkCmdBeginTransformFeedbackEXT")]
    #[inline]
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
    #[cfg(feature = "ext_transform_feedback")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedbackEXT.html>"]
    #[doc(alias = "vkCmdEndTransformFeedbackEXT")]
    #[inline]
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
    #[cfg(feature = "ext_transform_feedback")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQueryIndexedEXT.html>"]
    #[doc(alias = "vkCmdBeginQueryIndexedEXT")]
    #[inline]
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
    #[cfg(feature = "ext_transform_feedback")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQueryIndexedEXT.html>"]
    #[doc(alias = "vkCmdEndQueryIndexedEXT")]
    #[inline]
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
    #[cfg(feature = "ext_transform_feedback")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCountEXT.html>"]
    #[doc(alias = "vkCmdDrawIndirectByteCountEXT")]
    #[inline]
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
    #[cfg(feature = "ext_binary_import")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCuLaunchKernelNVX.html>"]
    #[doc(alias = "vkCmdCuLaunchKernelNVX")]
    #[inline]
    pub fn cu_launch_kernel_nvx(&self, p_launch_info: &CuLaunchInfoNVX) {
        unsafe {
            raw::cmd_cu_launch_kernel_nvx(self, p_launch_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_conditional_rendering")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRenderingEXT.html>"]
    #[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
    #[inline]
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
    #[cfg(feature = "ext_conditional_rendering")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndConditionalRenderingEXT.html>"]
    #[doc(alias = "vkCmdEndConditionalRenderingEXT")]
    #[inline]
    pub fn end_conditional_rendering_ext(&self) {
        unsafe { raw::cmd_end_conditional_rendering_ext(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_clip_space_w_scaling")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingNV.html>"]
    #[doc(alias = "vkCmdSetViewportWScalingNV")]
    #[inline]
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
    #[cfg(feature = "ext_discard_rectangles")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEXT.html>"]
    #[doc(alias = "vkCmdSetDiscardRectangleEXT")]
    #[inline]
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
    #[cfg(feature = "ext_discard_rectangles")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEnableEXT.html>"]
    #[doc(alias = "vkCmdSetDiscardRectangleEnableEXT")]
    #[inline]
    pub fn set_discard_rectangle_enable_ext(&self, discard_rectangle_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_discard_rectangle_enable_ext(
                self,
                discard_rectangle_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_discard_rectangles")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleModeEXT.html>"]
    #[doc(alias = "vkCmdSetDiscardRectangleModeEXT")]
    #[inline]
    pub fn set_discard_rectangle_mode_ext(&self, discard_rectangle_mode: DiscardRectangleModeEXT) {
        unsafe {
            raw::cmd_set_discard_rectangle_mode_ext(
                self,
                discard_rectangle_mode,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_debug_utils")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginDebugUtilsLabelEXT.html>"]
    #[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
    #[inline]
    pub fn begin_debug_utils_label_ext(&self, p_label_info: &DebugUtilsLabelEXT) {
        unsafe {
            raw::cmd_begin_debug_utils_label_ext(
                self,
                p_label_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_debug_utils")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndDebugUtilsLabelEXT.html>"]
    #[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
    #[inline]
    pub fn end_debug_utils_label_ext(&self) {
        unsafe { raw::cmd_end_debug_utils_label_ext(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_debug_utils")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInsertDebugUtilsLabelEXT.html>"]
    #[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
    #[inline]
    pub fn insert_debug_utils_label_ext(&self, p_label_info: &DebugUtilsLabelEXT) {
        unsafe {
            raw::cmd_insert_debug_utils_label_ext(
                self,
                p_label_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_shader_enqueue")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInitializeGraphScratchMemoryAMDX.html>"]
    #[doc(alias = "vkCmdInitializeGraphScratchMemoryAMDX")]
    #[inline]
    pub fn initialize_graph_scratch_memory_amdx(
        &self,
        execution_graph: &raw::Pipeline,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
    ) {
        unsafe {
            raw::cmd_initialize_graph_scratch_memory_amdx(
                self,
                execution_graph,
                scratch,
                scratch_size,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_shader_enqueue")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphAMDX.html>"]
    #[doc(alias = "vkCmdDispatchGraphAMDX")]
    #[inline]
    pub fn dispatch_graph_amdx(
        &self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        p_count_info: &DispatchGraphCountInfoAMDX,
    ) {
        unsafe {
            raw::cmd_dispatch_graph_amdx(
                self,
                scratch,
                scratch_size,
                p_count_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_shader_enqueue")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectAMDX.html>"]
    #[doc(alias = "vkCmdDispatchGraphIndirectAMDX")]
    #[inline]
    pub fn dispatch_graph_indirect_amdx(
        &self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        p_count_info: &DispatchGraphCountInfoAMDX,
    ) {
        unsafe {
            raw::cmd_dispatch_graph_indirect_amdx(
                self,
                scratch,
                scratch_size,
                p_count_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_shader_enqueue")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectCountAMDX.html>"]
    #[doc(alias = "vkCmdDispatchGraphIndirectCountAMDX")]
    #[inline]
    pub fn dispatch_graph_indirect_count_amdx(
        &self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: DeviceAddress,
    ) {
        unsafe {
            raw::cmd_dispatch_graph_indirect_count_amdx(
                self,
                scratch,
                scratch_size,
                count_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_descriptor_heap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindSamplerHeapEXT.html>"]
    #[doc(alias = "vkCmdBindSamplerHeapEXT")]
    #[inline]
    pub fn bind_sampler_heap_ext(&self, p_bind_info: &BindHeapInfoEXT) {
        unsafe {
            raw::cmd_bind_sampler_heap_ext(self, p_bind_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_descriptor_heap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindResourceHeapEXT.html>"]
    #[doc(alias = "vkCmdBindResourceHeapEXT")]
    #[inline]
    pub fn bind_resource_heap_ext(&self, p_bind_info: &BindHeapInfoEXT) {
        unsafe {
            raw::cmd_bind_resource_heap_ext(self, p_bind_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_descriptor_heap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDataEXT.html>"]
    #[doc(alias = "vkCmdPushDataEXT")]
    #[inline]
    pub fn push_data_ext(&self, p_push_data_info: &PushDataInfoEXT) {
        unsafe {
            raw::cmd_push_data_ext(self, p_push_data_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_sample_locations")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEXT.html>"]
    #[doc(alias = "vkCmdSetSampleLocationsEXT")]
    #[inline]
    pub fn set_sample_locations_ext(&self, p_sample_locations_info: &SampleLocationsInfoEXT) {
        unsafe {
            raw::cmd_set_sample_locations_ext(
                self,
                p_sample_locations_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresKHR.html>"]
    #[doc(alias = "vkCmdBuildAccelerationStructuresKHR")]
    #[inline]
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
    #[cfg(feature = "ext_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresIndirectKHR.html>"]
    #[doc(alias = "vkCmdBuildAccelerationStructuresIndirectKHR")]
    #[inline]
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
    #[cfg(feature = "ext_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureKHR.html>"]
    #[doc(alias = "vkCmdCopyAccelerationStructureKHR")]
    #[inline]
    pub fn copy_acceleration_structure_khr(&self, p_info: &CopyAccelerationStructureInfoKHR) {
        unsafe {
            raw::cmd_copy_acceleration_structure_khr(
                self,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureToMemoryKHR.html>"]
    #[doc(alias = "vkCmdCopyAccelerationStructureToMemoryKHR")]
    #[inline]
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
    #[cfg(feature = "ext_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToAccelerationStructureKHR.html>"]
    #[doc(alias = "vkCmdCopyMemoryToAccelerationStructureKHR")]
    #[inline]
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
    #[cfg(feature = "ext_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesKHR.html>"]
    #[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesKHR")]
    #[inline]
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
    #[cfg(feature = "ext_ray_tracing_pipeline")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysKHR.html>"]
    #[doc(alias = "vkCmdTraceRaysKHR")]
    #[inline]
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
    #[cfg(feature = "ext_ray_tracing_pipeline")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirectKHR.html>"]
    #[doc(alias = "vkCmdTraceRaysIndirectKHR")]
    #[inline]
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
    #[cfg(feature = "ext_ray_tracing_pipeline")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRayTracingPipelineStackSizeKHR.html>"]
    #[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
    #[inline]
    pub fn set_ray_tracing_pipeline_stack_size_khr(&self, pipeline_stack_size: u32) {
        unsafe {
            raw::cmd_set_ray_tracing_pipeline_stack_size_khr(
                self,
                pipeline_stack_size,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_shading_rate_image")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadingRateImageNV.html>"]
    #[doc(alias = "vkCmdBindShadingRateImageNV")]
    #[inline]
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
    #[cfg(feature = "ext_shading_rate_image")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportShadingRatePaletteNV.html>"]
    #[doc(alias = "vkCmdSetViewportShadingRatePaletteNV")]
    #[inline]
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
    #[cfg(feature = "ext_shading_rate_image")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoarseSampleOrderNV.html>"]
    #[doc(alias = "vkCmdSetCoarseSampleOrderNV")]
    #[inline]
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
    #[cfg(feature = "ext_ray_tracing")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructureNV.html>"]
    #[doc(alias = "vkCmdBuildAccelerationStructureNV")]
    #[inline]
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
    #[cfg(feature = "ext_ray_tracing")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureNV.html>"]
    #[doc(alias = "vkCmdCopyAccelerationStructureNV")]
    #[inline]
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
    #[cfg(feature = "ext_ray_tracing")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysNV.html>"]
    #[doc(alias = "vkCmdTraceRaysNV")]
    #[inline]
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
    #[cfg(feature = "ext_ray_tracing")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesNV.html>"]
    #[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesNV")]
    #[inline]
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
    #[cfg(feature = "ext_buffer_marker")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarkerAMD.html>"]
    #[doc(alias = "vkCmdWriteBufferMarkerAMD")]
    #[inline]
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
    #[cfg(all(
        feature = "ext_buffer_marker",
        any(feature = "version_1_3", feature = "ext_synchronization2")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarker2AMD.html>"]
    #[doc(alias = "vkCmdWriteBufferMarker2AMD")]
    #[inline]
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
    #[cfg(feature = "ext_mesh_shader")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksNV.html>"]
    #[doc(alias = "vkCmdDrawMeshTasksNV")]
    #[inline]
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
    #[cfg(feature = "ext_mesh_shader")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectNV.html>"]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
    #[inline]
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
    #[cfg(all(
        feature = "ext_mesh_shader",
        any(feature = "version_1_2", feature = "ext_draw_indirect_count")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountNV.html>"]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectCountNV")]
    #[inline]
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
    #[cfg(feature = "ext_scissor_exclusive")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorEnableNV.html>"]
    #[doc(alias = "vkCmdSetExclusiveScissorEnableNV")]
    #[inline]
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
    #[cfg(feature = "ext_scissor_exclusive")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorNV.html>"]
    #[doc(alias = "vkCmdSetExclusiveScissorNV")]
    #[inline]
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
    #[cfg(feature = "ext_device_diagnostic_checkpoints")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCheckpointNV.html>"]
    #[doc(alias = "vkCmdSetCheckpointNV")]
    #[inline]
    pub fn set_checkpoint_nv(&self, p_checkpoint_marker: VoidPtr) {
        unsafe {
            raw::cmd_set_checkpoint_nv(
                self,
                p_checkpoint_marker,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_performance_query")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceMarkerINTEL.html>"]
    #[doc(alias = "vkCmdSetPerformanceMarkerINTEL")]
    #[inline]
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
    #[cfg(feature = "ext_performance_query")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceStreamMarkerINTEL.html>"]
    #[doc(alias = "vkCmdSetPerformanceStreamMarkerINTEL")]
    #[inline]
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
    #[cfg(feature = "ext_performance_query")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceOverrideINTEL.html>"]
    #[doc(alias = "vkCmdSetPerformanceOverrideINTEL")]
    #[inline]
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
    #[cfg(feature = "ext_fragment_shading_rate")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateKHR.html>"]
    #[doc(alias = "vkCmdSetFragmentShadingRateKHR")]
    #[inline]
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
    #[cfg(feature = "ext_device_generated_commands")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsNV.html>"]
    #[doc(alias = "vkCmdPreprocessGeneratedCommandsNV")]
    #[inline]
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
    #[cfg(feature = "ext_device_generated_commands")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsNV.html>"]
    #[doc(alias = "vkCmdExecuteGeneratedCommandsNV")]
    #[inline]
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
    #[cfg(feature = "ext_device_generated_commands")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipelineShaderGroupNV.html>"]
    #[doc(alias = "vkCmdBindPipelineShaderGroupNV")]
    #[inline]
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
    #[cfg(feature = "ext_depth_bias_control")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias2EXT.html>"]
    #[doc(alias = "vkCmdSetDepthBias2EXT")]
    #[inline]
    pub fn set_depth_bias2_ext(&self, p_depth_bias_info: &DepthBiasInfoEXT) {
        unsafe {
            raw::cmd_set_depth_bias2_ext(
                self,
                p_depth_bias_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_cuda_kernel_launch")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCudaLaunchKernelNV.html>"]
    #[doc(alias = "vkCmdCudaLaunchKernelNV")]
    #[inline]
    pub fn cuda_launch_kernel_nv(&self, p_launch_info: &CudaLaunchInfoNV) {
        unsafe {
            raw::cmd_cuda_launch_kernel_nv(self, p_launch_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_tile_shading")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchTileQCOM.html>"]
    #[doc(alias = "vkCmdDispatchTileQCOM")]
    #[inline]
    pub fn dispatch_tile_qcom(&self, p_dispatch_tile_info: &DispatchTileInfoQCOM) {
        unsafe {
            raw::cmd_dispatch_tile_qcom(
                self,
                p_dispatch_tile_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_tile_shading")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginPerTileExecutionQCOM.html>"]
    #[doc(alias = "vkCmdBeginPerTileExecutionQCOM")]
    #[inline]
    pub fn begin_per_tile_execution_qcom(&self, p_per_tile_begin_info: &PerTileBeginInfoQCOM) {
        unsafe {
            raw::cmd_begin_per_tile_execution_qcom(
                self,
                p_per_tile_begin_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_tile_shading")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndPerTileExecutionQCOM.html>"]
    #[doc(alias = "vkCmdEndPerTileExecutionQCOM")]
    #[inline]
    pub fn end_per_tile_execution_qcom(&self, p_per_tile_end_info: &PerTileEndInfoQCOM) {
        unsafe {
            raw::cmd_end_per_tile_execution_qcom(
                self,
                p_per_tile_end_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_descriptor_buffer")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBuffersEXT.html>"]
    #[doc(alias = "vkCmdBindDescriptorBuffersEXT")]
    #[inline]
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
    #[cfg(feature = "ext_descriptor_buffer")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsetsEXT.html>"]
    #[doc(alias = "vkCmdSetDescriptorBufferOffsetsEXT")]
    #[inline]
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
    #[cfg(feature = "ext_descriptor_buffer")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html>"]
    #[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplersEXT")]
    #[inline]
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
    #[cfg(feature = "ext_fragment_shading_rate_enums")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateEnumNV.html>"]
    #[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
    #[inline]
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
    #[cfg(feature = "ext_mesh_shader")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksEXT.html>"]
    #[doc(alias = "vkCmdDrawMeshTasksEXT")]
    #[inline]
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
    #[cfg(feature = "ext_mesh_shader")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectEXT.html>"]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectEXT")]
    #[inline]
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
    #[cfg(all(
        feature = "ext_mesh_shader",
        any(feature = "version_1_2", feature = "ext_draw_indirect_count")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountEXT.html>"]
    #[doc(alias = "vkCmdDrawMeshTasksIndirectCountEXT")]
    #[inline]
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
    #[cfg(any(
        feature = "ext_vertex_input_dynamic_state",
        feature = "ext_shader_object"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetVertexInputEXT.html>"]
    #[doc(alias = "vkCmdSetVertexInputEXT")]
    #[inline]
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
    #[cfg(feature = "ext_subpass_shading")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSubpassShadingHUAWEI.html>"]
    #[doc(alias = "vkCmdSubpassShadingHUAWEI")]
    #[inline]
    pub fn subpass_shading_huawei(&self) {
        unsafe { raw::cmd_subpass_shading_huawei(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_invocation_mask")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindInvocationMaskHUAWEI.html>"]
    #[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
    #[inline]
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
    #[cfg(any(feature = "ext_extended_dynamic_state2", feature = "ext_shader_object"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPatchControlPointsEXT.html>"]
    #[doc(alias = "vkCmdSetPatchControlPointsEXT")]
    #[inline]
    pub fn set_patch_control_points_ext(&self, patch_control_points: u32) {
        unsafe {
            raw::cmd_set_patch_control_points_ext(
                self,
                patch_control_points,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_extended_dynamic_state2", feature = "ext_shader_object"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEXT.html>"]
    #[doc(alias = "vkCmdSetLogicOpEXT")]
    #[inline]
    pub fn set_logic_op_ext(&self, logic_op: LogicOp) {
        unsafe { raw::cmd_set_logic_op_ext(self, logic_op, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_color_write_enable")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteEnableEXT.html>"]
    #[doc(alias = "vkCmdSetColorWriteEnableEXT")]
    #[inline]
    pub fn set_color_write_enable_ext<'a>(&self, p_color_write_enables: impl AsSlice<'a, Bool32>) {
        unsafe {
            raw::cmd_set_color_write_enable_ext(
                self,
                p_color_write_enables,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(all(
        feature = "ext_ray_tracing_maintenance1",
        feature = "ext_ray_tracing_pipeline"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirect2KHR.html>"]
    #[doc(alias = "vkCmdTraceRaysIndirect2KHR")]
    #[inline]
    pub fn trace_rays_indirect2_khr(&self, indirect_device_address: DeviceAddress) {
        unsafe {
            raw::cmd_trace_rays_indirect2_khr(
                self,
                indirect_device_address,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_multi_draw")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiEXT.html>"]
    #[doc(alias = "vkCmdDrawMultiEXT")]
    #[inline]
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
    #[cfg(feature = "ext_multi_draw")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiIndexedEXT.html>"]
    #[doc(alias = "vkCmdDrawMultiIndexedEXT")]
    #[inline]
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
    #[cfg(feature = "ext_opacity_micromap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildMicromapsEXT.html>"]
    #[doc(alias = "vkCmdBuildMicromapsEXT")]
    #[inline]
    pub fn build_micromaps_ext<'a>(&self, p_infos: impl AsSlice<'a, MicromapBuildInfoEXT<'a>>) {
        unsafe { raw::cmd_build_micromaps_ext(self, p_infos, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_opacity_micromap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapEXT.html>"]
    #[doc(alias = "vkCmdCopyMicromapEXT")]
    #[inline]
    pub fn copy_micromap_ext(&self, p_info: &CopyMicromapInfoEXT) {
        unsafe { raw::cmd_copy_micromap_ext(self, p_info, self.disp.get_command_dispatcher()) }
    }
    #[cfg(feature = "ext_opacity_micromap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapToMemoryEXT.html>"]
    #[doc(alias = "vkCmdCopyMicromapToMemoryEXT")]
    #[inline]
    pub fn copy_micromap_to_memory_ext(&self, p_info: &CopyMicromapToMemoryInfoEXT) {
        unsafe {
            raw::cmd_copy_micromap_to_memory_ext(self, p_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_opacity_micromap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToMicromapEXT.html>"]
    #[doc(alias = "vkCmdCopyMemoryToMicromapEXT")]
    #[inline]
    pub fn copy_memory_to_micromap_ext(&self, p_info: &CopyMemoryToMicromapInfoEXT) {
        unsafe {
            raw::cmd_copy_memory_to_micromap_ext(self, p_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_opacity_micromap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMicromapsPropertiesEXT.html>"]
    #[doc(alias = "vkCmdWriteMicromapsPropertiesEXT")]
    #[inline]
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
    #[cfg(feature = "ext_cluster_culling_shader")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterHUAWEI.html>"]
    #[doc(alias = "vkCmdDrawClusterHUAWEI")]
    #[inline]
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
    #[cfg(feature = "ext_cluster_culling_shader")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterIndirectHUAWEI.html>"]
    #[doc(alias = "vkCmdDrawClusterIndirectHUAWEI")]
    #[inline]
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
    #[cfg(feature = "ext_copy_memory_indirect")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectNV.html>"]
    #[doc(alias = "vkCmdCopyMemoryIndirectNV")]
    #[inline]
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
    #[cfg(feature = "ext_copy_memory_indirect")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectNV.html>"]
    #[doc(alias = "vkCmdCopyMemoryToImageIndirectNV")]
    #[inline]
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
    #[cfg(feature = "ext_memory_decompression")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryNV.html>"]
    #[doc(alias = "vkCmdDecompressMemoryNV")]
    #[inline]
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
    #[cfg(feature = "ext_memory_decompression")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountNV.html>"]
    #[doc(alias = "vkCmdDecompressMemoryIndirectCountNV")]
    #[inline]
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
    #[cfg(feature = "ext_device_generated_commands_compute")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdatePipelineIndirectBufferNV.html>"]
    #[doc(alias = "vkCmdUpdatePipelineIndirectBufferNV")]
    #[inline]
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
    #[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampEnableEXT.html>"]
    #[doc(alias = "vkCmdSetDepthClampEnableEXT")]
    #[inline]
    pub fn set_depth_clamp_enable_ext(&self, depth_clamp_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_clamp_enable_ext(
                self,
                depth_clamp_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPolygonModeEXT.html>"]
    #[doc(alias = "vkCmdSetPolygonModeEXT")]
    #[inline]
    pub fn set_polygon_mode_ext(&self, polygon_mode: PolygonMode) {
        unsafe {
            raw::cmd_set_polygon_mode_ext(self, polygon_mode, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationSamplesEXT.html>"]
    #[doc(alias = "vkCmdSetRasterizationSamplesEXT")]
    #[inline]
    pub fn set_rasterization_samples_ext(&self, rasterization_samples: SampleCountFlags) {
        unsafe {
            raw::cmd_set_rasterization_samples_ext(
                self,
                rasterization_samples,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleMaskEXT.html>"]
    #[doc(alias = "vkCmdSetSampleMaskEXT")]
    #[inline]
    pub fn set_sample_mask_ext<'a>(
        &self,
        samples: SampleCountFlags,
        p_sample_mask: Option<impl AsSlice<'a, SampleMask>>,
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
    #[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToCoverageEnableEXT.html>"]
    #[doc(alias = "vkCmdSetAlphaToCoverageEnableEXT")]
    #[inline]
    pub fn set_alpha_to_coverage_enable_ext(&self, alpha_to_coverage_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_alpha_to_coverage_enable_ext(
                self,
                alpha_to_coverage_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToOneEnableEXT.html>"]
    #[doc(alias = "vkCmdSetAlphaToOneEnableEXT")]
    #[inline]
    pub fn set_alpha_to_one_enable_ext(&self, alpha_to_one_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_alpha_to_one_enable_ext(
                self,
                alpha_to_one_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEnableEXT.html>"]
    #[doc(alias = "vkCmdSetLogicOpEnableEXT")]
    #[inline]
    pub fn set_logic_op_enable_ext(&self, logic_op_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_logic_op_enable_ext(
                self,
                logic_op_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEnableEXT.html>"]
    #[doc(alias = "vkCmdSetColorBlendEnableEXT")]
    #[inline]
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
    #[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEquationEXT.html>"]
    #[doc(alias = "vkCmdSetColorBlendEquationEXT")]
    #[inline]
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
    #[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteMaskEXT.html>"]
    #[doc(alias = "vkCmdSetColorWriteMaskEXT")]
    #[inline]
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
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            any(feature = "ext_maintenance2", feature = "version_1_1")
        ),
        feature = "ext_shader_object"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetTessellationDomainOriginEXT.html>"]
    #[doc(alias = "vkCmdSetTessellationDomainOriginEXT")]
    #[inline]
    pub fn set_tessellation_domain_origin_ext(&self, domain_origin: TessellationDomainOrigin) {
        unsafe {
            raw::cmd_set_tessellation_domain_origin_ext(
                self,
                domain_origin,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_transform_feedback"
        ),
        all(feature = "ext_shader_object", feature = "ext_transform_feedback")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationStreamEXT.html>"]
    #[doc(alias = "vkCmdSetRasterizationStreamEXT")]
    #[inline]
    pub fn set_rasterization_stream_ext(&self, rasterization_stream: u32) {
        unsafe {
            raw::cmd_set_rasterization_stream_ext(
                self,
                rasterization_stream,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_conservative_rasterization"
        ),
        all(
            feature = "ext_shader_object",
            feature = "ext_conservative_rasterization"
        )
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetConservativeRasterizationModeEXT.html>"]
    #[doc(alias = "vkCmdSetConservativeRasterizationModeEXT")]
    #[inline]
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
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_conservative_rasterization"
        ),
        all(
            feature = "ext_shader_object",
            feature = "ext_conservative_rasterization"
        )
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html>"]
    #[doc(alias = "vkCmdSetExtraPrimitiveOverestimationSizeEXT")]
    #[inline]
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
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_depth_clip_enable"
        ),
        all(feature = "ext_shader_object", feature = "ext_depth_clip_enable")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipEnableEXT.html>"]
    #[doc(alias = "vkCmdSetDepthClipEnableEXT")]
    #[inline]
    pub fn set_depth_clip_enable_ext(&self, depth_clip_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_clip_enable_ext(
                self,
                depth_clip_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_sample_locations"
        ),
        all(feature = "ext_shader_object", feature = "ext_sample_locations")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEnableEXT.html>"]
    #[doc(alias = "vkCmdSetSampleLocationsEnableEXT")]
    #[inline]
    pub fn set_sample_locations_enable_ext(&self, sample_locations_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_sample_locations_enable_ext(
                self,
                sample_locations_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_blend_operation_advanced"
        ),
        all(
            feature = "ext_shader_object",
            feature = "ext_blend_operation_advanced"
        )
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendAdvancedEXT.html>"]
    #[doc(alias = "vkCmdSetColorBlendAdvancedEXT")]
    #[inline]
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
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_provoking_vertex"
        ),
        all(feature = "ext_shader_object", feature = "ext_provoking_vertex")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetProvokingVertexModeEXT.html>"]
    #[doc(alias = "vkCmdSetProvokingVertexModeEXT")]
    #[inline]
    pub fn set_provoking_vertex_mode_ext(&self, provoking_vertex_mode: ProvokingVertexModeEXT) {
        unsafe {
            raw::cmd_set_provoking_vertex_mode_ext(
                self,
                provoking_vertex_mode,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_line_rasterization"
        ),
        all(feature = "ext_shader_object", feature = "ext_line_rasterization")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineRasterizationModeEXT.html>"]
    #[doc(alias = "vkCmdSetLineRasterizationModeEXT")]
    #[inline]
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
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_line_rasterization"
        ),
        all(feature = "ext_shader_object", feature = "ext_line_rasterization")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEnableEXT.html>"]
    #[doc(alias = "vkCmdSetLineStippleEnableEXT")]
    #[inline]
    pub fn set_line_stipple_enable_ext(&self, stippled_line_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_line_stipple_enable_ext(
                self,
                stippled_line_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_depth_clip_control"
        ),
        all(feature = "ext_shader_object", feature = "ext_depth_clip_control")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipNegativeOneToOneEXT.html>"]
    #[doc(alias = "vkCmdSetDepthClipNegativeOneToOneEXT")]
    #[inline]
    pub fn set_depth_clip_negative_one_to_one_ext(&self, negative_one_to_one: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_depth_clip_negative_one_to_one_ext(
                self,
                negative_one_to_one,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_clip_space_w_scaling"
        ),
        all(feature = "ext_shader_object", feature = "ext_clip_space_w_scaling")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingEnableNV.html>"]
    #[doc(alias = "vkCmdSetViewportWScalingEnableNV")]
    #[inline]
    pub fn set_viewport_wscaling_enable_nv(&self, viewport_wscaling_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_viewport_wscaling_enable_nv(
                self,
                viewport_wscaling_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_viewport_swizzle"
        ),
        all(feature = "ext_shader_object", feature = "ext_viewport_swizzle")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportSwizzleNV.html>"]
    #[doc(alias = "vkCmdSetViewportSwizzleNV")]
    #[inline]
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
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_fragment_coverage_to_color"
        ),
        all(
            feature = "ext_shader_object",
            feature = "ext_fragment_coverage_to_color"
        )
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorEnableNV.html>"]
    #[doc(alias = "vkCmdSetCoverageToColorEnableNV")]
    #[inline]
    pub fn set_coverage_to_color_enable_nv(&self, coverage_to_color_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_coverage_to_color_enable_nv(
                self,
                coverage_to_color_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_fragment_coverage_to_color"
        ),
        all(
            feature = "ext_shader_object",
            feature = "ext_fragment_coverage_to_color"
        )
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorLocationNV.html>"]
    #[doc(alias = "vkCmdSetCoverageToColorLocationNV")]
    #[inline]
    pub fn set_coverage_to_color_location_nv(&self, coverage_to_color_location: u32) {
        unsafe {
            raw::cmd_set_coverage_to_color_location_nv(
                self,
                coverage_to_color_location,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_framebuffer_mixed_samples"
        ),
        all(
            feature = "ext_shader_object",
            feature = "ext_framebuffer_mixed_samples"
        )
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationModeNV.html>"]
    #[doc(alias = "vkCmdSetCoverageModulationModeNV")]
    #[inline]
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
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_framebuffer_mixed_samples"
        ),
        all(
            feature = "ext_shader_object",
            feature = "ext_framebuffer_mixed_samples"
        )
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableEnableNV.html>"]
    #[doc(alias = "vkCmdSetCoverageModulationTableEnableNV")]
    #[inline]
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
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_framebuffer_mixed_samples"
        ),
        all(
            feature = "ext_shader_object",
            feature = "ext_framebuffer_mixed_samples"
        )
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableNV.html>"]
    #[doc(alias = "vkCmdSetCoverageModulationTableNV")]
    #[inline]
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
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_shading_rate_image"
        ),
        all(feature = "ext_shader_object", feature = "ext_shading_rate_image")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetShadingRateImageEnableNV.html>"]
    #[doc(alias = "vkCmdSetShadingRateImageEnableNV")]
    #[inline]
    pub fn set_shading_rate_image_enable_nv(&self, shading_rate_image_enable: impl Into<Bool32>) {
        unsafe {
            raw::cmd_set_shading_rate_image_enable_nv(
                self,
                shading_rate_image_enable,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_representative_fragment_test"
        ),
        all(
            feature = "ext_shader_object",
            feature = "ext_representative_fragment_test"
        )
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRepresentativeFragmentTestEnableNV.html>"]
    #[doc(alias = "vkCmdSetRepresentativeFragmentTestEnableNV")]
    #[inline]
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
    #[cfg(any(
        all(
            feature = "ext_extended_dynamic_state3",
            feature = "ext_coverage_reduction_mode"
        ),
        all(feature = "ext_shader_object", feature = "ext_coverage_reduction_mode")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageReductionModeNV.html>"]
    #[doc(alias = "vkCmdSetCoverageReductionModeNV")]
    #[inline]
    pub fn set_coverage_reduction_mode_nv(&self, coverage_reduction_mode: CoverageReductionModeNV) {
        unsafe {
            raw::cmd_set_coverage_reduction_mode_nv(
                self,
                coverage_reduction_mode,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_tensors")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyTensorARM.html>"]
    #[doc(alias = "vkCmdCopyTensorARM")]
    #[inline]
    pub fn copy_tensor_arm(&self, p_copy_tensor_info: &CopyTensorInfoARM) {
        unsafe {
            raw::cmd_copy_tensor_arm(self, p_copy_tensor_info, self.disp.get_command_dispatcher())
        }
    }
    #[cfg(feature = "ext_optical_flow")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdOpticalFlowExecuteNV.html>"]
    #[doc(alias = "vkCmdOpticalFlowExecuteNV")]
    #[inline]
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
    #[cfg(feature = "ext_shader_object")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadersEXT.html>"]
    #[doc(alias = "vkCmdBindShadersEXT")]
    #[inline]
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
    #[cfg(all(feature = "ext_shader_object", feature = "ext_depth_clamp_control"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampRangeEXT.html>"]
    #[doc(alias = "vkCmdSetDepthClampRangeEXT")]
    #[inline]
    pub fn set_depth_clamp_range_ext(
        &self,
        depth_clamp_mode: DepthClampModeEXT,
        p_depth_clamp_range: Option<&DepthClampRangeEXT>,
    ) {
        unsafe {
            raw::cmd_set_depth_clamp_range_ext(
                self,
                depth_clamp_mode,
                p_depth_clamp_range,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_cooperative_vector")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdConvertCooperativeVectorMatrixNV.html>"]
    #[doc(alias = "vkCmdConvertCooperativeVectorMatrixNV")]
    #[inline]
    pub fn convert_cooperative_vector_matrix_nv<'a>(
        &self,
        p_infos: impl AsSlice<'a, ConvertCooperativeVectorMatrixInfoNV<'a>>,
    ) {
        unsafe {
            raw::cmd_convert_cooperative_vector_matrix_nv(
                self,
                p_infos,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_data_graph")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchDataGraphARM.html>"]
    #[doc(alias = "vkCmdDispatchDataGraphARM")]
    #[inline]
    pub fn dispatch_data_graph_arm(
        &self,
        session: &raw::DataGraphPipelineSessionARM,
        p_info: Option<&DataGraphPipelineDispatchInfoARM>,
    ) {
        unsafe {
            raw::cmd_dispatch_data_graph_arm(
                self,
                session,
                p_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_attachment_feedback_loop_dynamic_state")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAttachmentFeedbackLoopEnableEXT.html>"]
    #[doc(alias = "vkCmdSetAttachmentFeedbackLoopEnableEXT")]
    #[inline]
    pub fn set_attachment_feedback_loop_enable_ext(&self, aspect_mask: ImageAspectFlags) {
        unsafe {
            raw::cmd_set_attachment_feedback_loop_enable_ext(
                self,
                aspect_mask,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(all(feature = "ext_maintenance6", feature = "ext_descriptor_buffer"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsets2EXT.html>"]
    #[doc(alias = "vkCmdSetDescriptorBufferOffsets2EXT")]
    #[inline]
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
    #[cfg(all(feature = "ext_maintenance6", feature = "ext_descriptor_buffer"))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html>"]
    #[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplers2EXT")]
    #[inline]
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
    #[cfg(feature = "ext_tile_memory_heap")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTileMemoryQCOM.html>"]
    #[doc(alias = "vkCmdBindTileMemoryQCOM")]
    #[inline]
    pub fn bind_tile_memory_qcom(&self, p_tile_memory_bind_info: Option<&TileMemoryBindInfoQCOM>) {
        unsafe {
            raw::cmd_bind_tile_memory_qcom(
                self,
                p_tile_memory_bind_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_copy_memory_indirect")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectKHR.html>"]
    #[doc(alias = "vkCmdCopyMemoryIndirectKHR")]
    #[inline]
    pub fn copy_memory_indirect_khr(
        &self,
        p_copy_memory_indirect_info: &CopyMemoryIndirectInfoKHR,
    ) {
        unsafe {
            raw::cmd_copy_memory_indirect_khr(
                self,
                p_copy_memory_indirect_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_copy_memory_indirect")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectKHR.html>"]
    #[doc(alias = "vkCmdCopyMemoryToImageIndirectKHR")]
    #[inline]
    pub fn copy_memory_to_image_indirect_khr(
        &self,
        p_copy_memory_to_image_indirect_info: &CopyMemoryToImageIndirectInfoKHR,
    ) {
        unsafe {
            raw::cmd_copy_memory_to_image_indirect_khr(
                self,
                p_copy_memory_to_image_indirect_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_memory_decompression")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryEXT.html>"]
    #[doc(alias = "vkCmdDecompressMemoryEXT")]
    #[inline]
    pub fn decompress_memory_ext(&self, p_decompress_memory_info_ext: &DecompressMemoryInfoEXT) {
        unsafe {
            raw::cmd_decompress_memory_ext(
                self,
                p_decompress_memory_info_ext,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_memory_decompression")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountEXT.html>"]
    #[doc(alias = "vkCmdDecompressMemoryIndirectCountEXT")]
    #[inline]
    pub fn decompress_memory_indirect_count_ext(
        &self,
        decompression_method: MemoryDecompressionMethodFlagsEXT,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        max_decompression_count: u32,
        stride: u32,
    ) {
        unsafe {
            raw::cmd_decompress_memory_indirect_count_ext(
                self,
                decompression_method,
                indirect_commands_address,
                indirect_commands_count_address,
                max_decompression_count,
                stride,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_cluster_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildClusterAccelerationStructureIndirectNV.html>"]
    #[doc(alias = "vkCmdBuildClusterAccelerationStructureIndirectNV")]
    #[inline]
    pub fn build_cluster_acceleration_structure_indirect_nv(
        &self,
        p_command_infos: &ClusterAccelerationStructureCommandsInfoNV,
    ) {
        unsafe {
            raw::cmd_build_cluster_acceleration_structure_indirect_nv(
                self,
                p_command_infos,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_partitioned_acceleration_structure")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildPartitionedAccelerationStructuresNV.html>"]
    #[doc(alias = "vkCmdBuildPartitionedAccelerationStructuresNV")]
    #[inline]
    pub fn build_partitioned_acceleration_structures_nv(
        &self,
        p_build_info: &BuildPartitionedAccelerationStructureInfoNV,
    ) {
        unsafe {
            raw::cmd_build_partitioned_acceleration_structures_nv(
                self,
                p_build_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_device_generated_commands")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsEXT.html>"]
    #[doc(alias = "vkCmdPreprocessGeneratedCommandsEXT")]
    #[inline]
    pub fn preprocess_generated_commands_ext(
        &self,
        p_generated_commands_info: &GeneratedCommandsInfoEXT,
        state_command_buffer: &raw::CommandBuffer,
    ) {
        unsafe {
            raw::cmd_preprocess_generated_commands_ext(
                self,
                p_generated_commands_info,
                state_command_buffer,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_device_generated_commands")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsEXT.html>"]
    #[doc(alias = "vkCmdExecuteGeneratedCommandsEXT")]
    #[inline]
    pub fn execute_generated_commands_ext(
        &self,
        is_preprocessed: impl Into<Bool32>,
        p_generated_commands_info: &GeneratedCommandsInfoEXT,
    ) {
        unsafe {
            raw::cmd_execute_generated_commands_ext(
                self,
                is_preprocessed,
                p_generated_commands_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_shader_instrumentation")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginShaderInstrumentationARM.html>"]
    #[doc(alias = "vkCmdBeginShaderInstrumentationARM")]
    #[inline]
    pub fn begin_shader_instrumentation_arm(
        &self,
        instrumentation: &raw::ShaderInstrumentationARM,
    ) {
        unsafe {
            raw::cmd_begin_shader_instrumentation_arm(
                self,
                instrumentation,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_shader_instrumentation")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndShaderInstrumentationARM.html>"]
    #[doc(alias = "vkCmdEndShaderInstrumentationARM")]
    #[inline]
    pub fn end_shader_instrumentation_arm(&self) {
        unsafe { raw::cmd_end_shader_instrumentation_arm(self, self.disp.get_command_dispatcher()) }
    }
    #[cfg(all(
        feature = "ext_custom_resolve",
        any(feature = "ext_dynamic_rendering", feature = "version_1_3")
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginCustomResolveEXT.html>"]
    #[doc(alias = "vkCmdBeginCustomResolveEXT")]
    #[inline]
    pub fn begin_custom_resolve_ext(
        &self,
        p_begin_custom_resolve_info: Option<&BeginCustomResolveInfoEXT>,
    ) {
        unsafe {
            raw::cmd_begin_custom_resolve_ext(
                self,
                p_begin_custom_resolve_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_fragment_density_map_offset",
        feature = "ext_maintenance10"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2KHR.html>"]
    #[doc(alias = "vkCmdEndRendering2KHR")]
    #[inline]
    pub fn end_rendering2_khr(&self, p_rendering_end_info: Option<&RenderingEndInfoKHR>) {
        unsafe {
            raw::cmd_end_rendering2_khr(
                self,
                p_rendering_end_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(any(
        feature = "ext_fragment_density_map_offset",
        feature = "ext_maintenance10"
    ))]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2EXT.html>"]
    #[doc(alias = "vkCmdEndRendering2EXT")]
    #[inline]
    pub fn end_rendering2_ext(&self, p_rendering_end_info: Option<&RenderingEndInfoKHR>) {
        unsafe {
            raw::cmd_end_rendering2_ext(
                self,
                p_rendering_end_info,
                self.disp.get_command_dispatcher(),
            )
        }
    }
    #[cfg(feature = "ext_compute_occupancy_priority")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetComputeOccupancyPriorityNV.html>"]
    #[doc(alias = "vkCmdSetComputeOccupancyPriorityNV")]
    #[inline]
    pub fn set_compute_occupancy_priority_nv(
        &self,
        p_parameters: &ComputeOccupancyPriorityParametersNV,
    ) {
        unsafe {
            raw::cmd_set_compute_occupancy_priority_nv(
                self,
                p_parameters,
                self.disp.get_command_dispatcher(),
            )
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkEvent.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferView.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModule.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCache.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipeline.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayout.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSampler.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPool.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSet.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayout.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebuffer.html>"]
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
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPass.html>"]
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
#[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplate.html>"]
#[doc(alias = "VkDescriptorUpdateTemplate")]
pub struct DescriptorUpdateTemplate {
    inner: <raw::DescriptorUpdateTemplate as Handle>::InnerType,
}
#[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
unsafe impl Alias<raw::DescriptorUpdateTemplate> for DescriptorUpdateTemplate {}
#[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
impl Deref for DescriptorUpdateTemplate {
    type Target = raw::DescriptorUpdateTemplate;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
impl DescriptorUpdateTemplate {
    pub fn from_inner(handle: raw::DescriptorUpdateTemplate) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateKHR.html>"]
#[doc(alias = "VkDescriptorUpdateTemplateKHR")]
pub type DescriptorUpdateTemplateKHR = raw::DescriptorUpdateTemplate;
#[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversion.html>"]
#[doc(alias = "VkSamplerYcbcrConversion")]
pub struct SamplerYcbcrConversion {
    inner: <raw::SamplerYcbcrConversion as Handle>::InnerType,
}
#[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
unsafe impl Alias<raw::SamplerYcbcrConversion> for SamplerYcbcrConversion {}
#[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
impl Deref for SamplerYcbcrConversion {
    type Target = raw::SamplerYcbcrConversion;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
impl SamplerYcbcrConversion {
    pub fn from_inner(handle: raw::SamplerYcbcrConversion) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversionKHR.html>"]
#[doc(alias = "VkSamplerYcbcrConversionKHR")]
pub type SamplerYcbcrConversionKHR = raw::SamplerYcbcrConversion;
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlot.html>"]
#[doc(alias = "VkPrivateDataSlot")]
pub struct PrivateDataSlot {
    inner: <raw::PrivateDataSlot as Handle>::InnerType,
}
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
unsafe impl Alias<raw::PrivateDataSlot> for PrivateDataSlot {}
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
impl Deref for PrivateDataSlot {
    type Target = raw::PrivateDataSlot;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
impl PrivateDataSlot {
    pub fn from_inner(handle: raw::PrivateDataSlot) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotEXT.html>"]
#[doc(alias = "VkPrivateDataSlotEXT")]
pub type PrivateDataSlotEXT = raw::PrivateDataSlot;
#[cfg(feature = "ext_surface")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceKHR.html>"]
#[doc(alias = "VkSurfaceKHR")]
pub struct SurfaceKHR {
    inner: <raw::SurfaceKHR as Handle>::InnerType,
}
#[cfg(feature = "ext_surface")]
unsafe impl Alias<raw::SurfaceKHR> for SurfaceKHR {}
#[cfg(feature = "ext_surface")]
impl Deref for SurfaceKHR {
    type Target = raw::SurfaceKHR;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_surface")]
impl SurfaceKHR {
    pub fn from_inner(handle: raw::SurfaceKHR) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_swapchain")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainKHR.html>"]
#[doc(alias = "VkSwapchainKHR")]
pub struct SwapchainKHR {
    inner: <raw::SwapchainKHR as Handle>::InnerType,
}
#[cfg(feature = "ext_swapchain")]
unsafe impl Alias<raw::SwapchainKHR> for SwapchainKHR {}
#[cfg(feature = "ext_swapchain")]
impl Deref for SwapchainKHR {
    type Target = raw::SwapchainKHR;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_swapchain")]
impl SwapchainKHR {
    pub fn from_inner(handle: raw::SwapchainKHR) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_display")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayKHR.html>"]
#[doc(alias = "VkDisplayKHR")]
pub struct DisplayKHR {
    inner: <raw::DisplayKHR as Handle>::InnerType,
}
#[cfg(feature = "ext_display")]
unsafe impl Alias<raw::DisplayKHR> for DisplayKHR {}
#[cfg(feature = "ext_display")]
impl Deref for DisplayKHR {
    type Target = raw::DisplayKHR;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_display")]
impl DisplayKHR {
    pub fn from_inner(handle: raw::DisplayKHR) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_display")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeKHR.html>"]
#[doc(alias = "VkDisplayModeKHR")]
pub struct DisplayModeKHR {
    inner: <raw::DisplayModeKHR as Handle>::InnerType,
}
#[cfg(feature = "ext_display")]
unsafe impl Alias<raw::DisplayModeKHR> for DisplayModeKHR {}
#[cfg(feature = "ext_display")]
impl Deref for DisplayModeKHR {
    type Target = raw::DisplayModeKHR;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_display")]
impl DisplayModeKHR {
    pub fn from_inner(handle: raw::DisplayModeKHR) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_debug_report")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportCallbackEXT.html>"]
#[doc(alias = "VkDebugReportCallbackEXT")]
pub struct DebugReportCallbackEXT {
    inner: <raw::DebugReportCallbackEXT as Handle>::InnerType,
}
#[cfg(feature = "ext_debug_report")]
unsafe impl Alias<raw::DebugReportCallbackEXT> for DebugReportCallbackEXT {}
#[cfg(feature = "ext_debug_report")]
impl Deref for DebugReportCallbackEXT {
    type Target = raw::DebugReportCallbackEXT;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_debug_report")]
impl DebugReportCallbackEXT {
    pub fn from_inner(handle: raw::DebugReportCallbackEXT) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_binary_import")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCuModuleNVX.html>"]
#[doc(alias = "VkCuModuleNVX")]
pub struct CuModuleNVX {
    inner: <raw::CuModuleNVX as Handle>::InnerType,
}
#[cfg(feature = "ext_binary_import")]
unsafe impl Alias<raw::CuModuleNVX> for CuModuleNVX {}
#[cfg(feature = "ext_binary_import")]
impl Deref for CuModuleNVX {
    type Target = raw::CuModuleNVX;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_binary_import")]
impl CuModuleNVX {
    pub fn from_inner(handle: raw::CuModuleNVX) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_binary_import")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCuFunctionNVX.html>"]
#[doc(alias = "VkCuFunctionNVX")]
pub struct CuFunctionNVX {
    inner: <raw::CuFunctionNVX as Handle>::InnerType,
}
#[cfg(feature = "ext_binary_import")]
unsafe impl Alias<raw::CuFunctionNVX> for CuFunctionNVX {}
#[cfg(feature = "ext_binary_import")]
impl Deref for CuFunctionNVX {
    type Target = raw::CuFunctionNVX;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_binary_import")]
impl CuFunctionNVX {
    pub fn from_inner(handle: raw::CuFunctionNVX) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_debug_utils")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerEXT.html>"]
#[doc(alias = "VkDebugUtilsMessengerEXT")]
pub struct DebugUtilsMessengerEXT {
    inner: <raw::DebugUtilsMessengerEXT as Handle>::InnerType,
}
#[cfg(feature = "ext_debug_utils")]
unsafe impl Alias<raw::DebugUtilsMessengerEXT> for DebugUtilsMessengerEXT {}
#[cfg(feature = "ext_debug_utils")]
impl Deref for DebugUtilsMessengerEXT {
    type Target = raw::DebugUtilsMessengerEXT;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_debug_utils")]
impl DebugUtilsMessengerEXT {
    pub fn from_inner(handle: raw::DebugUtilsMessengerEXT) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(any(feature = "ext_descriptor_heap", feature = "ext_tensors"))]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorARM.html>"]
#[doc(alias = "VkTensorARM")]
pub struct TensorARM {
    inner: <raw::TensorARM as Handle>::InnerType,
}
#[cfg(any(feature = "ext_descriptor_heap", feature = "ext_tensors"))]
unsafe impl Alias<raw::TensorARM> for TensorARM {}
#[cfg(any(feature = "ext_descriptor_heap", feature = "ext_tensors"))]
impl Deref for TensorARM {
    type Target = raw::TensorARM;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(any(feature = "ext_descriptor_heap", feature = "ext_tensors"))]
impl TensorARM {
    pub fn from_inner(handle: raw::TensorARM) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_acceleration_structure")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureKHR.html>"]
#[doc(alias = "VkAccelerationStructureKHR")]
pub struct AccelerationStructureKHR {
    inner: <raw::AccelerationStructureKHR as Handle>::InnerType,
}
#[cfg(feature = "ext_acceleration_structure")]
unsafe impl Alias<raw::AccelerationStructureKHR> for AccelerationStructureKHR {}
#[cfg(feature = "ext_acceleration_structure")]
impl Deref for AccelerationStructureKHR {
    type Target = raw::AccelerationStructureKHR;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_acceleration_structure")]
impl AccelerationStructureKHR {
    pub fn from_inner(handle: raw::AccelerationStructureKHR) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_validation_cache")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCacheEXT.html>"]
#[doc(alias = "VkValidationCacheEXT")]
pub struct ValidationCacheEXT {
    inner: <raw::ValidationCacheEXT as Handle>::InnerType,
}
#[cfg(feature = "ext_validation_cache")]
unsafe impl Alias<raw::ValidationCacheEXT> for ValidationCacheEXT {}
#[cfg(feature = "ext_validation_cache")]
impl Deref for ValidationCacheEXT {
    type Target = raw::ValidationCacheEXT;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_validation_cache")]
impl ValidationCacheEXT {
    pub fn from_inner(handle: raw::ValidationCacheEXT) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_ray_tracing")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureNV.html>"]
#[doc(alias = "VkAccelerationStructureNV")]
pub struct AccelerationStructureNV {
    inner: <raw::AccelerationStructureNV as Handle>::InnerType,
}
#[cfg(feature = "ext_ray_tracing")]
unsafe impl Alias<raw::AccelerationStructureNV> for AccelerationStructureNV {}
#[cfg(feature = "ext_ray_tracing")]
impl Deref for AccelerationStructureNV {
    type Target = raw::AccelerationStructureNV;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_ray_tracing")]
impl AccelerationStructureNV {
    pub fn from_inner(handle: raw::AccelerationStructureNV) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_performance_query")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceConfigurationINTEL.html>"]
#[doc(alias = "VkPerformanceConfigurationINTEL")]
pub struct PerformanceConfigurationINTEL {
    inner: <raw::PerformanceConfigurationINTEL as Handle>::InnerType,
}
#[cfg(feature = "ext_performance_query")]
unsafe impl Alias<raw::PerformanceConfigurationINTEL> for PerformanceConfigurationINTEL {}
#[cfg(feature = "ext_performance_query")]
impl Deref for PerformanceConfigurationINTEL {
    type Target = raw::PerformanceConfigurationINTEL;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_performance_query")]
impl PerformanceConfigurationINTEL {
    pub fn from_inner(handle: raw::PerformanceConfigurationINTEL) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_deferred_host_operations")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeferredOperationKHR.html>"]
#[doc(alias = "VkDeferredOperationKHR")]
pub struct DeferredOperationKHR {
    inner: <raw::DeferredOperationKHR as Handle>::InnerType,
}
#[cfg(feature = "ext_deferred_host_operations")]
unsafe impl Alias<raw::DeferredOperationKHR> for DeferredOperationKHR {}
#[cfg(feature = "ext_deferred_host_operations")]
impl Deref for DeferredOperationKHR {
    type Target = raw::DeferredOperationKHR;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_deferred_host_operations")]
impl DeferredOperationKHR {
    pub fn from_inner(handle: raw::DeferredOperationKHR) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_device_generated_commands")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutNV.html>"]
#[doc(alias = "VkIndirectCommandsLayoutNV")]
pub struct IndirectCommandsLayoutNV {
    inner: <raw::IndirectCommandsLayoutNV as Handle>::InnerType,
}
#[cfg(feature = "ext_device_generated_commands")]
unsafe impl Alias<raw::IndirectCommandsLayoutNV> for IndirectCommandsLayoutNV {}
#[cfg(feature = "ext_device_generated_commands")]
impl Deref for IndirectCommandsLayoutNV {
    type Target = raw::IndirectCommandsLayoutNV;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_device_generated_commands")]
impl IndirectCommandsLayoutNV {
    pub fn from_inner(handle: raw::IndirectCommandsLayoutNV) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_cuda_kernel_launch")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaModuleNV.html>"]
#[doc(alias = "VkCudaModuleNV")]
pub struct CudaModuleNV {
    inner: <raw::CudaModuleNV as Handle>::InnerType,
}
#[cfg(feature = "ext_cuda_kernel_launch")]
unsafe impl Alias<raw::CudaModuleNV> for CudaModuleNV {}
#[cfg(feature = "ext_cuda_kernel_launch")]
impl Deref for CudaModuleNV {
    type Target = raw::CudaModuleNV;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_cuda_kernel_launch")]
impl CudaModuleNV {
    pub fn from_inner(handle: raw::CudaModuleNV) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_cuda_kernel_launch")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaFunctionNV.html>"]
#[doc(alias = "VkCudaFunctionNV")]
pub struct CudaFunctionNV {
    inner: <raw::CudaFunctionNV as Handle>::InnerType,
}
#[cfg(feature = "ext_cuda_kernel_launch")]
unsafe impl Alias<raw::CudaFunctionNV> for CudaFunctionNV {}
#[cfg(feature = "ext_cuda_kernel_launch")]
impl Deref for CudaFunctionNV {
    type Target = raw::CudaFunctionNV;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_cuda_kernel_launch")]
impl CudaFunctionNV {
    pub fn from_inner(handle: raw::CudaFunctionNV) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_buffer_collection")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionFUCHSIA.html>"]
#[doc(alias = "VkBufferCollectionFUCHSIA")]
pub struct BufferCollectionFUCHSIA {
    inner: <raw::BufferCollectionFUCHSIA as Handle>::InnerType,
}
#[cfg(feature = "ext_buffer_collection")]
unsafe impl Alias<raw::BufferCollectionFUCHSIA> for BufferCollectionFUCHSIA {}
#[cfg(feature = "ext_buffer_collection")]
impl Deref for BufferCollectionFUCHSIA {
    type Target = raw::BufferCollectionFUCHSIA;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_buffer_collection")]
impl BufferCollectionFUCHSIA {
    pub fn from_inner(handle: raw::BufferCollectionFUCHSIA) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_opacity_micromap")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapEXT.html>"]
#[doc(alias = "VkMicromapEXT")]
pub struct MicromapEXT {
    inner: <raw::MicromapEXT as Handle>::InnerType,
}
#[cfg(feature = "ext_opacity_micromap")]
unsafe impl Alias<raw::MicromapEXT> for MicromapEXT {}
#[cfg(feature = "ext_opacity_micromap")]
impl Deref for MicromapEXT {
    type Target = raw::MicromapEXT;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_opacity_micromap")]
impl MicromapEXT {
    pub fn from_inner(handle: raw::MicromapEXT) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_tensors")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewARM.html>"]
#[doc(alias = "VkTensorViewARM")]
pub struct TensorViewARM {
    inner: <raw::TensorViewARM as Handle>::InnerType,
}
#[cfg(feature = "ext_tensors")]
unsafe impl Alias<raw::TensorViewARM> for TensorViewARM {}
#[cfg(feature = "ext_tensors")]
impl Deref for TensorViewARM {
    type Target = raw::TensorViewARM;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_tensors")]
impl TensorViewARM {
    pub fn from_inner(handle: raw::TensorViewARM) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_optical_flow")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionNV.html>"]
#[doc(alias = "VkOpticalFlowSessionNV")]
pub struct OpticalFlowSessionNV {
    inner: <raw::OpticalFlowSessionNV as Handle>::InnerType,
}
#[cfg(feature = "ext_optical_flow")]
unsafe impl Alias<raw::OpticalFlowSessionNV> for OpticalFlowSessionNV {}
#[cfg(feature = "ext_optical_flow")]
impl Deref for OpticalFlowSessionNV {
    type Target = raw::OpticalFlowSessionNV;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_optical_flow")]
impl OpticalFlowSessionNV {
    pub fn from_inner(handle: raw::OpticalFlowSessionNV) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_shader_object")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderEXT.html>"]
#[doc(alias = "VkShaderEXT")]
pub struct ShaderEXT {
    inner: <raw::ShaderEXT as Handle>::InnerType,
}
#[cfg(feature = "ext_shader_object")]
unsafe impl Alias<raw::ShaderEXT> for ShaderEXT {}
#[cfg(feature = "ext_shader_object")]
impl Deref for ShaderEXT {
    type Target = raw::ShaderEXT;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_shader_object")]
impl ShaderEXT {
    pub fn from_inner(handle: raw::ShaderEXT) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_pipeline_binary")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryKHR.html>"]
#[doc(alias = "VkPipelineBinaryKHR")]
pub struct PipelineBinaryKHR {
    inner: <raw::PipelineBinaryKHR as Handle>::InnerType,
}
#[cfg(feature = "ext_pipeline_binary")]
unsafe impl Alias<raw::PipelineBinaryKHR> for PipelineBinaryKHR {}
#[cfg(feature = "ext_pipeline_binary")]
impl Deref for PipelineBinaryKHR {
    type Target = raw::PipelineBinaryKHR;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_pipeline_binary")]
impl PipelineBinaryKHR {
    pub fn from_inner(handle: raw::PipelineBinaryKHR) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_data_graph")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionARM.html>"]
#[doc(alias = "VkDataGraphPipelineSessionARM")]
pub struct DataGraphPipelineSessionARM {
    inner: <raw::DataGraphPipelineSessionARM as Handle>::InnerType,
}
#[cfg(feature = "ext_data_graph")]
unsafe impl Alias<raw::DataGraphPipelineSessionARM> for DataGraphPipelineSessionARM {}
#[cfg(feature = "ext_data_graph")]
impl Deref for DataGraphPipelineSessionARM {
    type Target = raw::DataGraphPipelineSessionARM;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_data_graph")]
impl DataGraphPipelineSessionARM {
    pub fn from_inner(handle: raw::DataGraphPipelineSessionARM) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_external_compute_queue")]
#[repr(C)]
#[derive(Clone)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalComputeQueueNV.html>"]
#[doc(alias = "VkExternalComputeQueueNV")]
pub struct ExternalComputeQueueNV<
    D: Dispatcher = DynamicDispatcher,
    A: BaseAllocator = DefaultAllocator,
> {
    inner: <raw::ExternalComputeQueueNV as Handle>::InnerType,
    disp: D,
    alloc: A,
}
#[cfg(feature = "ext_external_compute_queue")]
unsafe impl Alias<raw::ExternalComputeQueueNV> for ExternalComputeQueueNV {}
#[cfg(feature = "ext_external_compute_queue")]
impl<D: Dispatcher, A: BaseAllocator> Copy for ExternalComputeQueueNV<D, A>
where
    D: Copy,
    A: Copy,
{
}
#[cfg(feature = "ext_external_compute_queue")]
impl<D: Dispatcher, A: BaseAllocator> Deref for ExternalComputeQueueNV<D, A> {
    type Target = raw::ExternalComputeQueueNV;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_external_compute_queue")]
impl<D: Dispatcher, A: BaseAllocator> ExternalComputeQueueNV<D, A> {
    pub unsafe fn from_inner(handle: raw::ExternalComputeQueueNV, disp: D, alloc: A) -> Self {
        Self {
            inner: handle.as_raw(),
            disp,
            alloc,
        }
    }
    #[inline(always)]
    pub fn get_dispatcher(&self) -> &D {
        &self.disp
    }
    #[inline(always)]
    pub fn get_allocator(&self) -> &A {
        &self.alloc
    }
    #[cfg(feature = "ext_external_compute_queue")]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExternalComputeQueueDataNV.html>"]
    #[doc(alias = "vkGetExternalComputeQueueDataNV")]
    pub fn get_external_compute_queue_data_nv<
        S: StructureChainOut<ExternalComputeQueueDataParamsNV<'static>>,
    >(
        &self,
        p_data: VoidPtr,
    ) -> S {
        unsafe {
            raw::get_external_compute_queue_data_nv(
                self,
                p_data,
                self.disp.get_command_dispatcher(),
            )
        }
    }
}
#[cfg(feature = "ext_device_generated_commands")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutEXT.html>"]
#[doc(alias = "VkIndirectCommandsLayoutEXT")]
pub struct IndirectCommandsLayoutEXT {
    inner: <raw::IndirectCommandsLayoutEXT as Handle>::InnerType,
}
#[cfg(feature = "ext_device_generated_commands")]
unsafe impl Alias<raw::IndirectCommandsLayoutEXT> for IndirectCommandsLayoutEXT {}
#[cfg(feature = "ext_device_generated_commands")]
impl Deref for IndirectCommandsLayoutEXT {
    type Target = raw::IndirectCommandsLayoutEXT;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_device_generated_commands")]
impl IndirectCommandsLayoutEXT {
    pub fn from_inner(handle: raw::IndirectCommandsLayoutEXT) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_device_generated_commands")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectExecutionSetEXT.html>"]
#[doc(alias = "VkIndirectExecutionSetEXT")]
pub struct IndirectExecutionSetEXT {
    inner: <raw::IndirectExecutionSetEXT as Handle>::InnerType,
}
#[cfg(feature = "ext_device_generated_commands")]
unsafe impl Alias<raw::IndirectExecutionSetEXT> for IndirectExecutionSetEXT {}
#[cfg(feature = "ext_device_generated_commands")]
impl Deref for IndirectExecutionSetEXT {
    type Target = raw::IndirectExecutionSetEXT;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_device_generated_commands")]
impl IndirectExecutionSetEXT {
    pub fn from_inner(handle: raw::IndirectExecutionSetEXT) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
#[cfg(feature = "ext_shader_instrumentation")]
#[repr(C)]
#[derive(Clone, Copy)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationARM.html>"]
#[doc(alias = "VkShaderInstrumentationARM")]
pub struct ShaderInstrumentationARM {
    inner: <raw::ShaderInstrumentationARM as Handle>::InnerType,
}
#[cfg(feature = "ext_shader_instrumentation")]
unsafe impl Alias<raw::ShaderInstrumentationARM> for ShaderInstrumentationARM {}
#[cfg(feature = "ext_shader_instrumentation")]
impl Deref for ShaderInstrumentationARM {
    type Target = raw::ShaderInstrumentationARM;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.inner) }
    }
}
#[cfg(feature = "ext_shader_instrumentation")]
impl ShaderInstrumentationARM {
    pub fn from_inner(handle: raw::ShaderInstrumentationARM) -> Self {
        Self {
            inner: handle.as_raw(),
        }
    }
}
