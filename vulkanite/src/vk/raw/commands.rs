#![allow(unused_unsafe)]
#![allow(unused_mut)]
use crate::vk::raw::{self, *};
use crate::vk::*;
use crate::*;
#[allow(unused_imports)]
use std::ffi::{c_int, CStr};
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateInstance.html>"]
#[doc(alias = "vkCreateInstance")]
pub unsafe fn create_instance(
    p_create_info: &InstanceCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Instance> {
    let vulkan_command = dispatcher.create_instance.get();
    let mut p_instance = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_instance.as_mut_ptr(),
    );
    vk_status.map_success(|| p_instance.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyInstance.html>"]
#[doc(alias = "vkDestroyInstance")]
#[inline]
pub unsafe fn destroy_instance(
    instance: Option<&raw::Instance>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_instance.get();
    vulkan_command(
        instance.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDevices.html>"]
#[doc(alias = "vkEnumeratePhysicalDevices")]
pub unsafe fn enumerate_physical_devices<R: DynamicArray<PhysicalDevice>>(
    instance: &raw::Instance,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.enumerate_physical_devices.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_physical_device_count = vk_len.as_mut_ptr();
    let p_physical_devices = ptr::null_mut();
    vulkan_command(
        Some(instance.borrow()),
        p_physical_device_count,
        p_physical_devices,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_physical_device_count = ptr::from_mut(&mut vk_len);
    let mut p_physical_devices = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(instance.borrow()),
            p_physical_device_count,
            p_physical_devices,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_physical_device_count = ptr::from_mut(&mut vk_len);
        p_physical_devices = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures.html>"]
#[doc(alias = "vkGetPhysicalDeviceFeatures")]
pub unsafe fn get_physical_device_features(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> PhysicalDeviceFeatures {
    let vulkan_command = dispatcher.get_physical_device_features.get();
    let mut p_features = MaybeUninit::uninit();
    vulkan_command(Some(physical_device.borrow()), p_features.as_mut_ptr());
    p_features.assume_init()
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties.html>"]
#[doc(alias = "vkGetPhysicalDeviceFormatProperties")]
pub unsafe fn get_physical_device_format_properties(
    physical_device: &raw::PhysicalDevice,
    format: Format,
    dispatcher: &CommandsDispatcher,
) -> FormatProperties {
    let vulkan_command = dispatcher.get_physical_device_format_properties.get();
    let mut p_format_properties = MaybeUninit::uninit();
    vulkan_command(
        Some(physical_device.borrow()),
        format,
        p_format_properties.as_mut_ptr(),
    );
    p_format_properties.assume_init()
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties.html>"]
#[doc(alias = "vkGetPhysicalDeviceImageFormatProperties")]
pub unsafe fn get_physical_device_image_format_properties(
    physical_device: &raw::PhysicalDevice,
    format: Format,
    ty: ImageType,
    tiling: ImageTiling,
    usage: ImageUsageFlags,
    flags: ImageCreateFlags,
    dispatcher: &CommandsDispatcher,
) -> Result<ImageFormatProperties> {
    let vulkan_command = dispatcher.get_physical_device_image_format_properties.get();
    let mut p_image_format_properties = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(physical_device.borrow()),
        format,
        ty,
        tiling,
        usage,
        flags,
        p_image_format_properties.as_mut_ptr(),
    );
    vk_status.map_success(|| p_image_format_properties.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties.html>"]
#[doc(alias = "vkGetPhysicalDeviceProperties")]
pub unsafe fn get_physical_device_properties(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> PhysicalDeviceProperties {
    let vulkan_command = dispatcher.get_physical_device_properties.get();
    let mut p_properties = MaybeUninit::uninit();
    vulkan_command(Some(physical_device.borrow()), p_properties.as_mut_ptr());
    p_properties.assume_init()
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties.html>"]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties")]
pub unsafe fn get_physical_device_queue_family_properties<
    R: DynamicArray<QueueFamilyProperties>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher.get_physical_device_queue_family_properties.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_queue_family_property_count = vk_len.as_mut_ptr();
    let p_queue_family_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_queue_family_property_count,
        p_queue_family_properties,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_queue_family_property_count = ptr::from_mut(&mut vk_len);
    let mut p_queue_family_properties = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(physical_device.borrow()),
        p_queue_family_property_count,
        p_queue_family_properties,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties.html>"]
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties")]
pub unsafe fn get_physical_device_memory_properties(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> PhysicalDeviceMemoryProperties {
    let vulkan_command = dispatcher.get_physical_device_memory_properties.get();
    let mut p_memory_properties = MaybeUninit::uninit();
    vulkan_command(
        Some(physical_device.borrow()),
        p_memory_properties.as_mut_ptr(),
    );
    p_memory_properties.assume_init()
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html>"]
#[doc(alias = "vkGetInstanceProcAddr")]
#[inline]
pub unsafe fn get_instance_proc_addr(
    instance: Option<&raw::Instance>,
    p_name: &CStr,
    dispatcher: &CommandsDispatcher,
) -> FuncPtr {
    let vulkan_command = dispatcher.get_instance_proc_addr.get();
    vulkan_command(instance.map(|v| v.borrow()), p_name.as_ptr())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceProcAddr.html>"]
#[doc(alias = "vkGetDeviceProcAddr")]
#[inline]
pub unsafe fn get_device_proc_addr(
    device: &raw::Device,
    p_name: &CStr,
    dispatcher: &CommandsDispatcher,
) -> FuncPtr {
    let vulkan_command = dispatcher.get_device_proc_addr.get();
    vulkan_command(Some(device.borrow()), p_name.as_ptr())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDevice.html>"]
#[doc(alias = "vkCreateDevice")]
pub unsafe fn create_device(
    physical_device: &raw::PhysicalDevice,
    p_create_info: &DeviceCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Device> {
    let vulkan_command = dispatcher.create_device.get();
    let mut p_device = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_device.as_mut_ptr(),
    );
    vk_status.map_success(|| p_device.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDevice.html>"]
#[doc(alias = "vkDestroyDevice")]
#[inline]
pub unsafe fn destroy_device(
    device: Option<&raw::Device>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_device.get();
    vulkan_command(
        device.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html>"]
#[doc(alias = "vkEnumerateInstanceExtensionProperties")]
pub unsafe fn enumerate_instance_extension_properties<R: DynamicArray<ExtensionProperties>>(
    p_layer_name: Option<&CStr>,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.enumerate_instance_extension_properties.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        p_layer_name.map(|v| v.as_ptr()).unwrap_or(ptr::null()),
        p_property_count,
        p_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            p_layer_name.map(|v| v.as_ptr()).unwrap_or(ptr::null()),
            p_property_count,
            p_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_property_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceExtensionProperties.html>"]
#[doc(alias = "vkEnumerateDeviceExtensionProperties")]
pub unsafe fn enumerate_device_extension_properties<R: DynamicArray<ExtensionProperties>>(
    physical_device: &raw::PhysicalDevice,
    p_layer_name: Option<&CStr>,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.enumerate_device_extension_properties.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_layer_name.map(|v| v.as_ptr()).unwrap_or(ptr::null()),
        p_property_count,
        p_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_layer_name.map(|v| v.as_ptr()).unwrap_or(ptr::null()),
            p_property_count,
            p_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_property_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceLayerProperties.html>"]
#[doc(alias = "vkEnumerateInstanceLayerProperties")]
pub unsafe fn enumerate_instance_layer_properties<R: DynamicArray<LayerProperties>>(
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.enumerate_instance_layer_properties.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(p_property_count, p_properties).map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(p_property_count, p_properties);
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_property_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceLayerProperties.html>"]
#[doc(alias = "vkEnumerateDeviceLayerProperties")]
pub unsafe fn enumerate_device_layer_properties<R: DynamicArray<LayerProperties>>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.enumerate_device_layer_properties.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_property_count,
        p_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_property_count,
            p_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_property_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue.html>"]
#[doc(alias = "vkGetDeviceQueue")]
pub unsafe fn get_device_queue(
    device: &raw::Device,
    queue_family_index: u32,
    queue_index: u32,
    dispatcher: &CommandsDispatcher,
) -> Queue {
    let vulkan_command = dispatcher.get_device_queue.get();
    let mut p_queue = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        queue_family_index,
        queue_index,
        p_queue.as_mut_ptr(),
    );
    p_queue.assume_init()
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit.html>"]
#[doc(alias = "vkQueueSubmit")]
#[inline]
pub unsafe fn queue_submit<'a>(
    queue: &raw::Queue,
    p_submits: impl AsSlice<'a, SubmitInfo<'a>>,
    fence: Option<&raw::Fence>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.queue_submit.get();
    vulkan_command(
        Some(queue.borrow()),
        p_submits.as_slice().len() as _,
        p_submits.as_slice().as_ptr().cast(),
        fence.map(|v| v.borrow()),
    )
    .map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueWaitIdle.html>"]
#[doc(alias = "vkQueueWaitIdle")]
#[inline]
pub unsafe fn queue_wait_idle(queue: &raw::Queue, dispatcher: &CommandsDispatcher) -> Result<()> {
    let vulkan_command = dispatcher.queue_wait_idle.get();
    vulkan_command(Some(queue.borrow())).map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDeviceWaitIdle.html>"]
#[doc(alias = "vkDeviceWaitIdle")]
#[inline]
pub unsafe fn device_wait_idle(
    device: &raw::Device,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.device_wait_idle.get();
    vulkan_command(Some(device.borrow())).map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateMemory.html>"]
#[doc(alias = "vkAllocateMemory")]
pub unsafe fn allocate_memory(
    device: &raw::Device,
    p_allocate_info: &MemoryAllocateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DeviceMemory> {
    let vulkan_command = dispatcher.allocate_memory.get();
    let mut p_memory = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_allocate_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_memory.as_mut_ptr(),
    );
    vk_status.map_success(|| p_memory.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeMemory.html>"]
#[doc(alias = "vkFreeMemory")]
#[inline]
pub unsafe fn free_memory(
    device: &raw::Device,
    memory: Option<&raw::DeviceMemory>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.free_memory.get();
    vulkan_command(
        Some(device.borrow()),
        memory.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory.html>"]
#[doc(alias = "vkMapMemory")]
pub unsafe fn map_memory(
    device: &raw::Device,
    memory: &raw::DeviceMemory,
    offset: DeviceSize,
    size: DeviceSize,
    flags: MemoryMapFlags,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher.map_memory.get();
    let mut pp_data = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(memory.borrow()),
        offset,
        size,
        flags,
        pp_data.as_mut_ptr(),
    );
    vk_status.map_success(|| pp_data.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory.html>"]
#[doc(alias = "vkUnmapMemory")]
#[inline]
pub unsafe fn unmap_memory(
    device: &raw::Device,
    memory: &raw::DeviceMemory,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.unmap_memory.get();
    vulkan_command(Some(device.borrow()), Some(memory.borrow()))
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html>"]
#[doc(alias = "vkFlushMappedMemoryRanges")]
#[inline]
pub unsafe fn flush_mapped_memory_ranges<'a>(
    device: &raw::Device,
    p_memory_ranges: impl AsSlice<'a, MappedMemoryRange<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.flush_mapped_memory_ranges.get();
    vulkan_command(
        Some(device.borrow()),
        p_memory_ranges.as_slice().len() as _,
        p_memory_ranges.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkInvalidateMappedMemoryRanges.html>"]
#[doc(alias = "vkInvalidateMappedMemoryRanges")]
#[inline]
pub unsafe fn invalidate_mapped_memory_ranges<'a>(
    device: &raw::Device,
    p_memory_ranges: impl AsSlice<'a, MappedMemoryRange<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.invalidate_mapped_memory_ranges.get();
    vulkan_command(
        Some(device.borrow()),
        p_memory_ranges.as_slice().len() as _,
        p_memory_ranges.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryCommitment.html>"]
#[doc(alias = "vkGetDeviceMemoryCommitment")]
pub unsafe fn get_device_memory_commitment(
    device: &raw::Device,
    memory: &raw::DeviceMemory,
    dispatcher: &CommandsDispatcher,
) -> DeviceSize {
    let vulkan_command = dispatcher.get_device_memory_commitment.get();
    let mut p_committed_memory_in_bytes = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        Some(memory.borrow()),
        p_committed_memory_in_bytes.as_mut_ptr(),
    );
    p_committed_memory_in_bytes.assume_init()
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory.html>"]
#[doc(alias = "vkBindBufferMemory")]
#[inline]
pub unsafe fn bind_buffer_memory(
    device: &raw::Device,
    buffer: &raw::Buffer,
    memory: &raw::DeviceMemory,
    memory_offset: DeviceSize,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.bind_buffer_memory.get();
    vulkan_command(
        Some(device.borrow()),
        Some(buffer.borrow()),
        Some(memory.borrow()),
        memory_offset,
    )
    .map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory.html>"]
#[doc(alias = "vkBindImageMemory")]
#[inline]
pub unsafe fn bind_image_memory(
    device: &raw::Device,
    image: &raw::Image,
    memory: &raw::DeviceMemory,
    memory_offset: DeviceSize,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.bind_image_memory.get();
    vulkan_command(
        Some(device.borrow()),
        Some(image.borrow()),
        Some(memory.borrow()),
        memory_offset,
    )
    .map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements.html>"]
#[doc(alias = "vkGetBufferMemoryRequirements")]
pub unsafe fn get_buffer_memory_requirements(
    device: &raw::Device,
    buffer: &raw::Buffer,
    dispatcher: &CommandsDispatcher,
) -> MemoryRequirements {
    let vulkan_command = dispatcher.get_buffer_memory_requirements.get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        Some(buffer.borrow()),
        p_memory_requirements.as_mut_ptr(),
    );
    p_memory_requirements.assume_init()
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements.html>"]
#[doc(alias = "vkGetImageMemoryRequirements")]
pub unsafe fn get_image_memory_requirements(
    device: &raw::Device,
    image: &raw::Image,
    dispatcher: &CommandsDispatcher,
) -> MemoryRequirements {
    let vulkan_command = dispatcher.get_image_memory_requirements.get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        Some(image.borrow()),
        p_memory_requirements.as_mut_ptr(),
    );
    p_memory_requirements.assume_init()
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements.html>"]
#[doc(alias = "vkGetImageSparseMemoryRequirements")]
pub unsafe fn get_image_sparse_memory_requirements<
    R: DynamicArray<SparseImageMemoryRequirements>,
>(
    device: &raw::Device,
    image: &raw::Image,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher.get_image_sparse_memory_requirements.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_sparse_memory_requirement_count = vk_len.as_mut_ptr();
    let p_sparse_memory_requirements = ptr::null_mut();
    vulkan_command(
        Some(device.borrow()),
        Some(image.borrow()),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_sparse_memory_requirement_count = ptr::from_mut(&mut vk_len);
    let mut p_sparse_memory_requirements = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(device.borrow()),
        Some(image.borrow()),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties.html>"]
#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties")]
pub unsafe fn get_physical_device_sparse_image_format_properties<
    R: DynamicArray<SparseImageFormatProperties>,
>(
    physical_device: &raw::PhysicalDevice,
    format: Format,
    ty: ImageType,
    samples: SampleCountFlags,
    usage: ImageUsageFlags,
    tiling: ImageTiling,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher
        .get_physical_device_sparse_image_format_properties
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        format,
        ty,
        samples,
        usage,
        tiling,
        p_property_count,
        p_properties,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(physical_device.borrow()),
        format,
        ty,
        samples,
        usage,
        tiling,
        p_property_count,
        p_properties,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBindSparse.html>"]
#[doc(alias = "vkQueueBindSparse")]
#[inline]
pub unsafe fn queue_bind_sparse<'a>(
    queue: &raw::Queue,
    p_bind_info: impl AsSlice<'a, BindSparseInfo<'a>>,
    fence: Option<&raw::Fence>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.queue_bind_sparse.get();
    vulkan_command(
        Some(queue.borrow()),
        p_bind_info.as_slice().len() as _,
        p_bind_info.as_slice().as_ptr().cast(),
        fence.map(|v| v.borrow()),
    )
    .map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFence.html>"]
#[doc(alias = "vkCreateFence")]
pub unsafe fn create_fence(
    device: &raw::Device,
    p_create_info: &FenceCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Fence> {
    let vulkan_command = dispatcher.create_fence.get();
    let mut p_fence = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_fence.as_mut_ptr(),
    );
    vk_status.map_success(|| p_fence.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFence.html>"]
#[doc(alias = "vkDestroyFence")]
#[inline]
pub unsafe fn destroy_fence(
    device: &raw::Device,
    fence: Option<&raw::Fence>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_fence.get();
    vulkan_command(
        Some(device.borrow()),
        fence.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetFences.html>"]
#[doc(alias = "vkResetFences")]
#[inline]
pub unsafe fn reset_fences<'a, V2: Alias<raw::Fence> + 'a>(
    device: &raw::Device,
    p_fences: impl AsSlice<'a, V2>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.reset_fences.get();
    vulkan_command(
        Some(device.borrow()),
        p_fences.as_slice().len() as _,
        p_fences.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceStatus.html>"]
#[doc(alias = "vkGetFenceStatus")]
#[inline]
pub unsafe fn get_fence_status(
    device: &raw::Device,
    fence: &raw::Fence,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.get_fence_status.get();
    vulkan_command(Some(device.borrow()), Some(fence.borrow())).into_result()
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForFences.html>"]
#[doc(alias = "vkWaitForFences")]
#[inline]
pub unsafe fn wait_for_fences<'a, V2: Alias<raw::Fence> + 'a>(
    device: &raw::Device,
    p_fences: impl AsSlice<'a, V2>,
    wait_all: impl Into<Bool32>,
    timeout: u64,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.wait_for_fences.get();
    vulkan_command(
        Some(device.borrow()),
        p_fences.as_slice().len() as _,
        p_fences.as_slice().as_ptr().cast(),
        wait_all.into(),
        timeout,
    )
    .into_result()
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSemaphore.html>"]
#[doc(alias = "vkCreateSemaphore")]
pub unsafe fn create_semaphore(
    device: &raw::Device,
    p_create_info: &SemaphoreCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Semaphore> {
    let vulkan_command = dispatcher.create_semaphore.get();
    let mut p_semaphore = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_semaphore.as_mut_ptr(),
    );
    vk_status.map_success(|| p_semaphore.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySemaphore.html>"]
#[doc(alias = "vkDestroySemaphore")]
#[inline]
pub unsafe fn destroy_semaphore(
    device: &raw::Device,
    semaphore: Option<&raw::Semaphore>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_semaphore.get();
    vulkan_command(
        Some(device.borrow()),
        semaphore.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateQueryPool.html>"]
#[doc(alias = "vkCreateQueryPool")]
pub unsafe fn create_query_pool(
    device: &raw::Device,
    p_create_info: &QueryPoolCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<QueryPool> {
    let vulkan_command = dispatcher.create_query_pool.get();
    let mut p_query_pool = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_query_pool.as_mut_ptr(),
    );
    vk_status.map_success(|| p_query_pool.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyQueryPool.html>"]
#[doc(alias = "vkDestroyQueryPool")]
#[inline]
pub unsafe fn destroy_query_pool(
    device: &raw::Device,
    query_pool: Option<&raw::QueryPool>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_query_pool.get();
    vulkan_command(
        Some(device.borrow()),
        query_pool.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueryPoolResults.html>"]
#[doc(alias = "vkGetQueryPoolResults")]
#[inline]
pub unsafe fn get_query_pool_results(
    device: &raw::Device,
    query_pool: &raw::QueryPool,
    first_query: u32,
    query_count: u32,
    data_size: usize,
    p_data: VoidPtr,
    stride: DeviceSize,
    flags: QueryResultFlags,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.get_query_pool_results.get();
    vulkan_command(
        Some(device.borrow()),
        Some(query_pool.borrow()),
        first_query,
        query_count,
        data_size,
        p_data,
        stride,
        flags,
    )
    .into_result()
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBuffer.html>"]
#[doc(alias = "vkCreateBuffer")]
pub unsafe fn create_buffer(
    device: &raw::Device,
    p_create_info: &BufferCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Buffer> {
    let vulkan_command = dispatcher.create_buffer.get();
    let mut p_buffer = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_buffer.as_mut_ptr(),
    );
    vk_status.map_success(|| p_buffer.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBuffer.html>"]
#[doc(alias = "vkDestroyBuffer")]
#[inline]
pub unsafe fn destroy_buffer(
    device: &raw::Device,
    buffer: Option<&raw::Buffer>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_buffer.get();
    vulkan_command(
        Some(device.borrow()),
        buffer.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImage.html>"]
#[doc(alias = "vkCreateImage")]
pub unsafe fn create_image(
    device: &raw::Device,
    p_create_info: &ImageCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Image> {
    let vulkan_command = dispatcher.create_image.get();
    let mut p_image = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_image.as_mut_ptr(),
    );
    vk_status.map_success(|| p_image.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImage.html>"]
#[doc(alias = "vkDestroyImage")]
#[inline]
pub unsafe fn destroy_image(
    device: &raw::Device,
    image: Option<&raw::Image>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_image.get();
    vulkan_command(
        Some(device.borrow()),
        image.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout.html>"]
#[doc(alias = "vkGetImageSubresourceLayout")]
pub unsafe fn get_image_subresource_layout(
    device: &raw::Device,
    image: &raw::Image,
    p_subresource: &ImageSubresource,
    dispatcher: &CommandsDispatcher,
) -> SubresourceLayout {
    let vulkan_command = dispatcher.get_image_subresource_layout.get();
    let mut p_layout = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        Some(image.borrow()),
        ptr::from_ref(p_subresource),
        p_layout.as_mut_ptr(),
    );
    p_layout.assume_init()
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImageView.html>"]
#[doc(alias = "vkCreateImageView")]
pub unsafe fn create_image_view(
    device: &raw::Device,
    p_create_info: &ImageViewCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<ImageView> {
    let vulkan_command = dispatcher.create_image_view.get();
    let mut p_view = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_view.as_mut_ptr(),
    );
    vk_status.map_success(|| p_view.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImageView.html>"]
#[doc(alias = "vkDestroyImageView")]
#[inline]
pub unsafe fn destroy_image_view(
    device: &raw::Device,
    image_view: Option<&raw::ImageView>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_image_view.get();
    vulkan_command(
        Some(device.borrow()),
        image_view.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCommandPool.html>"]
#[doc(alias = "vkCreateCommandPool")]
pub unsafe fn create_command_pool(
    device: &raw::Device,
    p_create_info: &CommandPoolCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<CommandPool> {
    let vulkan_command = dispatcher.create_command_pool.get();
    let mut p_command_pool = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_command_pool.as_mut_ptr(),
    );
    vk_status.map_success(|| p_command_pool.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCommandPool.html>"]
#[doc(alias = "vkDestroyCommandPool")]
#[inline]
pub unsafe fn destroy_command_pool(
    device: &raw::Device,
    command_pool: Option<&raw::CommandPool>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_command_pool.get();
    vulkan_command(
        Some(device.borrow()),
        command_pool.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandPool.html>"]
#[doc(alias = "vkResetCommandPool")]
#[inline]
pub unsafe fn reset_command_pool(
    device: &raw::Device,
    command_pool: &raw::CommandPool,
    flags: CommandPoolResetFlags,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.reset_command_pool.get();
    vulkan_command(Some(device.borrow()), Some(command_pool.borrow()), flags).map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateCommandBuffers.html>"]
#[doc(alias = "vkAllocateCommandBuffers")]
pub unsafe fn allocate_command_buffers<R: DynamicArray<CommandBuffer>>(
    device: &raw::Device,
    p_allocate_info: &CommandBufferAllocateInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.allocate_command_buffers.get();
    let mut p_command_buffers = R::create_with_capacity(p_allocate_info.command_buffer_count as _);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_allocate_info),
        p_command_buffers.get_content_mut_ptr(),
    );
    vk_status.map_success(|| {
        p_command_buffers.resize_with_len(p_allocate_info.command_buffer_count as _);
        p_command_buffers
    })
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeCommandBuffers.html>"]
#[doc(alias = "vkFreeCommandBuffers")]
#[inline]
pub unsafe fn free_command_buffers<'a, V3: Alias<raw::CommandBuffer> + 'a>(
    device: &raw::Device,
    command_pool: &raw::CommandPool,
    p_command_buffers: impl AsSlice<'a, V3>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.free_command_buffers.get();
    vulkan_command(
        Some(device.borrow()),
        Some(command_pool.borrow()),
        p_command_buffers.as_slice().len() as _,
        p_command_buffers.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBeginCommandBuffer.html>"]
#[doc(alias = "vkBeginCommandBuffer")]
#[inline]
pub unsafe fn begin_command_buffer(
    command_buffer: &raw::CommandBuffer,
    p_begin_info: &CommandBufferBeginInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.begin_command_buffer.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_begin_info)).map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEndCommandBuffer.html>"]
#[doc(alias = "vkEndCommandBuffer")]
#[inline]
pub unsafe fn end_command_buffer(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.end_command_buffer.get();
    vulkan_command(Some(command_buffer.borrow())).map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandBuffer.html>"]
#[doc(alias = "vkResetCommandBuffer")]
#[inline]
pub unsafe fn reset_command_buffer(
    command_buffer: &raw::CommandBuffer,
    flags: CommandBufferResetFlags,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.reset_command_buffer.get();
    vulkan_command(Some(command_buffer.borrow()), flags).map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer.html>"]
#[doc(alias = "vkCmdCopyBuffer")]
#[inline]
pub unsafe fn cmd_copy_buffer<'a>(
    command_buffer: &raw::CommandBuffer,
    src_buffer: &raw::Buffer,
    dst_buffer: &raw::Buffer,
    p_regions: impl AsSlice<'a, BufferCopy>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_buffer.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(src_buffer.borrow()),
        Some(dst_buffer.borrow()),
        p_regions.as_slice().len() as _,
        p_regions.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage.html>"]
#[doc(alias = "vkCmdCopyImage")]
#[inline]
pub unsafe fn cmd_copy_image<'a>(
    command_buffer: &raw::CommandBuffer,
    src_image: &raw::Image,
    src_image_layout: ImageLayout,
    dst_image: &raw::Image,
    dst_image_layout: ImageLayout,
    p_regions: impl AsSlice<'a, ImageCopy>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_image.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(src_image.borrow()),
        src_image_layout,
        Some(dst_image.borrow()),
        dst_image_layout,
        p_regions.as_slice().len() as _,
        p_regions.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage.html>"]
#[doc(alias = "vkCmdCopyBufferToImage")]
#[inline]
pub unsafe fn cmd_copy_buffer_to_image<'a>(
    command_buffer: &raw::CommandBuffer,
    src_buffer: &raw::Buffer,
    dst_image: &raw::Image,
    dst_image_layout: ImageLayout,
    p_regions: impl AsSlice<'a, BufferImageCopy>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_buffer_to_image.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(src_buffer.borrow()),
        Some(dst_image.borrow()),
        dst_image_layout,
        p_regions.as_slice().len() as _,
        p_regions.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer.html>"]
#[doc(alias = "vkCmdCopyImageToBuffer")]
#[inline]
pub unsafe fn cmd_copy_image_to_buffer<'a>(
    command_buffer: &raw::CommandBuffer,
    src_image: &raw::Image,
    src_image_layout: ImageLayout,
    dst_buffer: &raw::Buffer,
    p_regions: impl AsSlice<'a, BufferImageCopy>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_image_to_buffer.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(src_image.borrow()),
        src_image_layout,
        Some(dst_buffer.borrow()),
        p_regions.as_slice().len() as _,
        p_regions.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateBuffer.html>"]
#[doc(alias = "vkCmdUpdateBuffer")]
#[inline]
pub unsafe fn cmd_update_buffer(
    command_buffer: &raw::CommandBuffer,
    dst_buffer: &raw::Buffer,
    dst_offset: DeviceSize,
    data_size: DeviceSize,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_update_buffer.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(dst_buffer.borrow()),
        dst_offset,
        data_size,
        p_data,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillBuffer.html>"]
#[doc(alias = "vkCmdFillBuffer")]
#[inline]
pub unsafe fn cmd_fill_buffer(
    command_buffer: &raw::CommandBuffer,
    dst_buffer: &raw::Buffer,
    dst_offset: DeviceSize,
    size: DeviceSize,
    data: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_fill_buffer.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(dst_buffer.borrow()),
        dst_offset,
        size,
        data,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier.html>"]
#[doc(alias = "vkCmdPipelineBarrier")]
#[inline]
pub unsafe fn cmd_pipeline_barrier<'a>(
    command_buffer: &raw::CommandBuffer,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    dependency_flags: DependencyFlags,
    p_memory_barriers: impl AsSlice<'a, MemoryBarrier<'a>>,
    p_buffer_memory_barriers: impl AsSlice<'a, BufferMemoryBarrier<'a>>,
    p_image_memory_barriers: impl AsSlice<'a, ImageMemoryBarrier<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_pipeline_barrier.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        src_stage_mask,
        dst_stage_mask,
        dependency_flags,
        p_memory_barriers.as_slice().len() as _,
        p_memory_barriers.as_slice().as_ptr().cast(),
        p_buffer_memory_barriers.as_slice().len() as _,
        p_buffer_memory_barriers.as_slice().as_ptr().cast(),
        p_image_memory_barriers.as_slice().len() as _,
        p_image_memory_barriers.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQuery.html>"]
#[doc(alias = "vkCmdBeginQuery")]
#[inline]
pub unsafe fn cmd_begin_query(
    command_buffer: &raw::CommandBuffer,
    query_pool: &raw::QueryPool,
    query: u32,
    flags: QueryControlFlags,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_begin_query.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(query_pool.borrow()),
        query,
        flags,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQuery.html>"]
#[doc(alias = "vkCmdEndQuery")]
#[inline]
pub unsafe fn cmd_end_query(
    command_buffer: &raw::CommandBuffer,
    query_pool: &raw::QueryPool,
    query: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_end_query.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(query_pool.borrow()),
        query,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetQueryPool.html>"]
#[doc(alias = "vkCmdResetQueryPool")]
#[inline]
pub unsafe fn cmd_reset_query_pool(
    command_buffer: &raw::CommandBuffer,
    query_pool: &raw::QueryPool,
    first_query: u32,
    query_count: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_reset_query_pool.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(query_pool.borrow()),
        first_query,
        query_count,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp.html>"]
#[doc(alias = "vkCmdWriteTimestamp")]
#[inline]
pub unsafe fn cmd_write_timestamp(
    command_buffer: &raw::CommandBuffer,
    pipeline_stage: PipelineStageFlags,
    query_pool: &raw::QueryPool,
    query: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_write_timestamp.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        pipeline_stage,
        Some(query_pool.borrow()),
        query,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResults.html>"]
#[doc(alias = "vkCmdCopyQueryPoolResults")]
#[inline]
pub unsafe fn cmd_copy_query_pool_results(
    command_buffer: &raw::CommandBuffer,
    query_pool: &raw::QueryPool,
    first_query: u32,
    query_count: u32,
    dst_buffer: &raw::Buffer,
    dst_offset: DeviceSize,
    stride: DeviceSize,
    flags: QueryResultFlags,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_query_pool_results.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(query_pool.borrow()),
        first_query,
        query_count,
        Some(dst_buffer.borrow()),
        dst_offset,
        stride,
        flags,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteCommands.html>"]
#[doc(alias = "vkCmdExecuteCommands")]
#[inline]
pub unsafe fn cmd_execute_commands<'a, V2: Alias<raw::CommandBuffer> + 'a>(
    command_buffer: &raw::CommandBuffer,
    p_command_buffers: impl AsSlice<'a, V2>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_execute_commands.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_command_buffers.as_slice().len() as _,
        p_command_buffers.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateEvent.html>"]
#[doc(alias = "vkCreateEvent")]
pub unsafe fn create_event(
    device: &raw::Device,
    p_create_info: &EventCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Event> {
    let vulkan_command = dispatcher.create_event.get();
    let mut p_event = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_event.as_mut_ptr(),
    );
    vk_status.map_success(|| p_event.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyEvent.html>"]
#[doc(alias = "vkDestroyEvent")]
#[inline]
pub unsafe fn destroy_event(
    device: &raw::Device,
    event: Option<&raw::Event>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_event.get();
    vulkan_command(
        Some(device.borrow()),
        event.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEventStatus.html>"]
#[doc(alias = "vkGetEventStatus")]
#[inline]
pub unsafe fn get_event_status(
    device: &raw::Device,
    event: &raw::Event,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.get_event_status.get();
    vulkan_command(Some(device.borrow()), Some(event.borrow())).into_result()
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetEvent.html>"]
#[doc(alias = "vkSetEvent")]
#[inline]
pub unsafe fn set_event(
    device: &raw::Device,
    event: &raw::Event,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.set_event.get();
    vulkan_command(Some(device.borrow()), Some(event.borrow())).map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetEvent.html>"]
#[doc(alias = "vkResetEvent")]
#[inline]
pub unsafe fn reset_event(
    device: &raw::Device,
    event: &raw::Event,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.reset_event.get();
    vulkan_command(Some(device.borrow()), Some(event.borrow())).map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferView.html>"]
#[doc(alias = "vkCreateBufferView")]
pub unsafe fn create_buffer_view(
    device: &raw::Device,
    p_create_info: &BufferViewCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<BufferView> {
    let vulkan_command = dispatcher.create_buffer_view.get();
    let mut p_view = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_view.as_mut_ptr(),
    );
    vk_status.map_success(|| p_view.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferView.html>"]
#[doc(alias = "vkDestroyBufferView")]
#[inline]
pub unsafe fn destroy_buffer_view(
    device: &raw::Device,
    buffer_view: Option<&raw::BufferView>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_buffer_view.get();
    vulkan_command(
        Some(device.borrow()),
        buffer_view.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderModule.html>"]
#[doc(alias = "vkCreateShaderModule")]
pub unsafe fn create_shader_module(
    device: &raw::Device,
    p_create_info: &ShaderModuleCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<ShaderModule> {
    let vulkan_command = dispatcher.create_shader_module.get();
    let mut p_shader_module = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_shader_module.as_mut_ptr(),
    );
    vk_status.map_success(|| p_shader_module.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderModule.html>"]
#[doc(alias = "vkDestroyShaderModule")]
#[inline]
pub unsafe fn destroy_shader_module(
    device: &raw::Device,
    shader_module: Option<&raw::ShaderModule>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_shader_module.get();
    vulkan_command(
        Some(device.borrow()),
        shader_module.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineCache.html>"]
#[doc(alias = "vkCreatePipelineCache")]
pub unsafe fn create_pipeline_cache(
    device: &raw::Device,
    p_create_info: &PipelineCacheCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<PipelineCache> {
    let vulkan_command = dispatcher.create_pipeline_cache.get();
    let mut p_pipeline_cache = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_pipeline_cache.as_mut_ptr(),
    );
    vk_status.map_success(|| p_pipeline_cache.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineCache.html>"]
#[doc(alias = "vkDestroyPipelineCache")]
#[inline]
pub unsafe fn destroy_pipeline_cache(
    device: &raw::Device,
    pipeline_cache: Option<&raw::PipelineCache>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_pipeline_cache.get();
    vulkan_command(
        Some(device.borrow()),
        pipeline_cache.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineCacheData.html>"]
#[doc(alias = "vkGetPipelineCacheData")]
pub unsafe fn get_pipeline_cache_data(
    device: &raw::Device,
    pipeline_cache: &raw::PipelineCache,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<usize> {
    let vulkan_command = dispatcher.get_pipeline_cache_data.get();
    let mut p_data_size = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(pipeline_cache.borrow()),
        p_data_size.as_mut_ptr(),
        p_data,
    );
    vk_status.map_success(|| p_data_size.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkMergePipelineCaches.html>"]
#[doc(alias = "vkMergePipelineCaches")]
#[inline]
pub unsafe fn merge_pipeline_caches<'a, V3: Alias<raw::PipelineCache> + 'a>(
    device: &raw::Device,
    dst_cache: &raw::PipelineCache,
    p_src_caches: impl AsSlice<'a, V3>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.merge_pipeline_caches.get();
    vulkan_command(
        Some(device.borrow()),
        Some(dst_cache.borrow()),
        p_src_caches.as_slice().len() as _,
        p_src_caches.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateComputePipelines.html>"]
#[doc(alias = "vkCreateComputePipelines")]
pub unsafe fn create_compute_pipelines<'a, R: DynamicArray<Pipeline>>(
    device: &raw::Device,
    pipeline_cache: Option<&raw::PipelineCache>,
    p_create_infos: impl AsSlice<'a, ComputePipelineCreateInfo<'a>>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, R)> {
    let vulkan_command = dispatcher.create_compute_pipelines.get();
    let mut p_pipelines = R::create_with_capacity(p_create_infos.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        pipeline_cache.map(|v| v.borrow()),
        p_create_infos.as_slice().len() as _,
        p_create_infos.as_slice().as_ptr().cast(),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_pipelines.get_content_mut_ptr(),
    );
    vk_status.map_successes(|| {
        p_pipelines.resize_with_len(p_create_infos.as_slice().len() as _);
        p_pipelines
    })
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipeline.html>"]
#[doc(alias = "vkDestroyPipeline")]
#[inline]
pub unsafe fn destroy_pipeline(
    device: &raw::Device,
    pipeline: Option<&raw::Pipeline>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_pipeline.get();
    vulkan_command(
        Some(device.borrow()),
        pipeline.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineLayout.html>"]
#[doc(alias = "vkCreatePipelineLayout")]
pub unsafe fn create_pipeline_layout(
    device: &raw::Device,
    p_create_info: &PipelineLayoutCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<PipelineLayout> {
    let vulkan_command = dispatcher.create_pipeline_layout.get();
    let mut p_pipeline_layout = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_pipeline_layout.as_mut_ptr(),
    );
    vk_status.map_success(|| p_pipeline_layout.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineLayout.html>"]
#[doc(alias = "vkDestroyPipelineLayout")]
#[inline]
pub unsafe fn destroy_pipeline_layout(
    device: &raw::Device,
    pipeline_layout: Option<&raw::PipelineLayout>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_pipeline_layout.get();
    vulkan_command(
        Some(device.borrow()),
        pipeline_layout.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSampler.html>"]
#[doc(alias = "vkCreateSampler")]
pub unsafe fn create_sampler(
    device: &raw::Device,
    p_create_info: &SamplerCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Sampler> {
    let vulkan_command = dispatcher.create_sampler.get();
    let mut p_sampler = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_sampler.as_mut_ptr(),
    );
    vk_status.map_success(|| p_sampler.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySampler.html>"]
#[doc(alias = "vkDestroySampler")]
#[inline]
pub unsafe fn destroy_sampler(
    device: &raw::Device,
    sampler: Option<&raw::Sampler>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_sampler.get();
    vulkan_command(
        Some(device.borrow()),
        sampler.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorSetLayout.html>"]
#[doc(alias = "vkCreateDescriptorSetLayout")]
pub unsafe fn create_descriptor_set_layout(
    device: &raw::Device,
    p_create_info: &DescriptorSetLayoutCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DescriptorSetLayout> {
    let vulkan_command = dispatcher.create_descriptor_set_layout.get();
    let mut p_set_layout = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_set_layout.as_mut_ptr(),
    );
    vk_status.map_success(|| p_set_layout.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorSetLayout.html>"]
#[doc(alias = "vkDestroyDescriptorSetLayout")]
#[inline]
pub unsafe fn destroy_descriptor_set_layout(
    device: &raw::Device,
    descriptor_set_layout: Option<&raw::DescriptorSetLayout>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_descriptor_set_layout.get();
    vulkan_command(
        Some(device.borrow()),
        descriptor_set_layout.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorPool.html>"]
#[doc(alias = "vkCreateDescriptorPool")]
pub unsafe fn create_descriptor_pool(
    device: &raw::Device,
    p_create_info: &DescriptorPoolCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DescriptorPool> {
    let vulkan_command = dispatcher.create_descriptor_pool.get();
    let mut p_descriptor_pool = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_descriptor_pool.as_mut_ptr(),
    );
    vk_status.map_success(|| p_descriptor_pool.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorPool.html>"]
#[doc(alias = "vkDestroyDescriptorPool")]
#[inline]
pub unsafe fn destroy_descriptor_pool(
    device: &raw::Device,
    descriptor_pool: Option<&raw::DescriptorPool>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_descriptor_pool.get();
    vulkan_command(
        Some(device.borrow()),
        descriptor_pool.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetDescriptorPool.html>"]
#[doc(alias = "vkResetDescriptorPool")]
#[inline]
pub unsafe fn reset_descriptor_pool(
    device: &raw::Device,
    descriptor_pool: &raw::DescriptorPool,
    flags: u32,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.reset_descriptor_pool.get();
    vulkan_command(Some(device.borrow()), Some(descriptor_pool.borrow()), flags).map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateDescriptorSets.html>"]
#[doc(alias = "vkAllocateDescriptorSets")]
pub unsafe fn allocate_descriptor_sets<R: DynamicArray<DescriptorSet>>(
    device: &raw::Device,
    p_allocate_info: &DescriptorSetAllocateInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.allocate_descriptor_sets.get();
    let mut p_descriptor_sets = R::create_with_capacity(p_allocate_info.descriptor_set_count as _);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_allocate_info),
        p_descriptor_sets.get_content_mut_ptr(),
    );
    vk_status.map_success(|| {
        p_descriptor_sets.resize_with_len(p_allocate_info.descriptor_set_count as _);
        p_descriptor_sets
    })
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeDescriptorSets.html>"]
#[doc(alias = "vkFreeDescriptorSets")]
#[inline]
pub unsafe fn free_descriptor_sets<'a, V3: Alias<raw::DescriptorSet> + 'a>(
    device: &raw::Device,
    descriptor_pool: &raw::DescriptorPool,
    p_descriptor_sets: impl AsSlice<'a, V3>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.free_descriptor_sets.get();
    vulkan_command(
        Some(device.borrow()),
        Some(descriptor_pool.borrow()),
        p_descriptor_sets.as_slice().len() as _,
        p_descriptor_sets.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSets.html>"]
#[doc(alias = "vkUpdateDescriptorSets")]
#[inline]
pub unsafe fn update_descriptor_sets<'a>(
    device: &raw::Device,
    p_descriptor_writes: impl AsSlice<'a, WriteDescriptorSet<'a>>,
    p_descriptor_copies: impl AsSlice<'a, CopyDescriptorSet<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.update_descriptor_sets.get();
    vulkan_command(
        Some(device.borrow()),
        p_descriptor_writes.as_slice().len() as _,
        p_descriptor_writes.as_slice().as_ptr().cast(),
        p_descriptor_copies.as_slice().len() as _,
        p_descriptor_copies.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipeline.html>"]
#[doc(alias = "vkCmdBindPipeline")]
#[inline]
pub unsafe fn cmd_bind_pipeline(
    command_buffer: &raw::CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: &raw::Pipeline,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_pipeline.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        pipeline_bind_point,
        Some(pipeline.borrow()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets.html>"]
#[doc(alias = "vkCmdBindDescriptorSets")]
#[inline]
pub unsafe fn cmd_bind_descriptor_sets<'a, V5: Alias<raw::DescriptorSet> + 'a>(
    command_buffer: &raw::CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: &raw::PipelineLayout,
    first_set: u32,
    p_descriptor_sets: impl AsSlice<'a, V5>,
    p_dynamic_offsets: impl AsSlice<'a, u32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_descriptor_sets.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        pipeline_bind_point,
        Some(layout.borrow()),
        first_set,
        p_descriptor_sets.as_slice().len() as _,
        p_descriptor_sets.as_slice().as_ptr().cast(),
        p_dynamic_offsets.as_slice().len() as _,
        p_dynamic_offsets.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearColorImage.html>"]
#[doc(alias = "vkCmdClearColorImage")]
#[inline]
pub unsafe fn cmd_clear_color_image<'a>(
    command_buffer: &raw::CommandBuffer,
    image: &raw::Image,
    image_layout: ImageLayout,
    p_color: &ClearColorValue,
    p_ranges: impl AsSlice<'a, ImageSubresourceRange>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_clear_color_image.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(image.borrow()),
        image_layout,
        ptr::from_ref(p_color),
        p_ranges.as_slice().len() as _,
        p_ranges.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatch.html>"]
#[doc(alias = "vkCmdDispatch")]
#[inline]
pub unsafe fn cmd_dispatch(
    command_buffer: &raw::CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_dispatch.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        group_count_x,
        group_count_y,
        group_count_z,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect.html>"]
#[doc(alias = "vkCmdDispatchIndirect")]
#[inline]
pub unsafe fn cmd_dispatch_indirect(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_dispatch_indirect.get();
    vulkan_command(Some(command_buffer.borrow()), Some(buffer.borrow()), offset)
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent.html>"]
#[doc(alias = "vkCmdSetEvent")]
#[inline]
pub unsafe fn cmd_set_event(
    command_buffer: &raw::CommandBuffer,
    event: &raw::Event,
    stage_mask: PipelineStageFlags,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_event.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(event.borrow()),
        stage_mask,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent.html>"]
#[doc(alias = "vkCmdResetEvent")]
#[inline]
pub unsafe fn cmd_reset_event(
    command_buffer: &raw::CommandBuffer,
    event: &raw::Event,
    stage_mask: PipelineStageFlags,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_reset_event.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(event.borrow()),
        stage_mask,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents.html>"]
#[doc(alias = "vkCmdWaitEvents")]
#[inline]
pub unsafe fn cmd_wait_events<'a, V2: Alias<raw::Event> + 'a>(
    command_buffer: &raw::CommandBuffer,
    p_events: impl AsSlice<'a, V2>,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    p_memory_barriers: impl AsSlice<'a, MemoryBarrier<'a>>,
    p_buffer_memory_barriers: impl AsSlice<'a, BufferMemoryBarrier<'a>>,
    p_image_memory_barriers: impl AsSlice<'a, ImageMemoryBarrier<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_wait_events.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_events.as_slice().len() as _,
        p_events.as_slice().as_ptr().cast(),
        src_stage_mask,
        dst_stage_mask,
        p_memory_barriers.as_slice().len() as _,
        p_memory_barriers.as_slice().as_ptr().cast(),
        p_buffer_memory_barriers.as_slice().len() as _,
        p_buffer_memory_barriers.as_slice().as_ptr().cast(),
        p_image_memory_barriers.as_slice().len() as _,
        p_image_memory_barriers.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants.html>"]
#[doc(alias = "vkCmdPushConstants")]
#[inline]
pub unsafe fn cmd_push_constants(
    command_buffer: &raw::CommandBuffer,
    layout: &raw::PipelineLayout,
    stage_flags: ShaderStageFlags,
    offset: u32,
    size: u32,
    p_values: VoidPtr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_push_constants.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(layout.borrow()),
        stage_flags,
        offset,
        size,
        p_values,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGraphicsPipelines.html>"]
#[doc(alias = "vkCreateGraphicsPipelines")]
pub unsafe fn create_graphics_pipelines<'a, R: DynamicArray<Pipeline>>(
    device: &raw::Device,
    pipeline_cache: Option<&raw::PipelineCache>,
    p_create_infos: impl AsSlice<'a, GraphicsPipelineCreateInfo<'a>>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, R)> {
    let vulkan_command = dispatcher.create_graphics_pipelines.get();
    let mut p_pipelines = R::create_with_capacity(p_create_infos.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        pipeline_cache.map(|v| v.borrow()),
        p_create_infos.as_slice().len() as _,
        p_create_infos.as_slice().as_ptr().cast(),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_pipelines.get_content_mut_ptr(),
    );
    vk_status.map_successes(|| {
        p_pipelines.resize_with_len(p_create_infos.as_slice().len() as _);
        p_pipelines
    })
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFramebuffer.html>"]
#[doc(alias = "vkCreateFramebuffer")]
pub unsafe fn create_framebuffer(
    device: &raw::Device,
    p_create_info: &FramebufferCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Framebuffer> {
    let vulkan_command = dispatcher.create_framebuffer.get();
    let mut p_framebuffer = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_framebuffer.as_mut_ptr(),
    );
    vk_status.map_success(|| p_framebuffer.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFramebuffer.html>"]
#[doc(alias = "vkDestroyFramebuffer")]
#[inline]
pub unsafe fn destroy_framebuffer(
    device: &raw::Device,
    framebuffer: Option<&raw::Framebuffer>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_framebuffer.get();
    vulkan_command(
        Some(device.borrow()),
        framebuffer.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass.html>"]
#[doc(alias = "vkCreateRenderPass")]
pub unsafe fn create_render_pass(
    device: &raw::Device,
    p_create_info: &RenderPassCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<RenderPass> {
    let vulkan_command = dispatcher.create_render_pass.get();
    let mut p_render_pass = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_render_pass.as_mut_ptr(),
    );
    vk_status.map_success(|| p_render_pass.assume_init())
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyRenderPass.html>"]
#[doc(alias = "vkDestroyRenderPass")]
#[inline]
pub unsafe fn destroy_render_pass(
    device: &raw::Device,
    render_pass: Option<&raw::RenderPass>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_render_pass.get();
    vulkan_command(
        Some(device.borrow()),
        render_pass.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderAreaGranularity.html>"]
#[doc(alias = "vkGetRenderAreaGranularity")]
pub unsafe fn get_render_area_granularity(
    device: &raw::Device,
    render_pass: &raw::RenderPass,
    dispatcher: &CommandsDispatcher,
) -> Extent2D {
    let vulkan_command = dispatcher.get_render_area_granularity.get();
    let mut p_granularity = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        Some(render_pass.borrow()),
        p_granularity.as_mut_ptr(),
    );
    p_granularity.assume_init()
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewport.html>"]
#[doc(alias = "vkCmdSetViewport")]
#[inline]
pub unsafe fn cmd_set_viewport<'a>(
    command_buffer: &raw::CommandBuffer,
    first_viewport: u32,
    p_viewports: impl AsSlice<'a, Viewport>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_viewport.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_viewport,
        p_viewports.as_slice().len() as _,
        p_viewports.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissor.html>"]
#[doc(alias = "vkCmdSetScissor")]
#[inline]
pub unsafe fn cmd_set_scissor<'a>(
    command_buffer: &raw::CommandBuffer,
    first_scissor: u32,
    p_scissors: impl AsSlice<'a, Rect2D>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_scissor.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_scissor,
        p_scissors.as_slice().len() as _,
        p_scissors.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineWidth.html>"]
#[doc(alias = "vkCmdSetLineWidth")]
#[inline]
pub unsafe fn cmd_set_line_width(
    command_buffer: &raw::CommandBuffer,
    line_width: f32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_line_width.get();
    vulkan_command(Some(command_buffer.borrow()), line_width)
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias.html>"]
#[doc(alias = "vkCmdSetDepthBias")]
#[inline]
pub unsafe fn cmd_set_depth_bias(
    command_buffer: &raw::CommandBuffer,
    depth_bias_constant_factor: f32,
    depth_bias_clamp: f32,
    depth_bias_slope_factor: f32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_bias.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        depth_bias_constant_factor,
        depth_bias_clamp,
        depth_bias_slope_factor,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetBlendConstants.html>"]
#[doc(alias = "vkCmdSetBlendConstants")]
#[inline]
pub unsafe fn cmd_set_blend_constants(
    command_buffer: &raw::CommandBuffer,
    blend_constants: [f32; 4u16 as _],
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_blend_constants.get();
    vulkan_command(Some(command_buffer.borrow()), blend_constants)
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBounds.html>"]
#[doc(alias = "vkCmdSetDepthBounds")]
#[inline]
pub unsafe fn cmd_set_depth_bounds(
    command_buffer: &raw::CommandBuffer,
    min_depth_bounds: f32,
    max_depth_bounds: f32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_bounds.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        min_depth_bounds,
        max_depth_bounds,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilCompareMask.html>"]
#[doc(alias = "vkCmdSetStencilCompareMask")]
#[inline]
pub unsafe fn cmd_set_stencil_compare_mask(
    command_buffer: &raw::CommandBuffer,
    face_mask: StencilFaceFlags,
    compare_mask: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_stencil_compare_mask.get();
    vulkan_command(Some(command_buffer.borrow()), face_mask, compare_mask)
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilWriteMask.html>"]
#[doc(alias = "vkCmdSetStencilWriteMask")]
#[inline]
pub unsafe fn cmd_set_stencil_write_mask(
    command_buffer: &raw::CommandBuffer,
    face_mask: StencilFaceFlags,
    write_mask: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_stencil_write_mask.get();
    vulkan_command(Some(command_buffer.borrow()), face_mask, write_mask)
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilReference.html>"]
#[doc(alias = "vkCmdSetStencilReference")]
#[inline]
pub unsafe fn cmd_set_stencil_reference(
    command_buffer: &raw::CommandBuffer,
    face_mask: StencilFaceFlags,
    reference: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_stencil_reference.get();
    vulkan_command(Some(command_buffer.borrow()), face_mask, reference)
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer.html>"]
#[doc(alias = "vkCmdBindIndexBuffer")]
#[inline]
pub unsafe fn cmd_bind_index_buffer(
    command_buffer: &raw::CommandBuffer,
    buffer: Option<&raw::Buffer>,
    offset: DeviceSize,
    index_type: IndexType,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_index_buffer.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        buffer.map(|v| v.borrow()),
        offset,
        index_type,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers.html>"]
#[doc(alias = "vkCmdBindVertexBuffers")]
#[inline]
pub unsafe fn cmd_bind_vertex_buffers<'a, V3: Alias<raw::Buffer> + 'a>(
    command_buffer: &raw::CommandBuffer,
    first_binding: u32,
    p_buffers: impl AsSlice<'a, V3>,
    p_offsets: impl AsSlice<'a, DeviceSize>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_vertex_buffers.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_binding,
        p_offsets.as_slice().len() as _,
        p_buffers.as_slice().as_ptr().cast(),
        p_offsets.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDraw.html>"]
#[doc(alias = "vkCmdDraw")]
#[inline]
pub unsafe fn cmd_draw(
    command_buffer: &raw::CommandBuffer,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        vertex_count,
        instance_count,
        first_vertex,
        first_instance,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexed.html>"]
#[doc(alias = "vkCmdDrawIndexed")]
#[inline]
pub unsafe fn cmd_draw_indexed(
    command_buffer: &raw::CommandBuffer,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_indexed.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        index_count,
        instance_count,
        first_index,
        vertex_offset,
        first_instance,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect.html>"]
#[doc(alias = "vkCmdDrawIndirect")]
#[inline]
pub unsafe fn cmd_draw_indirect(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_indirect.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(buffer.borrow()),
        offset,
        draw_count,
        stride,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect.html>"]
#[doc(alias = "vkCmdDrawIndexedIndirect")]
#[inline]
pub unsafe fn cmd_draw_indexed_indirect(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_indexed_indirect.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(buffer.borrow()),
        offset,
        draw_count,
        stride,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage.html>"]
#[doc(alias = "vkCmdBlitImage")]
#[inline]
pub unsafe fn cmd_blit_image<'a>(
    command_buffer: &raw::CommandBuffer,
    src_image: &raw::Image,
    src_image_layout: ImageLayout,
    dst_image: &raw::Image,
    dst_image_layout: ImageLayout,
    p_regions: impl AsSlice<'a, ImageBlit>,
    filter: Filter,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_blit_image.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(src_image.borrow()),
        src_image_layout,
        Some(dst_image.borrow()),
        dst_image_layout,
        p_regions.as_slice().len() as _,
        p_regions.as_slice().as_ptr().cast(),
        filter,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearDepthStencilImage.html>"]
#[doc(alias = "vkCmdClearDepthStencilImage")]
#[inline]
pub unsafe fn cmd_clear_depth_stencil_image<'a>(
    command_buffer: &raw::CommandBuffer,
    image: &raw::Image,
    image_layout: ImageLayout,
    p_depth_stencil: &ClearDepthStencilValue,
    p_ranges: impl AsSlice<'a, ImageSubresourceRange>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_clear_depth_stencil_image.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(image.borrow()),
        image_layout,
        ptr::from_ref(p_depth_stencil),
        p_ranges.as_slice().len() as _,
        p_ranges.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearAttachments.html>"]
#[doc(alias = "vkCmdClearAttachments")]
#[inline]
pub unsafe fn cmd_clear_attachments<'a>(
    command_buffer: &raw::CommandBuffer,
    p_attachments: impl AsSlice<'a, ClearAttachment>,
    p_rects: impl AsSlice<'a, ClearRect>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_clear_attachments.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_attachments.as_slice().len() as _,
        p_attachments.as_slice().as_ptr().cast(),
        p_rects.as_slice().len() as _,
        p_rects.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage.html>"]
#[doc(alias = "vkCmdResolveImage")]
#[inline]
pub unsafe fn cmd_resolve_image<'a>(
    command_buffer: &raw::CommandBuffer,
    src_image: &raw::Image,
    src_image_layout: ImageLayout,
    dst_image: &raw::Image,
    dst_image_layout: ImageLayout,
    p_regions: impl AsSlice<'a, ImageResolve>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_resolve_image.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(src_image.borrow()),
        src_image_layout,
        Some(dst_image.borrow()),
        dst_image_layout,
        p_regions.as_slice().len() as _,
        p_regions.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass.html>"]
#[doc(alias = "vkCmdBeginRenderPass")]
#[inline]
pub unsafe fn cmd_begin_render_pass(
    command_buffer: &raw::CommandBuffer,
    p_render_pass_begin: &RenderPassBeginInfo,
    contents: SubpassContents,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_begin_render_pass.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_render_pass_begin),
        contents,
    )
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass.html>"]
#[doc(alias = "vkCmdNextSubpass")]
#[inline]
pub unsafe fn cmd_next_subpass(
    command_buffer: &raw::CommandBuffer,
    contents: SubpassContents,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_next_subpass.get();
    vulkan_command(Some(command_buffer.borrow()), contents)
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass.html>"]
#[doc(alias = "vkCmdEndRenderPass")]
#[inline]
pub unsafe fn cmd_end_render_pass(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_end_render_pass.get();
    vulkan_command(Some(command_buffer.borrow()))
}
#[cfg(feature = "version_1_1")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceVersion.html>"]
#[doc(alias = "vkEnumerateInstanceVersion")]
pub unsafe fn enumerate_instance_version(dispatcher: &CommandsDispatcher) -> Result<u32> {
    let vulkan_command = dispatcher.enumerate_instance_version.get();
    let mut p_api_version = MaybeUninit::uninit();
    let vk_status = vulkan_command(p_api_version.as_mut_ptr());
    vk_status.map_success(|| p_api_version.assume_init())
}
#[cfg(any(feature = "ext_bind_memory2", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2.html>"]
#[doc(alias = "vkBindBufferMemory2")]
#[inline]
pub unsafe fn bind_buffer_memory2<'a>(
    device: &raw::Device,
    p_bind_infos: impl AsSlice<'a, BindBufferMemoryInfo<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.bind_buffer_memory2.get();
    vulkan_command(
        Some(device.borrow()),
        p_bind_infos.as_slice().len() as _,
        p_bind_infos.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_bind_memory2", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2KHR.html>"]
#[doc(alias = "vkBindBufferMemory2KHR")]
#[inline]
pub unsafe fn bind_buffer_memory2_khr<'a>(
    device: &raw::Device,
    p_bind_infos: impl AsSlice<'a, BindBufferMemoryInfo<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.bind_buffer_memory2_khr.get();
    vulkan_command(
        Some(device.borrow()),
        p_bind_infos.as_slice().len() as _,
        p_bind_infos.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_bind_memory2", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2.html>"]
#[doc(alias = "vkBindImageMemory2")]
#[inline]
pub unsafe fn bind_image_memory2<'a>(
    device: &raw::Device,
    p_bind_infos: impl AsSlice<'a, BindImageMemoryInfo<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.bind_image_memory2.get();
    vulkan_command(
        Some(device.borrow()),
        p_bind_infos.as_slice().len() as _,
        p_bind_infos.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_bind_memory2", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2KHR.html>"]
#[doc(alias = "vkBindImageMemory2KHR")]
#[inline]
pub unsafe fn bind_image_memory2_khr<'a>(
    device: &raw::Device,
    p_bind_infos: impl AsSlice<'a, BindImageMemoryInfo<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.bind_image_memory2_khr.get();
    vulkan_command(
        Some(device.borrow()),
        p_bind_infos.as_slice().len() as _,
        p_bind_infos.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_device_group", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeatures.html>"]
#[doc(alias = "vkGetDeviceGroupPeerMemoryFeatures")]
pub unsafe fn get_device_group_peer_memory_features(
    device: &raw::Device,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    dispatcher: &CommandsDispatcher,
) -> PeerMemoryFeatureFlags {
    let vulkan_command = dispatcher.get_device_group_peer_memory_features.get();
    let mut p_peer_memory_features = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        heap_index,
        local_device_index,
        remote_device_index,
        p_peer_memory_features.as_mut_ptr(),
    );
    p_peer_memory_features.assume_init()
}
#[cfg(any(feature = "ext_device_group", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeaturesKHR.html>"]
#[doc(alias = "vkGetDeviceGroupPeerMemoryFeaturesKHR")]
pub unsafe fn get_device_group_peer_memory_features_khr(
    device: &raw::Device,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    dispatcher: &CommandsDispatcher,
) -> PeerMemoryFeatureFlags {
    let vulkan_command = dispatcher.get_device_group_peer_memory_features_khr.get();
    let mut p_peer_memory_features = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        heap_index,
        local_device_index,
        remote_device_index,
        p_peer_memory_features.as_mut_ptr(),
    );
    p_peer_memory_features.assume_init()
}
#[cfg(any(feature = "ext_device_group", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMask.html>"]
#[doc(alias = "vkCmdSetDeviceMask")]
#[inline]
pub unsafe fn cmd_set_device_mask(
    command_buffer: &raw::CommandBuffer,
    device_mask: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_device_mask.get();
    vulkan_command(Some(command_buffer.borrow()), device_mask)
}
#[cfg(any(feature = "ext_device_group", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMaskKHR.html>"]
#[doc(alias = "vkCmdSetDeviceMaskKHR")]
#[inline]
pub unsafe fn cmd_set_device_mask_khr(
    command_buffer: &raw::CommandBuffer,
    device_mask: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_device_mask_khr.get();
    vulkan_command(Some(command_buffer.borrow()), device_mask)
}
#[cfg(any(feature = "ext_device_group_creation", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html>"]
#[doc(alias = "vkEnumeratePhysicalDeviceGroups")]
pub unsafe fn enumerate_physical_device_groups<
    R: DynamicArray<PhysicalDeviceGroupProperties<'static>>,
>(
    instance: &raw::Instance,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.enumerate_physical_device_groups.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_physical_device_group_count = vk_len.as_mut_ptr();
    let p_physical_device_group_properties = ptr::null_mut();
    vulkan_command(
        Some(instance.borrow()),
        p_physical_device_group_count,
        p_physical_device_group_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_physical_device_group_count = ptr::from_mut(&mut vk_len);
    let mut p_physical_device_group_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(instance.borrow()),
            p_physical_device_group_count,
            p_physical_device_group_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_physical_device_group_count = ptr::from_mut(&mut vk_len);
        p_physical_device_group_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(any(feature = "ext_device_group_creation", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroupsKHR.html>"]
#[doc(alias = "vkEnumeratePhysicalDeviceGroupsKHR")]
pub unsafe fn enumerate_physical_device_groups_khr<
    R: DynamicArray<PhysicalDeviceGroupProperties<'static>>,
>(
    instance: &raw::Instance,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.enumerate_physical_device_groups_khr.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_physical_device_group_count = vk_len.as_mut_ptr();
    let p_physical_device_group_properties = ptr::null_mut();
    vulkan_command(
        Some(instance.borrow()),
        p_physical_device_group_count,
        p_physical_device_group_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_physical_device_group_count = ptr::from_mut(&mut vk_len);
    let mut p_physical_device_group_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(instance.borrow()),
            p_physical_device_group_count,
            p_physical_device_group_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_physical_device_group_count = ptr::from_mut(&mut vk_len);
        p_physical_device_group_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(any(feature = "ext_get_memory_requirements2", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2.html>"]
#[doc(alias = "vkGetImageMemoryRequirements2")]
pub unsafe fn get_image_memory_requirements2<S: StructureChainOut<MemoryRequirements2<'static>>>(
    device: &raw::Device,
    p_info: &ImageMemoryRequirementsInfo2,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_image_memory_requirements2.get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[cfg(any(feature = "ext_get_memory_requirements2", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2KHR.html>"]
#[doc(alias = "vkGetImageMemoryRequirements2KHR")]
pub unsafe fn get_image_memory_requirements2_khr<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &ImageMemoryRequirementsInfo2,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_image_memory_requirements2_khr.get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[cfg(any(feature = "ext_get_memory_requirements2", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2.html>"]
#[doc(alias = "vkGetBufferMemoryRequirements2")]
pub unsafe fn get_buffer_memory_requirements2<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &BufferMemoryRequirementsInfo2,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_buffer_memory_requirements2.get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[cfg(any(feature = "ext_get_memory_requirements2", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2KHR.html>"]
#[doc(alias = "vkGetBufferMemoryRequirements2KHR")]
pub unsafe fn get_buffer_memory_requirements2_khr<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &BufferMemoryRequirementsInfo2,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_buffer_memory_requirements2_khr.get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[cfg(any(feature = "ext_get_memory_requirements2", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2.html>"]
#[doc(alias = "vkGetImageSparseMemoryRequirements2")]
pub unsafe fn get_image_sparse_memory_requirements2<
    R: DynamicArray<SparseImageMemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &ImageSparseMemoryRequirementsInfo2,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher.get_image_sparse_memory_requirements2.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_sparse_memory_requirement_count = vk_len.as_mut_ptr();
    let p_sparse_memory_requirements = ptr::null_mut();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_sparse_memory_requirement_count = ptr::from_mut(&mut vk_len);
    let mut p_sparse_memory_requirements = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[cfg(any(feature = "ext_get_memory_requirements2", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2KHR.html>"]
#[doc(alias = "vkGetImageSparseMemoryRequirements2KHR")]
pub unsafe fn get_image_sparse_memory_requirements2_khr<
    R: DynamicArray<SparseImageMemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &ImageSparseMemoryRequirementsInfo2,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher.get_image_sparse_memory_requirements2_khr.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_sparse_memory_requirement_count = vk_len.as_mut_ptr();
    let p_sparse_memory_requirements = ptr::null_mut();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_sparse_memory_requirement_count = ptr::from_mut(&mut vk_len);
    let mut p_sparse_memory_requirements = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[cfg(any(
    feature = "ext_get_physical_device_properties2",
    feature = "version_1_1"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2.html>"]
#[doc(alias = "vkGetPhysicalDeviceFeatures2")]
pub unsafe fn get_physical_device_features2<
    S: StructureChainOut<PhysicalDeviceFeatures2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_physical_device_features2.get();
    let mut p_features = MaybeUninit::uninit();
    S::setup_uninit(&mut p_features);
    vulkan_command(
        Some(physical_device.borrow()),
        S::get_uninit_head_ptr(p_features.as_mut_ptr()),
    );
    S::setup_cleanup(p_features.as_mut_ptr());
    p_features.assume_init()
}
#[cfg(any(
    feature = "ext_get_physical_device_properties2",
    feature = "version_1_1"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceFeatures2KHR")]
pub unsafe fn get_physical_device_features2_khr<
    S: StructureChainOut<PhysicalDeviceFeatures2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_physical_device_features2_khr.get();
    let mut p_features = MaybeUninit::uninit();
    S::setup_uninit(&mut p_features);
    vulkan_command(
        Some(physical_device.borrow()),
        S::get_uninit_head_ptr(p_features.as_mut_ptr()),
    );
    S::setup_cleanup(p_features.as_mut_ptr());
    p_features.assume_init()
}
#[cfg(any(
    feature = "ext_get_physical_device_properties2",
    feature = "version_1_1"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2.html>"]
#[doc(alias = "vkGetPhysicalDeviceProperties2")]
pub unsafe fn get_physical_device_properties2<
    S: StructureChainOut<PhysicalDeviceProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_physical_device_properties2.get();
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    vulkan_command(
        Some(physical_device.borrow()),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_properties.as_mut_ptr());
    p_properties.assume_init()
}
#[cfg(any(
    feature = "ext_get_physical_device_properties2",
    feature = "version_1_1"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceProperties2KHR")]
pub unsafe fn get_physical_device_properties2_khr<
    S: StructureChainOut<PhysicalDeviceProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_physical_device_properties2_khr.get();
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    vulkan_command(
        Some(physical_device.borrow()),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_properties.as_mut_ptr());
    p_properties.assume_init()
}
#[cfg(any(
    feature = "ext_get_physical_device_properties2",
    feature = "version_1_1"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2.html>"]
#[doc(alias = "vkGetPhysicalDeviceFormatProperties2")]
pub unsafe fn get_physical_device_format_properties2<
    S: StructureChainOut<FormatProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    format: Format,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_physical_device_format_properties2.get();
    let mut p_format_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_format_properties);
    vulkan_command(
        Some(physical_device.borrow()),
        format,
        S::get_uninit_head_ptr(p_format_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_format_properties.as_mut_ptr());
    p_format_properties.assume_init()
}
#[cfg(any(
    feature = "ext_get_physical_device_properties2",
    feature = "version_1_1"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceFormatProperties2KHR")]
pub unsafe fn get_physical_device_format_properties2_khr<
    S: StructureChainOut<FormatProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    format: Format,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_physical_device_format_properties2_khr.get();
    let mut p_format_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_format_properties);
    vulkan_command(
        Some(physical_device.borrow()),
        format,
        S::get_uninit_head_ptr(p_format_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_format_properties.as_mut_ptr());
    p_format_properties.assume_init()
}
#[cfg(any(
    feature = "ext_get_physical_device_properties2",
    feature = "version_1_1"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2.html>"]
#[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2")]
pub unsafe fn get_physical_device_image_format_properties2<
    S: StructureChainOut<ImageFormatProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_image_format_info: &PhysicalDeviceImageFormatInfo2,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_physical_device_image_format_properties2
        .get();
    let mut p_image_format_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_image_format_properties);
    let vk_status = vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_image_format_info),
        S::get_uninit_head_ptr(p_image_format_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_image_format_properties.as_mut_ptr());
        p_image_format_properties.assume_init()
    })
}
#[cfg(any(
    feature = "ext_get_physical_device_properties2",
    feature = "version_1_1"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2KHR")]
pub unsafe fn get_physical_device_image_format_properties2_khr<
    S: StructureChainOut<ImageFormatProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_image_format_info: &PhysicalDeviceImageFormatInfo2,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_physical_device_image_format_properties2_khr
        .get();
    let mut p_image_format_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_image_format_properties);
    let vk_status = vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_image_format_info),
        S::get_uninit_head_ptr(p_image_format_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_image_format_properties.as_mut_ptr());
        p_image_format_properties.assume_init()
    })
}
#[cfg(any(
    feature = "ext_get_physical_device_properties2",
    feature = "version_1_1"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2.html>"]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2")]
pub unsafe fn get_physical_device_queue_family_properties2<
    R: DynamicArray<QueueFamilyProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher
        .get_physical_device_queue_family_properties2
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_queue_family_property_count = vk_len.as_mut_ptr();
    let p_queue_family_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_queue_family_property_count,
        p_queue_family_properties,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_queue_family_property_count = ptr::from_mut(&mut vk_len);
    let mut p_queue_family_properties = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(physical_device.borrow()),
        p_queue_family_property_count,
        p_queue_family_properties,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[cfg(any(
    feature = "ext_get_physical_device_properties2",
    feature = "version_1_1"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2KHR")]
pub unsafe fn get_physical_device_queue_family_properties2_khr<
    R: DynamicArray<QueueFamilyProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher
        .get_physical_device_queue_family_properties2_khr
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_queue_family_property_count = vk_len.as_mut_ptr();
    let p_queue_family_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_queue_family_property_count,
        p_queue_family_properties,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_queue_family_property_count = ptr::from_mut(&mut vk_len);
    let mut p_queue_family_properties = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(physical_device.borrow()),
        p_queue_family_property_count,
        p_queue_family_properties,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[cfg(any(
    feature = "ext_get_physical_device_properties2",
    feature = "version_1_1"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2.html>"]
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties2")]
pub unsafe fn get_physical_device_memory_properties2<
    S: StructureChainOut<PhysicalDeviceMemoryProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_physical_device_memory_properties2.get();
    let mut p_memory_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_properties);
    vulkan_command(
        Some(physical_device.borrow()),
        S::get_uninit_head_ptr(p_memory_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_properties.as_mut_ptr());
    p_memory_properties.assume_init()
}
#[cfg(any(
    feature = "ext_get_physical_device_properties2",
    feature = "version_1_1"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties2KHR")]
pub unsafe fn get_physical_device_memory_properties2_khr<
    S: StructureChainOut<PhysicalDeviceMemoryProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_physical_device_memory_properties2_khr.get();
    let mut p_memory_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_properties);
    vulkan_command(
        Some(physical_device.borrow()),
        S::get_uninit_head_ptr(p_memory_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_properties.as_mut_ptr());
    p_memory_properties.assume_init()
}
#[cfg(any(
    feature = "ext_get_physical_device_properties2",
    feature = "version_1_1"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2.html>"]
#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2")]
pub unsafe fn get_physical_device_sparse_image_format_properties2<
    R: DynamicArray<SparseImageFormatProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_format_info: &PhysicalDeviceSparseImageFormatInfo2,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher
        .get_physical_device_sparse_image_format_properties2
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_format_info),
        p_property_count,
        p_properties,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_format_info),
        p_property_count,
        p_properties,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[cfg(any(
    feature = "ext_get_physical_device_properties2",
    feature = "version_1_1"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2KHR")]
pub unsafe fn get_physical_device_sparse_image_format_properties2_khr<
    R: DynamicArray<SparseImageFormatProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_format_info: &PhysicalDeviceSparseImageFormatInfo2,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher
        .get_physical_device_sparse_image_format_properties2_khr
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_format_info),
        p_property_count,
        p_properties,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_format_info),
        p_property_count,
        p_properties,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[cfg(any(feature = "ext_maintenance1", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPool.html>"]
#[doc(alias = "vkTrimCommandPool")]
#[inline]
pub unsafe fn trim_command_pool(
    device: &raw::Device,
    command_pool: &raw::CommandPool,
    flags: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.trim_command_pool.get();
    vulkan_command(Some(device.borrow()), Some(command_pool.borrow()), flags)
}
#[cfg(any(feature = "ext_maintenance1", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPoolKHR.html>"]
#[doc(alias = "vkTrimCommandPoolKHR")]
#[inline]
pub unsafe fn trim_command_pool_khr(
    device: &raw::Device,
    command_pool: &raw::CommandPool,
    flags: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.trim_command_pool_khr.get();
    vulkan_command(Some(device.borrow()), Some(command_pool.borrow()), flags)
}
#[cfg(feature = "version_1_1")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue2.html>"]
#[doc(alias = "vkGetDeviceQueue2")]
pub unsafe fn get_device_queue2(
    device: &raw::Device,
    p_queue_info: &DeviceQueueInfo2,
    dispatcher: &CommandsDispatcher,
) -> Queue {
    let vulkan_command = dispatcher.get_device_queue2.get();
    let mut p_queue = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_queue_info),
        p_queue.as_mut_ptr(),
    );
    p_queue.assume_init()
}
#[cfg(any(feature = "ext_external_memory_capabilities", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferProperties.html>"]
#[doc(alias = "vkGetPhysicalDeviceExternalBufferProperties")]
pub unsafe fn get_physical_device_external_buffer_properties<
    S: StructureChainOut<ExternalBufferProperties<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_external_buffer_info: &PhysicalDeviceExternalBufferInfo,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_external_buffer_properties
        .get();
    let mut p_external_buffer_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_external_buffer_properties);
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_external_buffer_info),
        S::get_uninit_head_ptr(p_external_buffer_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_external_buffer_properties.as_mut_ptr());
    p_external_buffer_properties.assume_init()
}
#[cfg(any(feature = "ext_external_memory_capabilities", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceExternalBufferPropertiesKHR")]
pub unsafe fn get_physical_device_external_buffer_properties_khr<
    S: StructureChainOut<ExternalBufferProperties<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_external_buffer_info: &PhysicalDeviceExternalBufferInfo,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_external_buffer_properties_khr
        .get();
    let mut p_external_buffer_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_external_buffer_properties);
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_external_buffer_info),
        S::get_uninit_head_ptr(p_external_buffer_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_external_buffer_properties.as_mut_ptr());
    p_external_buffer_properties.assume_init()
}
#[cfg(any(feature = "ext_external_fence_capabilities", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFenceProperties.html>"]
#[doc(alias = "vkGetPhysicalDeviceExternalFenceProperties")]
pub unsafe fn get_physical_device_external_fence_properties<
    S: StructureChainOut<ExternalFenceProperties<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_external_fence_info: &PhysicalDeviceExternalFenceInfo,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_external_fence_properties
        .get();
    let mut p_external_fence_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_external_fence_properties);
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_external_fence_info),
        S::get_uninit_head_ptr(p_external_fence_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_external_fence_properties.as_mut_ptr());
    p_external_fence_properties.assume_init()
}
#[cfg(any(feature = "ext_external_fence_capabilities", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFencePropertiesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceExternalFencePropertiesKHR")]
pub unsafe fn get_physical_device_external_fence_properties_khr<
    S: StructureChainOut<ExternalFenceProperties<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_external_fence_info: &PhysicalDeviceExternalFenceInfo,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_external_fence_properties_khr
        .get();
    let mut p_external_fence_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_external_fence_properties);
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_external_fence_info),
        S::get_uninit_head_ptr(p_external_fence_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_external_fence_properties.as_mut_ptr());
    p_external_fence_properties.assume_init()
}
#[cfg(any(
    feature = "ext_external_semaphore_capabilities",
    feature = "version_1_1"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphoreProperties.html>"]
#[doc(alias = "vkGetPhysicalDeviceExternalSemaphoreProperties")]
pub unsafe fn get_physical_device_external_semaphore_properties<
    S: StructureChainOut<ExternalSemaphoreProperties<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_external_semaphore_properties
        .get();
    let mut p_external_semaphore_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_external_semaphore_properties);
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_external_semaphore_info),
        S::get_uninit_head_ptr(p_external_semaphore_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_external_semaphore_properties.as_mut_ptr());
    p_external_semaphore_properties.assume_init()
}
#[cfg(any(
    feature = "ext_external_semaphore_capabilities",
    feature = "version_1_1"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")]
pub unsafe fn get_physical_device_external_semaphore_properties_khr<
    S: StructureChainOut<ExternalSemaphoreProperties<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_external_semaphore_properties_khr
        .get();
    let mut p_external_semaphore_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_external_semaphore_properties);
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_external_semaphore_info),
        S::get_uninit_head_ptr(p_external_semaphore_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_external_semaphore_properties.as_mut_ptr());
    p_external_semaphore_properties.assume_init()
}
#[cfg(any(feature = "ext_device_group", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBase.html>"]
#[doc(alias = "vkCmdDispatchBase")]
#[inline]
pub unsafe fn cmd_dispatch_base(
    command_buffer: &raw::CommandBuffer,
    base_group_x: u32,
    base_group_y: u32,
    base_group_z: u32,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_dispatch_base.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        base_group_x,
        base_group_y,
        base_group_z,
        group_count_x,
        group_count_y,
        group_count_z,
    )
}
#[cfg(any(feature = "ext_device_group", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBaseKHR.html>"]
#[doc(alias = "vkCmdDispatchBaseKHR")]
#[inline]
pub unsafe fn cmd_dispatch_base_khr(
    command_buffer: &raw::CommandBuffer,
    base_group_x: u32,
    base_group_y: u32,
    base_group_z: u32,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_dispatch_base_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        base_group_x,
        base_group_y,
        base_group_z,
        group_count_x,
        group_count_y,
        group_count_z,
    )
}
#[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplate.html>"]
#[doc(alias = "vkCreateDescriptorUpdateTemplate")]
pub unsafe fn create_descriptor_update_template(
    device: &raw::Device,
    p_create_info: &DescriptorUpdateTemplateCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DescriptorUpdateTemplate> {
    let vulkan_command = dispatcher.create_descriptor_update_template.get();
    let mut p_descriptor_update_template = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_descriptor_update_template.as_mut_ptr(),
    );
    vk_status.map_success(|| p_descriptor_update_template.assume_init())
}
#[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplateKHR.html>"]
#[doc(alias = "vkCreateDescriptorUpdateTemplateKHR")]
pub unsafe fn create_descriptor_update_template_khr(
    device: &raw::Device,
    p_create_info: &DescriptorUpdateTemplateCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DescriptorUpdateTemplate> {
    let vulkan_command = dispatcher.create_descriptor_update_template_khr.get();
    let mut p_descriptor_update_template = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_descriptor_update_template.as_mut_ptr(),
    );
    vk_status.map_success(|| p_descriptor_update_template.assume_init())
}
#[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html>"]
#[doc(alias = "vkDestroyDescriptorUpdateTemplate")]
#[inline]
pub unsafe fn destroy_descriptor_update_template(
    device: &raw::Device,
    descriptor_update_template: Option<&raw::DescriptorUpdateTemplate>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_descriptor_update_template.get();
    vulkan_command(
        Some(device.borrow()),
        descriptor_update_template.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplateKHR.html>"]
#[doc(alias = "vkDestroyDescriptorUpdateTemplateKHR")]
#[inline]
pub unsafe fn destroy_descriptor_update_template_khr(
    device: &raw::Device,
    descriptor_update_template: Option<&raw::DescriptorUpdateTemplate>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_descriptor_update_template_khr.get();
    vulkan_command(
        Some(device.borrow()),
        descriptor_update_template.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html>"]
#[doc(alias = "vkUpdateDescriptorSetWithTemplate")]
#[inline]
pub unsafe fn update_descriptor_set_with_template(
    device: &raw::Device,
    descriptor_set: &raw::DescriptorSet,
    descriptor_update_template: &raw::DescriptorUpdateTemplate,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.update_descriptor_set_with_template.get();
    vulkan_command(
        Some(device.borrow()),
        Some(descriptor_set.borrow()),
        Some(descriptor_update_template.borrow()),
        p_data,
    )
}
#[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplateKHR.html>"]
#[doc(alias = "vkUpdateDescriptorSetWithTemplateKHR")]
#[inline]
pub unsafe fn update_descriptor_set_with_template_khr(
    device: &raw::Device,
    descriptor_set: &raw::DescriptorSet,
    descriptor_update_template: &raw::DescriptorUpdateTemplate,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.update_descriptor_set_with_template_khr.get();
    vulkan_command(
        Some(device.borrow()),
        Some(descriptor_set.borrow()),
        Some(descriptor_update_template.borrow()),
        p_data,
    )
}
#[cfg(any(feature = "ext_maintenance3", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupport.html>"]
#[doc(alias = "vkGetDescriptorSetLayoutSupport")]
pub unsafe fn get_descriptor_set_layout_support<
    S: StructureChainOut<DescriptorSetLayoutSupport<'static>>,
>(
    device: &raw::Device,
    p_create_info: &DescriptorSetLayoutCreateInfo,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_descriptor_set_layout_support.get();
    let mut p_support = MaybeUninit::uninit();
    S::setup_uninit(&mut p_support);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        S::get_uninit_head_ptr(p_support.as_mut_ptr()),
    );
    S::setup_cleanup(p_support.as_mut_ptr());
    p_support.assume_init()
}
#[cfg(any(feature = "ext_maintenance3", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupportKHR.html>"]
#[doc(alias = "vkGetDescriptorSetLayoutSupportKHR")]
pub unsafe fn get_descriptor_set_layout_support_khr<
    S: StructureChainOut<DescriptorSetLayoutSupport<'static>>,
>(
    device: &raw::Device,
    p_create_info: &DescriptorSetLayoutCreateInfo,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_descriptor_set_layout_support_khr.get();
    let mut p_support = MaybeUninit::uninit();
    S::setup_uninit(&mut p_support);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        S::get_uninit_head_ptr(p_support.as_mut_ptr()),
    );
    S::setup_cleanup(p_support.as_mut_ptr());
    p_support.assume_init()
}
#[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversion.html>"]
#[doc(alias = "vkCreateSamplerYcbcrConversion")]
pub unsafe fn create_sampler_ycbcr_conversion(
    device: &raw::Device,
    p_create_info: &SamplerYcbcrConversionCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SamplerYcbcrConversion> {
    let vulkan_command = dispatcher.create_sampler_ycbcr_conversion.get();
    let mut p_ycbcr_conversion = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_ycbcr_conversion.as_mut_ptr(),
    );
    vk_status.map_success(|| p_ycbcr_conversion.assume_init())
}
#[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversionKHR.html>"]
#[doc(alias = "vkCreateSamplerYcbcrConversionKHR")]
pub unsafe fn create_sampler_ycbcr_conversion_khr(
    device: &raw::Device,
    p_create_info: &SamplerYcbcrConversionCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SamplerYcbcrConversion> {
    let vulkan_command = dispatcher.create_sampler_ycbcr_conversion_khr.get();
    let mut p_ycbcr_conversion = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_ycbcr_conversion.as_mut_ptr(),
    );
    vk_status.map_success(|| p_ycbcr_conversion.assume_init())
}
#[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversion.html>"]
#[doc(alias = "vkDestroySamplerYcbcrConversion")]
#[inline]
pub unsafe fn destroy_sampler_ycbcr_conversion(
    device: &raw::Device,
    ycbcr_conversion: Option<&raw::SamplerYcbcrConversion>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_sampler_ycbcr_conversion.get();
    vulkan_command(
        Some(device.borrow()),
        ycbcr_conversion.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversionKHR.html>"]
#[doc(alias = "vkDestroySamplerYcbcrConversionKHR")]
#[inline]
pub unsafe fn destroy_sampler_ycbcr_conversion_khr(
    device: &raw::Device,
    ycbcr_conversion: Option<&raw::SamplerYcbcrConversion>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_sampler_ycbcr_conversion_khr.get();
    vulkan_command(
        Some(device.borrow()),
        ycbcr_conversion.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(any(feature = "ext_host_query_reset", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPool.html>"]
#[doc(alias = "vkResetQueryPool")]
#[inline]
pub unsafe fn reset_query_pool(
    device: &raw::Device,
    query_pool: &raw::QueryPool,
    first_query: u32,
    query_count: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.reset_query_pool.get();
    vulkan_command(
        Some(device.borrow()),
        Some(query_pool.borrow()),
        first_query,
        query_count,
    )
}
#[cfg(any(feature = "ext_host_query_reset", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPoolEXT.html>"]
#[doc(alias = "vkResetQueryPoolEXT")]
#[inline]
pub unsafe fn reset_query_pool_ext(
    device: &raw::Device,
    query_pool: &raw::QueryPool,
    first_query: u32,
    query_count: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.reset_query_pool_ext.get();
    vulkan_command(
        Some(device.borrow()),
        Some(query_pool.borrow()),
        first_query,
        query_count,
    )
}
#[cfg(any(feature = "ext_timeline_semaphore", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValue.html>"]
#[doc(alias = "vkGetSemaphoreCounterValue")]
pub unsafe fn get_semaphore_counter_value(
    device: &raw::Device,
    semaphore: &raw::Semaphore,
    dispatcher: &CommandsDispatcher,
) -> Result<u64> {
    let vulkan_command = dispatcher.get_semaphore_counter_value.get();
    let mut p_value = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(semaphore.borrow()),
        p_value.as_mut_ptr(),
    );
    vk_status.map_success(|| p_value.assume_init())
}
#[cfg(any(feature = "ext_timeline_semaphore", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValueKHR.html>"]
#[doc(alias = "vkGetSemaphoreCounterValueKHR")]
pub unsafe fn get_semaphore_counter_value_khr(
    device: &raw::Device,
    semaphore: &raw::Semaphore,
    dispatcher: &CommandsDispatcher,
) -> Result<u64> {
    let vulkan_command = dispatcher.get_semaphore_counter_value_khr.get();
    let mut p_value = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(semaphore.borrow()),
        p_value.as_mut_ptr(),
    );
    vk_status.map_success(|| p_value.assume_init())
}
#[cfg(any(feature = "ext_timeline_semaphore", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphores.html>"]
#[doc(alias = "vkWaitSemaphores")]
#[inline]
pub unsafe fn wait_semaphores(
    device: &raw::Device,
    p_wait_info: &SemaphoreWaitInfo,
    timeout: u64,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.wait_semaphores.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_wait_info), timeout).into_result()
}
#[cfg(any(feature = "ext_timeline_semaphore", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphoresKHR.html>"]
#[doc(alias = "vkWaitSemaphoresKHR")]
#[inline]
pub unsafe fn wait_semaphores_khr(
    device: &raw::Device,
    p_wait_info: &SemaphoreWaitInfo,
    timeout: u64,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.wait_semaphores_khr.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_wait_info), timeout).into_result()
}
#[cfg(any(feature = "ext_timeline_semaphore", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphore.html>"]
#[doc(alias = "vkSignalSemaphore")]
#[inline]
pub unsafe fn signal_semaphore(
    device: &raw::Device,
    p_signal_info: &SemaphoreSignalInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.signal_semaphore.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_signal_info)).map_success(|| ())
}
#[cfg(any(feature = "ext_timeline_semaphore", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphoreKHR.html>"]
#[doc(alias = "vkSignalSemaphoreKHR")]
#[inline]
pub unsafe fn signal_semaphore_khr(
    device: &raw::Device,
    p_signal_info: &SemaphoreSignalInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.signal_semaphore_khr.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_signal_info)).map_success(|| ())
}
#[cfg(any(feature = "ext_buffer_device_address", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html>"]
#[doc(alias = "vkGetBufferDeviceAddress")]
#[inline]
pub unsafe fn get_buffer_device_address(
    device: &raw::Device,
    p_info: &BufferDeviceAddressInfo,
    dispatcher: &CommandsDispatcher,
) -> DeviceAddress {
    let vulkan_command = dispatcher.get_buffer_device_address.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info))
}
#[cfg(any(feature = "ext_buffer_device_address", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressKHR.html>"]
#[doc(alias = "vkGetBufferDeviceAddressKHR")]
#[inline]
pub unsafe fn get_buffer_device_address_khr(
    device: &raw::Device,
    p_info: &BufferDeviceAddressInfo,
    dispatcher: &CommandsDispatcher,
) -> DeviceAddress {
    let vulkan_command = dispatcher.get_buffer_device_address_khr.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info))
}
#[cfg(any(feature = "ext_buffer_device_address", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressEXT.html>"]
#[doc(alias = "vkGetBufferDeviceAddressEXT")]
#[inline]
pub unsafe fn get_buffer_device_address_ext(
    device: &raw::Device,
    p_info: &BufferDeviceAddressInfo,
    dispatcher: &CommandsDispatcher,
) -> DeviceAddress {
    let vulkan_command = dispatcher.get_buffer_device_address_ext.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info))
}
#[cfg(any(feature = "ext_buffer_device_address", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddress.html>"]
#[doc(alias = "vkGetBufferOpaqueCaptureAddress")]
#[inline]
pub unsafe fn get_buffer_opaque_capture_address(
    device: &raw::Device,
    p_info: &BufferDeviceAddressInfo,
    dispatcher: &CommandsDispatcher,
) -> u64 {
    let vulkan_command = dispatcher.get_buffer_opaque_capture_address.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info))
}
#[cfg(any(feature = "ext_buffer_device_address", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddressKHR.html>"]
#[doc(alias = "vkGetBufferOpaqueCaptureAddressKHR")]
#[inline]
pub unsafe fn get_buffer_opaque_capture_address_khr(
    device: &raw::Device,
    p_info: &BufferDeviceAddressInfo,
    dispatcher: &CommandsDispatcher,
) -> u64 {
    let vulkan_command = dispatcher.get_buffer_opaque_capture_address_khr.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info))
}
#[cfg(any(feature = "ext_buffer_device_address", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddress.html>"]
#[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddress")]
#[inline]
pub unsafe fn get_device_memory_opaque_capture_address(
    device: &raw::Device,
    p_info: &DeviceMemoryOpaqueCaptureAddressInfo,
    dispatcher: &CommandsDispatcher,
) -> u64 {
    let vulkan_command = dispatcher.get_device_memory_opaque_capture_address.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info))
}
#[cfg(any(feature = "ext_buffer_device_address", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html>"]
#[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddressKHR")]
#[inline]
pub unsafe fn get_device_memory_opaque_capture_address_khr(
    device: &raw::Device,
    p_info: &DeviceMemoryOpaqueCaptureAddressInfo,
    dispatcher: &CommandsDispatcher,
) -> u64 {
    let vulkan_command = dispatcher
        .get_device_memory_opaque_capture_address_khr
        .get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info))
}
#[cfg(any(feature = "ext_draw_indirect_count", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html>"]
#[doc(alias = "vkCmdDrawIndirectCount")]
#[inline]
pub unsafe fn cmd_draw_indirect_count(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    count_buffer: &raw::Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_indirect_count.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(buffer.borrow()),
        offset,
        Some(count_buffer.borrow()),
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[cfg(any(feature = "ext_draw_indirect_count", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountKHR.html>"]
#[doc(alias = "vkCmdDrawIndirectCountKHR")]
#[inline]
pub unsafe fn cmd_draw_indirect_count_khr(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    count_buffer: &raw::Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_indirect_count_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(buffer.borrow()),
        offset,
        Some(count_buffer.borrow()),
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[cfg(any(feature = "ext_draw_indirect_count", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountAMD.html>"]
#[doc(alias = "vkCmdDrawIndirectCountAMD")]
#[inline]
pub unsafe fn cmd_draw_indirect_count_amd(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    count_buffer: &raw::Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_indirect_count_amd.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(buffer.borrow()),
        offset,
        Some(count_buffer.borrow()),
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[cfg(any(feature = "ext_draw_indirect_count", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html>"]
#[doc(alias = "vkCmdDrawIndexedIndirectCount")]
#[inline]
pub unsafe fn cmd_draw_indexed_indirect_count(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    count_buffer: &raw::Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_indexed_indirect_count.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(buffer.borrow()),
        offset,
        Some(count_buffer.borrow()),
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[cfg(any(feature = "ext_draw_indirect_count", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountKHR.html>"]
#[doc(alias = "vkCmdDrawIndexedIndirectCountKHR")]
#[inline]
pub unsafe fn cmd_draw_indexed_indirect_count_khr(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    count_buffer: &raw::Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_indexed_indirect_count_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(buffer.borrow()),
        offset,
        Some(count_buffer.borrow()),
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[cfg(any(feature = "ext_draw_indirect_count", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountAMD.html>"]
#[doc(alias = "vkCmdDrawIndexedIndirectCountAMD")]
#[inline]
pub unsafe fn cmd_draw_indexed_indirect_count_amd(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    count_buffer: &raw::Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_indexed_indirect_count_amd.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(buffer.borrow()),
        offset,
        Some(count_buffer.borrow()),
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[cfg(any(feature = "ext_create_renderpass2", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2.html>"]
#[doc(alias = "vkCreateRenderPass2")]
pub unsafe fn create_render_pass2(
    device: &raw::Device,
    p_create_info: &RenderPassCreateInfo2,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<RenderPass> {
    let vulkan_command = dispatcher.create_render_pass2.get();
    let mut p_render_pass = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_render_pass.as_mut_ptr(),
    );
    vk_status.map_success(|| p_render_pass.assume_init())
}
#[cfg(any(feature = "ext_create_renderpass2", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2KHR.html>"]
#[doc(alias = "vkCreateRenderPass2KHR")]
pub unsafe fn create_render_pass2_khr(
    device: &raw::Device,
    p_create_info: &RenderPassCreateInfo2,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<RenderPass> {
    let vulkan_command = dispatcher.create_render_pass2_khr.get();
    let mut p_render_pass = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_render_pass.as_mut_ptr(),
    );
    vk_status.map_success(|| p_render_pass.assume_init())
}
#[cfg(any(feature = "ext_create_renderpass2", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2.html>"]
#[doc(alias = "vkCmdBeginRenderPass2")]
#[inline]
pub unsafe fn cmd_begin_render_pass2(
    command_buffer: &raw::CommandBuffer,
    p_render_pass_begin: &RenderPassBeginInfo,
    p_subpass_begin_info: &SubpassBeginInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_begin_render_pass2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_render_pass_begin),
        ptr::from_ref(p_subpass_begin_info),
    )
}
#[cfg(any(feature = "ext_create_renderpass2", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2KHR.html>"]
#[doc(alias = "vkCmdBeginRenderPass2KHR")]
#[inline]
pub unsafe fn cmd_begin_render_pass2_khr(
    command_buffer: &raw::CommandBuffer,
    p_render_pass_begin: &RenderPassBeginInfo,
    p_subpass_begin_info: &SubpassBeginInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_begin_render_pass2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_render_pass_begin),
        ptr::from_ref(p_subpass_begin_info),
    )
}
#[cfg(any(feature = "ext_create_renderpass2", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2.html>"]
#[doc(alias = "vkCmdNextSubpass2")]
#[inline]
pub unsafe fn cmd_next_subpass2(
    command_buffer: &raw::CommandBuffer,
    p_subpass_begin_info: &SubpassBeginInfo,
    p_subpass_end_info: &SubpassEndInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_next_subpass2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_subpass_begin_info),
        ptr::from_ref(p_subpass_end_info),
    )
}
#[cfg(any(feature = "ext_create_renderpass2", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2KHR.html>"]
#[doc(alias = "vkCmdNextSubpass2KHR")]
#[inline]
pub unsafe fn cmd_next_subpass2_khr(
    command_buffer: &raw::CommandBuffer,
    p_subpass_begin_info: &SubpassBeginInfo,
    p_subpass_end_info: &SubpassEndInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_next_subpass2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_subpass_begin_info),
        ptr::from_ref(p_subpass_end_info),
    )
}
#[cfg(any(feature = "ext_create_renderpass2", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2.html>"]
#[doc(alias = "vkCmdEndRenderPass2")]
#[inline]
pub unsafe fn cmd_end_render_pass2(
    command_buffer: &raw::CommandBuffer,
    p_subpass_end_info: &SubpassEndInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_end_render_pass2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_subpass_end_info),
    )
}
#[cfg(any(feature = "ext_create_renderpass2", feature = "version_1_2"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2KHR.html>"]
#[doc(alias = "vkCmdEndRenderPass2KHR")]
#[inline]
pub unsafe fn cmd_end_render_pass2_khr(
    command_buffer: &raw::CommandBuffer,
    p_subpass_end_info: &SubpassEndInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_end_render_pass2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_subpass_end_info),
    )
}
#[cfg(any(feature = "ext_tooling_info", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolProperties.html>"]
#[doc(alias = "vkGetPhysicalDeviceToolProperties")]
pub unsafe fn get_physical_device_tool_properties<
    R: DynamicArray<PhysicalDeviceToolProperties<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_physical_device_tool_properties.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_tool_count = vk_len.as_mut_ptr();
    let p_tool_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_tool_count,
        p_tool_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_tool_count = ptr::from_mut(&mut vk_len);
    let mut p_tool_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_tool_count,
            p_tool_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_tool_count = ptr::from_mut(&mut vk_len);
        p_tool_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(any(feature = "ext_tooling_info", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolPropertiesEXT.html>"]
#[doc(alias = "vkGetPhysicalDeviceToolPropertiesEXT")]
pub unsafe fn get_physical_device_tool_properties_ext<
    R: DynamicArray<PhysicalDeviceToolProperties<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_physical_device_tool_properties_ext.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_tool_count = vk_len.as_mut_ptr();
    let p_tool_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_tool_count,
        p_tool_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_tool_count = ptr::from_mut(&mut vk_len);
    let mut p_tool_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_tool_count,
            p_tool_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_tool_count = ptr::from_mut(&mut vk_len);
        p_tool_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlot.html>"]
#[doc(alias = "vkCreatePrivateDataSlot")]
pub unsafe fn create_private_data_slot(
    device: &raw::Device,
    p_create_info: &PrivateDataSlotCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<PrivateDataSlot> {
    let vulkan_command = dispatcher.create_private_data_slot.get();
    let mut p_private_data_slot = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_private_data_slot.as_mut_ptr(),
    );
    vk_status.map_success(|| p_private_data_slot.assume_init())
}
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlotEXT.html>"]
#[doc(alias = "vkCreatePrivateDataSlotEXT")]
pub unsafe fn create_private_data_slot_ext(
    device: &raw::Device,
    p_create_info: &PrivateDataSlotCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<PrivateDataSlot> {
    let vulkan_command = dispatcher.create_private_data_slot_ext.get();
    let mut p_private_data_slot = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_private_data_slot.as_mut_ptr(),
    );
    vk_status.map_success(|| p_private_data_slot.assume_init())
}
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlot.html>"]
#[doc(alias = "vkDestroyPrivateDataSlot")]
#[inline]
pub unsafe fn destroy_private_data_slot(
    device: &raw::Device,
    private_data_slot: Option<&raw::PrivateDataSlot>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_private_data_slot.get();
    vulkan_command(
        Some(device.borrow()),
        private_data_slot.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlotEXT.html>"]
#[doc(alias = "vkDestroyPrivateDataSlotEXT")]
#[inline]
pub unsafe fn destroy_private_data_slot_ext(
    device: &raw::Device,
    private_data_slot: Option<&raw::PrivateDataSlot>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_private_data_slot_ext.get();
    vulkan_command(
        Some(device.borrow()),
        private_data_slot.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateData.html>"]
#[doc(alias = "vkSetPrivateData")]
#[inline]
pub unsafe fn set_private_data(
    device: &raw::Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: &raw::PrivateDataSlot,
    data: u64,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.set_private_data.get();
    vulkan_command(
        Some(device.borrow()),
        object_type,
        object_handle,
        Some(private_data_slot.borrow()),
        data,
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateDataEXT.html>"]
#[doc(alias = "vkSetPrivateDataEXT")]
#[inline]
pub unsafe fn set_private_data_ext(
    device: &raw::Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: &raw::PrivateDataSlot,
    data: u64,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.set_private_data_ext.get();
    vulkan_command(
        Some(device.borrow()),
        object_type,
        object_handle,
        Some(private_data_slot.borrow()),
        data,
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateData.html>"]
#[doc(alias = "vkGetPrivateData")]
pub unsafe fn get_private_data(
    device: &raw::Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: &raw::PrivateDataSlot,
    dispatcher: &CommandsDispatcher,
) -> u64 {
    let vulkan_command = dispatcher.get_private_data.get();
    let mut p_data = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        object_type,
        object_handle,
        Some(private_data_slot.borrow()),
        p_data.as_mut_ptr(),
    );
    p_data.assume_init()
}
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateDataEXT.html>"]
#[doc(alias = "vkGetPrivateDataEXT")]
pub unsafe fn get_private_data_ext(
    device: &raw::Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: &raw::PrivateDataSlot,
    dispatcher: &CommandsDispatcher,
) -> u64 {
    let vulkan_command = dispatcher.get_private_data_ext.get();
    let mut p_data = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        object_type,
        object_handle,
        Some(private_data_slot.borrow()),
        p_data.as_mut_ptr(),
    );
    p_data.assume_init()
}
#[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2.html>"]
#[doc(alias = "vkCmdPipelineBarrier2")]
#[inline]
pub unsafe fn cmd_pipeline_barrier2(
    command_buffer: &raw::CommandBuffer,
    p_dependency_info: &DependencyInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_pipeline_barrier2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_dependency_info),
    )
}
#[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2KHR.html>"]
#[doc(alias = "vkCmdPipelineBarrier2KHR")]
#[inline]
pub unsafe fn cmd_pipeline_barrier2_khr(
    command_buffer: &raw::CommandBuffer,
    p_dependency_info: &DependencyInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_pipeline_barrier2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_dependency_info),
    )
}
#[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2.html>"]
#[doc(alias = "vkCmdWriteTimestamp2")]
#[inline]
pub unsafe fn cmd_write_timestamp2(
    command_buffer: &raw::CommandBuffer,
    stage: PipelineStageFlags2,
    query_pool: &raw::QueryPool,
    query: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_write_timestamp2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        stage,
        Some(query_pool.borrow()),
        query,
    )
}
#[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2KHR.html>"]
#[doc(alias = "vkCmdWriteTimestamp2KHR")]
#[inline]
pub unsafe fn cmd_write_timestamp2_khr(
    command_buffer: &raw::CommandBuffer,
    stage: PipelineStageFlags2,
    query_pool: &raw::QueryPool,
    query: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_write_timestamp2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        stage,
        Some(query_pool.borrow()),
        query,
    )
}
#[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html>"]
#[doc(alias = "vkQueueSubmit2")]
#[inline]
pub unsafe fn queue_submit2<'a>(
    queue: &raw::Queue,
    p_submits: impl AsSlice<'a, SubmitInfo2<'a>>,
    fence: Option<&raw::Fence>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.queue_submit2.get();
    vulkan_command(
        Some(queue.borrow()),
        p_submits.as_slice().len() as _,
        p_submits.as_slice().as_ptr().cast(),
        fence.map(|v| v.borrow()),
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2KHR.html>"]
#[doc(alias = "vkQueueSubmit2KHR")]
#[inline]
pub unsafe fn queue_submit2_khr<'a>(
    queue: &raw::Queue,
    p_submits: impl AsSlice<'a, SubmitInfo2<'a>>,
    fence: Option<&raw::Fence>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.queue_submit2_khr.get();
    vulkan_command(
        Some(queue.borrow()),
        p_submits.as_slice().len() as _,
        p_submits.as_slice().as_ptr().cast(),
        fence.map(|v| v.borrow()),
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2.html>"]
#[doc(alias = "vkCmdCopyBuffer2")]
#[inline]
pub unsafe fn cmd_copy_buffer2(
    command_buffer: &raw::CommandBuffer,
    p_copy_buffer_info: &CopyBufferInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_buffer2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_copy_buffer_info),
    )
}
#[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2KHR.html>"]
#[doc(alias = "vkCmdCopyBuffer2KHR")]
#[inline]
pub unsafe fn cmd_copy_buffer2_khr(
    command_buffer: &raw::CommandBuffer,
    p_copy_buffer_info: &CopyBufferInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_buffer2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_copy_buffer_info),
    )
}
#[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2.html>"]
#[doc(alias = "vkCmdCopyImage2")]
#[inline]
pub unsafe fn cmd_copy_image2(
    command_buffer: &raw::CommandBuffer,
    p_copy_image_info: &CopyImageInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_image2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_copy_image_info),
    )
}
#[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2KHR.html>"]
#[doc(alias = "vkCmdCopyImage2KHR")]
#[inline]
pub unsafe fn cmd_copy_image2_khr(
    command_buffer: &raw::CommandBuffer,
    p_copy_image_info: &CopyImageInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_image2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_copy_image_info),
    )
}
#[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2.html>"]
#[doc(alias = "vkCmdCopyBufferToImage2")]
#[inline]
pub unsafe fn cmd_copy_buffer_to_image2(
    command_buffer: &raw::CommandBuffer,
    p_copy_buffer_to_image_info: &CopyBufferToImageInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_buffer_to_image2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_copy_buffer_to_image_info),
    )
}
#[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2KHR.html>"]
#[doc(alias = "vkCmdCopyBufferToImage2KHR")]
#[inline]
pub unsafe fn cmd_copy_buffer_to_image2_khr(
    command_buffer: &raw::CommandBuffer,
    p_copy_buffer_to_image_info: &CopyBufferToImageInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_buffer_to_image2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_copy_buffer_to_image_info),
    )
}
#[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2.html>"]
#[doc(alias = "vkCmdCopyImageToBuffer2")]
#[inline]
pub unsafe fn cmd_copy_image_to_buffer2(
    command_buffer: &raw::CommandBuffer,
    p_copy_image_to_buffer_info: &CopyImageToBufferInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_image_to_buffer2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_copy_image_to_buffer_info),
    )
}
#[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2KHR.html>"]
#[doc(alias = "vkCmdCopyImageToBuffer2KHR")]
#[inline]
pub unsafe fn cmd_copy_image_to_buffer2_khr(
    command_buffer: &raw::CommandBuffer,
    p_copy_image_to_buffer_info: &CopyImageToBufferInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_image_to_buffer2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_copy_image_to_buffer_info),
    )
}
#[cfg(any(feature = "ext_maintenance4", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirements.html>"]
#[doc(alias = "vkGetDeviceBufferMemoryRequirements")]
pub unsafe fn get_device_buffer_memory_requirements<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &DeviceBufferMemoryRequirements,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_device_buffer_memory_requirements.get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[cfg(any(feature = "ext_maintenance4", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirementsKHR.html>"]
#[doc(alias = "vkGetDeviceBufferMemoryRequirementsKHR")]
pub unsafe fn get_device_buffer_memory_requirements_khr<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &DeviceBufferMemoryRequirements,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_device_buffer_memory_requirements_khr.get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[cfg(any(feature = "ext_maintenance4", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirements.html>"]
#[doc(alias = "vkGetDeviceImageMemoryRequirements")]
pub unsafe fn get_device_image_memory_requirements<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &DeviceImageMemoryRequirements,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_device_image_memory_requirements.get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[cfg(any(feature = "ext_maintenance4", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirementsKHR.html>"]
#[doc(alias = "vkGetDeviceImageMemoryRequirementsKHR")]
pub unsafe fn get_device_image_memory_requirements_khr<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &DeviceImageMemoryRequirements,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_device_image_memory_requirements_khr.get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[cfg(any(feature = "ext_maintenance4", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirements.html>"]
#[doc(alias = "vkGetDeviceImageSparseMemoryRequirements")]
pub unsafe fn get_device_image_sparse_memory_requirements<
    R: DynamicArray<SparseImageMemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &DeviceImageMemoryRequirements,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher.get_device_image_sparse_memory_requirements.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_sparse_memory_requirement_count = vk_len.as_mut_ptr();
    let p_sparse_memory_requirements = ptr::null_mut();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_sparse_memory_requirement_count = ptr::from_mut(&mut vk_len);
    let mut p_sparse_memory_requirements = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[cfg(any(feature = "ext_maintenance4", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirementsKHR.html>"]
#[doc(alias = "vkGetDeviceImageSparseMemoryRequirementsKHR")]
pub unsafe fn get_device_image_sparse_memory_requirements_khr<
    R: DynamicArray<SparseImageMemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &DeviceImageMemoryRequirements,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher
        .get_device_image_sparse_memory_requirements_khr
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_sparse_memory_requirement_count = vk_len.as_mut_ptr();
    let p_sparse_memory_requirements = ptr::null_mut();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_sparse_memory_requirement_count = ptr::from_mut(&mut vk_len);
    let mut p_sparse_memory_requirements = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2.html>"]
#[doc(alias = "vkCmdSetEvent2")]
#[inline]
pub unsafe fn cmd_set_event2(
    command_buffer: &raw::CommandBuffer,
    event: &raw::Event,
    p_dependency_info: &DependencyInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_event2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(event.borrow()),
        ptr::from_ref(p_dependency_info),
    )
}
#[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2KHR.html>"]
#[doc(alias = "vkCmdSetEvent2KHR")]
#[inline]
pub unsafe fn cmd_set_event2_khr(
    command_buffer: &raw::CommandBuffer,
    event: &raw::Event,
    p_dependency_info: &DependencyInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_event2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(event.borrow()),
        ptr::from_ref(p_dependency_info),
    )
}
#[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2.html>"]
#[doc(alias = "vkCmdResetEvent2")]
#[inline]
pub unsafe fn cmd_reset_event2(
    command_buffer: &raw::CommandBuffer,
    event: &raw::Event,
    stage_mask: PipelineStageFlags2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_reset_event2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(event.borrow()),
        stage_mask,
    )
}
#[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2KHR.html>"]
#[doc(alias = "vkCmdResetEvent2KHR")]
#[inline]
pub unsafe fn cmd_reset_event2_khr(
    command_buffer: &raw::CommandBuffer,
    event: &raw::Event,
    stage_mask: PipelineStageFlags2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_reset_event2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(event.borrow()),
        stage_mask,
    )
}
#[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2.html>"]
#[doc(alias = "vkCmdWaitEvents2")]
#[inline]
pub unsafe fn cmd_wait_events2<'a, V2: Alias<raw::Event> + 'a>(
    command_buffer: &raw::CommandBuffer,
    p_events: impl AsSlice<'a, V2>,
    p_dependency_infos: impl AsSlice<'a, DependencyInfo<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_wait_events2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_dependency_infos.as_slice().len() as _,
        p_events.as_slice().as_ptr().cast(),
        p_dependency_infos.as_slice().as_ptr().cast(),
    )
}
#[cfg(any(feature = "ext_synchronization2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2KHR.html>"]
#[doc(alias = "vkCmdWaitEvents2KHR")]
#[inline]
pub unsafe fn cmd_wait_events2_khr<'a, V2: Alias<raw::Event> + 'a>(
    command_buffer: &raw::CommandBuffer,
    p_events: impl AsSlice<'a, V2>,
    p_dependency_infos: impl AsSlice<'a, DependencyInfo<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_wait_events2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_dependency_infos.as_slice().len() as _,
        p_events.as_slice().as_ptr().cast(),
        p_dependency_infos.as_slice().as_ptr().cast(),
    )
}
#[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html>"]
#[doc(alias = "vkCmdBlitImage2")]
#[inline]
pub unsafe fn cmd_blit_image2(
    command_buffer: &raw::CommandBuffer,
    p_blit_image_info: &BlitImageInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_blit_image2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_blit_image_info),
    )
}
#[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2KHR.html>"]
#[doc(alias = "vkCmdBlitImage2KHR")]
#[inline]
pub unsafe fn cmd_blit_image2_khr(
    command_buffer: &raw::CommandBuffer,
    p_blit_image_info: &BlitImageInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_blit_image2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_blit_image_info),
    )
}
#[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2.html>"]
#[doc(alias = "vkCmdResolveImage2")]
#[inline]
pub unsafe fn cmd_resolve_image2(
    command_buffer: &raw::CommandBuffer,
    p_resolve_image_info: &ResolveImageInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_resolve_image2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_resolve_image_info),
    )
}
#[cfg(any(feature = "ext_copy_commands2", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2KHR.html>"]
#[doc(alias = "vkCmdResolveImage2KHR")]
#[inline]
pub unsafe fn cmd_resolve_image2_khr(
    command_buffer: &raw::CommandBuffer,
    p_resolve_image_info: &ResolveImageInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_resolve_image2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_resolve_image_info),
    )
}
#[cfg(any(feature = "ext_dynamic_rendering", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRendering.html>"]
#[doc(alias = "vkCmdBeginRendering")]
#[inline]
pub unsafe fn cmd_begin_rendering(
    command_buffer: &raw::CommandBuffer,
    p_rendering_info: &RenderingInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_begin_rendering.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_rendering_info),
    )
}
#[cfg(any(feature = "ext_dynamic_rendering", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderingKHR.html>"]
#[doc(alias = "vkCmdBeginRenderingKHR")]
#[inline]
pub unsafe fn cmd_begin_rendering_khr(
    command_buffer: &raw::CommandBuffer,
    p_rendering_info: &RenderingInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_begin_rendering_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_rendering_info),
    )
}
#[cfg(any(feature = "ext_dynamic_rendering", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering.html>"]
#[doc(alias = "vkCmdEndRendering")]
#[inline]
pub unsafe fn cmd_end_rendering(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_end_rendering.get();
    vulkan_command(Some(command_buffer.borrow()))
}
#[cfg(any(feature = "ext_dynamic_rendering", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderingKHR.html>"]
#[doc(alias = "vkCmdEndRenderingKHR")]
#[inline]
pub unsafe fn cmd_end_rendering_khr(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_end_rendering_khr.get();
    vulkan_command(Some(command_buffer.borrow()))
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullMode.html>"]
#[doc(alias = "vkCmdSetCullMode")]
#[inline]
pub unsafe fn cmd_set_cull_mode(
    command_buffer: &raw::CommandBuffer,
    cull_mode: CullModeFlags,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_cull_mode.get();
    vulkan_command(Some(command_buffer.borrow()), cull_mode)
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullModeEXT.html>"]
#[doc(alias = "vkCmdSetCullModeEXT")]
#[inline]
pub unsafe fn cmd_set_cull_mode_ext(
    command_buffer: &raw::CommandBuffer,
    cull_mode: CullModeFlags,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_cull_mode_ext.get();
    vulkan_command(Some(command_buffer.borrow()), cull_mode)
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFace.html>"]
#[doc(alias = "vkCmdSetFrontFace")]
#[inline]
pub unsafe fn cmd_set_front_face(
    command_buffer: &raw::CommandBuffer,
    front_face: FrontFace,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_front_face.get();
    vulkan_command(Some(command_buffer.borrow()), front_face)
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFaceEXT.html>"]
#[doc(alias = "vkCmdSetFrontFaceEXT")]
#[inline]
pub unsafe fn cmd_set_front_face_ext(
    command_buffer: &raw::CommandBuffer,
    front_face: FrontFace,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_front_face_ext.get();
    vulkan_command(Some(command_buffer.borrow()), front_face)
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopology.html>"]
#[doc(alias = "vkCmdSetPrimitiveTopology")]
#[inline]
pub unsafe fn cmd_set_primitive_topology(
    command_buffer: &raw::CommandBuffer,
    primitive_topology: PrimitiveTopology,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_primitive_topology.get();
    vulkan_command(Some(command_buffer.borrow()), primitive_topology)
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopologyEXT.html>"]
#[doc(alias = "vkCmdSetPrimitiveTopologyEXT")]
#[inline]
pub unsafe fn cmd_set_primitive_topology_ext(
    command_buffer: &raw::CommandBuffer,
    primitive_topology: PrimitiveTopology,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_primitive_topology_ext.get();
    vulkan_command(Some(command_buffer.borrow()), primitive_topology)
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html>"]
#[doc(alias = "vkCmdSetViewportWithCount")]
#[inline]
pub unsafe fn cmd_set_viewport_with_count<'a>(
    command_buffer: &raw::CommandBuffer,
    p_viewports: impl AsSlice<'a, Viewport>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_viewport_with_count.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_viewports.as_slice().len() as _,
        p_viewports.as_slice().as_ptr().cast(),
    )
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCountEXT.html>"]
#[doc(alias = "vkCmdSetViewportWithCountEXT")]
#[inline]
pub unsafe fn cmd_set_viewport_with_count_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    p_viewports: impl AsSlice<'a, Viewport>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_viewport_with_count_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_viewports.as_slice().len() as _,
        p_viewports.as_slice().as_ptr().cast(),
    )
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html>"]
#[doc(alias = "vkCmdSetScissorWithCount")]
#[inline]
pub unsafe fn cmd_set_scissor_with_count<'a>(
    command_buffer: &raw::CommandBuffer,
    p_scissors: impl AsSlice<'a, Rect2D>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_scissor_with_count.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_scissors.as_slice().len() as _,
        p_scissors.as_slice().as_ptr().cast(),
    )
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCountEXT.html>"]
#[doc(alias = "vkCmdSetScissorWithCountEXT")]
#[inline]
pub unsafe fn cmd_set_scissor_with_count_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    p_scissors: impl AsSlice<'a, Rect2D>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_scissor_with_count_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_scissors.as_slice().len() as _,
        p_scissors.as_slice().as_ptr().cast(),
    )
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html>"]
#[doc(alias = "vkCmdBindVertexBuffers2")]
#[inline]
pub unsafe fn cmd_bind_vertex_buffers2<'a, V3: Alias<raw::Buffer> + 'a>(
    command_buffer: &raw::CommandBuffer,
    first_binding: u32,
    p_buffers: impl AsSlice<'a, V3>,
    p_offsets: impl AsSlice<'a, DeviceSize>,
    p_sizes: Option<impl AsSlice<'a, DeviceSize>>,
    p_strides: Option<impl AsSlice<'a, DeviceSize>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_vertex_buffers2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_binding,
        p_offsets.as_slice().len() as _,
        p_buffers.as_slice().as_ptr().cast(),
        p_offsets.as_slice().as_ptr().cast(),
        p_sizes
            .map(|p| p.as_slice().as_ptr().cast())
            .unwrap_or(ptr::null()),
        p_strides
            .map(|p| p.as_slice().as_ptr().cast())
            .unwrap_or(ptr::null()),
    )
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2EXT.html>"]
#[doc(alias = "vkCmdBindVertexBuffers2EXT")]
#[inline]
pub unsafe fn cmd_bind_vertex_buffers2_ext<'a, V3: Alias<raw::Buffer> + 'a>(
    command_buffer: &raw::CommandBuffer,
    first_binding: u32,
    p_buffers: impl AsSlice<'a, V3>,
    p_offsets: impl AsSlice<'a, DeviceSize>,
    p_sizes: Option<impl AsSlice<'a, DeviceSize>>,
    p_strides: Option<impl AsSlice<'a, DeviceSize>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_vertex_buffers2_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_binding,
        p_offsets.as_slice().len() as _,
        p_buffers.as_slice().as_ptr().cast(),
        p_offsets.as_slice().as_ptr().cast(),
        p_sizes
            .map(|p| p.as_slice().as_ptr().cast())
            .unwrap_or(ptr::null()),
        p_strides
            .map(|p| p.as_slice().as_ptr().cast())
            .unwrap_or(ptr::null()),
    )
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnable.html>"]
#[doc(alias = "vkCmdSetDepthTestEnable")]
#[inline]
pub unsafe fn cmd_set_depth_test_enable(
    command_buffer: &raw::CommandBuffer,
    depth_test_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_test_enable.get();
    vulkan_command(Some(command_buffer.borrow()), depth_test_enable.into())
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnableEXT.html>"]
#[doc(alias = "vkCmdSetDepthTestEnableEXT")]
#[inline]
pub unsafe fn cmd_set_depth_test_enable_ext(
    command_buffer: &raw::CommandBuffer,
    depth_test_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_test_enable_ext.get();
    vulkan_command(Some(command_buffer.borrow()), depth_test_enable.into())
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnable.html>"]
#[doc(alias = "vkCmdSetDepthWriteEnable")]
#[inline]
pub unsafe fn cmd_set_depth_write_enable(
    command_buffer: &raw::CommandBuffer,
    depth_write_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_write_enable.get();
    vulkan_command(Some(command_buffer.borrow()), depth_write_enable.into())
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnableEXT.html>"]
#[doc(alias = "vkCmdSetDepthWriteEnableEXT")]
#[inline]
pub unsafe fn cmd_set_depth_write_enable_ext(
    command_buffer: &raw::CommandBuffer,
    depth_write_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_write_enable_ext.get();
    vulkan_command(Some(command_buffer.borrow()), depth_write_enable.into())
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOp.html>"]
#[doc(alias = "vkCmdSetDepthCompareOp")]
#[inline]
pub unsafe fn cmd_set_depth_compare_op(
    command_buffer: &raw::CommandBuffer,
    depth_compare_op: CompareOp,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_compare_op.get();
    vulkan_command(Some(command_buffer.borrow()), depth_compare_op)
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOpEXT.html>"]
#[doc(alias = "vkCmdSetDepthCompareOpEXT")]
#[inline]
pub unsafe fn cmd_set_depth_compare_op_ext(
    command_buffer: &raw::CommandBuffer,
    depth_compare_op: CompareOp,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_compare_op_ext.get();
    vulkan_command(Some(command_buffer.borrow()), depth_compare_op)
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnable.html>"]
#[doc(alias = "vkCmdSetDepthBoundsTestEnable")]
#[inline]
pub unsafe fn cmd_set_depth_bounds_test_enable(
    command_buffer: &raw::CommandBuffer,
    depth_bounds_test_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_bounds_test_enable.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        depth_bounds_test_enable.into(),
    )
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnableEXT.html>"]
#[doc(alias = "vkCmdSetDepthBoundsTestEnableEXT")]
#[inline]
pub unsafe fn cmd_set_depth_bounds_test_enable_ext(
    command_buffer: &raw::CommandBuffer,
    depth_bounds_test_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_bounds_test_enable_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        depth_bounds_test_enable.into(),
    )
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnable.html>"]
#[doc(alias = "vkCmdSetStencilTestEnable")]
#[inline]
pub unsafe fn cmd_set_stencil_test_enable(
    command_buffer: &raw::CommandBuffer,
    stencil_test_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_stencil_test_enable.get();
    vulkan_command(Some(command_buffer.borrow()), stencil_test_enable.into())
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnableEXT.html>"]
#[doc(alias = "vkCmdSetStencilTestEnableEXT")]
#[inline]
pub unsafe fn cmd_set_stencil_test_enable_ext(
    command_buffer: &raw::CommandBuffer,
    stencil_test_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_stencil_test_enable_ext.get();
    vulkan_command(Some(command_buffer.borrow()), stencil_test_enable.into())
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOp.html>"]
#[doc(alias = "vkCmdSetStencilOp")]
#[inline]
pub unsafe fn cmd_set_stencil_op(
    command_buffer: &raw::CommandBuffer,
    face_mask: StencilFaceFlags,
    fail_op: StencilOp,
    pass_op: StencilOp,
    depth_fail_op: StencilOp,
    compare_op: CompareOp,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_stencil_op.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        face_mask,
        fail_op,
        pass_op,
        depth_fail_op,
        compare_op,
    )
}
#[cfg(any(
    feature = "ext_extended_dynamic_state",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOpEXT.html>"]
#[doc(alias = "vkCmdSetStencilOpEXT")]
#[inline]
pub unsafe fn cmd_set_stencil_op_ext(
    command_buffer: &raw::CommandBuffer,
    face_mask: StencilFaceFlags,
    fail_op: StencilOp,
    pass_op: StencilOp,
    depth_fail_op: StencilOp,
    compare_op: CompareOp,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_stencil_op_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        face_mask,
        fail_op,
        pass_op,
        depth_fail_op,
        compare_op,
    )
}
#[cfg(any(
    feature = "ext_extended_dynamic_state2",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnable.html>"]
#[doc(alias = "vkCmdSetRasterizerDiscardEnable")]
#[inline]
pub unsafe fn cmd_set_rasterizer_discard_enable(
    command_buffer: &raw::CommandBuffer,
    rasterizer_discard_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_rasterizer_discard_enable.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        rasterizer_discard_enable.into(),
    )
}
#[cfg(any(
    feature = "ext_extended_dynamic_state2",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnableEXT.html>"]
#[doc(alias = "vkCmdSetRasterizerDiscardEnableEXT")]
#[inline]
pub unsafe fn cmd_set_rasterizer_discard_enable_ext(
    command_buffer: &raw::CommandBuffer,
    rasterizer_discard_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_rasterizer_discard_enable_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        rasterizer_discard_enable.into(),
    )
}
#[cfg(any(
    feature = "ext_extended_dynamic_state2",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnable.html>"]
#[doc(alias = "vkCmdSetDepthBiasEnable")]
#[inline]
pub unsafe fn cmd_set_depth_bias_enable(
    command_buffer: &raw::CommandBuffer,
    depth_bias_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_bias_enable.get();
    vulkan_command(Some(command_buffer.borrow()), depth_bias_enable.into())
}
#[cfg(any(
    feature = "ext_extended_dynamic_state2",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnableEXT.html>"]
#[doc(alias = "vkCmdSetDepthBiasEnableEXT")]
#[inline]
pub unsafe fn cmd_set_depth_bias_enable_ext(
    command_buffer: &raw::CommandBuffer,
    depth_bias_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_bias_enable_ext.get();
    vulkan_command(Some(command_buffer.borrow()), depth_bias_enable.into())
}
#[cfg(any(
    feature = "ext_extended_dynamic_state2",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnable.html>"]
#[doc(alias = "vkCmdSetPrimitiveRestartEnable")]
#[inline]
pub unsafe fn cmd_set_primitive_restart_enable(
    command_buffer: &raw::CommandBuffer,
    primitive_restart_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_primitive_restart_enable.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        primitive_restart_enable.into(),
    )
}
#[cfg(any(
    feature = "ext_extended_dynamic_state2",
    feature = "ext_shader_object",
    feature = "version_1_3"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnableEXT.html>"]
#[doc(alias = "vkCmdSetPrimitiveRestartEnableEXT")]
#[inline]
pub unsafe fn cmd_set_primitive_restart_enable_ext(
    command_buffer: &raw::CommandBuffer,
    primitive_restart_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_primitive_restart_enable_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        primitive_restart_enable.into(),
    )
}
#[cfg(any(feature = "ext_map_memory2", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2.html>"]
#[doc(alias = "vkMapMemory2")]
pub unsafe fn map_memory2(
    device: &raw::Device,
    p_memory_map_info: &MemoryMapInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher.map_memory2.get();
    let mut pp_data = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_memory_map_info),
        pp_data.as_mut_ptr(),
    );
    vk_status.map_success(|| pp_data.assume_init())
}
#[cfg(any(feature = "ext_map_memory2", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2KHR.html>"]
#[doc(alias = "vkMapMemory2KHR")]
pub unsafe fn map_memory2_khr(
    device: &raw::Device,
    p_memory_map_info: &MemoryMapInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher.map_memory2_khr.get();
    let mut pp_data = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_memory_map_info),
        pp_data.as_mut_ptr(),
    );
    vk_status.map_success(|| pp_data.assume_init())
}
#[cfg(any(feature = "ext_map_memory2", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2.html>"]
#[doc(alias = "vkUnmapMemory2")]
#[inline]
pub unsafe fn unmap_memory2(
    device: &raw::Device,
    p_memory_unmap_info: &MemoryUnmapInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.unmap_memory2.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_memory_unmap_info)).map_success(|| ())
}
#[cfg(any(feature = "ext_map_memory2", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2KHR.html>"]
#[doc(alias = "vkUnmapMemory2KHR")]
#[inline]
pub unsafe fn unmap_memory2_khr(
    device: &raw::Device,
    p_memory_unmap_info: &MemoryUnmapInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.unmap_memory2_khr.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_memory_unmap_info)).map_success(|| ())
}
#[cfg(any(feature = "ext_maintenance5", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayout.html>"]
#[doc(alias = "vkGetDeviceImageSubresourceLayout")]
pub unsafe fn get_device_image_subresource_layout<
    S: StructureChainOut<SubresourceLayout2<'static>>,
>(
    device: &raw::Device,
    p_info: &DeviceImageSubresourceInfo,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_device_image_subresource_layout.get();
    let mut p_layout = MaybeUninit::uninit();
    S::setup_uninit(&mut p_layout);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_layout.as_mut_ptr()),
    );
    S::setup_cleanup(p_layout.as_mut_ptr());
    p_layout.assume_init()
}
#[cfg(any(feature = "ext_maintenance5", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayoutKHR.html>"]
#[doc(alias = "vkGetDeviceImageSubresourceLayoutKHR")]
pub unsafe fn get_device_image_subresource_layout_khr<
    S: StructureChainOut<SubresourceLayout2<'static>>,
>(
    device: &raw::Device,
    p_info: &DeviceImageSubresourceInfo,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_device_image_subresource_layout_khr.get();
    let mut p_layout = MaybeUninit::uninit();
    S::setup_uninit(&mut p_layout);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_layout.as_mut_ptr()),
    );
    S::setup_cleanup(p_layout.as_mut_ptr());
    p_layout.assume_init()
}
#[cfg(any(
    feature = "ext_host_image_copy",
    feature = "ext_image_compression_control",
    feature = "ext_maintenance5",
    feature = "version_1_4"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html>"]
#[doc(alias = "vkGetImageSubresourceLayout2")]
pub unsafe fn get_image_subresource_layout2<S: StructureChainOut<SubresourceLayout2<'static>>>(
    device: &raw::Device,
    image: &raw::Image,
    p_subresource: &ImageSubresource2,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_image_subresource_layout2.get();
    let mut p_layout = MaybeUninit::uninit();
    S::setup_uninit(&mut p_layout);
    vulkan_command(
        Some(device.borrow()),
        Some(image.borrow()),
        ptr::from_ref(p_subresource),
        S::get_uninit_head_ptr(p_layout.as_mut_ptr()),
    );
    S::setup_cleanup(p_layout.as_mut_ptr());
    p_layout.assume_init()
}
#[cfg(any(
    feature = "ext_host_image_copy",
    feature = "ext_image_compression_control",
    feature = "ext_maintenance5",
    feature = "version_1_4"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2KHR.html>"]
#[doc(alias = "vkGetImageSubresourceLayout2KHR")]
pub unsafe fn get_image_subresource_layout2_khr<
    S: StructureChainOut<SubresourceLayout2<'static>>,
>(
    device: &raw::Device,
    image: &raw::Image,
    p_subresource: &ImageSubresource2,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_image_subresource_layout2_khr.get();
    let mut p_layout = MaybeUninit::uninit();
    S::setup_uninit(&mut p_layout);
    vulkan_command(
        Some(device.borrow()),
        Some(image.borrow()),
        ptr::from_ref(p_subresource),
        S::get_uninit_head_ptr(p_layout.as_mut_ptr()),
    );
    S::setup_cleanup(p_layout.as_mut_ptr());
    p_layout.assume_init()
}
#[cfg(any(
    feature = "ext_host_image_copy",
    feature = "ext_image_compression_control",
    feature = "ext_maintenance5",
    feature = "version_1_4"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2EXT.html>"]
#[doc(alias = "vkGetImageSubresourceLayout2EXT")]
pub unsafe fn get_image_subresource_layout2_ext<
    S: StructureChainOut<SubresourceLayout2<'static>>,
>(
    device: &raw::Device,
    image: &raw::Image,
    p_subresource: &ImageSubresource2,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_image_subresource_layout2_ext.get();
    let mut p_layout = MaybeUninit::uninit();
    S::setup_uninit(&mut p_layout);
    vulkan_command(
        Some(device.borrow()),
        Some(image.borrow()),
        ptr::from_ref(p_subresource),
        S::get_uninit_head_ptr(p_layout.as_mut_ptr()),
    );
    S::setup_cleanup(p_layout.as_mut_ptr());
    p_layout.assume_init()
}
#[cfg(any(feature = "ext_host_image_copy", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImage.html>"]
#[doc(alias = "vkCopyMemoryToImage")]
#[inline]
pub unsafe fn copy_memory_to_image(
    device: &raw::Device,
    p_copy_memory_to_image_info: &CopyMemoryToImageInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.copy_memory_to_image.get();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_copy_memory_to_image_info),
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_host_image_copy", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImageEXT.html>"]
#[doc(alias = "vkCopyMemoryToImageEXT")]
#[inline]
pub unsafe fn copy_memory_to_image_ext(
    device: &raw::Device,
    p_copy_memory_to_image_info: &CopyMemoryToImageInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.copy_memory_to_image_ext.get();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_copy_memory_to_image_info),
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_host_image_copy", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemory.html>"]
#[doc(alias = "vkCopyImageToMemory")]
#[inline]
pub unsafe fn copy_image_to_memory(
    device: &raw::Device,
    p_copy_image_to_memory_info: &CopyImageToMemoryInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.copy_image_to_memory.get();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_copy_image_to_memory_info),
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_host_image_copy", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemoryEXT.html>"]
#[doc(alias = "vkCopyImageToMemoryEXT")]
#[inline]
pub unsafe fn copy_image_to_memory_ext(
    device: &raw::Device,
    p_copy_image_to_memory_info: &CopyImageToMemoryInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.copy_image_to_memory_ext.get();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_copy_image_to_memory_info),
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_host_image_copy", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImage.html>"]
#[doc(alias = "vkCopyImageToImage")]
#[inline]
pub unsafe fn copy_image_to_image(
    device: &raw::Device,
    p_copy_image_to_image_info: &CopyImageToImageInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.copy_image_to_image.get();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_copy_image_to_image_info),
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_host_image_copy", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImageEXT.html>"]
#[doc(alias = "vkCopyImageToImageEXT")]
#[inline]
pub unsafe fn copy_image_to_image_ext(
    device: &raw::Device,
    p_copy_image_to_image_info: &CopyImageToImageInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.copy_image_to_image_ext.get();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_copy_image_to_image_info),
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_host_image_copy", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayout.html>"]
#[doc(alias = "vkTransitionImageLayout")]
#[inline]
pub unsafe fn transition_image_layout<'a>(
    device: &raw::Device,
    p_transitions: impl AsSlice<'a, HostImageLayoutTransitionInfo<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.transition_image_layout.get();
    vulkan_command(
        Some(device.borrow()),
        p_transitions.as_slice().len() as _,
        p_transitions.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_host_image_copy", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayoutEXT.html>"]
#[doc(alias = "vkTransitionImageLayoutEXT")]
#[inline]
pub unsafe fn transition_image_layout_ext<'a>(
    device: &raw::Device,
    p_transitions: impl AsSlice<'a, HostImageLayoutTransitionInfo<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.transition_image_layout_ext.get();
    vulkan_command(
        Some(device.borrow()),
        p_transitions.as_slice().len() as _,
        p_transitions.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_push_descriptor", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet.html>"]
#[doc(alias = "vkCmdPushDescriptorSet")]
#[inline]
pub unsafe fn cmd_push_descriptor_set<'a>(
    command_buffer: &raw::CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: &raw::PipelineLayout,
    set: u32,
    p_descriptor_writes: impl AsSlice<'a, WriteDescriptorSet<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_push_descriptor_set.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        pipeline_bind_point,
        Some(layout.borrow()),
        set,
        p_descriptor_writes.as_slice().len() as _,
        p_descriptor_writes.as_slice().as_ptr().cast(),
    )
}
#[cfg(any(feature = "ext_push_descriptor", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetKHR.html>"]
#[doc(alias = "vkCmdPushDescriptorSetKHR")]
#[inline]
pub unsafe fn cmd_push_descriptor_set_khr<'a>(
    command_buffer: &raw::CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: &raw::PipelineLayout,
    set: u32,
    p_descriptor_writes: impl AsSlice<'a, WriteDescriptorSet<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_push_descriptor_set_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        pipeline_bind_point,
        Some(layout.borrow()),
        set,
        p_descriptor_writes.as_slice().len() as _,
        p_descriptor_writes.as_slice().as_ptr().cast(),
    )
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
pub unsafe fn cmd_push_descriptor_set_with_template(
    command_buffer: &raw::CommandBuffer,
    descriptor_update_template: &raw::DescriptorUpdateTemplate,
    layout: &raw::PipelineLayout,
    set: u32,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_push_descriptor_set_with_template.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(descriptor_update_template.borrow()),
        Some(layout.borrow()),
        set,
        p_data,
    )
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
pub unsafe fn cmd_push_descriptor_set_with_template_khr(
    command_buffer: &raw::CommandBuffer,
    descriptor_update_template: &raw::DescriptorUpdateTemplate,
    layout: &raw::PipelineLayout,
    set: u32,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_push_descriptor_set_with_template_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(descriptor_update_template.borrow()),
        Some(layout.borrow()),
        set,
        p_data,
    )
}
#[cfg(any(feature = "ext_maintenance6", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html>"]
#[doc(alias = "vkCmdBindDescriptorSets2")]
#[inline]
pub unsafe fn cmd_bind_descriptor_sets2(
    command_buffer: &raw::CommandBuffer,
    p_bind_descriptor_sets_info: &BindDescriptorSetsInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_descriptor_sets2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_bind_descriptor_sets_info),
    )
}
#[cfg(any(feature = "ext_maintenance6", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2KHR.html>"]
#[doc(alias = "vkCmdBindDescriptorSets2KHR")]
#[inline]
pub unsafe fn cmd_bind_descriptor_sets2_khr(
    command_buffer: &raw::CommandBuffer,
    p_bind_descriptor_sets_info: &BindDescriptorSetsInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_descriptor_sets2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_bind_descriptor_sets_info),
    )
}
#[cfg(any(feature = "ext_maintenance6", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2.html>"]
#[doc(alias = "vkCmdPushConstants2")]
#[inline]
pub unsafe fn cmd_push_constants2(
    command_buffer: &raw::CommandBuffer,
    p_push_constants_info: &PushConstantsInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_push_constants2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_push_constants_info),
    )
}
#[cfg(any(feature = "ext_maintenance6", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2KHR.html>"]
#[doc(alias = "vkCmdPushConstants2KHR")]
#[inline]
pub unsafe fn cmd_push_constants2_khr(
    command_buffer: &raw::CommandBuffer,
    p_push_constants_info: &PushConstantsInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_push_constants2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_push_constants_info),
    )
}
#[cfg(any(
    all(feature = "ext_maintenance6", feature = "ext_push_descriptor"),
    feature = "version_1_4"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html>"]
#[doc(alias = "vkCmdPushDescriptorSet2")]
#[inline]
pub unsafe fn cmd_push_descriptor_set2(
    command_buffer: &raw::CommandBuffer,
    p_push_descriptor_set_info: &PushDescriptorSetInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_push_descriptor_set2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_push_descriptor_set_info),
    )
}
#[cfg(any(
    all(feature = "ext_maintenance6", feature = "ext_push_descriptor"),
    feature = "version_1_4"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2KHR.html>"]
#[doc(alias = "vkCmdPushDescriptorSet2KHR")]
#[inline]
pub unsafe fn cmd_push_descriptor_set2_khr(
    command_buffer: &raw::CommandBuffer,
    p_push_descriptor_set_info: &PushDescriptorSetInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_push_descriptor_set2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_push_descriptor_set_info),
    )
}
#[cfg(any(
    all(feature = "ext_maintenance6", feature = "ext_push_descriptor"),
    feature = "version_1_4"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2.html>"]
#[doc(alias = "vkCmdPushDescriptorSetWithTemplate2")]
#[inline]
pub unsafe fn cmd_push_descriptor_set_with_template2(
    command_buffer: &raw::CommandBuffer,
    p_push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_push_descriptor_set_with_template2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_push_descriptor_set_with_template_info),
    )
}
#[cfg(any(
    all(feature = "ext_maintenance6", feature = "ext_push_descriptor"),
    feature = "version_1_4"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2KHR.html>"]
#[doc(alias = "vkCmdPushDescriptorSetWithTemplate2KHR")]
#[inline]
pub unsafe fn cmd_push_descriptor_set_with_template2_khr(
    command_buffer: &raw::CommandBuffer,
    p_push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_push_descriptor_set_with_template2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_push_descriptor_set_with_template_info),
    )
}
#[cfg(any(feature = "ext_line_rasterization", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html>"]
#[doc(alias = "vkCmdSetLineStipple")]
#[inline]
pub unsafe fn cmd_set_line_stipple(
    command_buffer: &raw::CommandBuffer,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_line_stipple.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        line_stipple_factor,
        line_stipple_pattern,
    )
}
#[cfg(any(feature = "ext_line_rasterization", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleKHR.html>"]
#[doc(alias = "vkCmdSetLineStippleKHR")]
#[inline]
pub unsafe fn cmd_set_line_stipple_khr(
    command_buffer: &raw::CommandBuffer,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_line_stipple_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        line_stipple_factor,
        line_stipple_pattern,
    )
}
#[cfg(any(feature = "ext_line_rasterization", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEXT.html>"]
#[doc(alias = "vkCmdSetLineStippleEXT")]
#[inline]
pub unsafe fn cmd_set_line_stipple_ext(
    command_buffer: &raw::CommandBuffer,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_line_stipple_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        line_stipple_factor,
        line_stipple_pattern,
    )
}
#[cfg(any(feature = "ext_maintenance5", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2.html>"]
#[doc(alias = "vkCmdBindIndexBuffer2")]
#[inline]
pub unsafe fn cmd_bind_index_buffer2(
    command_buffer: &raw::CommandBuffer,
    buffer: Option<&raw::Buffer>,
    offset: DeviceSize,
    size: DeviceSize,
    index_type: IndexType,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_index_buffer2.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        buffer.map(|v| v.borrow()),
        offset,
        size,
        index_type,
    )
}
#[cfg(any(feature = "ext_maintenance5", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2KHR.html>"]
#[doc(alias = "vkCmdBindIndexBuffer2KHR")]
#[inline]
pub unsafe fn cmd_bind_index_buffer2_khr(
    command_buffer: &raw::CommandBuffer,
    buffer: Option<&raw::Buffer>,
    offset: DeviceSize,
    size: DeviceSize,
    index_type: IndexType,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_index_buffer2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        buffer.map(|v| v.borrow()),
        offset,
        size,
        index_type,
    )
}
#[cfg(any(feature = "ext_maintenance5", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularity.html>"]
#[doc(alias = "vkGetRenderingAreaGranularity")]
pub unsafe fn get_rendering_area_granularity(
    device: &raw::Device,
    p_rendering_area_info: &RenderingAreaInfo,
    dispatcher: &CommandsDispatcher,
) -> Extent2D {
    let vulkan_command = dispatcher.get_rendering_area_granularity.get();
    let mut p_granularity = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_rendering_area_info),
        p_granularity.as_mut_ptr(),
    );
    p_granularity.assume_init()
}
#[cfg(any(feature = "ext_maintenance5", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularityKHR.html>"]
#[doc(alias = "vkGetRenderingAreaGranularityKHR")]
pub unsafe fn get_rendering_area_granularity_khr(
    device: &raw::Device,
    p_rendering_area_info: &RenderingAreaInfo,
    dispatcher: &CommandsDispatcher,
) -> Extent2D {
    let vulkan_command = dispatcher.get_rendering_area_granularity_khr.get();
    let mut p_granularity = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_rendering_area_info),
        p_granularity.as_mut_ptr(),
    );
    p_granularity.assume_init()
}
#[cfg(any(feature = "ext_dynamic_rendering_local_read", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html>"]
#[doc(alias = "vkCmdSetRenderingAttachmentLocations")]
#[inline]
pub unsafe fn cmd_set_rendering_attachment_locations(
    command_buffer: &raw::CommandBuffer,
    p_location_info: &RenderingAttachmentLocationInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_rendering_attachment_locations.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_location_info),
    )
}
#[cfg(any(feature = "ext_dynamic_rendering_local_read", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocationsKHR.html>"]
#[doc(alias = "vkCmdSetRenderingAttachmentLocationsKHR")]
#[inline]
pub unsafe fn cmd_set_rendering_attachment_locations_khr(
    command_buffer: &raw::CommandBuffer,
    p_location_info: &RenderingAttachmentLocationInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_rendering_attachment_locations_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_location_info),
    )
}
#[cfg(any(feature = "ext_dynamic_rendering_local_read", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html>"]
#[doc(alias = "vkCmdSetRenderingInputAttachmentIndices")]
#[inline]
pub unsafe fn cmd_set_rendering_input_attachment_indices(
    command_buffer: &raw::CommandBuffer,
    p_input_attachment_index_info: &RenderingInputAttachmentIndexInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_rendering_input_attachment_indices.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_input_attachment_index_info),
    )
}
#[cfg(any(feature = "ext_dynamic_rendering_local_read", feature = "version_1_4"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndicesKHR.html>"]
#[doc(alias = "vkCmdSetRenderingInputAttachmentIndicesKHR")]
#[inline]
pub unsafe fn cmd_set_rendering_input_attachment_indices_khr(
    command_buffer: &raw::CommandBuffer,
    p_input_attachment_index_info: &RenderingInputAttachmentIndexInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_rendering_input_attachment_indices_khr
        .get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_input_attachment_index_info),
    )
}
#[cfg(feature = "ext_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySurfaceKHR.html>"]
#[doc(alias = "vkDestroySurfaceKHR")]
#[inline]
pub unsafe fn destroy_surface_khr(
    instance: &raw::Instance,
    surface: Option<&raw::SurfaceKHR>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_surface_khr.get();
    vulkan_command(
        Some(instance.borrow()),
        surface.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceSupportKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
pub unsafe fn get_physical_device_surface_support_khr(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    surface: &raw::SurfaceKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<bool> {
    let vulkan_command = dispatcher.get_physical_device_surface_support_khr.get();
    let mut p_supported = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(physical_device.borrow()),
        queue_family_index,
        Some(surface.borrow()),
        p_supported.as_mut_ptr(),
    );
    vk_status.map_success(|| p_supported.assume_init().into())
}
#[cfg(feature = "ext_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
pub unsafe fn get_physical_device_surface_capabilities_khr(
    physical_device: &raw::PhysicalDevice,
    surface: &raw::SurfaceKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceCapabilitiesKHR> {
    let vulkan_command = dispatcher
        .get_physical_device_surface_capabilities_khr
        .get();
    let mut p_surface_capabilities = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(physical_device.borrow()),
        Some(surface.borrow()),
        p_surface_capabilities.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface_capabilities.assume_init())
}
#[cfg(feature = "ext_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormatsKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
pub unsafe fn get_physical_device_surface_formats_khr<R: DynamicArray<SurfaceFormatKHR>>(
    physical_device: &raw::PhysicalDevice,
    surface: Option<&raw::SurfaceKHR>,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_physical_device_surface_formats_khr.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_surface_format_count = vk_len.as_mut_ptr();
    let p_surface_formats = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        surface.map(|v| v.borrow()),
        p_surface_format_count,
        p_surface_formats,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_surface_format_count = ptr::from_mut(&mut vk_len);
    let mut p_surface_formats = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            surface.map(|v| v.borrow()),
            p_surface_format_count,
            p_surface_formats,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_surface_format_count = ptr::from_mut(&mut vk_len);
        p_surface_formats = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
pub unsafe fn get_physical_device_surface_present_modes_khr<R: DynamicArray<PresentModeKHR>>(
    physical_device: &raw::PhysicalDevice,
    surface: Option<&raw::SurfaceKHR>,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_surface_present_modes_khr
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_present_mode_count = vk_len.as_mut_ptr();
    let p_present_modes = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        surface.map(|v| v.borrow()),
        p_present_mode_count,
        p_present_modes,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_present_mode_count = ptr::from_mut(&mut vk_len);
    let mut p_present_modes = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            surface.map(|v| v.borrow()),
            p_present_mode_count,
            p_present_modes,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_present_mode_count = ptr::from_mut(&mut vk_len);
        p_present_modes = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_swapchain")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSwapchainKHR.html>"]
#[doc(alias = "vkCreateSwapchainKHR")]
pub unsafe fn create_swapchain_khr(
    device: &raw::Device,
    p_create_info: &SwapchainCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SwapchainKHR> {
    let vulkan_command = dispatcher.create_swapchain_khr.get();
    let mut p_swapchain = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_swapchain.as_mut_ptr(),
    );
    vk_status.map_success(|| p_swapchain.assume_init())
}
#[cfg(feature = "ext_swapchain")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySwapchainKHR.html>"]
#[doc(alias = "vkDestroySwapchainKHR")]
#[inline]
pub unsafe fn destroy_swapchain_khr(
    device: &raw::Device,
    swapchain: Option<&raw::SwapchainKHR>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_swapchain_khr.get();
    vulkan_command(
        Some(device.borrow()),
        swapchain.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_swapchain")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainImagesKHR.html>"]
#[doc(alias = "vkGetSwapchainImagesKHR")]
pub unsafe fn get_swapchain_images_khr<R: DynamicArray<Image>>(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_swapchain_images_khr.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_swapchain_image_count = vk_len.as_mut_ptr();
    let p_swapchain_images = ptr::null_mut();
    vulkan_command(
        Some(device.borrow()),
        Some(swapchain.borrow()),
        p_swapchain_image_count,
        p_swapchain_images,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_swapchain_image_count = ptr::from_mut(&mut vk_len);
    let mut p_swapchain_images = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(device.borrow()),
            Some(swapchain.borrow()),
            p_swapchain_image_count,
            p_swapchain_images,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_swapchain_image_count = ptr::from_mut(&mut vk_len);
        p_swapchain_images = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_swapchain")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImageKHR.html>"]
#[doc(alias = "vkAcquireNextImageKHR")]
pub unsafe fn acquire_next_image_khr(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    timeout: u64,
    semaphore: Option<&raw::Semaphore>,
    fence: Option<&raw::Fence>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, u32)> {
    let vulkan_command = dispatcher.acquire_next_image_khr.get();
    let mut p_image_index = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(swapchain.borrow()),
        timeout,
        semaphore.map(|v| v.borrow()),
        fence.map(|v| v.borrow()),
        p_image_index.as_mut_ptr(),
    );
    vk_status.map_successes(|| p_image_index.assume_init())
}
#[cfg(feature = "ext_swapchain")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueuePresentKHR.html>"]
#[doc(alias = "vkQueuePresentKHR")]
#[inline]
pub unsafe fn queue_present_khr(
    queue: &raw::Queue,
    p_present_info: &PresentInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.queue_present_khr.get();
    vulkan_command(Some(queue.borrow()), ptr::from_ref(p_present_info)).into_result()
}
#[cfg(any(
    all(feature = "ext_swapchain", feature = "version_1_1"),
    all(feature = "ext_device_group", feature = "ext_surface")
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPresentCapabilitiesKHR.html>"]
#[doc(alias = "vkGetDeviceGroupPresentCapabilitiesKHR")]
pub unsafe fn get_device_group_present_capabilities_khr<
    S: StructureChainOut<DeviceGroupPresentCapabilitiesKHR<'static>>,
>(
    device: &raw::Device,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.get_device_group_present_capabilities_khr.get();
    let mut p_device_group_present_capabilities = MaybeUninit::uninit();
    S::setup_uninit(&mut p_device_group_present_capabilities);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        S::get_uninit_head_ptr(p_device_group_present_capabilities.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_device_group_present_capabilities.as_mut_ptr());
        p_device_group_present_capabilities.assume_init()
    })
}
#[cfg(any(
    all(feature = "ext_swapchain", feature = "version_1_1"),
    all(feature = "ext_device_group", feature = "ext_surface")
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModesKHR.html>"]
#[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
pub unsafe fn get_device_group_surface_present_modes_khr(
    device: &raw::Device,
    surface: &raw::SurfaceKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<DeviceGroupPresentModeFlagsKHR> {
    let vulkan_command = dispatcher.get_device_group_surface_present_modes_khr.get();
    let mut p_modes = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(surface.borrow()),
        p_modes.as_mut_ptr(),
    );
    vk_status.map_success(|| p_modes.assume_init())
}
#[cfg(any(
    all(feature = "ext_swapchain", feature = "version_1_1"),
    all(feature = "ext_device_group", feature = "ext_surface")
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDevicePresentRectanglesKHR.html>"]
#[doc(alias = "vkGetPhysicalDevicePresentRectanglesKHR")]
pub unsafe fn get_physical_device_present_rectangles_khr<R: DynamicArray<Rect2D>>(
    physical_device: &raw::PhysicalDevice,
    surface: &raw::SurfaceKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_physical_device_present_rectangles_khr.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_rect_count = vk_len.as_mut_ptr();
    let p_rects = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        Some(surface.borrow()),
        p_rect_count,
        p_rects,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_rect_count = ptr::from_mut(&mut vk_len);
    let mut p_rects = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            Some(surface.borrow()),
            p_rect_count,
            p_rects,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_rect_count = ptr::from_mut(&mut vk_len);
        p_rects = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(any(
    all(feature = "ext_swapchain", feature = "version_1_1"),
    all(feature = "ext_device_group", feature = "ext_swapchain")
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImage2KHR.html>"]
#[doc(alias = "vkAcquireNextImage2KHR")]
pub unsafe fn acquire_next_image2_khr(
    device: &raw::Device,
    p_acquire_info: &AcquireNextImageInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, u32)> {
    let vulkan_command = dispatcher.acquire_next_image2_khr.get();
    let mut p_image_index = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_acquire_info),
        p_image_index.as_mut_ptr(),
    );
    vk_status.map_successes(|| p_image_index.assume_init())
}
#[cfg(feature = "ext_display")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPropertiesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceDisplayPropertiesKHR")]
pub unsafe fn get_physical_device_display_properties_khr<
    R: DynamicArray<DisplayPropertiesKHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_physical_device_display_properties_khr.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_property_count,
        p_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_property_count,
            p_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_property_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_display")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")]
pub unsafe fn get_physical_device_display_plane_properties_khr<
    R: DynamicArray<DisplayPlanePropertiesKHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_display_plane_properties_khr
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_property_count,
        p_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_property_count,
            p_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_property_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_display")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneSupportedDisplaysKHR.html>"]
#[doc(alias = "vkGetDisplayPlaneSupportedDisplaysKHR")]
pub unsafe fn get_display_plane_supported_displays_khr<R: DynamicArray<DisplayKHR>>(
    physical_device: &raw::PhysicalDevice,
    plane_index: u32,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_display_plane_supported_displays_khr.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_display_count = vk_len.as_mut_ptr();
    let p_displays = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        plane_index,
        p_display_count,
        p_displays,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_display_count = ptr::from_mut(&mut vk_len);
    let mut p_displays = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            plane_index,
            p_display_count,
            p_displays,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_display_count = ptr::from_mut(&mut vk_len);
        p_displays = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_display")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModePropertiesKHR.html>"]
#[doc(alias = "vkGetDisplayModePropertiesKHR")]
pub unsafe fn get_display_mode_properties_khr<
    R: DynamicArray<DisplayModePropertiesKHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    display: &raw::DisplayKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_display_mode_properties_khr.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        Some(display.borrow()),
        p_property_count,
        p_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            Some(display.borrow()),
            p_property_count,
            p_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_property_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_display")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayModeKHR.html>"]
#[doc(alias = "vkCreateDisplayModeKHR")]
pub unsafe fn create_display_mode_khr(
    physical_device: &raw::PhysicalDevice,
    display: &raw::DisplayKHR,
    p_create_info: &DisplayModeCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DisplayModeKHR> {
    let vulkan_command = dispatcher.create_display_mode_khr.get();
    let mut p_mode = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(physical_device.borrow()),
        Some(display.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_mode.as_mut_ptr(),
    );
    vk_status.map_success(|| p_mode.assume_init())
}
#[cfg(feature = "ext_display")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilitiesKHR.html>"]
#[doc(alias = "vkGetDisplayPlaneCapabilitiesKHR")]
pub unsafe fn get_display_plane_capabilities_khr(
    physical_device: &raw::PhysicalDevice,
    mode: &raw::DisplayModeKHR,
    plane_index: u32,
    dispatcher: &CommandsDispatcher,
) -> Result<DisplayPlaneCapabilitiesKHR> {
    let vulkan_command = dispatcher.get_display_plane_capabilities_khr.get();
    let mut p_capabilities = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(physical_device.borrow()),
        Some(mode.borrow()),
        plane_index,
        p_capabilities.as_mut_ptr(),
    );
    vk_status.map_success(|| p_capabilities.assume_init())
}
#[cfg(feature = "ext_display")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayPlaneSurfaceKHR.html>"]
#[doc(alias = "vkCreateDisplayPlaneSurfaceKHR")]
pub unsafe fn create_display_plane_surface_khr(
    instance: &raw::Instance,
    p_create_info: &DisplaySurfaceCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_display_plane_surface_khr.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_display_swapchain")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSharedSwapchainsKHR.html>"]
#[doc(alias = "vkCreateSharedSwapchainsKHR")]
pub unsafe fn create_shared_swapchains_khr<'a, R: DynamicArray<SwapchainKHR>>(
    device: &raw::Device,
    p_create_infos: impl AsSlice<'a, SwapchainCreateInfoKHR<'a>>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.create_shared_swapchains_khr.get();
    let mut p_swapchains = R::create_with_capacity(p_create_infos.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        p_create_infos.as_slice().len() as _,
        p_create_infos.as_slice().as_ptr().cast(),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_swapchains.get_content_mut_ptr(),
    );
    vk_status.map_success(|| {
        p_swapchains.resize_with_len(p_create_infos.as_slice().len() as _);
        p_swapchains
    })
}
#[cfg(feature = "ext_xlib_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXlibSurfaceKHR.html>"]
#[doc(alias = "vkCreateXlibSurfaceKHR")]
pub unsafe fn create_xlib_surface_khr(
    instance: &raw::Instance,
    p_create_info: &XlibSurfaceCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_xlib_surface_khr.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_xlib_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXlibPresentationSupportKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
#[inline]
pub unsafe fn get_physical_device_xlib_presentation_support_khr(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    dpy: &VoidPtr,
    visual_id: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> bool {
    let vulkan_command = dispatcher
        .get_physical_device_xlib_presentation_support_khr
        .get();
    vulkan_command(
        Some(physical_device.borrow()),
        queue_family_index,
        ptr::from_ref(dpy),
        visual_id,
    )
    .into()
}
#[cfg(feature = "ext_xcb_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXcbSurfaceKHR.html>"]
#[doc(alias = "vkCreateXcbSurfaceKHR")]
pub unsafe fn create_xcb_surface_khr(
    instance: &raw::Instance,
    p_create_info: &XcbSurfaceCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_xcb_surface_khr.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_xcb_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXcbPresentationSupportKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceXcbPresentationSupportKHR")]
#[inline]
pub unsafe fn get_physical_device_xcb_presentation_support_khr(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    connection: &VoidPtr,
    visualid: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> bool {
    let vulkan_command = dispatcher
        .get_physical_device_xcb_presentation_support_khr
        .get();
    vulkan_command(
        Some(physical_device.borrow()),
        queue_family_index,
        ptr::from_ref(connection),
        visualid,
    )
    .into()
}
#[cfg(feature = "ext_wayland_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWaylandSurfaceKHR.html>"]
#[doc(alias = "vkCreateWaylandSurfaceKHR")]
pub unsafe fn create_wayland_surface_khr(
    instance: &raw::Instance,
    p_create_info: &WaylandSurfaceCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_wayland_surface_khr.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_wayland_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
#[inline]
pub unsafe fn get_physical_device_wayland_presentation_support_khr(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    display: &VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> bool {
    let vulkan_command = dispatcher
        .get_physical_device_wayland_presentation_support_khr
        .get();
    vulkan_command(
        Some(physical_device.borrow()),
        queue_family_index,
        ptr::from_ref(display),
    )
    .into()
}
#[cfg(feature = "ext_android_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAndroidSurfaceKHR.html>"]
#[doc(alias = "vkCreateAndroidSurfaceKHR")]
pub unsafe fn create_android_surface_khr(
    instance: &raw::Instance,
    p_create_info: &AndroidSurfaceCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_android_surface_khr.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_win32_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWin32SurfaceKHR.html>"]
#[doc(alias = "vkCreateWin32SurfaceKHR")]
pub unsafe fn create_win32_surface_khr(
    instance: &raw::Instance,
    p_create_info: &Win32SurfaceCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_win32_surface_khr.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_win32_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWin32PresentationSupportKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceWin32PresentationSupportKHR")]
#[inline]
pub unsafe fn get_physical_device_win32_presentation_support_khr(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    dispatcher: &CommandsDispatcher,
) -> bool {
    let vulkan_command = dispatcher
        .get_physical_device_win32_presentation_support_khr
        .get();
    vulkan_command(Some(physical_device.borrow()), queue_family_index).into()
}
#[cfg(feature = "ext_debug_report")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugReportCallbackEXT.html>"]
#[doc(alias = "vkCreateDebugReportCallbackEXT")]
pub unsafe fn create_debug_report_callback_ext(
    instance: &raw::Instance,
    p_create_info: &DebugReportCallbackCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DebugReportCallbackEXT> {
    let vulkan_command = dispatcher.create_debug_report_callback_ext.get();
    let mut p_callback = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_callback.as_mut_ptr(),
    );
    vk_status.map_success(|| p_callback.assume_init())
}
#[cfg(feature = "ext_debug_report")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugReportCallbackEXT.html>"]
#[doc(alias = "vkDestroyDebugReportCallbackEXT")]
#[inline]
pub unsafe fn destroy_debug_report_callback_ext(
    instance: &raw::Instance,
    callback: Option<&raw::DebugReportCallbackEXT>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_debug_report_callback_ext.get();
    vulkan_command(
        Some(instance.borrow()),
        callback.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_debug_report")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugReportMessageEXT.html>"]
#[doc(alias = "vkDebugReportMessageEXT")]
#[inline]
pub unsafe fn debug_report_message_ext(
    instance: &raw::Instance,
    flags: DebugReportFlagsEXT,
    object_type: DebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    p_layer_prefix: &CStr,
    p_message: &CStr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.debug_report_message_ext.get();
    vulkan_command(
        Some(instance.borrow()),
        flags,
        object_type,
        object,
        location,
        message_code,
        p_layer_prefix.as_ptr(),
        p_message.as_ptr(),
    )
}
#[cfg(feature = "ext_debug_marker")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectTagEXT.html>"]
#[doc(alias = "vkDebugMarkerSetObjectTagEXT")]
#[inline]
pub unsafe fn debug_marker_set_object_tag_ext(
    device: &raw::Device,
    p_tag_info: &DebugMarkerObjectTagInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.debug_marker_set_object_tag_ext.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_tag_info)).map_success(|| ())
}
#[cfg(feature = "ext_debug_marker")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectNameEXT.html>"]
#[doc(alias = "vkDebugMarkerSetObjectNameEXT")]
#[inline]
pub unsafe fn debug_marker_set_object_name_ext(
    device: &raw::Device,
    p_name_info: &DebugMarkerObjectNameInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.debug_marker_set_object_name_ext.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_name_info)).map_success(|| ())
}
#[cfg(feature = "ext_debug_marker")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerBeginEXT.html>"]
#[doc(alias = "vkCmdDebugMarkerBeginEXT")]
#[inline]
pub unsafe fn cmd_debug_marker_begin_ext(
    command_buffer: &raw::CommandBuffer,
    p_marker_info: &DebugMarkerMarkerInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_debug_marker_begin_ext.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_marker_info))
}
#[cfg(feature = "ext_debug_marker")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerEndEXT.html>"]
#[doc(alias = "vkCmdDebugMarkerEndEXT")]
#[inline]
pub unsafe fn cmd_debug_marker_end_ext(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_debug_marker_end_ext.get();
    vulkan_command(Some(command_buffer.borrow()))
}
#[cfg(feature = "ext_debug_marker")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerInsertEXT.html>"]
#[doc(alias = "vkCmdDebugMarkerInsertEXT")]
#[inline]
pub unsafe fn cmd_debug_marker_insert_ext(
    command_buffer: &raw::CommandBuffer,
    p_marker_info: &DebugMarkerMarkerInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_debug_marker_insert_ext.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_marker_info))
}
#[cfg(feature = "ext_transform_feedback")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffersEXT.html>"]
#[doc(alias = "vkCmdBindTransformFeedbackBuffersEXT")]
#[inline]
pub unsafe fn cmd_bind_transform_feedback_buffers_ext<'a, V3: Alias<raw::Buffer> + 'a>(
    command_buffer: &raw::CommandBuffer,
    first_binding: u32,
    p_buffers: impl AsSlice<'a, V3>,
    p_offsets: impl AsSlice<'a, DeviceSize>,
    p_sizes: Option<impl AsSlice<'a, DeviceSize>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_transform_feedback_buffers_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_binding,
        p_offsets.as_slice().len() as _,
        p_buffers.as_slice().as_ptr().cast(),
        p_offsets.as_slice().as_ptr().cast(),
        p_sizes
            .map(|p| p.as_slice().as_ptr().cast())
            .unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_transform_feedback")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedbackEXT.html>"]
#[doc(alias = "vkCmdBeginTransformFeedbackEXT")]
#[inline]
pub unsafe fn cmd_begin_transform_feedback_ext<'a, V3: Alias<raw::Buffer> + 'a>(
    command_buffer: &raw::CommandBuffer,
    first_counter_buffer: u32,
    p_counter_buffers: impl AsSlice<'a, V3>,
    p_counter_buffer_offsets: Option<impl AsSlice<'a, DeviceSize>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_begin_transform_feedback_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_counter_buffer,
        p_counter_buffers.as_slice().len() as _,
        p_counter_buffers.as_slice().as_ptr().cast(),
        p_counter_buffer_offsets
            .map(|p| p.as_slice().as_ptr().cast())
            .unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_transform_feedback")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedbackEXT.html>"]
#[doc(alias = "vkCmdEndTransformFeedbackEXT")]
#[inline]
pub unsafe fn cmd_end_transform_feedback_ext<'a, V3: Alias<raw::Buffer> + 'a>(
    command_buffer: &raw::CommandBuffer,
    first_counter_buffer: u32,
    p_counter_buffers: impl AsSlice<'a, V3>,
    p_counter_buffer_offsets: Option<impl AsSlice<'a, DeviceSize>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_end_transform_feedback_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_counter_buffer,
        p_counter_buffers.as_slice().len() as _,
        p_counter_buffers.as_slice().as_ptr().cast(),
        p_counter_buffer_offsets
            .map(|p| p.as_slice().as_ptr().cast())
            .unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_transform_feedback")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQueryIndexedEXT.html>"]
#[doc(alias = "vkCmdBeginQueryIndexedEXT")]
#[inline]
pub unsafe fn cmd_begin_query_indexed_ext(
    command_buffer: &raw::CommandBuffer,
    query_pool: &raw::QueryPool,
    query: u32,
    flags: QueryControlFlags,
    index: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_begin_query_indexed_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(query_pool.borrow()),
        query,
        flags,
        index,
    )
}
#[cfg(feature = "ext_transform_feedback")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQueryIndexedEXT.html>"]
#[doc(alias = "vkCmdEndQueryIndexedEXT")]
#[inline]
pub unsafe fn cmd_end_query_indexed_ext(
    command_buffer: &raw::CommandBuffer,
    query_pool: &raw::QueryPool,
    query: u32,
    index: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_end_query_indexed_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(query_pool.borrow()),
        query,
        index,
    )
}
#[cfg(feature = "ext_transform_feedback")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCountEXT.html>"]
#[doc(alias = "vkCmdDrawIndirectByteCountEXT")]
#[inline]
pub unsafe fn cmd_draw_indirect_byte_count_ext(
    command_buffer: &raw::CommandBuffer,
    instance_count: u32,
    first_instance: u32,
    counter_buffer: &raw::Buffer,
    counter_buffer_offset: DeviceSize,
    counter_offset: u32,
    vertex_stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_indirect_byte_count_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        instance_count,
        first_instance,
        Some(counter_buffer.borrow()),
        counter_buffer_offset,
        counter_offset,
        vertex_stride,
    )
}
#[cfg(feature = "ext_binary_import")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuModuleNVX.html>"]
#[doc(alias = "vkCreateCuModuleNVX")]
pub unsafe fn create_cu_module_nvx(
    device: &raw::Device,
    p_create_info: &CuModuleCreateInfoNVX,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<CuModuleNVX> {
    let vulkan_command = dispatcher.create_cu_module_nvx.get();
    let mut p_module = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_module.as_mut_ptr(),
    );
    vk_status.map_success(|| p_module.assume_init())
}
#[cfg(feature = "ext_binary_import")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuFunctionNVX.html>"]
#[doc(alias = "vkCreateCuFunctionNVX")]
pub unsafe fn create_cu_function_nvx(
    device: &raw::Device,
    p_create_info: &CuFunctionCreateInfoNVX,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<CuFunctionNVX> {
    let vulkan_command = dispatcher.create_cu_function_nvx.get();
    let mut p_function = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_function.as_mut_ptr(),
    );
    vk_status.map_success(|| p_function.assume_init())
}
#[cfg(feature = "ext_binary_import")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuModuleNVX.html>"]
#[doc(alias = "vkDestroyCuModuleNVX")]
#[inline]
pub unsafe fn destroy_cu_module_nvx(
    device: &raw::Device,
    module: &raw::CuModuleNVX,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_cu_module_nvx.get();
    vulkan_command(
        Some(device.borrow()),
        Some(module.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_binary_import")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuFunctionNVX.html>"]
#[doc(alias = "vkDestroyCuFunctionNVX")]
#[inline]
pub unsafe fn destroy_cu_function_nvx(
    device: &raw::Device,
    function: &raw::CuFunctionNVX,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_cu_function_nvx.get();
    vulkan_command(
        Some(device.borrow()),
        Some(function.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_binary_import")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCuLaunchKernelNVX.html>"]
#[doc(alias = "vkCmdCuLaunchKernelNVX")]
#[inline]
pub unsafe fn cmd_cu_launch_kernel_nvx(
    command_buffer: &raw::CommandBuffer,
    p_launch_info: &CuLaunchInfoNVX,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_cu_launch_kernel_nvx.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_launch_info))
}
#[cfg(feature = "ext_image_view_handle")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandleNVX.html>"]
#[doc(alias = "vkGetImageViewHandleNVX")]
#[inline]
pub unsafe fn get_image_view_handle_nvx(
    device: &raw::Device,
    p_info: &ImageViewHandleInfoNVX,
    dispatcher: &CommandsDispatcher,
) -> u32 {
    let vulkan_command = dispatcher.get_image_view_handle_nvx.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info))
}
#[cfg(feature = "ext_image_view_handle")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandle64NVX.html>"]
#[doc(alias = "vkGetImageViewHandle64NVX")]
#[inline]
pub unsafe fn get_image_view_handle64_nvx(
    device: &raw::Device,
    p_info: &ImageViewHandleInfoNVX,
    dispatcher: &CommandsDispatcher,
) -> u64 {
    let vulkan_command = dispatcher.get_image_view_handle64_nvx.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info))
}
#[cfg(feature = "ext_image_view_handle")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewAddressNVX.html>"]
#[doc(alias = "vkGetImageViewAddressNVX")]
pub unsafe fn get_image_view_address_nvx<
    S: StructureChainOut<ImageViewAddressPropertiesNVX<'static>>,
>(
    device: &raw::Device,
    image_view: &raw::ImageView,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.get_image_view_address_nvx.get();
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(image_view.borrow()),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_properties.as_mut_ptr());
        p_properties.assume_init()
    })
}
#[cfg(feature = "ext_image_view_handle")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceCombinedImageSamplerIndexNVX.html>"]
#[doc(alias = "vkGetDeviceCombinedImageSamplerIndexNVX")]
#[inline]
pub unsafe fn get_device_combined_image_sampler_index_nvx(
    device: &raw::Device,
    image_view_index: u64,
    sampler_index: u64,
    dispatcher: &CommandsDispatcher,
) -> u64 {
    let vulkan_command = dispatcher.get_device_combined_image_sampler_index_nvx.get();
    vulkan_command(Some(device.borrow()), image_view_index, sampler_index)
}
#[cfg(feature = "ext_shader_info")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInfoAMD.html>"]
#[doc(alias = "vkGetShaderInfoAMD")]
pub unsafe fn get_shader_info_amd(
    device: &raw::Device,
    pipeline: &raw::Pipeline,
    shader_stage: ShaderStageFlags,
    info_type: ShaderInfoTypeAMD,
    p_info: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<usize> {
    let vulkan_command = dispatcher.get_shader_info_amd.get();
    let mut p_info_size = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(pipeline.borrow()),
        shader_stage,
        info_type,
        p_info_size.as_mut_ptr(),
        p_info,
    );
    vk_status.map_success(|| p_info_size.assume_init())
}
#[cfg(feature = "ext_stream_descriptor_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateStreamDescriptorSurfaceGGP.html>"]
#[doc(alias = "vkCreateStreamDescriptorSurfaceGGP")]
pub unsafe fn create_stream_descriptor_surface_ggp(
    instance: &raw::Instance,
    p_create_info: &StreamDescriptorSurfaceCreateInfoGGP,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_stream_descriptor_surface_ggp.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_external_memory_capabilities")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html>"]
#[doc(alias = "vkGetPhysicalDeviceExternalImageFormatPropertiesNV")]
pub unsafe fn get_physical_device_external_image_format_properties_nv(
    physical_device: &raw::PhysicalDevice,
    format: Format,
    ty: ImageType,
    tiling: ImageTiling,
    usage: ImageUsageFlags,
    flags: ImageCreateFlags,
    external_handle_type: ExternalMemoryHandleTypeFlagsNV,
    dispatcher: &CommandsDispatcher,
) -> Result<ExternalImageFormatPropertiesNV> {
    let vulkan_command = dispatcher
        .get_physical_device_external_image_format_properties_nv
        .get();
    let mut p_external_image_format_properties = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(physical_device.borrow()),
        format,
        ty,
        tiling,
        usage,
        flags,
        external_handle_type,
        p_external_image_format_properties.as_mut_ptr(),
    );
    vk_status.map_success(|| p_external_image_format_properties.assume_init())
}
#[cfg(feature = "ext_external_memory_win32")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleNV.html>"]
#[doc(alias = "vkGetMemoryWin32HandleNV")]
pub unsafe fn get_memory_win32_handle_nv(
    device: &raw::Device,
    memory: &raw::DeviceMemory,
    handle_type: ExternalMemoryHandleTypeFlagsNV,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher.get_memory_win32_handle_nv.get();
    let mut p_handle = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(memory.borrow()),
        handle_type,
        p_handle.as_mut_ptr(),
    );
    vk_status.map_success(|| p_handle.assume_init())
}
#[cfg(feature = "ext_vi_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateViSurfaceNN.html>"]
#[doc(alias = "vkCreateViSurfaceNN")]
pub unsafe fn create_vi_surface_nn(
    instance: &raw::Instance,
    p_create_info: &ViSurfaceCreateInfoNN,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_vi_surface_nn.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_external_memory_win32")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleKHR.html>"]
#[doc(alias = "vkGetMemoryWin32HandleKHR")]
pub unsafe fn get_memory_win32_handle_khr(
    device: &raw::Device,
    p_get_win32_handle_info: &MemoryGetWin32HandleInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher.get_memory_win32_handle_khr.get();
    let mut p_handle = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_get_win32_handle_info),
        p_handle.as_mut_ptr(),
    );
    vk_status.map_success(|| p_handle.assume_init())
}
#[cfg(feature = "ext_external_memory_win32")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandlePropertiesKHR.html>"]
#[doc(alias = "vkGetMemoryWin32HandlePropertiesKHR")]
pub unsafe fn get_memory_win32_handle_properties_khr<
    S: StructureChainOut<MemoryWin32HandlePropertiesKHR<'static>>,
>(
    device: &raw::Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    handle: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.get_memory_win32_handle_properties_khr.get();
    let mut p_memory_win32_handle_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_win32_handle_properties);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        handle_type,
        handle,
        S::get_uninit_head_ptr(p_memory_win32_handle_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_memory_win32_handle_properties.as_mut_ptr());
        p_memory_win32_handle_properties.assume_init()
    })
}
#[cfg(feature = "ext_external_memory_fd")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdKHR.html>"]
#[doc(alias = "vkGetMemoryFdKHR")]
pub unsafe fn get_memory_fd_khr(
    device: &raw::Device,
    p_get_fd_info: &MemoryGetFdInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<c_int> {
    let vulkan_command = dispatcher.get_memory_fd_khr.get();
    let mut p_fd = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_get_fd_info),
        p_fd.as_mut_ptr(),
    );
    vk_status.map_success(|| p_fd.assume_init())
}
#[cfg(feature = "ext_external_memory_fd")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdPropertiesKHR.html>"]
#[doc(alias = "vkGetMemoryFdPropertiesKHR")]
pub unsafe fn get_memory_fd_properties_khr<S: StructureChainOut<MemoryFdPropertiesKHR<'static>>>(
    device: &raw::Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    fd: c_int,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.get_memory_fd_properties_khr.get();
    let mut p_memory_fd_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_fd_properties);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        handle_type,
        fd,
        S::get_uninit_head_ptr(p_memory_fd_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_memory_fd_properties.as_mut_ptr());
        p_memory_fd_properties.assume_init()
    })
}
#[cfg(feature = "ext_external_semaphore_win32")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreWin32HandleKHR.html>"]
#[doc(alias = "vkImportSemaphoreWin32HandleKHR")]
#[inline]
pub unsafe fn import_semaphore_win32_handle_khr(
    device: &raw::Device,
    p_import_semaphore_win32_handle_info: &ImportSemaphoreWin32HandleInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.import_semaphore_win32_handle_khr.get();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_import_semaphore_win32_handle_info),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_external_semaphore_win32")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreWin32HandleKHR.html>"]
#[doc(alias = "vkGetSemaphoreWin32HandleKHR")]
pub unsafe fn get_semaphore_win32_handle_khr(
    device: &raw::Device,
    p_get_win32_handle_info: &SemaphoreGetWin32HandleInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher.get_semaphore_win32_handle_khr.get();
    let mut p_handle = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_get_win32_handle_info),
        p_handle.as_mut_ptr(),
    );
    vk_status.map_success(|| p_handle.assume_init())
}
#[cfg(feature = "ext_external_semaphore_fd")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreFdKHR.html>"]
#[doc(alias = "vkImportSemaphoreFdKHR")]
#[inline]
pub unsafe fn import_semaphore_fd_khr(
    device: &raw::Device,
    p_import_semaphore_fd_info: &ImportSemaphoreFdInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.import_semaphore_fd_khr.get();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_import_semaphore_fd_info),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_external_semaphore_fd")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreFdKHR.html>"]
#[doc(alias = "vkGetSemaphoreFdKHR")]
pub unsafe fn get_semaphore_fd_khr(
    device: &raw::Device,
    p_get_fd_info: &SemaphoreGetFdInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<c_int> {
    let vulkan_command = dispatcher.get_semaphore_fd_khr.get();
    let mut p_fd = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_get_fd_info),
        p_fd.as_mut_ptr(),
    );
    vk_status.map_success(|| p_fd.assume_init())
}
#[cfg(feature = "ext_conditional_rendering")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRenderingEXT.html>"]
#[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
#[inline]
pub unsafe fn cmd_begin_conditional_rendering_ext(
    command_buffer: &raw::CommandBuffer,
    p_conditional_rendering_begin: &ConditionalRenderingBeginInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_begin_conditional_rendering_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_conditional_rendering_begin),
    )
}
#[cfg(feature = "ext_conditional_rendering")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndConditionalRenderingEXT.html>"]
#[doc(alias = "vkCmdEndConditionalRenderingEXT")]
#[inline]
pub unsafe fn cmd_end_conditional_rendering_ext(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_end_conditional_rendering_ext.get();
    vulkan_command(Some(command_buffer.borrow()))
}
#[cfg(feature = "ext_clip_space_w_scaling")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingNV.html>"]
#[doc(alias = "vkCmdSetViewportWScalingNV")]
#[inline]
pub unsafe fn cmd_set_viewport_wscaling_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    first_viewport: u32,
    p_viewport_wscalings: impl AsSlice<'a, ViewportWScalingNV>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_viewport_wscaling_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_viewport,
        p_viewport_wscalings.as_slice().len() as _,
        p_viewport_wscalings.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_direct_mode_display")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseDisplayEXT.html>"]
#[doc(alias = "vkReleaseDisplayEXT")]
#[inline]
pub unsafe fn release_display_ext(
    physical_device: &raw::PhysicalDevice,
    display: &raw::DisplayKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.release_display_ext.get();
    vulkan_command(Some(physical_device.borrow()), Some(display.borrow())).map_success(|| ())
}
#[cfg(feature = "ext_acquire_xlib_display")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireXlibDisplayEXT.html>"]
#[doc(alias = "vkAcquireXlibDisplayEXT")]
#[inline]
pub unsafe fn acquire_xlib_display_ext(
    physical_device: &raw::PhysicalDevice,
    dpy: &VoidPtr,
    display: &raw::DisplayKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.acquire_xlib_display_ext.get();
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(dpy),
        Some(display.borrow()),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_acquire_xlib_display")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRandROutputDisplayEXT.html>"]
#[doc(alias = "vkGetRandROutputDisplayEXT")]
pub unsafe fn get_rand_routput_display_ext(
    physical_device: &raw::PhysicalDevice,
    dpy: &VoidPtr,
    rr_output: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<DisplayKHR> {
    let vulkan_command = dispatcher.get_rand_routput_display_ext.get();
    let mut p_display = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(dpy),
        rr_output,
        p_display.as_mut_ptr(),
    );
    vk_status.map_success(|| p_display.assume_init())
}
#[cfg(feature = "ext_display_surface_counter")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html>"]
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2EXT")]
pub unsafe fn get_physical_device_surface_capabilities2_ext<
    S: StructureChainOut<SurfaceCapabilities2EXT<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    surface: &raw::SurfaceKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_physical_device_surface_capabilities2_ext
        .get();
    let mut p_surface_capabilities = MaybeUninit::uninit();
    S::setup_uninit(&mut p_surface_capabilities);
    let vk_status = vulkan_command(
        Some(physical_device.borrow()),
        Some(surface.borrow()),
        S::get_uninit_head_ptr(p_surface_capabilities.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_surface_capabilities.as_mut_ptr());
        p_surface_capabilities.assume_init()
    })
}
#[cfg(feature = "ext_display_control")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDisplayPowerControlEXT.html>"]
#[doc(alias = "vkDisplayPowerControlEXT")]
#[inline]
pub unsafe fn display_power_control_ext(
    device: &raw::Device,
    display: &raw::DisplayKHR,
    p_display_power_info: &DisplayPowerInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.display_power_control_ext.get();
    vulkan_command(
        Some(device.borrow()),
        Some(display.borrow()),
        ptr::from_ref(p_display_power_info),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_display_control")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDeviceEventEXT.html>"]
#[doc(alias = "vkRegisterDeviceEventEXT")]
pub unsafe fn register_device_event_ext(
    device: &raw::Device,
    p_device_event_info: &DeviceEventInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Fence> {
    let vulkan_command = dispatcher.register_device_event_ext.get();
    let mut p_fence = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_device_event_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_fence.as_mut_ptr(),
    );
    vk_status.map_success(|| p_fence.assume_init())
}
#[cfg(feature = "ext_display_control")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDisplayEventEXT.html>"]
#[doc(alias = "vkRegisterDisplayEventEXT")]
pub unsafe fn register_display_event_ext(
    device: &raw::Device,
    display: &raw::DisplayKHR,
    p_display_event_info: &DisplayEventInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Fence> {
    let vulkan_command = dispatcher.register_display_event_ext.get();
    let mut p_fence = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(display.borrow()),
        ptr::from_ref(p_display_event_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_fence.as_mut_ptr(),
    );
    vk_status.map_success(|| p_fence.assume_init())
}
#[cfg(feature = "ext_display_control")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainCounterEXT.html>"]
#[doc(alias = "vkGetSwapchainCounterEXT")]
pub unsafe fn get_swapchain_counter_ext(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    counter: SurfaceCounterFlagsEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<u64> {
    let vulkan_command = dispatcher.get_swapchain_counter_ext.get();
    let mut p_counter_value = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(swapchain.borrow()),
        counter,
        p_counter_value.as_mut_ptr(),
    );
    vk_status.map_success(|| p_counter_value.assume_init())
}
#[cfg(feature = "ext_display_timing")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRefreshCycleDurationGOOGLE.html>"]
#[doc(alias = "vkGetRefreshCycleDurationGOOGLE")]
pub unsafe fn get_refresh_cycle_duration_google(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<RefreshCycleDurationGOOGLE> {
    let vulkan_command = dispatcher.get_refresh_cycle_duration_google.get();
    let mut p_display_timing_properties = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(swapchain.borrow()),
        p_display_timing_properties.as_mut_ptr(),
    );
    vk_status.map_success(|| p_display_timing_properties.assume_init())
}
#[cfg(feature = "ext_display_timing")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingGOOGLE.html>"]
#[doc(alias = "vkGetPastPresentationTimingGOOGLE")]
pub unsafe fn get_past_presentation_timing_google<R: DynamicArray<PastPresentationTimingGOOGLE>>(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_past_presentation_timing_google.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_presentation_timing_count = vk_len.as_mut_ptr();
    let p_presentation_timings = ptr::null_mut();
    vulkan_command(
        Some(device.borrow()),
        Some(swapchain.borrow()),
        p_presentation_timing_count,
        p_presentation_timings,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_presentation_timing_count = ptr::from_mut(&mut vk_len);
    let mut p_presentation_timings = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(device.borrow()),
            Some(swapchain.borrow()),
            p_presentation_timing_count,
            p_presentation_timings,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_presentation_timing_count = ptr::from_mut(&mut vk_len);
        p_presentation_timings = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_discard_rectangles")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEXT.html>"]
#[doc(alias = "vkCmdSetDiscardRectangleEXT")]
#[inline]
pub unsafe fn cmd_set_discard_rectangle_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    first_discard_rectangle: u32,
    p_discard_rectangles: impl AsSlice<'a, Rect2D>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_discard_rectangle_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_discard_rectangle,
        p_discard_rectangles.as_slice().len() as _,
        p_discard_rectangles.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_discard_rectangles")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEnableEXT.html>"]
#[doc(alias = "vkCmdSetDiscardRectangleEnableEXT")]
#[inline]
pub unsafe fn cmd_set_discard_rectangle_enable_ext(
    command_buffer: &raw::CommandBuffer,
    discard_rectangle_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_discard_rectangle_enable_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        discard_rectangle_enable.into(),
    )
}
#[cfg(feature = "ext_discard_rectangles")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleModeEXT.html>"]
#[doc(alias = "vkCmdSetDiscardRectangleModeEXT")]
#[inline]
pub unsafe fn cmd_set_discard_rectangle_mode_ext(
    command_buffer: &raw::CommandBuffer,
    discard_rectangle_mode: DiscardRectangleModeEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_discard_rectangle_mode_ext.get();
    vulkan_command(Some(command_buffer.borrow()), discard_rectangle_mode)
}
#[cfg(feature = "ext_hdr_metadata")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetHdrMetadataEXT.html>"]
#[doc(alias = "vkSetHdrMetadataEXT")]
#[inline]
pub unsafe fn set_hdr_metadata_ext<'a, V2: Alias<raw::SwapchainKHR> + 'a>(
    device: &raw::Device,
    p_swapchains: impl AsSlice<'a, V2>,
    p_metadata: impl AsSlice<'a, HdrMetadataEXT<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.set_hdr_metadata_ext.get();
    vulkan_command(
        Some(device.borrow()),
        p_metadata.as_slice().len() as _,
        p_swapchains.as_slice().as_ptr().cast(),
        p_metadata.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_shared_presentable_image")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainStatusKHR.html>"]
#[doc(alias = "vkGetSwapchainStatusKHR")]
#[inline]
pub unsafe fn get_swapchain_status_khr(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.get_swapchain_status_khr.get();
    vulkan_command(Some(device.borrow()), Some(swapchain.borrow())).into_result()
}
#[cfg(feature = "ext_external_fence_win32")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceWin32HandleKHR.html>"]
#[doc(alias = "vkImportFenceWin32HandleKHR")]
#[inline]
pub unsafe fn import_fence_win32_handle_khr(
    device: &raw::Device,
    p_import_fence_win32_handle_info: &ImportFenceWin32HandleInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.import_fence_win32_handle_khr.get();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_import_fence_win32_handle_info),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_external_fence_win32")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceWin32HandleKHR.html>"]
#[doc(alias = "vkGetFenceWin32HandleKHR")]
pub unsafe fn get_fence_win32_handle_khr(
    device: &raw::Device,
    p_get_win32_handle_info: &FenceGetWin32HandleInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher.get_fence_win32_handle_khr.get();
    let mut p_handle = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_get_win32_handle_info),
        p_handle.as_mut_ptr(),
    );
    vk_status.map_success(|| p_handle.assume_init())
}
#[cfg(feature = "ext_external_fence_fd")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceFdKHR.html>"]
#[doc(alias = "vkImportFenceFdKHR")]
#[inline]
pub unsafe fn import_fence_fd_khr(
    device: &raw::Device,
    p_import_fence_fd_info: &ImportFenceFdInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.import_fence_fd_khr.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_import_fence_fd_info)).map_success(|| ())
}
#[cfg(feature = "ext_external_fence_fd")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceFdKHR.html>"]
#[doc(alias = "vkGetFenceFdKHR")]
pub unsafe fn get_fence_fd_khr(
    device: &raw::Device,
    p_get_fd_info: &FenceGetFdInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<c_int> {
    let vulkan_command = dispatcher.get_fence_fd_khr.get();
    let mut p_fd = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_get_fd_info),
        p_fd.as_mut_ptr(),
    );
    vk_status.map_success(|| p_fd.assume_init())
}
#[cfg(feature = "ext_performance_query")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")]
pub unsafe fn get_physical_device_queue_family_performance_query_passes_khr(
    physical_device: &raw::PhysicalDevice,
    p_performance_query_create_info: &QueryPoolPerformanceCreateInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> u32 {
    let vulkan_command = dispatcher
        .get_physical_device_queue_family_performance_query_passes_khr
        .get();
    let mut p_num_passes = MaybeUninit::uninit();
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_performance_query_create_info),
        p_num_passes.as_mut_ptr(),
    );
    p_num_passes.assume_init()
}
#[cfg(feature = "ext_performance_query")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireProfilingLockKHR.html>"]
#[doc(alias = "vkAcquireProfilingLockKHR")]
#[inline]
pub unsafe fn acquire_profiling_lock_khr(
    device: &raw::Device,
    p_info: &AcquireProfilingLockInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.acquire_profiling_lock_khr.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info)).map_success(|| ())
}
#[cfg(feature = "ext_performance_query")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseProfilingLockKHR.html>"]
#[doc(alias = "vkReleaseProfilingLockKHR")]
#[inline]
pub unsafe fn release_profiling_lock_khr(device: &raw::Device, dispatcher: &CommandsDispatcher) {
    let vulkan_command = dispatcher.release_profiling_lock_khr.get();
    vulkan_command(Some(device.borrow()))
}
#[cfg(feature = "ext_get_surface_capabilities2")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2KHR")]
pub unsafe fn get_physical_device_surface_capabilities2_khr<
    S: StructureChainOut<SurfaceCapabilities2KHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_physical_device_surface_capabilities2_khr
        .get();
    let mut p_surface_capabilities = MaybeUninit::uninit();
    S::setup_uninit(&mut p_surface_capabilities);
    let vk_status = vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_surface_info),
        S::get_uninit_head_ptr(p_surface_capabilities.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_surface_capabilities.as_mut_ptr());
        p_surface_capabilities.assume_init()
    })
}
#[cfg(feature = "ext_get_surface_capabilities2")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormats2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceSurfaceFormats2KHR")]
pub unsafe fn get_physical_device_surface_formats2_khr<
    R: DynamicArray<SurfaceFormat2KHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_physical_device_surface_formats2_khr.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_surface_format_count = vk_len.as_mut_ptr();
    let p_surface_formats = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_surface_info),
        p_surface_format_count,
        p_surface_formats,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_surface_format_count = ptr::from_mut(&mut vk_len);
    let mut p_surface_formats = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            ptr::from_ref(p_surface_info),
            p_surface_format_count,
            p_surface_formats,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_surface_format_count = ptr::from_mut(&mut vk_len);
        p_surface_formats = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_get_display_properties2")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayProperties2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
pub unsafe fn get_physical_device_display_properties2_khr<
    R: DynamicArray<DisplayProperties2KHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_physical_device_display_properties2_khr.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_property_count,
        p_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_property_count,
            p_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_property_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_get_display_properties2")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
pub unsafe fn get_physical_device_display_plane_properties2_khr<
    R: DynamicArray<DisplayPlaneProperties2KHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_display_plane_properties2_khr
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_property_count,
        p_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_property_count,
            p_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_property_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_get_display_properties2")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModeProperties2KHR.html>"]
#[doc(alias = "vkGetDisplayModeProperties2KHR")]
pub unsafe fn get_display_mode_properties2_khr<
    R: DynamicArray<DisplayModeProperties2KHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    display: &raw::DisplayKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_display_mode_properties2_khr.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        Some(display.borrow()),
        p_property_count,
        p_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            Some(display.borrow()),
            p_property_count,
            p_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_property_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_get_display_properties2")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilities2KHR.html>"]
#[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
pub unsafe fn get_display_plane_capabilities2_khr<
    S: StructureChainOut<DisplayPlaneCapabilities2KHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_display_plane_info: &DisplayPlaneInfo2KHR,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.get_display_plane_capabilities2_khr.get();
    let mut p_capabilities = MaybeUninit::uninit();
    S::setup_uninit(&mut p_capabilities);
    let vk_status = vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_display_plane_info),
        S::get_uninit_head_ptr(p_capabilities.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_capabilities.as_mut_ptr());
        p_capabilities.assume_init()
    })
}
#[cfg(feature = "ext_ios_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIOSSurfaceMVK.html>"]
#[doc(alias = "vkCreateIOSSurfaceMVK")]
pub unsafe fn create_iossurface_mvk(
    instance: &raw::Instance,
    p_create_info: &IOSSurfaceCreateInfoMVK,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_iossurface_mvk.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_macos_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMacOSSurfaceMVK.html>"]
#[doc(alias = "vkCreateMacOSSurfaceMVK")]
pub unsafe fn create_mac_ossurface_mvk(
    instance: &raw::Instance,
    p_create_info: &MacOSSurfaceCreateInfoMVK,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_mac_ossurface_mvk.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_debug_utils")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectNameEXT.html>"]
#[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
#[inline]
pub unsafe fn set_debug_utils_object_name_ext(
    device: &raw::Device,
    p_name_info: &DebugUtilsObjectNameInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.set_debug_utils_object_name_ext.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_name_info)).map_success(|| ())
}
#[cfg(feature = "ext_debug_utils")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectTagEXT.html>"]
#[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
#[inline]
pub unsafe fn set_debug_utils_object_tag_ext(
    device: &raw::Device,
    p_tag_info: &DebugUtilsObjectTagInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.set_debug_utils_object_tag_ext.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_tag_info)).map_success(|| ())
}
#[cfg(feature = "ext_debug_utils")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBeginDebugUtilsLabelEXT.html>"]
#[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
#[inline]
pub unsafe fn queue_begin_debug_utils_label_ext(
    queue: &raw::Queue,
    p_label_info: &DebugUtilsLabelEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.queue_begin_debug_utils_label_ext.get();
    vulkan_command(Some(queue.borrow()), ptr::from_ref(p_label_info))
}
#[cfg(feature = "ext_debug_utils")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueEndDebugUtilsLabelEXT.html>"]
#[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
#[inline]
pub unsafe fn queue_end_debug_utils_label_ext(queue: &raw::Queue, dispatcher: &CommandsDispatcher) {
    let vulkan_command = dispatcher.queue_end_debug_utils_label_ext.get();
    vulkan_command(Some(queue.borrow()))
}
#[cfg(feature = "ext_debug_utils")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueInsertDebugUtilsLabelEXT.html>"]
#[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
#[inline]
pub unsafe fn queue_insert_debug_utils_label_ext(
    queue: &raw::Queue,
    p_label_info: &DebugUtilsLabelEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.queue_insert_debug_utils_label_ext.get();
    vulkan_command(Some(queue.borrow()), ptr::from_ref(p_label_info))
}
#[cfg(feature = "ext_debug_utils")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginDebugUtilsLabelEXT.html>"]
#[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
#[inline]
pub unsafe fn cmd_begin_debug_utils_label_ext(
    command_buffer: &raw::CommandBuffer,
    p_label_info: &DebugUtilsLabelEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_begin_debug_utils_label_ext.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_label_info))
}
#[cfg(feature = "ext_debug_utils")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndDebugUtilsLabelEXT.html>"]
#[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
#[inline]
pub unsafe fn cmd_end_debug_utils_label_ext(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_end_debug_utils_label_ext.get();
    vulkan_command(Some(command_buffer.borrow()))
}
#[cfg(feature = "ext_debug_utils")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInsertDebugUtilsLabelEXT.html>"]
#[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
#[inline]
pub unsafe fn cmd_insert_debug_utils_label_ext(
    command_buffer: &raw::CommandBuffer,
    p_label_info: &DebugUtilsLabelEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_insert_debug_utils_label_ext.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_label_info))
}
#[cfg(feature = "ext_debug_utils")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugUtilsMessengerEXT.html>"]
#[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
pub unsafe fn create_debug_utils_messenger_ext(
    instance: &raw::Instance,
    p_create_info: &DebugUtilsMessengerCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DebugUtilsMessengerEXT> {
    let vulkan_command = dispatcher.create_debug_utils_messenger_ext.get();
    let mut p_messenger = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_messenger.as_mut_ptr(),
    );
    vk_status.map_success(|| p_messenger.assume_init())
}
#[cfg(feature = "ext_debug_utils")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugUtilsMessengerEXT.html>"]
#[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
#[inline]
pub unsafe fn destroy_debug_utils_messenger_ext(
    instance: &raw::Instance,
    messenger: Option<&raw::DebugUtilsMessengerEXT>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_debug_utils_messenger_ext.get();
    vulkan_command(
        Some(instance.borrow()),
        messenger.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_debug_utils")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSubmitDebugUtilsMessageEXT.html>"]
#[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
#[inline]
pub unsafe fn submit_debug_utils_message_ext(
    instance: &raw::Instance,
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: &DebugUtilsMessengerCallbackDataEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.submit_debug_utils_message_ext.get();
    vulkan_command(
        Some(instance.borrow()),
        message_severity,
        message_types,
        ptr::from_ref(p_callback_data),
    )
}
#[cfg(feature = "ext_external_memory_android_hardware_buffer")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAndroidHardwareBufferPropertiesANDROID.html>"]
#[doc(alias = "vkGetAndroidHardwareBufferPropertiesANDROID")]
pub unsafe fn get_android_hardware_buffer_properties_android<
    S: StructureChainOut<AndroidHardwareBufferPropertiesANDROID<'static>>,
>(
    device: &raw::Device,
    buffer: &AHardwareBuffer,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_android_hardware_buffer_properties_android
        .get();
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(buffer),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_properties.as_mut_ptr());
        p_properties.assume_init()
    })
}
#[cfg(feature = "ext_external_memory_android_hardware_buffer")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryAndroidHardwareBufferANDROID.html>"]
#[doc(alias = "vkGetMemoryAndroidHardwareBufferANDROID")]
#[inline]
pub unsafe fn get_memory_android_hardware_buffer_android(
    device: &raw::Device,
    p_info: &MemoryGetAndroidHardwareBufferInfoANDROID,
    p_buffer: &&AHardwareBuffer,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.get_memory_android_hardware_buffer_android.get();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        ptr::from_ref(p_buffer).cast(),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_gpa_interface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGpaSessionAMD.html>"]
#[doc(alias = "vkCreateGpaSessionAMD")]
pub unsafe fn create_gpa_session_amd(
    device: &raw::Device,
    p_create_info: &GpaSessionCreateInfoAMD,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<GpaSessionAMD> {
    let vulkan_command = dispatcher.create_gpa_session_amd.get();
    let mut p_gpa_session = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_gpa_session.as_mut_ptr(),
    );
    vk_status.map_success(|| p_gpa_session.assume_init())
}
#[cfg(feature = "ext_gpa_interface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyGpaSessionAMD.html>"]
#[doc(alias = "vkDestroyGpaSessionAMD")]
#[inline]
pub unsafe fn destroy_gpa_session_amd(
    device: &raw::Device,
    gpa_session: Option<&raw::GpaSessionAMD>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_gpa_session_amd.get();
    vulkan_command(
        Some(device.borrow()),
        gpa_session.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_gpa_interface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetGpaDeviceClockModeAMD.html>"]
#[doc(alias = "vkSetGpaDeviceClockModeAMD")]
pub unsafe fn set_gpa_device_clock_mode_amd<
    S: StructureChainOut<GpaDeviceClockModeInfoAMD<'static>>,
>(
    device: &raw::Device,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.set_gpa_device_clock_mode_amd.get();
    let mut p_info = MaybeUninit::uninit();
    S::setup_uninit(&mut p_info);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        S::get_uninit_head_ptr(p_info.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_info.as_mut_ptr());
        p_info.assume_init()
    })
}
#[cfg(feature = "ext_gpa_interface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaDeviceClockInfoAMD.html>"]
#[doc(alias = "vkGetGpaDeviceClockInfoAMD")]
pub unsafe fn get_gpa_device_clock_info_amd<
    S: StructureChainOut<GpaDeviceGetClockInfoAMD<'static>>,
>(
    device: &raw::Device,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.get_gpa_device_clock_info_amd.get();
    let mut p_info = MaybeUninit::uninit();
    S::setup_uninit(&mut p_info);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        S::get_uninit_head_ptr(p_info.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_info.as_mut_ptr());
        p_info.assume_init()
    })
}
#[cfg(feature = "ext_gpa_interface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSessionAMD.html>"]
#[doc(alias = "vkCmdBeginGpaSessionAMD")]
#[inline]
pub unsafe fn cmd_begin_gpa_session_amd(
    command_buffer: &raw::CommandBuffer,
    gpa_session: &raw::GpaSessionAMD,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.cmd_begin_gpa_session_amd.get();
    vulkan_command(Some(command_buffer.borrow()), Some(gpa_session.borrow())).map_success(|| ())
}
#[cfg(feature = "ext_gpa_interface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSessionAMD.html>"]
#[doc(alias = "vkCmdEndGpaSessionAMD")]
#[inline]
pub unsafe fn cmd_end_gpa_session_amd(
    command_buffer: &raw::CommandBuffer,
    gpa_session: &raw::GpaSessionAMD,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.cmd_end_gpa_session_amd.get();
    vulkan_command(Some(command_buffer.borrow()), Some(gpa_session.borrow())).map_success(|| ())
}
#[cfg(feature = "ext_gpa_interface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSampleAMD.html>"]
#[doc(alias = "vkCmdBeginGpaSampleAMD")]
pub unsafe fn cmd_begin_gpa_sample_amd(
    command_buffer: &raw::CommandBuffer,
    gpa_session: &raw::GpaSessionAMD,
    p_gpa_sample_begin_info: &GpaSampleBeginInfoAMD,
    dispatcher: &CommandsDispatcher,
) -> Result<u32> {
    let vulkan_command = dispatcher.cmd_begin_gpa_sample_amd.get();
    let mut p_sample_id = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(command_buffer.borrow()),
        Some(gpa_session.borrow()),
        ptr::from_ref(p_gpa_sample_begin_info),
        p_sample_id.as_mut_ptr(),
    );
    vk_status.map_success(|| p_sample_id.assume_init())
}
#[cfg(feature = "ext_gpa_interface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSampleAMD.html>"]
#[doc(alias = "vkCmdEndGpaSampleAMD")]
#[inline]
pub unsafe fn cmd_end_gpa_sample_amd(
    command_buffer: &raw::CommandBuffer,
    gpa_session: &raw::GpaSessionAMD,
    sample_id: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_end_gpa_sample_amd.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(gpa_session.borrow()),
        sample_id,
    )
}
#[cfg(feature = "ext_gpa_interface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionStatusAMD.html>"]
#[doc(alias = "vkGetGpaSessionStatusAMD")]
#[inline]
pub unsafe fn get_gpa_session_status_amd(
    device: &raw::Device,
    gpa_session: &raw::GpaSessionAMD,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.get_gpa_session_status_amd.get();
    vulkan_command(Some(device.borrow()), Some(gpa_session.borrow())).map_success(|| ())
}
#[cfg(feature = "ext_gpa_interface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionResultsAMD.html>"]
#[doc(alias = "vkGetGpaSessionResultsAMD")]
pub unsafe fn get_gpa_session_results_amd(
    device: &raw::Device,
    gpa_session: &raw::GpaSessionAMD,
    sample_id: u32,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<usize> {
    let vulkan_command = dispatcher.get_gpa_session_results_amd.get();
    let mut p_size_in_bytes = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(gpa_session.borrow()),
        sample_id,
        p_size_in_bytes.as_mut_ptr(),
        p_data,
    );
    vk_status.map_success(|| p_size_in_bytes.assume_init())
}
#[cfg(feature = "ext_gpa_interface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetGpaSessionAMD.html>"]
#[doc(alias = "vkResetGpaSessionAMD")]
#[inline]
pub unsafe fn reset_gpa_session_amd(
    device: &raw::Device,
    gpa_session: &raw::GpaSessionAMD,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.reset_gpa_session_amd.get();
    vulkan_command(Some(device.borrow()), Some(gpa_session.borrow())).map_success(|| ())
}
#[cfg(feature = "ext_gpa_interface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyGpaSessionResultsAMD.html>"]
#[doc(alias = "vkCmdCopyGpaSessionResultsAMD")]
#[inline]
pub unsafe fn cmd_copy_gpa_session_results_amd(
    command_buffer: &raw::CommandBuffer,
    gpa_session: &raw::GpaSessionAMD,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_gpa_session_results_amd.get();
    vulkan_command(Some(command_buffer.borrow()), Some(gpa_session.borrow()))
}
#[cfg(feature = "ext_shader_enqueue")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExecutionGraphPipelinesAMDX.html>"]
#[doc(alias = "vkCreateExecutionGraphPipelinesAMDX")]
pub unsafe fn create_execution_graph_pipelines_amdx<'a, R: DynamicArray<Pipeline>>(
    device: &raw::Device,
    pipeline_cache: Option<&raw::PipelineCache>,
    p_create_infos: impl AsSlice<'a, ExecutionGraphPipelineCreateInfoAMDX<'a>>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, R)> {
    let vulkan_command = dispatcher.create_execution_graph_pipelines_amdx.get();
    let mut p_pipelines = R::create_with_capacity(p_create_infos.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        pipeline_cache.map(|v| v.borrow()),
        p_create_infos.as_slice().len() as _,
        p_create_infos.as_slice().as_ptr().cast(),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_pipelines.get_content_mut_ptr(),
    );
    vk_status.map_successes(|| {
        p_pipelines.resize_with_len(p_create_infos.as_slice().len() as _);
        p_pipelines
    })
}
#[cfg(feature = "ext_shader_enqueue")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineScratchSizeAMDX.html>"]
#[doc(alias = "vkGetExecutionGraphPipelineScratchSizeAMDX")]
pub unsafe fn get_execution_graph_pipeline_scratch_size_amdx<
    S: StructureChainOut<ExecutionGraphPipelineScratchSizeAMDX<'static>>,
>(
    device: &raw::Device,
    execution_graph: &raw::Pipeline,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_execution_graph_pipeline_scratch_size_amdx
        .get();
    let mut p_size_info = MaybeUninit::uninit();
    S::setup_uninit(&mut p_size_info);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(execution_graph.borrow()),
        S::get_uninit_head_ptr(p_size_info.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_size_info.as_mut_ptr());
        p_size_info.assume_init()
    })
}
#[cfg(feature = "ext_shader_enqueue")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineNodeIndexAMDX.html>"]
#[doc(alias = "vkGetExecutionGraphPipelineNodeIndexAMDX")]
pub unsafe fn get_execution_graph_pipeline_node_index_amdx(
    device: &raw::Device,
    execution_graph: &raw::Pipeline,
    p_node_info: &PipelineShaderStageNodeCreateInfoAMDX,
    dispatcher: &CommandsDispatcher,
) -> Result<u32> {
    let vulkan_command = dispatcher
        .get_execution_graph_pipeline_node_index_amdx
        .get();
    let mut p_node_index = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(execution_graph.borrow()),
        ptr::from_ref(p_node_info),
        p_node_index.as_mut_ptr(),
    );
    vk_status.map_success(|| p_node_index.assume_init())
}
#[cfg(feature = "ext_shader_enqueue")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInitializeGraphScratchMemoryAMDX.html>"]
#[doc(alias = "vkCmdInitializeGraphScratchMemoryAMDX")]
#[inline]
pub unsafe fn cmd_initialize_graph_scratch_memory_amdx(
    command_buffer: &raw::CommandBuffer,
    execution_graph: &raw::Pipeline,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_initialize_graph_scratch_memory_amdx.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(execution_graph.borrow()),
        scratch,
        scratch_size,
    )
}
#[cfg(feature = "ext_shader_enqueue")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphAMDX.html>"]
#[doc(alias = "vkCmdDispatchGraphAMDX")]
#[inline]
pub unsafe fn cmd_dispatch_graph_amdx(
    command_buffer: &raw::CommandBuffer,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    p_count_info: &DispatchGraphCountInfoAMDX,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_dispatch_graph_amdx.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        scratch,
        scratch_size,
        ptr::from_ref(p_count_info),
    )
}
#[cfg(feature = "ext_shader_enqueue")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectAMDX.html>"]
#[doc(alias = "vkCmdDispatchGraphIndirectAMDX")]
#[inline]
pub unsafe fn cmd_dispatch_graph_indirect_amdx(
    command_buffer: &raw::CommandBuffer,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    p_count_info: &DispatchGraphCountInfoAMDX,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_dispatch_graph_indirect_amdx.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        scratch,
        scratch_size,
        ptr::from_ref(p_count_info),
    )
}
#[cfg(feature = "ext_shader_enqueue")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectCountAMDX.html>"]
#[doc(alias = "vkCmdDispatchGraphIndirectCountAMDX")]
#[inline]
pub unsafe fn cmd_dispatch_graph_indirect_count_amdx(
    command_buffer: &raw::CommandBuffer,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    count_info: DeviceAddress,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_dispatch_graph_indirect_count_amdx.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        scratch,
        scratch_size,
        count_info,
    )
}
#[cfg(feature = "ext_descriptor_heap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteSamplerDescriptorsEXT.html>"]
#[doc(alias = "vkWriteSamplerDescriptorsEXT")]
#[inline]
pub unsafe fn write_sampler_descriptors_ext<'a>(
    device: &raw::Device,
    p_samplers: impl AsSlice<'a, SamplerCreateInfo<'a>>,
    p_descriptors: impl AsSlice<'a, HostAddressRangeEXT<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.write_sampler_descriptors_ext.get();
    vulkan_command(
        Some(device.borrow()),
        p_descriptors.as_slice().len() as _,
        p_samplers.as_slice().as_ptr().cast(),
        p_descriptors.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_descriptor_heap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteResourceDescriptorsEXT.html>"]
#[doc(alias = "vkWriteResourceDescriptorsEXT")]
#[inline]
pub unsafe fn write_resource_descriptors_ext<'a>(
    device: &raw::Device,
    p_resources: impl AsSlice<'a, ResourceDescriptorInfoEXT<'a>>,
    p_descriptors: impl AsSlice<'a, HostAddressRangeEXT<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.write_resource_descriptors_ext.get();
    vulkan_command(
        Some(device.borrow()),
        p_descriptors.as_slice().len() as _,
        p_resources.as_slice().as_ptr().cast(),
        p_descriptors.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_descriptor_heap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindSamplerHeapEXT.html>"]
#[doc(alias = "vkCmdBindSamplerHeapEXT")]
#[inline]
pub unsafe fn cmd_bind_sampler_heap_ext(
    command_buffer: &raw::CommandBuffer,
    p_bind_info: &BindHeapInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_sampler_heap_ext.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_bind_info))
}
#[cfg(feature = "ext_descriptor_heap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindResourceHeapEXT.html>"]
#[doc(alias = "vkCmdBindResourceHeapEXT")]
#[inline]
pub unsafe fn cmd_bind_resource_heap_ext(
    command_buffer: &raw::CommandBuffer,
    p_bind_info: &BindHeapInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_resource_heap_ext.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_bind_info))
}
#[cfg(feature = "ext_descriptor_heap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDataEXT.html>"]
#[doc(alias = "vkCmdPushDataEXT")]
#[inline]
pub unsafe fn cmd_push_data_ext(
    command_buffer: &raw::CommandBuffer,
    p_push_data_info: &PushDataInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_push_data_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_push_data_info),
    )
}
#[cfg(feature = "ext_descriptor_heap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDataEXT.html>"]
#[doc(alias = "vkGetImageOpaqueCaptureDataEXT")]
pub unsafe fn get_image_opaque_capture_data_ext<
    'a,
    R: DynamicArray<HostAddressRangeEXT<'static>>,
    V2: Alias<raw::Image> + 'a,
>(
    device: &raw::Device,
    p_images: impl AsSlice<'a, V2>,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_image_opaque_capture_data_ext.get();
    let mut p_datas = R::create_with_capacity(p_images.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        p_images.as_slice().len() as _,
        p_images.as_slice().as_ptr().cast(),
        p_datas.get_content_mut_ptr(),
    );
    vk_status.map_success(|| {
        p_datas.resize_with_len(p_images.as_slice().len() as _);
        p_datas
    })
}
#[cfg(feature = "ext_descriptor_heap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDescriptorSizeEXT.html>"]
#[doc(alias = "vkGetPhysicalDeviceDescriptorSizeEXT")]
#[inline]
pub unsafe fn get_physical_device_descriptor_size_ext(
    physical_device: &raw::PhysicalDevice,
    descriptor_type: DescriptorType,
    dispatcher: &CommandsDispatcher,
) -> DeviceSize {
    let vulkan_command = dispatcher.get_physical_device_descriptor_size_ext.get();
    vulkan_command(Some(physical_device.borrow()), descriptor_type)
}
#[cfg(all(feature = "ext_descriptor_heap", feature = "ext_custom_border_color"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterCustomBorderColorEXT.html>"]
#[doc(alias = "vkRegisterCustomBorderColorEXT")]
pub unsafe fn register_custom_border_color_ext(
    device: &raw::Device,
    p_border_color: &SamplerCustomBorderColorCreateInfoEXT,
    request_index: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) -> Result<u32> {
    let vulkan_command = dispatcher.register_custom_border_color_ext.get();
    let mut p_index = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_border_color),
        request_index.into(),
        p_index.as_mut_ptr(),
    );
    vk_status.map_success(|| p_index.assume_init())
}
#[cfg(all(feature = "ext_descriptor_heap", feature = "ext_custom_border_color"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUnregisterCustomBorderColorEXT.html>"]
#[doc(alias = "vkUnregisterCustomBorderColorEXT")]
#[inline]
pub unsafe fn unregister_custom_border_color_ext(
    device: &raw::Device,
    index: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.unregister_custom_border_color_ext.get();
    vulkan_command(Some(device.borrow()), index)
}
#[cfg(all(feature = "ext_descriptor_heap", feature = "ext_tensors"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDataARM.html>"]
#[doc(alias = "vkGetTensorOpaqueCaptureDataARM")]
pub unsafe fn get_tensor_opaque_capture_data_arm<
    'a,
    R: DynamicArray<HostAddressRangeEXT<'static>>,
    V2: Alias<raw::TensorARM> + 'a,
>(
    device: &raw::Device,
    p_tensors: impl AsSlice<'a, V2>,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_tensor_opaque_capture_data_arm.get();
    let mut p_datas = R::create_with_capacity(p_tensors.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        p_tensors.as_slice().len() as _,
        p_tensors.as_slice().as_ptr().cast(),
        p_datas.get_content_mut_ptr(),
    );
    vk_status.map_success(|| {
        p_datas.resize_with_len(p_tensors.as_slice().len() as _);
        p_datas
    })
}
#[cfg(feature = "ext_sample_locations")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEXT.html>"]
#[doc(alias = "vkCmdSetSampleLocationsEXT")]
#[inline]
pub unsafe fn cmd_set_sample_locations_ext(
    command_buffer: &raw::CommandBuffer,
    p_sample_locations_info: &SampleLocationsInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_sample_locations_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_sample_locations_info),
    )
}
#[cfg(feature = "ext_sample_locations")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMultisamplePropertiesEXT.html>"]
#[doc(alias = "vkGetPhysicalDeviceMultisamplePropertiesEXT")]
pub unsafe fn get_physical_device_multisample_properties_ext<
    S: StructureChainOut<MultisamplePropertiesEXT<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    samples: SampleCountFlags,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_multisample_properties_ext
        .get();
    let mut p_multisample_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_multisample_properties);
    vulkan_command(
        Some(physical_device.borrow()),
        samples,
        S::get_uninit_head_ptr(p_multisample_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_multisample_properties.as_mut_ptr());
    p_multisample_properties.assume_init()
}
#[cfg(feature = "ext_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureKHR.html>"]
#[doc(alias = "vkCreateAccelerationStructureKHR")]
pub unsafe fn create_acceleration_structure_khr(
    device: &raw::Device,
    p_create_info: &AccelerationStructureCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<AccelerationStructureKHR> {
    let vulkan_command = dispatcher.create_acceleration_structure_khr.get();
    let mut p_acceleration_structure = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_acceleration_structure.as_mut_ptr(),
    );
    vk_status.map_success(|| p_acceleration_structure.assume_init())
}
#[cfg(feature = "ext_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureKHR.html>"]
#[doc(alias = "vkDestroyAccelerationStructureKHR")]
#[inline]
pub unsafe fn destroy_acceleration_structure_khr(
    device: &raw::Device,
    acceleration_structure: Option<&raw::AccelerationStructureKHR>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_acceleration_structure_khr.get();
    vulkan_command(
        Some(device.borrow()),
        acceleration_structure.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresKHR.html>"]
#[doc(alias = "vkCmdBuildAccelerationStructuresKHR")]
#[inline]
pub unsafe fn cmd_build_acceleration_structures_khr<'a>(
    command_buffer: &raw::CommandBuffer,
    p_infos: impl AsSlice<'a, AccelerationStructureBuildGeometryInfoKHR<'a>>,
    pp_build_range_infos: &&AccelerationStructureBuildRangeInfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_build_acceleration_structures_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_infos.as_slice().len() as _,
        p_infos.as_slice().as_ptr().cast(),
        ptr::from_ref(pp_build_range_infos).cast(),
    )
}
#[cfg(feature = "ext_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresIndirectKHR.html>"]
#[doc(alias = "vkCmdBuildAccelerationStructuresIndirectKHR")]
#[inline]
pub unsafe fn cmd_build_acceleration_structures_indirect_khr<'a>(
    command_buffer: &raw::CommandBuffer,
    p_infos: impl AsSlice<'a, AccelerationStructureBuildGeometryInfoKHR<'a>>,
    p_indirect_device_addresses: impl AsSlice<'a, DeviceAddress>,
    p_indirect_strides: impl AsSlice<'a, u32>,
    pp_max_primitive_counts: &&u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_build_acceleration_structures_indirect_khr
        .get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_indirect_strides.as_slice().len() as _,
        p_infos.as_slice().as_ptr().cast(),
        p_indirect_device_addresses.as_slice().as_ptr().cast(),
        p_indirect_strides.as_slice().as_ptr().cast(),
        ptr::from_ref(pp_max_primitive_counts).cast(),
    )
}
#[cfg(feature = "ext_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildAccelerationStructuresKHR.html>"]
#[doc(alias = "vkBuildAccelerationStructuresKHR")]
#[inline]
pub unsafe fn build_acceleration_structures_khr<'a>(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    p_infos: impl AsSlice<'a, AccelerationStructureBuildGeometryInfoKHR<'a>>,
    pp_build_range_infos: &&AccelerationStructureBuildRangeInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.build_acceleration_structures_khr.get();
    vulkan_command(
        Some(device.borrow()),
        deferred_operation.map(|v| v.borrow()),
        p_infos.as_slice().len() as _,
        p_infos.as_slice().as_ptr().cast(),
        ptr::from_ref(pp_build_range_infos).cast(),
    )
    .into_result()
}
#[cfg(feature = "ext_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureKHR.html>"]
#[doc(alias = "vkCopyAccelerationStructureKHR")]
#[inline]
pub unsafe fn copy_acceleration_structure_khr(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    p_info: &CopyAccelerationStructureInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.copy_acceleration_structure_khr.get();
    vulkan_command(
        Some(device.borrow()),
        deferred_operation.map(|v| v.borrow()),
        ptr::from_ref(p_info),
    )
    .into_result()
}
#[cfg(feature = "ext_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureToMemoryKHR.html>"]
#[doc(alias = "vkCopyAccelerationStructureToMemoryKHR")]
#[inline]
pub unsafe fn copy_acceleration_structure_to_memory_khr(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    p_info: &CopyAccelerationStructureToMemoryInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.copy_acceleration_structure_to_memory_khr.get();
    vulkan_command(
        Some(device.borrow()),
        deferred_operation.map(|v| v.borrow()),
        ptr::from_ref(p_info),
    )
    .into_result()
}
#[cfg(feature = "ext_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToAccelerationStructureKHR.html>"]
#[doc(alias = "vkCopyMemoryToAccelerationStructureKHR")]
#[inline]
pub unsafe fn copy_memory_to_acceleration_structure_khr(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    p_info: &CopyMemoryToAccelerationStructureInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.copy_memory_to_acceleration_structure_khr.get();
    vulkan_command(
        Some(device.borrow()),
        deferred_operation.map(|v| v.borrow()),
        ptr::from_ref(p_info),
    )
    .into_result()
}
#[cfg(feature = "ext_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteAccelerationStructuresPropertiesKHR.html>"]
#[doc(alias = "vkWriteAccelerationStructuresPropertiesKHR")]
#[inline]
pub unsafe fn write_acceleration_structures_properties_khr<
    'a,
    V2: Alias<raw::AccelerationStructureKHR> + 'a,
>(
    device: &raw::Device,
    p_acceleration_structures: impl AsSlice<'a, V2>,
    query_type: QueryType,
    data_size: usize,
    p_data: VoidPtr,
    stride: usize,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .write_acceleration_structures_properties_khr
        .get();
    vulkan_command(
        Some(device.borrow()),
        p_acceleration_structures.as_slice().len() as _,
        p_acceleration_structures.as_slice().as_ptr().cast(),
        query_type,
        data_size,
        p_data,
        stride,
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureKHR.html>"]
#[doc(alias = "vkCmdCopyAccelerationStructureKHR")]
#[inline]
pub unsafe fn cmd_copy_acceleration_structure_khr(
    command_buffer: &raw::CommandBuffer,
    p_info: &CopyAccelerationStructureInfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_acceleration_structure_khr.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_info))
}
#[cfg(feature = "ext_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureToMemoryKHR.html>"]
#[doc(alias = "vkCmdCopyAccelerationStructureToMemoryKHR")]
#[inline]
pub unsafe fn cmd_copy_acceleration_structure_to_memory_khr(
    command_buffer: &raw::CommandBuffer,
    p_info: &CopyAccelerationStructureToMemoryInfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_acceleration_structure_to_memory_khr
        .get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_info))
}
#[cfg(feature = "ext_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToAccelerationStructureKHR.html>"]
#[doc(alias = "vkCmdCopyMemoryToAccelerationStructureKHR")]
#[inline]
pub unsafe fn cmd_copy_memory_to_acceleration_structure_khr(
    command_buffer: &raw::CommandBuffer,
    p_info: &CopyMemoryToAccelerationStructureInfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_memory_to_acceleration_structure_khr
        .get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_info))
}
#[cfg(feature = "ext_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureDeviceAddressKHR.html>"]
#[doc(alias = "vkGetAccelerationStructureDeviceAddressKHR")]
#[inline]
pub unsafe fn get_acceleration_structure_device_address_khr(
    device: &raw::Device,
    p_info: &AccelerationStructureDeviceAddressInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> DeviceAddress {
    let vulkan_command = dispatcher
        .get_acceleration_structure_device_address_khr
        .get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info))
}
#[cfg(feature = "ext_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesKHR.html>"]
#[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesKHR")]
#[inline]
pub unsafe fn cmd_write_acceleration_structures_properties_khr<
    'a,
    V2: Alias<raw::AccelerationStructureKHR> + 'a,
>(
    command_buffer: &raw::CommandBuffer,
    p_acceleration_structures: impl AsSlice<'a, V2>,
    query_type: QueryType,
    query_pool: &raw::QueryPool,
    first_query: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_write_acceleration_structures_properties_khr
        .get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_acceleration_structures.as_slice().len() as _,
        p_acceleration_structures.as_slice().as_ptr().cast(),
        query_type,
        Some(query_pool.borrow()),
        first_query,
    )
}
#[cfg(feature = "ext_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceAccelerationStructureCompatibilityKHR.html>"]
#[doc(alias = "vkGetDeviceAccelerationStructureCompatibilityKHR")]
pub unsafe fn get_device_acceleration_structure_compatibility_khr(
    device: &raw::Device,
    p_version_info: &AccelerationStructureVersionInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> AccelerationStructureCompatibilityKHR {
    let vulkan_command = dispatcher
        .get_device_acceleration_structure_compatibility_khr
        .get();
    let mut p_compatibility = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_version_info),
        p_compatibility.as_mut_ptr(),
    );
    p_compatibility.assume_init()
}
#[cfg(feature = "ext_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureBuildSizesKHR.html>"]
#[doc(alias = "vkGetAccelerationStructureBuildSizesKHR")]
pub unsafe fn get_acceleration_structure_build_sizes_khr<
    'a,
    S: StructureChainOut<AccelerationStructureBuildSizesInfoKHR<'static>>,
>(
    device: &raw::Device,
    build_type: AccelerationStructureBuildTypeKHR,
    p_build_info: &AccelerationStructureBuildGeometryInfoKHR,
    p_max_primitive_counts: Option<impl AsSlice<'a, u32>>,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_acceleration_structure_build_sizes_khr.get();
    let mut p_size_info = MaybeUninit::uninit();
    S::setup_uninit(&mut p_size_info);
    vulkan_command(
        Some(device.borrow()),
        build_type,
        ptr::from_ref(p_build_info),
        p_max_primitive_counts
            .map(|p| p.as_slice().as_ptr().cast())
            .unwrap_or(ptr::null()),
        S::get_uninit_head_ptr(p_size_info.as_mut_ptr()),
    );
    S::setup_cleanup(p_size_info.as_mut_ptr());
    p_size_info.assume_init()
}
#[cfg(feature = "ext_ray_tracing_pipeline")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysKHR.html>"]
#[doc(alias = "vkCmdTraceRaysKHR")]
#[inline]
pub unsafe fn cmd_trace_rays_khr(
    command_buffer: &raw::CommandBuffer,
    p_raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
    p_miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
    p_hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
    p_callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
    width: u32,
    height: u32,
    depth: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_trace_rays_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_raygen_shader_binding_table),
        ptr::from_ref(p_miss_shader_binding_table),
        ptr::from_ref(p_hit_shader_binding_table),
        ptr::from_ref(p_callable_shader_binding_table),
        width,
        height,
        depth,
    )
}
#[cfg(feature = "ext_ray_tracing_pipeline")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesKHR.html>"]
#[doc(alias = "vkCreateRayTracingPipelinesKHR")]
pub unsafe fn create_ray_tracing_pipelines_khr<'a, R: DynamicArray<Pipeline>>(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    pipeline_cache: Option<&raw::PipelineCache>,
    p_create_infos: impl AsSlice<'a, RayTracingPipelineCreateInfoKHR<'a>>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, R)> {
    let vulkan_command = dispatcher.create_ray_tracing_pipelines_khr.get();
    let mut p_pipelines = R::create_with_capacity(p_create_infos.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        deferred_operation.map(|v| v.borrow()),
        pipeline_cache.map(|v| v.borrow()),
        p_create_infos.as_slice().len() as _,
        p_create_infos.as_slice().as_ptr().cast(),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_pipelines.get_content_mut_ptr(),
    );
    vk_status.map_successes(|| {
        p_pipelines.resize_with_len(p_create_infos.as_slice().len() as _);
        p_pipelines
    })
}
#[cfg(any(feature = "ext_ray_tracing_pipeline", feature = "ext_ray_tracing"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesKHR.html>"]
#[doc(alias = "vkGetRayTracingShaderGroupHandlesKHR")]
#[inline]
pub unsafe fn get_ray_tracing_shader_group_handles_khr(
    device: &raw::Device,
    pipeline: &raw::Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.get_ray_tracing_shader_group_handles_khr.get();
    vulkan_command(
        Some(device.borrow()),
        Some(pipeline.borrow()),
        first_group,
        group_count,
        data_size,
        p_data,
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_ray_tracing_pipeline", feature = "ext_ray_tracing"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesNV.html>"]
#[doc(alias = "vkGetRayTracingShaderGroupHandlesNV")]
#[inline]
pub unsafe fn get_ray_tracing_shader_group_handles_nv(
    device: &raw::Device,
    pipeline: &raw::Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.get_ray_tracing_shader_group_handles_nv.get();
    vulkan_command(
        Some(device.borrow()),
        Some(pipeline.borrow()),
        first_group,
        group_count,
        data_size,
        p_data,
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_ray_tracing_pipeline")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html>"]
#[doc(alias = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")]
#[inline]
pub unsafe fn get_ray_tracing_capture_replay_shader_group_handles_khr(
    device: &raw::Device,
    pipeline: &raw::Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_ray_tracing_capture_replay_shader_group_handles_khr
        .get();
    vulkan_command(
        Some(device.borrow()),
        Some(pipeline.borrow()),
        first_group,
        group_count,
        data_size,
        p_data,
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_ray_tracing_pipeline")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirectKHR.html>"]
#[doc(alias = "vkCmdTraceRaysIndirectKHR")]
#[inline]
pub unsafe fn cmd_trace_rays_indirect_khr(
    command_buffer: &raw::CommandBuffer,
    p_raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
    p_miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
    p_hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
    p_callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
    indirect_device_address: DeviceAddress,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_trace_rays_indirect_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_raygen_shader_binding_table),
        ptr::from_ref(p_miss_shader_binding_table),
        ptr::from_ref(p_hit_shader_binding_table),
        ptr::from_ref(p_callable_shader_binding_table),
        indirect_device_address,
    )
}
#[cfg(feature = "ext_ray_tracing_pipeline")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupStackSizeKHR.html>"]
#[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
#[inline]
pub unsafe fn get_ray_tracing_shader_group_stack_size_khr(
    device: &raw::Device,
    pipeline: &raw::Pipeline,
    group: u32,
    group_shader: ShaderGroupShaderKHR,
    dispatcher: &CommandsDispatcher,
) -> DeviceSize {
    let vulkan_command = dispatcher.get_ray_tracing_shader_group_stack_size_khr.get();
    vulkan_command(
        Some(device.borrow()),
        Some(pipeline.borrow()),
        group,
        group_shader,
    )
}
#[cfg(feature = "ext_ray_tracing_pipeline")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRayTracingPipelineStackSizeKHR.html>"]
#[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
#[inline]
pub unsafe fn cmd_set_ray_tracing_pipeline_stack_size_khr(
    command_buffer: &raw::CommandBuffer,
    pipeline_stack_size: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_ray_tracing_pipeline_stack_size_khr.get();
    vulkan_command(Some(command_buffer.borrow()), pipeline_stack_size)
}
#[cfg(feature = "ext_image_drm_format_modifier")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageDrmFormatModifierPropertiesEXT.html>"]
#[doc(alias = "vkGetImageDrmFormatModifierPropertiesEXT")]
pub unsafe fn get_image_drm_format_modifier_properties_ext<
    S: StructureChainOut<ImageDrmFormatModifierPropertiesEXT<'static>>,
>(
    device: &raw::Device,
    image: &raw::Image,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_image_drm_format_modifier_properties_ext
        .get();
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(image.borrow()),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_properties.as_mut_ptr());
        p_properties.assume_init()
    })
}
#[cfg(feature = "ext_validation_cache")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateValidationCacheEXT.html>"]
#[doc(alias = "vkCreateValidationCacheEXT")]
pub unsafe fn create_validation_cache_ext(
    device: &raw::Device,
    p_create_info: &ValidationCacheCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<ValidationCacheEXT> {
    let vulkan_command = dispatcher.create_validation_cache_ext.get();
    let mut p_validation_cache = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_validation_cache.as_mut_ptr(),
    );
    vk_status.map_success(|| p_validation_cache.assume_init())
}
#[cfg(feature = "ext_validation_cache")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyValidationCacheEXT.html>"]
#[doc(alias = "vkDestroyValidationCacheEXT")]
#[inline]
pub unsafe fn destroy_validation_cache_ext(
    device: &raw::Device,
    validation_cache: Option<&raw::ValidationCacheEXT>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_validation_cache_ext.get();
    vulkan_command(
        Some(device.borrow()),
        validation_cache.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_validation_cache")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkMergeValidationCachesEXT.html>"]
#[doc(alias = "vkMergeValidationCachesEXT")]
#[inline]
pub unsafe fn merge_validation_caches_ext<'a, V3: Alias<raw::ValidationCacheEXT> + 'a>(
    device: &raw::Device,
    dst_cache: &raw::ValidationCacheEXT,
    p_src_caches: impl AsSlice<'a, V3>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.merge_validation_caches_ext.get();
    vulkan_command(
        Some(device.borrow()),
        Some(dst_cache.borrow()),
        p_src_caches.as_slice().len() as _,
        p_src_caches.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_validation_cache")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetValidationCacheDataEXT.html>"]
#[doc(alias = "vkGetValidationCacheDataEXT")]
pub unsafe fn get_validation_cache_data_ext(
    device: &raw::Device,
    validation_cache: &raw::ValidationCacheEXT,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<usize> {
    let vulkan_command = dispatcher.get_validation_cache_data_ext.get();
    let mut p_data_size = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(validation_cache.borrow()),
        p_data_size.as_mut_ptr(),
        p_data,
    );
    vk_status.map_success(|| p_data_size.assume_init())
}
#[cfg(feature = "ext_shading_rate_image")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadingRateImageNV.html>"]
#[doc(alias = "vkCmdBindShadingRateImageNV")]
#[inline]
pub unsafe fn cmd_bind_shading_rate_image_nv(
    command_buffer: &raw::CommandBuffer,
    image_view: Option<&raw::ImageView>,
    image_layout: ImageLayout,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_shading_rate_image_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        image_view.map(|v| v.borrow()),
        image_layout,
    )
}
#[cfg(feature = "ext_shading_rate_image")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportShadingRatePaletteNV.html>"]
#[doc(alias = "vkCmdSetViewportShadingRatePaletteNV")]
#[inline]
pub unsafe fn cmd_set_viewport_shading_rate_palette_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    first_viewport: u32,
    p_shading_rate_palettes: impl AsSlice<'a, ShadingRatePaletteNV<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_viewport_shading_rate_palette_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_viewport,
        p_shading_rate_palettes.as_slice().len() as _,
        p_shading_rate_palettes.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_shading_rate_image")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoarseSampleOrderNV.html>"]
#[doc(alias = "vkCmdSetCoarseSampleOrderNV")]
#[inline]
pub unsafe fn cmd_set_coarse_sample_order_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    sample_order_type: CoarseSampleOrderTypeNV,
    p_custom_sample_orders: impl AsSlice<'a, CoarseSampleOrderCustomNV<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_coarse_sample_order_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        sample_order_type,
        p_custom_sample_orders.as_slice().len() as _,
        p_custom_sample_orders.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_ray_tracing")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureNV.html>"]
#[doc(alias = "vkCreateAccelerationStructureNV")]
pub unsafe fn create_acceleration_structure_nv(
    device: &raw::Device,
    p_create_info: &AccelerationStructureCreateInfoNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<AccelerationStructureNV> {
    let vulkan_command = dispatcher.create_acceleration_structure_nv.get();
    let mut p_acceleration_structure = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_acceleration_structure.as_mut_ptr(),
    );
    vk_status.map_success(|| p_acceleration_structure.assume_init())
}
#[cfg(feature = "ext_ray_tracing")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureNV.html>"]
#[doc(alias = "vkDestroyAccelerationStructureNV")]
#[inline]
pub unsafe fn destroy_acceleration_structure_nv(
    device: &raw::Device,
    acceleration_structure: Option<&raw::AccelerationStructureNV>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_acceleration_structure_nv.get();
    vulkan_command(
        Some(device.borrow()),
        acceleration_structure.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_ray_tracing")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureMemoryRequirementsNV.html>"]
#[doc(alias = "vkGetAccelerationStructureMemoryRequirementsNV")]
pub unsafe fn get_acceleration_structure_memory_requirements_nv<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &AccelerationStructureMemoryRequirementsInfoNV,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_acceleration_structure_memory_requirements_nv
        .get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[cfg(feature = "ext_ray_tracing")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindAccelerationStructureMemoryNV.html>"]
#[doc(alias = "vkBindAccelerationStructureMemoryNV")]
#[inline]
pub unsafe fn bind_acceleration_structure_memory_nv<'a>(
    device: &raw::Device,
    p_bind_infos: impl AsSlice<'a, BindAccelerationStructureMemoryInfoNV<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.bind_acceleration_structure_memory_nv.get();
    vulkan_command(
        Some(device.borrow()),
        p_bind_infos.as_slice().len() as _,
        p_bind_infos.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_ray_tracing")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructureNV.html>"]
#[doc(alias = "vkCmdBuildAccelerationStructureNV")]
#[inline]
pub unsafe fn cmd_build_acceleration_structure_nv(
    command_buffer: &raw::CommandBuffer,
    p_info: &AccelerationStructureInfoNV,
    instance_data: Option<&raw::Buffer>,
    instance_offset: DeviceSize,
    update: impl Into<Bool32>,
    dst: &raw::AccelerationStructureNV,
    src: Option<&raw::AccelerationStructureNV>,
    scratch: &raw::Buffer,
    scratch_offset: DeviceSize,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_build_acceleration_structure_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_info),
        instance_data.map(|v| v.borrow()),
        instance_offset,
        update.into(),
        Some(dst.borrow()),
        src.map(|v| v.borrow()),
        Some(scratch.borrow()),
        scratch_offset,
    )
}
#[cfg(feature = "ext_ray_tracing")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureNV.html>"]
#[doc(alias = "vkCmdCopyAccelerationStructureNV")]
#[inline]
pub unsafe fn cmd_copy_acceleration_structure_nv(
    command_buffer: &raw::CommandBuffer,
    dst: &raw::AccelerationStructureNV,
    src: &raw::AccelerationStructureNV,
    mode: CopyAccelerationStructureModeKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_acceleration_structure_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(dst.borrow()),
        Some(src.borrow()),
        mode,
    )
}
#[cfg(feature = "ext_ray_tracing")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysNV.html>"]
#[doc(alias = "vkCmdTraceRaysNV")]
#[inline]
pub unsafe fn cmd_trace_rays_nv(
    command_buffer: &raw::CommandBuffer,
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
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_trace_rays_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(raygen_shader_binding_table_buffer.borrow()),
        raygen_shader_binding_offset,
        miss_shader_binding_table_buffer.map(|v| v.borrow()),
        miss_shader_binding_offset,
        miss_shader_binding_stride,
        hit_shader_binding_table_buffer.map(|v| v.borrow()),
        hit_shader_binding_offset,
        hit_shader_binding_stride,
        callable_shader_binding_table_buffer.map(|v| v.borrow()),
        callable_shader_binding_offset,
        callable_shader_binding_stride,
        width,
        height,
        depth,
    )
}
#[cfg(feature = "ext_ray_tracing")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesNV.html>"]
#[doc(alias = "vkCreateRayTracingPipelinesNV")]
pub unsafe fn create_ray_tracing_pipelines_nv<'a, R: DynamicArray<Pipeline>>(
    device: &raw::Device,
    pipeline_cache: Option<&raw::PipelineCache>,
    p_create_infos: impl AsSlice<'a, RayTracingPipelineCreateInfoNV<'a>>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, R)> {
    let vulkan_command = dispatcher.create_ray_tracing_pipelines_nv.get();
    let mut p_pipelines = R::create_with_capacity(p_create_infos.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        pipeline_cache.map(|v| v.borrow()),
        p_create_infos.as_slice().len() as _,
        p_create_infos.as_slice().as_ptr().cast(),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_pipelines.get_content_mut_ptr(),
    );
    vk_status.map_successes(|| {
        p_pipelines.resize_with_len(p_create_infos.as_slice().len() as _);
        p_pipelines
    })
}
#[cfg(feature = "ext_ray_tracing")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureHandleNV.html>"]
#[doc(alias = "vkGetAccelerationStructureHandleNV")]
#[inline]
pub unsafe fn get_acceleration_structure_handle_nv(
    device: &raw::Device,
    acceleration_structure: &raw::AccelerationStructureNV,
    data_size: usize,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.get_acceleration_structure_handle_nv.get();
    vulkan_command(
        Some(device.borrow()),
        Some(acceleration_structure.borrow()),
        data_size,
        p_data,
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_ray_tracing")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesNV.html>"]
#[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesNV")]
#[inline]
pub unsafe fn cmd_write_acceleration_structures_properties_nv<
    'a,
    V2: Alias<raw::AccelerationStructureNV> + 'a,
>(
    command_buffer: &raw::CommandBuffer,
    p_acceleration_structures: impl AsSlice<'a, V2>,
    query_type: QueryType,
    query_pool: &raw::QueryPool,
    first_query: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_write_acceleration_structures_properties_nv
        .get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_acceleration_structures.as_slice().len() as _,
        p_acceleration_structures.as_slice().as_ptr().cast(),
        query_type,
        Some(query_pool.borrow()),
        first_query,
    )
}
#[cfg(feature = "ext_ray_tracing")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCompileDeferredNV.html>"]
#[doc(alias = "vkCompileDeferredNV")]
#[inline]
pub unsafe fn compile_deferred_nv(
    device: &raw::Device,
    pipeline: &raw::Pipeline,
    shader: u32,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.compile_deferred_nv.get();
    vulkan_command(Some(device.borrow()), Some(pipeline.borrow()), shader).map_success(|| ())
}
#[cfg(feature = "ext_external_memory_host")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryHostPointerPropertiesEXT.html>"]
#[doc(alias = "vkGetMemoryHostPointerPropertiesEXT")]
pub unsafe fn get_memory_host_pointer_properties_ext<
    S: StructureChainOut<MemoryHostPointerPropertiesEXT<'static>>,
>(
    device: &raw::Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    p_host_pointer: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.get_memory_host_pointer_properties_ext.get();
    let mut p_memory_host_pointer_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_host_pointer_properties);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        handle_type,
        p_host_pointer,
        S::get_uninit_head_ptr(p_memory_host_pointer_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_memory_host_pointer_properties.as_mut_ptr());
        p_memory_host_pointer_properties.assume_init()
    })
}
#[cfg(feature = "ext_buffer_marker")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarkerAMD.html>"]
#[doc(alias = "vkCmdWriteBufferMarkerAMD")]
#[inline]
pub unsafe fn cmd_write_buffer_marker_amd(
    command_buffer: &raw::CommandBuffer,
    pipeline_stage: PipelineStageFlags,
    dst_buffer: &raw::Buffer,
    dst_offset: DeviceSize,
    marker: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_write_buffer_marker_amd.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        pipeline_stage,
        Some(dst_buffer.borrow()),
        dst_offset,
        marker,
    )
}
#[cfg(all(
    feature = "ext_buffer_marker",
    any(feature = "version_1_3", feature = "ext_synchronization2")
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarker2AMD.html>"]
#[doc(alias = "vkCmdWriteBufferMarker2AMD")]
#[inline]
pub unsafe fn cmd_write_buffer_marker2_amd(
    command_buffer: &raw::CommandBuffer,
    stage: PipelineStageFlags2,
    dst_buffer: &raw::Buffer,
    dst_offset: DeviceSize,
    marker: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_write_buffer_marker2_amd.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        stage,
        Some(dst_buffer.borrow()),
        dst_offset,
        marker,
    )
}
#[cfg(feature = "ext_mesh_shader")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksNV.html>"]
#[doc(alias = "vkCmdDrawMeshTasksNV")]
#[inline]
pub unsafe fn cmd_draw_mesh_tasks_nv(
    command_buffer: &raw::CommandBuffer,
    task_count: u32,
    first_task: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_mesh_tasks_nv.get();
    vulkan_command(Some(command_buffer.borrow()), task_count, first_task)
}
#[cfg(feature = "ext_mesh_shader")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectNV.html>"]
#[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
#[inline]
pub unsafe fn cmd_draw_mesh_tasks_indirect_nv(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_mesh_tasks_indirect_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(buffer.borrow()),
        offset,
        draw_count,
        stride,
    )
}
#[cfg(all(
    feature = "ext_mesh_shader",
    any(feature = "version_1_2", feature = "ext_draw_indirect_count")
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountNV.html>"]
#[doc(alias = "vkCmdDrawMeshTasksIndirectCountNV")]
#[inline]
pub unsafe fn cmd_draw_mesh_tasks_indirect_count_nv(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    count_buffer: &raw::Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_mesh_tasks_indirect_count_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(buffer.borrow()),
        offset,
        Some(count_buffer.borrow()),
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[cfg(feature = "ext_scissor_exclusive")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorEnableNV.html>"]
#[doc(alias = "vkCmdSetExclusiveScissorEnableNV")]
#[inline]
pub unsafe fn cmd_set_exclusive_scissor_enable_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    first_exclusive_scissor: u32,
    p_exclusive_scissor_enables: impl AsSlice<'a, Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_exclusive_scissor_enable_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_exclusive_scissor,
        p_exclusive_scissor_enables.as_slice().len() as _,
        p_exclusive_scissor_enables.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_scissor_exclusive")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorNV.html>"]
#[doc(alias = "vkCmdSetExclusiveScissorNV")]
#[inline]
pub unsafe fn cmd_set_exclusive_scissor_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    first_exclusive_scissor: u32,
    p_exclusive_scissors: impl AsSlice<'a, Rect2D>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_exclusive_scissor_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_exclusive_scissor,
        p_exclusive_scissors.as_slice().len() as _,
        p_exclusive_scissors.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_device_diagnostic_checkpoints")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCheckpointNV.html>"]
#[doc(alias = "vkCmdSetCheckpointNV")]
#[inline]
pub unsafe fn cmd_set_checkpoint_nv(
    command_buffer: &raw::CommandBuffer,
    p_checkpoint_marker: VoidPtr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_checkpoint_nv.get();
    vulkan_command(Some(command_buffer.borrow()), p_checkpoint_marker)
}
#[cfg(feature = "ext_device_diagnostic_checkpoints")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointDataNV.html>"]
#[doc(alias = "vkGetQueueCheckpointDataNV")]
pub unsafe fn get_queue_checkpoint_data_nv<R: DynamicArray<CheckpointDataNV<'static>>>(
    queue: &raw::Queue,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher.get_queue_checkpoint_data_nv.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_checkpoint_data_count = vk_len.as_mut_ptr();
    let p_checkpoint_data = ptr::null_mut();
    vulkan_command(
        Some(queue.borrow()),
        p_checkpoint_data_count,
        p_checkpoint_data,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_checkpoint_data_count = ptr::from_mut(&mut vk_len);
    let mut p_checkpoint_data = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(queue.borrow()),
        p_checkpoint_data_count,
        p_checkpoint_data,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[cfg(all(
    feature = "ext_device_diagnostic_checkpoints",
    any(feature = "version_1_3", feature = "ext_synchronization2")
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointData2NV.html>"]
#[doc(alias = "vkGetQueueCheckpointData2NV")]
pub unsafe fn get_queue_checkpoint_data2_nv<R: DynamicArray<CheckpointData2NV<'static>>>(
    queue: &raw::Queue,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher.get_queue_checkpoint_data2_nv.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_checkpoint_data_count = vk_len.as_mut_ptr();
    let p_checkpoint_data = ptr::null_mut();
    vulkan_command(
        Some(queue.borrow()),
        p_checkpoint_data_count,
        p_checkpoint_data,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_checkpoint_data_count = ptr::from_mut(&mut vk_len);
    let mut p_checkpoint_data = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(queue.borrow()),
        p_checkpoint_data_count,
        p_checkpoint_data,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[cfg(feature = "ext_present_timing")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetSwapchainPresentTimingQueueSizeEXT.html>"]
#[doc(alias = "vkSetSwapchainPresentTimingQueueSizeEXT")]
#[inline]
pub unsafe fn set_swapchain_present_timing_queue_size_ext(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    size: u32,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.set_swapchain_present_timing_queue_size_ext.get();
    vulkan_command(Some(device.borrow()), Some(swapchain.borrow()), size).into_result()
}
#[cfg(feature = "ext_present_timing")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingEXT.html>"]
#[doc(alias = "vkGetPastPresentationTimingEXT")]
pub unsafe fn get_past_presentation_timing_ext<
    S: StructureChainOut<PastPresentationTimingPropertiesEXT<'static>>,
>(
    device: &raw::Device,
    p_past_presentation_timing_info: &PastPresentationTimingInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.get_past_presentation_timing_ext.get();
    let mut p_past_presentation_timing_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_past_presentation_timing_properties);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_past_presentation_timing_info),
        S::get_uninit_head_ptr(p_past_presentation_timing_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_past_presentation_timing_properties.as_mut_ptr());
        p_past_presentation_timing_properties.assume_init()
    })
}
#[cfg(feature = "ext_performance_query")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkInitializePerformanceApiINTEL.html>"]
#[doc(alias = "vkInitializePerformanceApiINTEL")]
#[inline]
pub unsafe fn initialize_performance_api_intel(
    device: &raw::Device,
    p_initialize_info: &InitializePerformanceApiInfoINTEL,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.initialize_performance_api_intel.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_initialize_info)).map_success(|| ())
}
#[cfg(feature = "ext_performance_query")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUninitializePerformanceApiINTEL.html>"]
#[doc(alias = "vkUninitializePerformanceApiINTEL")]
#[inline]
pub unsafe fn uninitialize_performance_api_intel(
    device: &raw::Device,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.uninitialize_performance_api_intel.get();
    vulkan_command(Some(device.borrow()))
}
#[cfg(feature = "ext_performance_query")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceMarkerINTEL.html>"]
#[doc(alias = "vkCmdSetPerformanceMarkerINTEL")]
#[inline]
pub unsafe fn cmd_set_performance_marker_intel(
    command_buffer: &raw::CommandBuffer,
    p_marker_info: &PerformanceMarkerInfoINTEL,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.cmd_set_performance_marker_intel.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_marker_info)).map_success(|| ())
}
#[cfg(feature = "ext_performance_query")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceStreamMarkerINTEL.html>"]
#[doc(alias = "vkCmdSetPerformanceStreamMarkerINTEL")]
#[inline]
pub unsafe fn cmd_set_performance_stream_marker_intel(
    command_buffer: &raw::CommandBuffer,
    p_marker_info: &PerformanceStreamMarkerInfoINTEL,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.cmd_set_performance_stream_marker_intel.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_marker_info)).map_success(|| ())
}
#[cfg(feature = "ext_performance_query")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceOverrideINTEL.html>"]
#[doc(alias = "vkCmdSetPerformanceOverrideINTEL")]
#[inline]
pub unsafe fn cmd_set_performance_override_intel(
    command_buffer: &raw::CommandBuffer,
    p_override_info: &PerformanceOverrideInfoINTEL,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.cmd_set_performance_override_intel.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_override_info),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_performance_query")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquirePerformanceConfigurationINTEL.html>"]
#[doc(alias = "vkAcquirePerformanceConfigurationINTEL")]
pub unsafe fn acquire_performance_configuration_intel(
    device: &raw::Device,
    p_acquire_info: &PerformanceConfigurationAcquireInfoINTEL,
    dispatcher: &CommandsDispatcher,
) -> Result<PerformanceConfigurationINTEL> {
    let vulkan_command = dispatcher.acquire_performance_configuration_intel.get();
    let mut p_configuration = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_acquire_info),
        p_configuration.as_mut_ptr(),
    );
    vk_status.map_success(|| p_configuration.assume_init())
}
#[cfg(feature = "ext_performance_query")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkReleasePerformanceConfigurationINTEL.html>"]
#[doc(alias = "vkReleasePerformanceConfigurationINTEL")]
#[inline]
pub unsafe fn release_performance_configuration_intel(
    device: &raw::Device,
    configuration: Option<&raw::PerformanceConfigurationINTEL>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.release_performance_configuration_intel.get();
    vulkan_command(Some(device.borrow()), configuration.map(|v| v.borrow())).map_success(|| ())
}
#[cfg(feature = "ext_performance_query")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerformanceConfigurationINTEL.html>"]
#[doc(alias = "vkQueueSetPerformanceConfigurationINTEL")]
#[inline]
pub unsafe fn queue_set_performance_configuration_intel(
    queue: &raw::Queue,
    configuration: &raw::PerformanceConfigurationINTEL,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.queue_set_performance_configuration_intel.get();
    vulkan_command(Some(queue.borrow()), Some(configuration.borrow())).map_success(|| ())
}
#[cfg(feature = "ext_performance_query")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPerformanceParameterINTEL.html>"]
#[doc(alias = "vkGetPerformanceParameterINTEL")]
pub unsafe fn get_performance_parameter_intel(
    device: &raw::Device,
    parameter: PerformanceParameterTypeINTEL,
    dispatcher: &CommandsDispatcher,
) -> Result<PerformanceValueINTEL> {
    let vulkan_command = dispatcher.get_performance_parameter_intel.get();
    let mut p_value = MaybeUninit::uninit();
    let vk_status = vulkan_command(Some(device.borrow()), parameter, p_value.as_mut_ptr());
    vk_status.map_success(|| p_value.assume_init())
}
#[cfg(feature = "ext_display_native_hdr")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLocalDimmingAMD.html>"]
#[doc(alias = "vkSetLocalDimmingAMD")]
#[inline]
pub unsafe fn set_local_dimming_amd(
    device: &raw::Device,
    swap_chain: &raw::SwapchainKHR,
    local_dimming_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.set_local_dimming_amd.get();
    vulkan_command(
        Some(device.borrow()),
        Some(swap_chain.borrow()),
        local_dimming_enable.into(),
    )
}
#[cfg(feature = "ext_imagepipe_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImagePipeSurfaceFUCHSIA.html>"]
#[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
pub unsafe fn create_image_pipe_surface_fuchsia(
    instance: &raw::Instance,
    p_create_info: &ImagePipeSurfaceCreateInfoFUCHSIA,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_image_pipe_surface_fuchsia.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_metal_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMetalSurfaceEXT.html>"]
#[doc(alias = "vkCreateMetalSurfaceEXT")]
pub unsafe fn create_metal_surface_ext(
    instance: &raw::Instance,
    p_create_info: &MetalSurfaceCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_metal_surface_ext.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_fragment_shading_rate")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFragmentShadingRatesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceFragmentShadingRatesKHR")]
pub unsafe fn get_physical_device_fragment_shading_rates_khr<
    R: DynamicArray<PhysicalDeviceFragmentShadingRateKHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_fragment_shading_rates_khr
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_fragment_shading_rate_count = vk_len.as_mut_ptr();
    let p_fragment_shading_rates = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_fragment_shading_rate_count,
        p_fragment_shading_rates,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_fragment_shading_rate_count = ptr::from_mut(&mut vk_len);
    let mut p_fragment_shading_rates = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_fragment_shading_rate_count,
            p_fragment_shading_rates,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_fragment_shading_rate_count = ptr::from_mut(&mut vk_len);
        p_fragment_shading_rates = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_fragment_shading_rate")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateKHR.html>"]
#[doc(alias = "vkCmdSetFragmentShadingRateKHR")]
#[inline]
pub unsafe fn cmd_set_fragment_shading_rate_khr(
    command_buffer: &raw::CommandBuffer,
    p_fragment_size: &Extent2D,
    combiner_ops: [FragmentShadingRateCombinerOpKHR; 2u16 as _],
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_fragment_shading_rate_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_fragment_size),
        combiner_ops,
    )
}
#[cfg(feature = "ext_present_wait")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresentKHR.html>"]
#[doc(alias = "vkWaitForPresentKHR")]
#[inline]
pub unsafe fn wait_for_present_khr(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    present_id: u64,
    timeout: u64,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.wait_for_present_khr.get();
    vulkan_command(
        Some(device.borrow()),
        Some(swapchain.borrow()),
        present_id,
        timeout,
    )
    .into_result()
}
#[cfg(feature = "ext_cooperative_matrix")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html>"]
#[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV")]
pub unsafe fn get_physical_device_cooperative_matrix_properties_nv<
    R: DynamicArray<CooperativeMatrixPropertiesNV<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_cooperative_matrix_properties_nv
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_property_count,
        p_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_property_count,
            p_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_property_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_coverage_reduction_mode")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html>"]
#[doc(alias = "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")]
pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv<
    R: DynamicArray<FramebufferMixedSamplesCombinationNV<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_combination_count = vk_len.as_mut_ptr();
    let p_combinations = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_combination_count,
        p_combinations,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_combination_count = ptr::from_mut(&mut vk_len);
    let mut p_combinations = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_combination_count,
            p_combinations,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_combination_count = ptr::from_mut(&mut vk_len);
        p_combinations = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_full_screen_exclusive")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModes2EXT.html>"]
#[doc(alias = "vkGetPhysicalDeviceSurfacePresentModes2EXT")]
pub unsafe fn get_physical_device_surface_present_modes2_ext<R: DynamicArray<PresentModeKHR>>(
    physical_device: &raw::PhysicalDevice,
    p_surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_surface_present_modes2_ext
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_present_mode_count = vk_len.as_mut_ptr();
    let p_present_modes = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_surface_info),
        p_present_mode_count,
        p_present_modes,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_present_mode_count = ptr::from_mut(&mut vk_len);
    let mut p_present_modes = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            ptr::from_ref(p_surface_info),
            p_present_mode_count,
            p_present_modes,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_present_mode_count = ptr::from_mut(&mut vk_len);
        p_present_modes = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_full_screen_exclusive")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireFullScreenExclusiveModeEXT.html>"]
#[doc(alias = "vkAcquireFullScreenExclusiveModeEXT")]
#[inline]
pub unsafe fn acquire_full_screen_exclusive_mode_ext(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.acquire_full_screen_exclusive_mode_ext.get();
    vulkan_command(Some(device.borrow()), Some(swapchain.borrow())).map_success(|| ())
}
#[cfg(feature = "ext_full_screen_exclusive")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseFullScreenExclusiveModeEXT.html>"]
#[doc(alias = "vkReleaseFullScreenExclusiveModeEXT")]
#[inline]
pub unsafe fn release_full_screen_exclusive_mode_ext(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.release_full_screen_exclusive_mode_ext.get();
    vulkan_command(Some(device.borrow()), Some(swapchain.borrow())).map_success(|| ())
}
#[cfg(all(
    feature = "ext_full_screen_exclusive",
    any(feature = "ext_device_group", feature = "version_1_1")
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModes2EXT.html>"]
#[doc(alias = "vkGetDeviceGroupSurfacePresentModes2EXT")]
pub unsafe fn get_device_group_surface_present_modes2_ext(
    device: &raw::Device,
    p_surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    dispatcher: &CommandsDispatcher,
) -> Result<DeviceGroupPresentModeFlagsKHR> {
    let vulkan_command = dispatcher.get_device_group_surface_present_modes2_ext.get();
    let mut p_modes = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_surface_info),
        p_modes.as_mut_ptr(),
    );
    vk_status.map_success(|| p_modes.assume_init())
}
#[cfg(feature = "ext_headless_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateHeadlessSurfaceEXT.html>"]
#[doc(alias = "vkCreateHeadlessSurfaceEXT")]
pub unsafe fn create_headless_surface_ext(
    instance: &raw::Instance,
    p_create_info: &HeadlessSurfaceCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_headless_surface_ext.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_deferred_host_operations")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDeferredOperationKHR.html>"]
#[doc(alias = "vkCreateDeferredOperationKHR")]
pub unsafe fn create_deferred_operation_khr(
    device: &raw::Device,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DeferredOperationKHR> {
    let vulkan_command = dispatcher.create_deferred_operation_khr.get();
    let mut p_deferred_operation = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_deferred_operation.as_mut_ptr(),
    );
    vk_status.map_success(|| p_deferred_operation.assume_init())
}
#[cfg(feature = "ext_deferred_host_operations")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDeferredOperationKHR.html>"]
#[doc(alias = "vkDestroyDeferredOperationKHR")]
#[inline]
pub unsafe fn destroy_deferred_operation_khr(
    device: &raw::Device,
    operation: Option<&raw::DeferredOperationKHR>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_deferred_operation_khr.get();
    vulkan_command(
        Some(device.borrow()),
        operation.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_deferred_host_operations")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationMaxConcurrencyKHR.html>"]
#[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
#[inline]
pub unsafe fn get_deferred_operation_max_concurrency_khr(
    device: &raw::Device,
    operation: &raw::DeferredOperationKHR,
    dispatcher: &CommandsDispatcher,
) -> u32 {
    let vulkan_command = dispatcher.get_deferred_operation_max_concurrency_khr.get();
    vulkan_command(Some(device.borrow()), Some(operation.borrow()))
}
#[cfg(feature = "ext_deferred_host_operations")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationResultKHR.html>"]
#[doc(alias = "vkGetDeferredOperationResultKHR")]
#[inline]
pub unsafe fn get_deferred_operation_result_khr(
    device: &raw::Device,
    operation: &raw::DeferredOperationKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.get_deferred_operation_result_khr.get();
    vulkan_command(Some(device.borrow()), Some(operation.borrow())).into_result()
}
#[cfg(feature = "ext_deferred_host_operations")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDeferredOperationJoinKHR.html>"]
#[doc(alias = "vkDeferredOperationJoinKHR")]
#[inline]
pub unsafe fn deferred_operation_join_khr(
    device: &raw::Device,
    operation: &raw::DeferredOperationKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.deferred_operation_join_khr.get();
    vulkan_command(Some(device.borrow()), Some(operation.borrow())).into_result()
}
#[cfg(feature = "ext_pipeline_executable_properties")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutablePropertiesKHR.html>"]
#[doc(alias = "vkGetPipelineExecutablePropertiesKHR")]
pub unsafe fn get_pipeline_executable_properties_khr<
    R: DynamicArray<PipelineExecutablePropertiesKHR<'static>>,
>(
    device: &raw::Device,
    p_pipeline_info: &PipelineInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_pipeline_executable_properties_khr.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_executable_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_pipeline_info),
        p_executable_count,
        p_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_executable_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(device.borrow()),
            ptr::from_ref(p_pipeline_info),
            p_executable_count,
            p_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_executable_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_pipeline_executable_properties")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableStatisticsKHR.html>"]
#[doc(alias = "vkGetPipelineExecutableStatisticsKHR")]
pub unsafe fn get_pipeline_executable_statistics_khr<
    R: DynamicArray<PipelineExecutableStatisticKHR<'static>>,
>(
    device: &raw::Device,
    p_executable_info: &PipelineExecutableInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_pipeline_executable_statistics_khr.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_statistic_count = vk_len.as_mut_ptr();
    let p_statistics = ptr::null_mut();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_executable_info),
        p_statistic_count,
        p_statistics,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_statistic_count = ptr::from_mut(&mut vk_len);
    let mut p_statistics = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(device.borrow()),
            ptr::from_ref(p_executable_info),
            p_statistic_count,
            p_statistics,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_statistic_count = ptr::from_mut(&mut vk_len);
        p_statistics = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_pipeline_executable_properties")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableInternalRepresentationsKHR.html>"]
#[doc(alias = "vkGetPipelineExecutableInternalRepresentationsKHR")]
pub unsafe fn get_pipeline_executable_internal_representations_khr<
    R: DynamicArray<PipelineExecutableInternalRepresentationKHR<'static>>,
>(
    device: &raw::Device,
    p_executable_info: &PipelineExecutableInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_pipeline_executable_internal_representations_khr
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_internal_representation_count = vk_len.as_mut_ptr();
    let p_internal_representations = ptr::null_mut();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_executable_info),
        p_internal_representation_count,
        p_internal_representations,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_internal_representation_count = ptr::from_mut(&mut vk_len);
    let mut p_internal_representations = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(device.borrow()),
            ptr::from_ref(p_executable_info),
            p_internal_representation_count,
            p_internal_representations,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_internal_representation_count = ptr::from_mut(&mut vk_len);
        p_internal_representations = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_device_generated_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsNV.html>"]
#[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsNV")]
pub unsafe fn get_generated_commands_memory_requirements_nv<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &GeneratedCommandsMemoryRequirementsInfoNV,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_generated_commands_memory_requirements_nv
        .get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[cfg(feature = "ext_device_generated_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsNV.html>"]
#[doc(alias = "vkCmdPreprocessGeneratedCommandsNV")]
#[inline]
pub unsafe fn cmd_preprocess_generated_commands_nv(
    command_buffer: &raw::CommandBuffer,
    p_generated_commands_info: &GeneratedCommandsInfoNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_preprocess_generated_commands_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_generated_commands_info),
    )
}
#[cfg(feature = "ext_device_generated_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsNV.html>"]
#[doc(alias = "vkCmdExecuteGeneratedCommandsNV")]
#[inline]
pub unsafe fn cmd_execute_generated_commands_nv(
    command_buffer: &raw::CommandBuffer,
    is_preprocessed: impl Into<Bool32>,
    p_generated_commands_info: &GeneratedCommandsInfoNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_execute_generated_commands_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        is_preprocessed.into(),
        ptr::from_ref(p_generated_commands_info),
    )
}
#[cfg(feature = "ext_device_generated_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipelineShaderGroupNV.html>"]
#[doc(alias = "vkCmdBindPipelineShaderGroupNV")]
#[inline]
pub unsafe fn cmd_bind_pipeline_shader_group_nv(
    command_buffer: &raw::CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: &raw::Pipeline,
    group_index: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_pipeline_shader_group_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        pipeline_bind_point,
        Some(pipeline.borrow()),
        group_index,
    )
}
#[cfg(feature = "ext_device_generated_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutNV.html>"]
#[doc(alias = "vkCreateIndirectCommandsLayoutNV")]
pub unsafe fn create_indirect_commands_layout_nv(
    device: &raw::Device,
    p_create_info: &IndirectCommandsLayoutCreateInfoNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<IndirectCommandsLayoutNV> {
    let vulkan_command = dispatcher.create_indirect_commands_layout_nv.get();
    let mut p_indirect_commands_layout = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_indirect_commands_layout.as_mut_ptr(),
    );
    vk_status.map_success(|| p_indirect_commands_layout.assume_init())
}
#[cfg(feature = "ext_device_generated_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutNV.html>"]
#[doc(alias = "vkDestroyIndirectCommandsLayoutNV")]
#[inline]
pub unsafe fn destroy_indirect_commands_layout_nv(
    device: &raw::Device,
    indirect_commands_layout: Option<&raw::IndirectCommandsLayoutNV>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_indirect_commands_layout_nv.get();
    vulkan_command(
        Some(device.borrow()),
        indirect_commands_layout.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_depth_bias_control")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias2EXT.html>"]
#[doc(alias = "vkCmdSetDepthBias2EXT")]
#[inline]
pub unsafe fn cmd_set_depth_bias2_ext(
    command_buffer: &raw::CommandBuffer,
    p_depth_bias_info: &DepthBiasInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_bias2_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_depth_bias_info),
    )
}
#[cfg(feature = "ext_acquire_drm_display")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireDrmDisplayEXT.html>"]
#[doc(alias = "vkAcquireDrmDisplayEXT")]
#[inline]
pub unsafe fn acquire_drm_display_ext(
    physical_device: &raw::PhysicalDevice,
    drm_fd: i32,
    display: &raw::DisplayKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.acquire_drm_display_ext.get();
    vulkan_command(
        Some(physical_device.borrow()),
        drm_fd,
        Some(display.borrow()),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_acquire_drm_display")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDrmDisplayEXT.html>"]
#[doc(alias = "vkGetDrmDisplayEXT")]
pub unsafe fn get_drm_display_ext(
    physical_device: &raw::PhysicalDevice,
    drm_fd: i32,
    connector_id: u32,
    dispatcher: &CommandsDispatcher,
) -> Result<DisplayKHR> {
    let vulkan_command = dispatcher.get_drm_display_ext.get();
    let mut display = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(physical_device.borrow()),
        drm_fd,
        connector_id,
        display.as_mut_ptr(),
    );
    vk_status.map_success(|| display.assume_init())
}
#[cfg(feature = "ext_queue_perf_hint")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerfHintQCOM.html>"]
#[doc(alias = "vkQueueSetPerfHintQCOM")]
#[inline]
pub unsafe fn queue_set_perf_hint_qcom(
    queue: &raw::Queue,
    p_perf_hint_info: &PerfHintInfoQCOM,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.queue_set_perf_hint_qcom.get();
    vulkan_command(Some(queue.borrow()), ptr::from_ref(p_perf_hint_info)).map_success(|| ())
}
#[cfg(feature = "ext_cuda_kernel_launch")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaModuleNV.html>"]
#[doc(alias = "vkCreateCudaModuleNV")]
pub unsafe fn create_cuda_module_nv(
    device: &raw::Device,
    p_create_info: &CudaModuleCreateInfoNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<CudaModuleNV> {
    let vulkan_command = dispatcher.create_cuda_module_nv.get();
    let mut p_module = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_module.as_mut_ptr(),
    );
    vk_status.map_success(|| p_module.assume_init())
}
#[cfg(feature = "ext_cuda_kernel_launch")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCudaModuleCacheNV.html>"]
#[doc(alias = "vkGetCudaModuleCacheNV")]
pub unsafe fn get_cuda_module_cache_nv(
    device: &raw::Device,
    module: &raw::CudaModuleNV,
    p_cache_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<usize> {
    let vulkan_command = dispatcher.get_cuda_module_cache_nv.get();
    let mut p_cache_size = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(module.borrow()),
        p_cache_size.as_mut_ptr(),
        p_cache_data,
    );
    vk_status.map_success(|| p_cache_size.assume_init())
}
#[cfg(feature = "ext_cuda_kernel_launch")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaFunctionNV.html>"]
#[doc(alias = "vkCreateCudaFunctionNV")]
pub unsafe fn create_cuda_function_nv(
    device: &raw::Device,
    p_create_info: &CudaFunctionCreateInfoNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<CudaFunctionNV> {
    let vulkan_command = dispatcher.create_cuda_function_nv.get();
    let mut p_function = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_function.as_mut_ptr(),
    );
    vk_status.map_success(|| p_function.assume_init())
}
#[cfg(feature = "ext_cuda_kernel_launch")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaModuleNV.html>"]
#[doc(alias = "vkDestroyCudaModuleNV")]
#[inline]
pub unsafe fn destroy_cuda_module_nv(
    device: &raw::Device,
    module: &raw::CudaModuleNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_cuda_module_nv.get();
    vulkan_command(
        Some(device.borrow()),
        Some(module.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_cuda_kernel_launch")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaFunctionNV.html>"]
#[doc(alias = "vkDestroyCudaFunctionNV")]
#[inline]
pub unsafe fn destroy_cuda_function_nv(
    device: &raw::Device,
    function: &raw::CudaFunctionNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_cuda_function_nv.get();
    vulkan_command(
        Some(device.borrow()),
        Some(function.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_cuda_kernel_launch")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCudaLaunchKernelNV.html>"]
#[doc(alias = "vkCmdCudaLaunchKernelNV")]
#[inline]
pub unsafe fn cmd_cuda_launch_kernel_nv(
    command_buffer: &raw::CommandBuffer,
    p_launch_info: &CudaLaunchInfoNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_cuda_launch_kernel_nv.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_launch_info))
}
#[cfg(feature = "ext_tile_shading")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchTileQCOM.html>"]
#[doc(alias = "vkCmdDispatchTileQCOM")]
#[inline]
pub unsafe fn cmd_dispatch_tile_qcom(
    command_buffer: &raw::CommandBuffer,
    p_dispatch_tile_info: &DispatchTileInfoQCOM,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_dispatch_tile_qcom.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_dispatch_tile_info),
    )
}
#[cfg(feature = "ext_tile_shading")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginPerTileExecutionQCOM.html>"]
#[doc(alias = "vkCmdBeginPerTileExecutionQCOM")]
#[inline]
pub unsafe fn cmd_begin_per_tile_execution_qcom(
    command_buffer: &raw::CommandBuffer,
    p_per_tile_begin_info: &PerTileBeginInfoQCOM,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_begin_per_tile_execution_qcom.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_per_tile_begin_info),
    )
}
#[cfg(feature = "ext_tile_shading")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndPerTileExecutionQCOM.html>"]
#[doc(alias = "vkCmdEndPerTileExecutionQCOM")]
#[inline]
pub unsafe fn cmd_end_per_tile_execution_qcom(
    command_buffer: &raw::CommandBuffer,
    p_per_tile_end_info: &PerTileEndInfoQCOM,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_end_per_tile_execution_qcom.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_per_tile_end_info),
    )
}
#[cfg(feature = "ext_metal_objects")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkExportMetalObjectsEXT.html>"]
#[doc(alias = "vkExportMetalObjectsEXT")]
pub unsafe fn export_metal_objects_ext<S: StructureChainOut<ExportMetalObjectsInfoEXT<'static>>>(
    device: &raw::Device,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.export_metal_objects_ext.get();
    let mut p_metal_objects_info = MaybeUninit::uninit();
    S::setup_uninit(&mut p_metal_objects_info);
    vulkan_command(
        Some(device.borrow()),
        S::get_uninit_head_ptr(p_metal_objects_info.as_mut_ptr()),
    );
    S::setup_cleanup(p_metal_objects_info.as_mut_ptr());
    p_metal_objects_info.assume_init()
}
#[cfg(feature = "ext_descriptor_buffer")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSizeEXT.html>"]
#[doc(alias = "vkGetDescriptorSetLayoutSizeEXT")]
pub unsafe fn get_descriptor_set_layout_size_ext(
    device: &raw::Device,
    layout: &raw::DescriptorSetLayout,
    dispatcher: &CommandsDispatcher,
) -> DeviceSize {
    let vulkan_command = dispatcher.get_descriptor_set_layout_size_ext.get();
    let mut p_layout_size_in_bytes = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        Some(layout.borrow()),
        p_layout_size_in_bytes.as_mut_ptr(),
    );
    p_layout_size_in_bytes.assume_init()
}
#[cfg(feature = "ext_descriptor_buffer")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutBindingOffsetEXT.html>"]
#[doc(alias = "vkGetDescriptorSetLayoutBindingOffsetEXT")]
pub unsafe fn get_descriptor_set_layout_binding_offset_ext(
    device: &raw::Device,
    layout: &raw::DescriptorSetLayout,
    binding: u32,
    dispatcher: &CommandsDispatcher,
) -> DeviceSize {
    let vulkan_command = dispatcher
        .get_descriptor_set_layout_binding_offset_ext
        .get();
    let mut p_offset = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        Some(layout.borrow()),
        binding,
        p_offset.as_mut_ptr(),
    );
    p_offset.assume_init()
}
#[cfg(feature = "ext_descriptor_buffer")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorEXT.html>"]
#[doc(alias = "vkGetDescriptorEXT")]
#[inline]
pub unsafe fn get_descriptor_ext(
    device: &raw::Device,
    p_descriptor_info: &DescriptorGetInfoEXT,
    data_size: usize,
    p_descriptor: VoidPtr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.get_descriptor_ext.get();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_descriptor_info),
        data_size,
        p_descriptor,
    )
}
#[cfg(feature = "ext_descriptor_buffer")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBuffersEXT.html>"]
#[doc(alias = "vkCmdBindDescriptorBuffersEXT")]
#[inline]
pub unsafe fn cmd_bind_descriptor_buffers_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    p_binding_infos: impl AsSlice<'a, DescriptorBufferBindingInfoEXT<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_descriptor_buffers_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_binding_infos.as_slice().len() as _,
        p_binding_infos.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_descriptor_buffer")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsetsEXT.html>"]
#[doc(alias = "vkCmdSetDescriptorBufferOffsetsEXT")]
#[inline]
pub unsafe fn cmd_set_descriptor_buffer_offsets_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: &raw::PipelineLayout,
    first_set: u32,
    p_buffer_indices: impl AsSlice<'a, u32>,
    p_offsets: impl AsSlice<'a, DeviceSize>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_descriptor_buffer_offsets_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        pipeline_bind_point,
        Some(layout.borrow()),
        first_set,
        p_offsets.as_slice().len() as _,
        p_buffer_indices.as_slice().as_ptr().cast(),
        p_offsets.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_descriptor_buffer")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html>"]
#[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplersEXT")]
#[inline]
pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers_ext(
    command_buffer: &raw::CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: &raw::PipelineLayout,
    set: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_descriptor_buffer_embedded_samplers_ext
        .get();
    vulkan_command(
        Some(command_buffer.borrow()),
        pipeline_bind_point,
        Some(layout.borrow()),
        set,
    )
}
#[cfg(feature = "ext_descriptor_buffer")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureDescriptorDataEXT.html>"]
#[doc(alias = "vkGetBufferOpaqueCaptureDescriptorDataEXT")]
#[inline]
pub unsafe fn get_buffer_opaque_capture_descriptor_data_ext(
    device: &raw::Device,
    p_info: &BufferCaptureDescriptorDataInfoEXT,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_buffer_opaque_capture_descriptor_data_ext
        .get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info), p_data).map_success(|| ())
}
#[cfg(feature = "ext_descriptor_buffer")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDescriptorDataEXT.html>"]
#[doc(alias = "vkGetImageOpaqueCaptureDescriptorDataEXT")]
#[inline]
pub unsafe fn get_image_opaque_capture_descriptor_data_ext(
    device: &raw::Device,
    p_info: &ImageCaptureDescriptorDataInfoEXT,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_image_opaque_capture_descriptor_data_ext
        .get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info), p_data).map_success(|| ())
}
#[cfg(feature = "ext_descriptor_buffer")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html>"]
#[doc(alias = "vkGetImageViewOpaqueCaptureDescriptorDataEXT")]
#[inline]
pub unsafe fn get_image_view_opaque_capture_descriptor_data_ext(
    device: &raw::Device,
    p_info: &ImageViewCaptureDescriptorDataInfoEXT,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_image_view_opaque_capture_descriptor_data_ext
        .get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info), p_data).map_success(|| ())
}
#[cfg(feature = "ext_descriptor_buffer")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html>"]
#[doc(alias = "vkGetSamplerOpaqueCaptureDescriptorDataEXT")]
#[inline]
pub unsafe fn get_sampler_opaque_capture_descriptor_data_ext(
    device: &raw::Device,
    p_info: &SamplerCaptureDescriptorDataInfoEXT,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_sampler_opaque_capture_descriptor_data_ext
        .get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info), p_data).map_success(|| ())
}
#[cfg(all(
    feature = "ext_descriptor_buffer",
    any(feature = "ext_acceleration_structure", feature = "ext_ray_tracing")
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html>"]
#[doc(alias = "vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT")]
#[inline]
pub unsafe fn get_acceleration_structure_opaque_capture_descriptor_data_ext(
    device: &raw::Device,
    p_info: &AccelerationStructureCaptureDescriptorDataInfoEXT,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_acceleration_structure_opaque_capture_descriptor_data_ext
        .get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info), p_data).map_success(|| ())
}
#[cfg(feature = "ext_device_address_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer3KHR.html>"]
#[doc(alias = "vkCmdBindIndexBuffer3KHR")]
#[inline]
pub unsafe fn cmd_bind_index_buffer3_khr(
    command_buffer: &raw::CommandBuffer,
    p_info: &BindIndexBuffer3InfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_index_buffer3_khr.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_info))
}
#[cfg(feature = "ext_device_address_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers3KHR.html>"]
#[doc(alias = "vkCmdBindVertexBuffers3KHR")]
#[inline]
pub unsafe fn cmd_bind_vertex_buffers3_khr<'a>(
    command_buffer: &raw::CommandBuffer,
    first_binding: u32,
    p_binding_infos: impl AsSlice<'a, BindVertexBuffer3InfoKHR<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_vertex_buffers3_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_binding,
        p_binding_infos.as_slice().len() as _,
        p_binding_infos.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_device_address_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect2KHR.html>"]
#[doc(alias = "vkCmdDrawIndirect2KHR")]
#[inline]
pub unsafe fn cmd_draw_indirect2_khr(
    command_buffer: &raw::CommandBuffer,
    p_info: &DrawIndirect2InfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_indirect2_khr.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_info))
}
#[cfg(feature = "ext_device_address_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect2KHR.html>"]
#[doc(alias = "vkCmdDrawIndexedIndirect2KHR")]
#[inline]
pub unsafe fn cmd_draw_indexed_indirect2_khr(
    command_buffer: &raw::CommandBuffer,
    p_info: &DrawIndirect2InfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_indexed_indirect2_khr.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_info))
}
#[cfg(feature = "ext_device_address_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect2KHR.html>"]
#[doc(alias = "vkCmdDispatchIndirect2KHR")]
#[inline]
pub unsafe fn cmd_dispatch_indirect2_khr(
    command_buffer: &raw::CommandBuffer,
    p_info: &DispatchIndirect2InfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_dispatch_indirect2_khr.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_info))
}
#[cfg(feature = "ext_device_address_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryKHR.html>"]
#[doc(alias = "vkCmdCopyMemoryKHR")]
#[inline]
pub unsafe fn cmd_copy_memory_khr(
    command_buffer: &raw::CommandBuffer,
    p_copy_memory_info: Option<&CopyDeviceMemoryInfoKHR>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_memory_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_copy_memory_info
            .map(|v| ptr::from_ref(v))
            .unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_device_address_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageKHR.html>"]
#[doc(alias = "vkCmdCopyMemoryToImageKHR")]
#[inline]
pub unsafe fn cmd_copy_memory_to_image_khr(
    command_buffer: &raw::CommandBuffer,
    p_copy_memory_info: Option<&CopyDeviceMemoryImageInfoKHR>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_memory_to_image_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_copy_memory_info
            .map(|v| ptr::from_ref(v))
            .unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_device_address_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToMemoryKHR.html>"]
#[doc(alias = "vkCmdCopyImageToMemoryKHR")]
#[inline]
pub unsafe fn cmd_copy_image_to_memory_khr(
    command_buffer: &raw::CommandBuffer,
    p_copy_memory_info: Option<&CopyDeviceMemoryImageInfoKHR>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_image_to_memory_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_copy_memory_info
            .map(|v| ptr::from_ref(v))
            .unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_device_address_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateMemoryKHR.html>"]
#[doc(alias = "vkCmdUpdateMemoryKHR")]
#[inline]
pub unsafe fn cmd_update_memory_khr(
    command_buffer: &raw::CommandBuffer,
    p_dst_range: &DeviceAddressRangeKHR,
    dst_flags: AddressCommandFlagsKHR,
    data_size: DeviceSize,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_update_memory_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_dst_range),
        dst_flags,
        data_size,
        p_data,
    )
}
#[cfg(feature = "ext_device_address_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillMemoryKHR.html>"]
#[doc(alias = "vkCmdFillMemoryKHR")]
#[inline]
pub unsafe fn cmd_fill_memory_khr(
    command_buffer: &raw::CommandBuffer,
    p_dst_range: &DeviceAddressRangeKHR,
    dst_flags: AddressCommandFlagsKHR,
    data: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_fill_memory_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_dst_range),
        dst_flags,
        data,
    )
}
#[cfg(feature = "ext_device_address_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResultsToMemoryKHR.html>"]
#[doc(alias = "vkCmdCopyQueryPoolResultsToMemoryKHR")]
#[inline]
pub unsafe fn cmd_copy_query_pool_results_to_memory_khr(
    command_buffer: &raw::CommandBuffer,
    query_pool: &raw::QueryPool,
    first_query: u32,
    query_count: u32,
    p_dst_range: &StridedDeviceAddressRangeKHR,
    dst_flags: AddressCommandFlagsKHR,
    query_result_flags: QueryResultFlags,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_query_pool_results_to_memory_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(query_pool.borrow()),
        first_query,
        query_count,
        ptr::from_ref(p_dst_range),
        dst_flags,
        query_result_flags,
    )
}
#[cfg(all(
    feature = "ext_device_address_commands",
    any(feature = "ext_draw_indirect_count", feature = "version_1_2")
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount2KHR.html>"]
#[doc(alias = "vkCmdDrawIndirectCount2KHR")]
#[inline]
pub unsafe fn cmd_draw_indirect_count2_khr(
    command_buffer: &raw::CommandBuffer,
    p_info: &DrawIndirectCount2InfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_indirect_count2_khr.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_info))
}
#[cfg(all(
    feature = "ext_device_address_commands",
    any(feature = "ext_draw_indirect_count", feature = "version_1_2")
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount2KHR.html>"]
#[doc(alias = "vkCmdDrawIndexedIndirectCount2KHR")]
#[inline]
pub unsafe fn cmd_draw_indexed_indirect_count2_khr(
    command_buffer: &raw::CommandBuffer,
    p_info: &DrawIndirectCount2InfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_indexed_indirect_count2_khr.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_info))
}
#[cfg(all(
    feature = "ext_device_address_commands",
    feature = "ext_conditional_rendering"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRendering2EXT.html>"]
#[doc(alias = "vkCmdBeginConditionalRendering2EXT")]
#[inline]
pub unsafe fn cmd_begin_conditional_rendering2_ext(
    command_buffer: &raw::CommandBuffer,
    p_conditional_rendering_begin: &ConditionalRenderingBeginInfo2EXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_begin_conditional_rendering2_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_conditional_rendering_begin),
    )
}
#[cfg(all(
    feature = "ext_device_address_commands",
    feature = "ext_transform_feedback"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffers2EXT.html>"]
#[doc(alias = "vkCmdBindTransformFeedbackBuffers2EXT")]
#[inline]
pub unsafe fn cmd_bind_transform_feedback_buffers2_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_binding_infos: Option<impl AsSlice<'a, BindTransformFeedbackBuffer2InfoEXT<'a>>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_transform_feedback_buffers2_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_binding,
        binding_count,
        p_binding_infos
            .map(|p| p.as_slice().as_ptr().cast())
            .unwrap_or(ptr::null()),
    )
}
#[cfg(all(
    feature = "ext_device_address_commands",
    feature = "ext_transform_feedback"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedback2EXT.html>"]
#[doc(alias = "vkCmdBeginTransformFeedback2EXT")]
#[inline]
pub unsafe fn cmd_begin_transform_feedback2_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    first_counter_range: u32,
    counter_range_count: u32,
    p_counter_infos: Option<impl AsSlice<'a, BindTransformFeedbackBuffer2InfoEXT<'a>>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_begin_transform_feedback2_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_counter_range,
        counter_range_count,
        p_counter_infos
            .map(|p| p.as_slice().as_ptr().cast())
            .unwrap_or(ptr::null()),
    )
}
#[cfg(all(
    feature = "ext_device_address_commands",
    feature = "ext_transform_feedback"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedback2EXT.html>"]
#[doc(alias = "vkCmdEndTransformFeedback2EXT")]
#[inline]
pub unsafe fn cmd_end_transform_feedback2_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    first_counter_range: u32,
    counter_range_count: u32,
    p_counter_infos: Option<impl AsSlice<'a, BindTransformFeedbackBuffer2InfoEXT<'a>>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_end_transform_feedback2_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_counter_range,
        counter_range_count,
        p_counter_infos
            .map(|p| p.as_slice().as_ptr().cast())
            .unwrap_or(ptr::null()),
    )
}
#[cfg(all(
    feature = "ext_device_address_commands",
    feature = "ext_transform_feedback"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCount2EXT.html>"]
#[doc(alias = "vkCmdDrawIndirectByteCount2EXT")]
#[inline]
pub unsafe fn cmd_draw_indirect_byte_count2_ext(
    command_buffer: &raw::CommandBuffer,
    instance_count: u32,
    first_instance: u32,
    p_counter_info: &BindTransformFeedbackBuffer2InfoEXT,
    counter_offset: u32,
    vertex_stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_indirect_byte_count2_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        instance_count,
        first_instance,
        ptr::from_ref(p_counter_info),
        counter_offset,
        vertex_stride,
    )
}
#[cfg(all(feature = "ext_device_address_commands", feature = "ext_mesh_shader"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirect2EXT.html>"]
#[doc(alias = "vkCmdDrawMeshTasksIndirect2EXT")]
#[inline]
pub unsafe fn cmd_draw_mesh_tasks_indirect2_ext(
    command_buffer: &raw::CommandBuffer,
    p_info: &DrawIndirect2InfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_mesh_tasks_indirect2_ext.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_info))
}
#[cfg(all(
    feature = "ext_device_address_commands",
    all(
        any(feature = "ext_draw_indirect_count", feature = "version_1_2"),
        feature = "ext_mesh_shader"
    )
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCount2EXT.html>"]
#[doc(alias = "vkCmdDrawMeshTasksIndirectCount2EXT")]
#[inline]
pub unsafe fn cmd_draw_mesh_tasks_indirect_count2_ext(
    command_buffer: &raw::CommandBuffer,
    p_info: &DrawIndirectCount2InfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_mesh_tasks_indirect_count2_ext.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_info))
}
#[cfg(all(feature = "ext_device_address_commands", feature = "ext_buffer_marker"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMarkerToMemoryAMD.html>"]
#[doc(alias = "vkCmdWriteMarkerToMemoryAMD")]
#[inline]
pub unsafe fn cmd_write_marker_to_memory_amd(
    command_buffer: &raw::CommandBuffer,
    p_info: &MemoryMarkerInfoAMD,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_write_marker_to_memory_amd.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_info))
}
#[cfg(all(
    feature = "ext_device_address_commands",
    feature = "ext_acceleration_structure"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructure2KHR.html>"]
#[doc(alias = "vkCreateAccelerationStructure2KHR")]
pub unsafe fn create_acceleration_structure2_khr(
    device: &raw::Device,
    p_create_info: &AccelerationStructureCreateInfo2KHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<AccelerationStructureKHR> {
    let vulkan_command = dispatcher.create_acceleration_structure2_khr.get();
    let mut p_acceleration_structure = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_acceleration_structure.as_mut_ptr(),
    );
    vk_status.map_success(|| p_acceleration_structure.assume_init())
}
#[cfg(feature = "ext_fragment_shading_rate_enums")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateEnumNV.html>"]
#[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
#[inline]
pub unsafe fn cmd_set_fragment_shading_rate_enum_nv(
    command_buffer: &raw::CommandBuffer,
    shading_rate: FragmentShadingRateNV,
    combiner_ops: [FragmentShadingRateCombinerOpKHR; 2u16 as _],
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_fragment_shading_rate_enum_nv.get();
    vulkan_command(Some(command_buffer.borrow()), shading_rate, combiner_ops)
}
#[cfg(feature = "ext_mesh_shader")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksEXT.html>"]
#[doc(alias = "vkCmdDrawMeshTasksEXT")]
#[inline]
pub unsafe fn cmd_draw_mesh_tasks_ext(
    command_buffer: &raw::CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_mesh_tasks_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        group_count_x,
        group_count_y,
        group_count_z,
    )
}
#[cfg(feature = "ext_mesh_shader")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectEXT.html>"]
#[doc(alias = "vkCmdDrawMeshTasksIndirectEXT")]
#[inline]
pub unsafe fn cmd_draw_mesh_tasks_indirect_ext(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_mesh_tasks_indirect_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(buffer.borrow()),
        offset,
        draw_count,
        stride,
    )
}
#[cfg(all(
    feature = "ext_mesh_shader",
    any(feature = "version_1_2", feature = "ext_draw_indirect_count")
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountEXT.html>"]
#[doc(alias = "vkCmdDrawMeshTasksIndirectCountEXT")]
#[inline]
pub unsafe fn cmd_draw_mesh_tasks_indirect_count_ext(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    count_buffer: &raw::Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_mesh_tasks_indirect_count_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(buffer.borrow()),
        offset,
        Some(count_buffer.borrow()),
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[cfg(feature = "ext_acquire_winrt_display")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireWinrtDisplayNV.html>"]
#[doc(alias = "vkAcquireWinrtDisplayNV")]
#[inline]
pub unsafe fn acquire_winrt_display_nv(
    physical_device: &raw::PhysicalDevice,
    display: &raw::DisplayKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.acquire_winrt_display_nv.get();
    vulkan_command(Some(physical_device.borrow()), Some(display.borrow())).map_success(|| ())
}
#[cfg(feature = "ext_acquire_winrt_display")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetWinrtDisplayNV.html>"]
#[doc(alias = "vkGetWinrtDisplayNV")]
pub unsafe fn get_winrt_display_nv(
    physical_device: &raw::PhysicalDevice,
    device_relative_id: u32,
    dispatcher: &CommandsDispatcher,
) -> Result<DisplayKHR> {
    let vulkan_command = dispatcher.get_winrt_display_nv.get();
    let mut p_display = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(physical_device.borrow()),
        device_relative_id,
        p_display.as_mut_ptr(),
    );
    vk_status.map_success(|| p_display.assume_init())
}
#[cfg(feature = "ext_directfb_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDirectFBSurfaceEXT.html>"]
#[doc(alias = "vkCreateDirectFBSurfaceEXT")]
pub unsafe fn create_direct_fbsurface_ext(
    instance: &raw::Instance,
    p_create_info: &DirectFBSurfaceCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_direct_fbsurface_ext.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_directfb_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html>"]
#[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
#[inline]
pub unsafe fn get_physical_device_direct_fbpresentation_support_ext(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    dfb: &VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> bool {
    let vulkan_command = dispatcher
        .get_physical_device_direct_fbpresentation_support_ext
        .get();
    vulkan_command(
        Some(physical_device.borrow()),
        queue_family_index,
        ptr::from_ref(dfb),
    )
    .into()
}
#[cfg(any(
    feature = "ext_vertex_input_dynamic_state",
    feature = "ext_shader_object"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetVertexInputEXT.html>"]
#[doc(alias = "vkCmdSetVertexInputEXT")]
#[inline]
pub unsafe fn cmd_set_vertex_input_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    p_vertex_binding_descriptions: impl AsSlice<'a, VertexInputBindingDescription2EXT<'a>>,
    p_vertex_attribute_descriptions: impl AsSlice<'a, VertexInputAttributeDescription2EXT<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_vertex_input_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_vertex_binding_descriptions.as_slice().len() as _,
        p_vertex_binding_descriptions.as_slice().as_ptr().cast(),
        p_vertex_attribute_descriptions.as_slice().len() as _,
        p_vertex_attribute_descriptions.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_fuchsia_external_memory")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandleFUCHSIA.html>"]
#[doc(alias = "vkGetMemoryZirconHandleFUCHSIA")]
pub unsafe fn get_memory_zircon_handle_fuchsia(
    device: &raw::Device,
    p_get_zircon_handle_info: &MemoryGetZirconHandleInfoFUCHSIA,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher.get_memory_zircon_handle_fuchsia.get();
    let mut p_zircon_handle = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_get_zircon_handle_info),
        p_zircon_handle.as_mut_ptr(),
    );
    vk_status.map_success(|| p_zircon_handle.assume_init())
}
#[cfg(feature = "ext_fuchsia_external_memory")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandlePropertiesFUCHSIA.html>"]
#[doc(alias = "vkGetMemoryZirconHandlePropertiesFUCHSIA")]
pub unsafe fn get_memory_zircon_handle_properties_fuchsia<
    S: StructureChainOut<MemoryZirconHandlePropertiesFUCHSIA<'static>>,
>(
    device: &raw::Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    zircon_handle: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.get_memory_zircon_handle_properties_fuchsia.get();
    let mut p_memory_zircon_handle_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_zircon_handle_properties);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        handle_type,
        zircon_handle,
        S::get_uninit_head_ptr(p_memory_zircon_handle_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_memory_zircon_handle_properties.as_mut_ptr());
        p_memory_zircon_handle_properties.assume_init()
    })
}
#[cfg(feature = "ext_fuchsia_external_semaphore")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreZirconHandleFUCHSIA.html>"]
#[doc(alias = "vkImportSemaphoreZirconHandleFUCHSIA")]
#[inline]
pub unsafe fn import_semaphore_zircon_handle_fuchsia(
    device: &raw::Device,
    p_import_semaphore_zircon_handle_info: &ImportSemaphoreZirconHandleInfoFUCHSIA,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.import_semaphore_zircon_handle_fuchsia.get();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_import_semaphore_zircon_handle_info),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_fuchsia_external_semaphore")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreZirconHandleFUCHSIA.html>"]
#[doc(alias = "vkGetSemaphoreZirconHandleFUCHSIA")]
pub unsafe fn get_semaphore_zircon_handle_fuchsia(
    device: &raw::Device,
    p_get_zircon_handle_info: &SemaphoreGetZirconHandleInfoFUCHSIA,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher.get_semaphore_zircon_handle_fuchsia.get();
    let mut p_zircon_handle = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_get_zircon_handle_info),
        p_zircon_handle.as_mut_ptr(),
    );
    vk_status.map_success(|| p_zircon_handle.assume_init())
}
#[cfg(feature = "ext_buffer_collection")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferCollectionFUCHSIA.html>"]
#[doc(alias = "vkCreateBufferCollectionFUCHSIA")]
pub unsafe fn create_buffer_collection_fuchsia(
    device: &raw::Device,
    p_create_info: &BufferCollectionCreateInfoFUCHSIA,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<BufferCollectionFUCHSIA> {
    let vulkan_command = dispatcher.create_buffer_collection_fuchsia.get();
    let mut p_collection = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_collection.as_mut_ptr(),
    );
    vk_status.map_success(|| p_collection.assume_init())
}
#[cfg(feature = "ext_buffer_collection")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionImageConstraintsFUCHSIA.html>"]
#[doc(alias = "vkSetBufferCollectionImageConstraintsFUCHSIA")]
#[inline]
pub unsafe fn set_buffer_collection_image_constraints_fuchsia(
    device: &raw::Device,
    collection: &raw::BufferCollectionFUCHSIA,
    p_image_constraints_info: &ImageConstraintsInfoFUCHSIA,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .set_buffer_collection_image_constraints_fuchsia
        .get();
    vulkan_command(
        Some(device.borrow()),
        Some(collection.borrow()),
        ptr::from_ref(p_image_constraints_info),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_buffer_collection")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionBufferConstraintsFUCHSIA.html>"]
#[doc(alias = "vkSetBufferCollectionBufferConstraintsFUCHSIA")]
#[inline]
pub unsafe fn set_buffer_collection_buffer_constraints_fuchsia(
    device: &raw::Device,
    collection: &raw::BufferCollectionFUCHSIA,
    p_buffer_constraints_info: &BufferConstraintsInfoFUCHSIA,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .set_buffer_collection_buffer_constraints_fuchsia
        .get();
    vulkan_command(
        Some(device.borrow()),
        Some(collection.borrow()),
        ptr::from_ref(p_buffer_constraints_info),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_buffer_collection")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferCollectionFUCHSIA.html>"]
#[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
#[inline]
pub unsafe fn destroy_buffer_collection_fuchsia(
    device: &raw::Device,
    collection: &raw::BufferCollectionFUCHSIA,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_buffer_collection_fuchsia.get();
    vulkan_command(
        Some(device.borrow()),
        Some(collection.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_buffer_collection")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferCollectionPropertiesFUCHSIA.html>"]
#[doc(alias = "vkGetBufferCollectionPropertiesFUCHSIA")]
pub unsafe fn get_buffer_collection_properties_fuchsia<
    S: StructureChainOut<BufferCollectionPropertiesFUCHSIA<'static>>,
>(
    device: &raw::Device,
    collection: &raw::BufferCollectionFUCHSIA,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.get_buffer_collection_properties_fuchsia.get();
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(collection.borrow()),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_properties.as_mut_ptr());
        p_properties.assume_init()
    })
}
#[cfg(feature = "ext_subpass_shading")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html>"]
#[doc(alias = "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")]
pub unsafe fn get_device_subpass_shading_max_workgroup_size_huawei<R: DynamicArray<Extent2D>>(
    device: &raw::Device,
    renderpass: &raw::RenderPass,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_device_subpass_shading_max_workgroup_size_huawei
        .get();
    let mut p_max_workgroup_size = R::create_with_capacity(1 as _);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(renderpass.borrow()),
        p_max_workgroup_size.get_content_mut_ptr(),
    );
    vk_status.map_success(|| {
        p_max_workgroup_size.resize_with_len(1 as _);
        p_max_workgroup_size
    })
}
#[cfg(feature = "ext_subpass_shading")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSubpassShadingHUAWEI.html>"]
#[doc(alias = "vkCmdSubpassShadingHUAWEI")]
#[inline]
pub unsafe fn cmd_subpass_shading_huawei(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_subpass_shading_huawei.get();
    vulkan_command(Some(command_buffer.borrow()))
}
#[cfg(feature = "ext_invocation_mask")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindInvocationMaskHUAWEI.html>"]
#[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
#[inline]
pub unsafe fn cmd_bind_invocation_mask_huawei(
    command_buffer: &raw::CommandBuffer,
    image_view: Option<&raw::ImageView>,
    image_layout: ImageLayout,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_invocation_mask_huawei.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        image_view.map(|v| v.borrow()),
        image_layout,
    )
}
#[cfg(feature = "ext_external_memory_rdma")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryRemoteAddressNV.html>"]
#[doc(alias = "vkGetMemoryRemoteAddressNV")]
pub unsafe fn get_memory_remote_address_nv(
    device: &raw::Device,
    p_memory_get_remote_address_info: &MemoryGetRemoteAddressInfoNV,
    dispatcher: &CommandsDispatcher,
) -> Result<RemoteAddressNV> {
    let vulkan_command = dispatcher.get_memory_remote_address_nv.get();
    let mut p_address = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_memory_get_remote_address_info),
        p_address.as_mut_ptr(),
    );
    vk_status.map_success(|| p_address.assume_init())
}
#[cfg(feature = "ext_pipeline_properties")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelinePropertiesEXT.html>"]
#[doc(alias = "vkGetPipelinePropertiesEXT")]
pub unsafe fn get_pipeline_properties_ext(
    device: &raw::Device,
    p_pipeline_info: &PipelineInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<BaseOutStructure<'static>> {
    let vulkan_command = dispatcher.get_pipeline_properties_ext.get();
    let mut p_pipeline_properties = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_pipeline_info),
        p_pipeline_properties.as_mut_ptr(),
    );
    vk_status.map_success(|| p_pipeline_properties.assume_init())
}
#[cfg(any(feature = "ext_extended_dynamic_state2", feature = "ext_shader_object"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPatchControlPointsEXT.html>"]
#[doc(alias = "vkCmdSetPatchControlPointsEXT")]
#[inline]
pub unsafe fn cmd_set_patch_control_points_ext(
    command_buffer: &raw::CommandBuffer,
    patch_control_points: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_patch_control_points_ext.get();
    vulkan_command(Some(command_buffer.borrow()), patch_control_points)
}
#[cfg(any(feature = "ext_extended_dynamic_state2", feature = "ext_shader_object"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEXT.html>"]
#[doc(alias = "vkCmdSetLogicOpEXT")]
#[inline]
pub unsafe fn cmd_set_logic_op_ext(
    command_buffer: &raw::CommandBuffer,
    logic_op: LogicOp,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_logic_op_ext.get();
    vulkan_command(Some(command_buffer.borrow()), logic_op)
}
#[cfg(feature = "ext_screen_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateScreenSurfaceQNX.html>"]
#[doc(alias = "vkCreateScreenSurfaceQNX")]
pub unsafe fn create_screen_surface_qnx(
    instance: &raw::Instance,
    p_create_info: &ScreenSurfaceCreateInfoQNX,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_screen_surface_qnx.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_screen_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceScreenPresentationSupportQNX.html>"]
#[doc(alias = "vkGetPhysicalDeviceScreenPresentationSupportQNX")]
#[inline]
pub unsafe fn get_physical_device_screen_presentation_support_qnx(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    window: &VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> bool {
    let vulkan_command = dispatcher
        .get_physical_device_screen_presentation_support_qnx
        .get();
    vulkan_command(
        Some(physical_device.borrow()),
        queue_family_index,
        ptr::from_ref(window),
    )
    .into()
}
#[cfg(feature = "ext_color_write_enable")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteEnableEXT.html>"]
#[doc(alias = "vkCmdSetColorWriteEnableEXT")]
#[inline]
pub unsafe fn cmd_set_color_write_enable_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    p_color_write_enables: impl AsSlice<'a, Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_color_write_enable_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_color_write_enables.as_slice().len() as _,
        p_color_write_enables.as_slice().as_ptr().cast(),
    )
}
#[cfg(all(
    feature = "ext_ray_tracing_maintenance1",
    feature = "ext_ray_tracing_pipeline"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirect2KHR.html>"]
#[doc(alias = "vkCmdTraceRaysIndirect2KHR")]
#[inline]
pub unsafe fn cmd_trace_rays_indirect2_khr(
    command_buffer: &raw::CommandBuffer,
    indirect_device_address: DeviceAddress,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_trace_rays_indirect2_khr.get();
    vulkan_command(Some(command_buffer.borrow()), indirect_device_address)
}
#[cfg(feature = "ext_multi_draw")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiEXT.html>"]
#[doc(alias = "vkCmdDrawMultiEXT")]
#[inline]
pub unsafe fn cmd_draw_multi_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    p_vertex_info: impl AsSlice<'a, MultiDrawInfoEXT>,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_multi_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_vertex_info.as_slice().len() as _,
        p_vertex_info.as_slice().as_ptr().cast(),
        instance_count,
        first_instance,
        stride,
    )
}
#[cfg(feature = "ext_multi_draw")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiIndexedEXT.html>"]
#[doc(alias = "vkCmdDrawMultiIndexedEXT")]
#[inline]
pub unsafe fn cmd_draw_multi_indexed_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    p_index_info: impl AsSlice<'a, MultiDrawIndexedInfoEXT>,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
    p_vertex_offset: Option<&i32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_multi_indexed_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_index_info.as_slice().len() as _,
        p_index_info.as_slice().as_ptr().cast(),
        instance_count,
        first_instance,
        stride,
        p_vertex_offset
            .map(|v| ptr::from_ref(v))
            .unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_opacity_micromap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMicromapEXT.html>"]
#[doc(alias = "vkCreateMicromapEXT")]
pub unsafe fn create_micromap_ext(
    device: &raw::Device,
    p_create_info: &MicromapCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<MicromapEXT> {
    let vulkan_command = dispatcher.create_micromap_ext.get();
    let mut p_micromap = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_micromap.as_mut_ptr(),
    );
    vk_status.map_success(|| p_micromap.assume_init())
}
#[cfg(feature = "ext_opacity_micromap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyMicromapEXT.html>"]
#[doc(alias = "vkDestroyMicromapEXT")]
#[inline]
pub unsafe fn destroy_micromap_ext(
    device: &raw::Device,
    micromap: Option<&raw::MicromapEXT>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_micromap_ext.get();
    vulkan_command(
        Some(device.borrow()),
        micromap.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_opacity_micromap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildMicromapsEXT.html>"]
#[doc(alias = "vkCmdBuildMicromapsEXT")]
#[inline]
pub unsafe fn cmd_build_micromaps_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    p_infos: impl AsSlice<'a, MicromapBuildInfoEXT<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_build_micromaps_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_infos.as_slice().len() as _,
        p_infos.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_opacity_micromap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildMicromapsEXT.html>"]
#[doc(alias = "vkBuildMicromapsEXT")]
#[inline]
pub unsafe fn build_micromaps_ext<'a>(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    p_infos: impl AsSlice<'a, MicromapBuildInfoEXT<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.build_micromaps_ext.get();
    vulkan_command(
        Some(device.borrow()),
        deferred_operation.map(|v| v.borrow()),
        p_infos.as_slice().len() as _,
        p_infos.as_slice().as_ptr().cast(),
    )
    .into_result()
}
#[cfg(feature = "ext_opacity_micromap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapEXT.html>"]
#[doc(alias = "vkCopyMicromapEXT")]
#[inline]
pub unsafe fn copy_micromap_ext(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    p_info: &CopyMicromapInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.copy_micromap_ext.get();
    vulkan_command(
        Some(device.borrow()),
        deferred_operation.map(|v| v.borrow()),
        ptr::from_ref(p_info),
    )
    .into_result()
}
#[cfg(feature = "ext_opacity_micromap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapToMemoryEXT.html>"]
#[doc(alias = "vkCopyMicromapToMemoryEXT")]
#[inline]
pub unsafe fn copy_micromap_to_memory_ext(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    p_info: &CopyMicromapToMemoryInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.copy_micromap_to_memory_ext.get();
    vulkan_command(
        Some(device.borrow()),
        deferred_operation.map(|v| v.borrow()),
        ptr::from_ref(p_info),
    )
    .into_result()
}
#[cfg(feature = "ext_opacity_micromap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToMicromapEXT.html>"]
#[doc(alias = "vkCopyMemoryToMicromapEXT")]
#[inline]
pub unsafe fn copy_memory_to_micromap_ext(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    p_info: &CopyMemoryToMicromapInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.copy_memory_to_micromap_ext.get();
    vulkan_command(
        Some(device.borrow()),
        deferred_operation.map(|v| v.borrow()),
        ptr::from_ref(p_info),
    )
    .into_result()
}
#[cfg(feature = "ext_opacity_micromap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteMicromapsPropertiesEXT.html>"]
#[doc(alias = "vkWriteMicromapsPropertiesEXT")]
#[inline]
pub unsafe fn write_micromaps_properties_ext<'a, V2: Alias<raw::MicromapEXT> + 'a>(
    device: &raw::Device,
    p_micromaps: impl AsSlice<'a, V2>,
    query_type: QueryType,
    data_size: usize,
    p_data: VoidPtr,
    stride: usize,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.write_micromaps_properties_ext.get();
    vulkan_command(
        Some(device.borrow()),
        p_micromaps.as_slice().len() as _,
        p_micromaps.as_slice().as_ptr().cast(),
        query_type,
        data_size,
        p_data,
        stride,
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_opacity_micromap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapEXT.html>"]
#[doc(alias = "vkCmdCopyMicromapEXT")]
#[inline]
pub unsafe fn cmd_copy_micromap_ext(
    command_buffer: &raw::CommandBuffer,
    p_info: &CopyMicromapInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_micromap_ext.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_info))
}
#[cfg(feature = "ext_opacity_micromap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapToMemoryEXT.html>"]
#[doc(alias = "vkCmdCopyMicromapToMemoryEXT")]
#[inline]
pub unsafe fn cmd_copy_micromap_to_memory_ext(
    command_buffer: &raw::CommandBuffer,
    p_info: &CopyMicromapToMemoryInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_micromap_to_memory_ext.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_info))
}
#[cfg(feature = "ext_opacity_micromap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToMicromapEXT.html>"]
#[doc(alias = "vkCmdCopyMemoryToMicromapEXT")]
#[inline]
pub unsafe fn cmd_copy_memory_to_micromap_ext(
    command_buffer: &raw::CommandBuffer,
    p_info: &CopyMemoryToMicromapInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_memory_to_micromap_ext.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_info))
}
#[cfg(feature = "ext_opacity_micromap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMicromapsPropertiesEXT.html>"]
#[doc(alias = "vkCmdWriteMicromapsPropertiesEXT")]
#[inline]
pub unsafe fn cmd_write_micromaps_properties_ext<'a, V2: Alias<raw::MicromapEXT> + 'a>(
    command_buffer: &raw::CommandBuffer,
    p_micromaps: impl AsSlice<'a, V2>,
    query_type: QueryType,
    query_pool: &raw::QueryPool,
    first_query: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_write_micromaps_properties_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_micromaps.as_slice().len() as _,
        p_micromaps.as_slice().as_ptr().cast(),
        query_type,
        Some(query_pool.borrow()),
        first_query,
    )
}
#[cfg(feature = "ext_opacity_micromap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMicromapCompatibilityEXT.html>"]
#[doc(alias = "vkGetDeviceMicromapCompatibilityEXT")]
pub unsafe fn get_device_micromap_compatibility_ext(
    device: &raw::Device,
    p_version_info: &MicromapVersionInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> AccelerationStructureCompatibilityKHR {
    let vulkan_command = dispatcher.get_device_micromap_compatibility_ext.get();
    let mut p_compatibility = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_version_info),
        p_compatibility.as_mut_ptr(),
    );
    p_compatibility.assume_init()
}
#[cfg(feature = "ext_opacity_micromap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMicromapBuildSizesEXT.html>"]
#[doc(alias = "vkGetMicromapBuildSizesEXT")]
pub unsafe fn get_micromap_build_sizes_ext<
    S: StructureChainOut<MicromapBuildSizesInfoEXT<'static>>,
>(
    device: &raw::Device,
    build_type: AccelerationStructureBuildTypeKHR,
    p_build_info: &MicromapBuildInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_micromap_build_sizes_ext.get();
    let mut p_size_info = MaybeUninit::uninit();
    S::setup_uninit(&mut p_size_info);
    vulkan_command(
        Some(device.borrow()),
        build_type,
        ptr::from_ref(p_build_info),
        S::get_uninit_head_ptr(p_size_info.as_mut_ptr()),
    );
    S::setup_cleanup(p_size_info.as_mut_ptr());
    p_size_info.assume_init()
}
#[cfg(feature = "ext_cluster_culling_shader")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterHUAWEI.html>"]
#[doc(alias = "vkCmdDrawClusterHUAWEI")]
#[inline]
pub unsafe fn cmd_draw_cluster_huawei(
    command_buffer: &raw::CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_cluster_huawei.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        group_count_x,
        group_count_y,
        group_count_z,
    )
}
#[cfg(feature = "ext_cluster_culling_shader")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterIndirectHUAWEI.html>"]
#[doc(alias = "vkCmdDrawClusterIndirectHUAWEI")]
#[inline]
pub unsafe fn cmd_draw_cluster_indirect_huawei(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_draw_cluster_indirect_huawei.get();
    vulkan_command(Some(command_buffer.borrow()), Some(buffer.borrow()), offset)
}
#[cfg(feature = "ext_pageable_device_local_memory")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDeviceMemoryPriorityEXT.html>"]
#[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
#[inline]
pub unsafe fn set_device_memory_priority_ext(
    device: &raw::Device,
    memory: &raw::DeviceMemory,
    priority: f32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.set_device_memory_priority_ext.get();
    vulkan_command(Some(device.borrow()), Some(memory.borrow()), priority)
}
#[cfg(feature = "ext_scheduling_controls")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDispatchParametersARM.html>"]
#[doc(alias = "vkCmdSetDispatchParametersARM")]
#[inline]
pub unsafe fn cmd_set_dispatch_parameters_arm(
    command_buffer: &raw::CommandBuffer,
    p_dispatch_parameters: &DispatchParametersARM,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_dispatch_parameters_arm.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_dispatch_parameters),
    )
}
#[cfg(feature = "ext_descriptor_set_host_mapping")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html>"]
#[doc(alias = "vkGetDescriptorSetLayoutHostMappingInfoVALVE")]
pub unsafe fn get_descriptor_set_layout_host_mapping_info_valve<
    S: StructureChainOut<DescriptorSetLayoutHostMappingInfoVALVE<'static>>,
>(
    device: &raw::Device,
    p_binding_reference: &DescriptorSetBindingReferenceVALVE,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_descriptor_set_layout_host_mapping_info_valve
        .get();
    let mut p_host_mapping = MaybeUninit::uninit();
    S::setup_uninit(&mut p_host_mapping);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_binding_reference),
        S::get_uninit_head_ptr(p_host_mapping.as_mut_ptr()),
    );
    S::setup_cleanup(p_host_mapping.as_mut_ptr());
    p_host_mapping.assume_init()
}
#[cfg(feature = "ext_descriptor_set_host_mapping")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetHostMappingVALVE.html>"]
#[doc(alias = "vkGetDescriptorSetHostMappingVALVE")]
pub unsafe fn get_descriptor_set_host_mapping_valve(
    device: &raw::Device,
    descriptor_set: &raw::DescriptorSet,
    dispatcher: &CommandsDispatcher,
) -> VoidPtr {
    let vulkan_command = dispatcher.get_descriptor_set_host_mapping_valve.get();
    let mut pp_data = MaybeUninit::uninit();
    vulkan_command(
        Some(device.borrow()),
        Some(descriptor_set.borrow()),
        pp_data.as_mut_ptr(),
    );
    pp_data.assume_init()
}
#[cfg(feature = "ext_copy_memory_indirect")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectNV.html>"]
#[doc(alias = "vkCmdCopyMemoryIndirectNV")]
#[inline]
pub unsafe fn cmd_copy_memory_indirect_nv(
    command_buffer: &raw::CommandBuffer,
    copy_buffer_address: DeviceAddress,
    copy_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_memory_indirect_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        copy_buffer_address,
        copy_count,
        stride,
    )
}
#[cfg(feature = "ext_copy_memory_indirect")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectNV.html>"]
#[doc(alias = "vkCmdCopyMemoryToImageIndirectNV")]
#[inline]
pub unsafe fn cmd_copy_memory_to_image_indirect_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    copy_buffer_address: DeviceAddress,
    stride: u32,
    dst_image: &raw::Image,
    dst_image_layout: ImageLayout,
    p_image_subresources: impl AsSlice<'a, ImageSubresourceLayers>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_memory_to_image_indirect_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        copy_buffer_address,
        p_image_subresources.as_slice().len() as _,
        stride,
        Some(dst_image.borrow()),
        dst_image_layout,
        p_image_subresources.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_memory_decompression")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryNV.html>"]
#[doc(alias = "vkCmdDecompressMemoryNV")]
#[inline]
pub unsafe fn cmd_decompress_memory_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    p_decompress_memory_regions: impl AsSlice<'a, DecompressMemoryRegionNV>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_decompress_memory_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_decompress_memory_regions.as_slice().len() as _,
        p_decompress_memory_regions.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_memory_decompression")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountNV.html>"]
#[doc(alias = "vkCmdDecompressMemoryIndirectCountNV")]
#[inline]
pub unsafe fn cmd_decompress_memory_indirect_count_nv(
    command_buffer: &raw::CommandBuffer,
    indirect_commands_address: DeviceAddress,
    indirect_commands_count_address: DeviceAddress,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_decompress_memory_indirect_count_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        indirect_commands_address,
        indirect_commands_count_address,
        stride,
    )
}
#[cfg(feature = "ext_device_generated_commands_compute")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectMemoryRequirementsNV.html>"]
#[doc(alias = "vkGetPipelineIndirectMemoryRequirementsNV")]
pub unsafe fn get_pipeline_indirect_memory_requirements_nv<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_create_info: &ComputePipelineCreateInfo,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_pipeline_indirect_memory_requirements_nv
        .get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[cfg(feature = "ext_device_generated_commands_compute")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdatePipelineIndirectBufferNV.html>"]
#[doc(alias = "vkCmdUpdatePipelineIndirectBufferNV")]
#[inline]
pub unsafe fn cmd_update_pipeline_indirect_buffer_nv(
    command_buffer: &raw::CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: &raw::Pipeline,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_update_pipeline_indirect_buffer_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        pipeline_bind_point,
        Some(pipeline.borrow()),
    )
}
#[cfg(feature = "ext_device_generated_commands_compute")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectDeviceAddressNV.html>"]
#[doc(alias = "vkGetPipelineIndirectDeviceAddressNV")]
#[inline]
pub unsafe fn get_pipeline_indirect_device_address_nv(
    device: &raw::Device,
    p_info: &PipelineIndirectDeviceAddressInfoNV,
    dispatcher: &CommandsDispatcher,
) -> DeviceAddress {
    let vulkan_command = dispatcher.get_pipeline_indirect_device_address_nv.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info))
}
#[cfg(feature = "ext_ohos_external_memory")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetNativeBufferPropertiesOHOS.html>"]
#[doc(alias = "vkGetNativeBufferPropertiesOHOS")]
pub unsafe fn get_native_buffer_properties_ohos<
    S: StructureChainOut<NativeBufferPropertiesOHOS<'static>>,
>(
    device: &raw::Device,
    buffer: &OH_NativeBuffer,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.get_native_buffer_properties_ohos.get();
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(buffer),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_properties.as_mut_ptr());
        p_properties.assume_init()
    })
}
#[cfg(feature = "ext_ohos_external_memory")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryNativeBufferOHOS.html>"]
#[doc(alias = "vkGetMemoryNativeBufferOHOS")]
#[inline]
pub unsafe fn get_memory_native_buffer_ohos(
    device: &raw::Device,
    p_info: &MemoryGetNativeBufferInfoOHOS,
    p_buffer: &&OH_NativeBuffer,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.get_memory_native_buffer_ohos.get();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        ptr::from_ref(p_buffer).cast(),
    )
    .map_success(|| ())
}
#[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampEnableEXT.html>"]
#[doc(alias = "vkCmdSetDepthClampEnableEXT")]
#[inline]
pub unsafe fn cmd_set_depth_clamp_enable_ext(
    command_buffer: &raw::CommandBuffer,
    depth_clamp_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_clamp_enable_ext.get();
    vulkan_command(Some(command_buffer.borrow()), depth_clamp_enable.into())
}
#[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPolygonModeEXT.html>"]
#[doc(alias = "vkCmdSetPolygonModeEXT")]
#[inline]
pub unsafe fn cmd_set_polygon_mode_ext(
    command_buffer: &raw::CommandBuffer,
    polygon_mode: PolygonMode,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_polygon_mode_ext.get();
    vulkan_command(Some(command_buffer.borrow()), polygon_mode)
}
#[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationSamplesEXT.html>"]
#[doc(alias = "vkCmdSetRasterizationSamplesEXT")]
#[inline]
pub unsafe fn cmd_set_rasterization_samples_ext(
    command_buffer: &raw::CommandBuffer,
    rasterization_samples: SampleCountFlags,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_rasterization_samples_ext.get();
    vulkan_command(Some(command_buffer.borrow()), rasterization_samples)
}
#[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleMaskEXT.html>"]
#[doc(alias = "vkCmdSetSampleMaskEXT")]
#[inline]
pub unsafe fn cmd_set_sample_mask_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    samples: SampleCountFlags,
    p_sample_mask: Option<impl AsSlice<'a, SampleMask>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_sample_mask_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        samples,
        p_sample_mask
            .map(|p| p.as_slice().as_ptr().cast())
            .unwrap_or(ptr::null()),
    )
}
#[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToCoverageEnableEXT.html>"]
#[doc(alias = "vkCmdSetAlphaToCoverageEnableEXT")]
#[inline]
pub unsafe fn cmd_set_alpha_to_coverage_enable_ext(
    command_buffer: &raw::CommandBuffer,
    alpha_to_coverage_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_alpha_to_coverage_enable_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        alpha_to_coverage_enable.into(),
    )
}
#[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToOneEnableEXT.html>"]
#[doc(alias = "vkCmdSetAlphaToOneEnableEXT")]
#[inline]
pub unsafe fn cmd_set_alpha_to_one_enable_ext(
    command_buffer: &raw::CommandBuffer,
    alpha_to_one_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_alpha_to_one_enable_ext.get();
    vulkan_command(Some(command_buffer.borrow()), alpha_to_one_enable.into())
}
#[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEnableEXT.html>"]
#[doc(alias = "vkCmdSetLogicOpEnableEXT")]
#[inline]
pub unsafe fn cmd_set_logic_op_enable_ext(
    command_buffer: &raw::CommandBuffer,
    logic_op_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_logic_op_enable_ext.get();
    vulkan_command(Some(command_buffer.borrow()), logic_op_enable.into())
}
#[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEnableEXT.html>"]
#[doc(alias = "vkCmdSetColorBlendEnableEXT")]
#[inline]
pub unsafe fn cmd_set_color_blend_enable_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    first_attachment: u32,
    p_color_blend_enables: impl AsSlice<'a, Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_color_blend_enable_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_attachment,
        p_color_blend_enables.as_slice().len() as _,
        p_color_blend_enables.as_slice().as_ptr().cast(),
    )
}
#[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEquationEXT.html>"]
#[doc(alias = "vkCmdSetColorBlendEquationEXT")]
#[inline]
pub unsafe fn cmd_set_color_blend_equation_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    first_attachment: u32,
    p_color_blend_equations: impl AsSlice<'a, ColorBlendEquationEXT>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_color_blend_equation_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_attachment,
        p_color_blend_equations.as_slice().len() as _,
        p_color_blend_equations.as_slice().as_ptr().cast(),
    )
}
#[cfg(any(feature = "ext_extended_dynamic_state3", feature = "ext_shader_object"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteMaskEXT.html>"]
#[doc(alias = "vkCmdSetColorWriteMaskEXT")]
#[inline]
pub unsafe fn cmd_set_color_write_mask_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    first_attachment: u32,
    p_color_write_masks: impl AsSlice<'a, ColorComponentFlags>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_color_write_mask_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_attachment,
        p_color_write_masks.as_slice().len() as _,
        p_color_write_masks.as_slice().as_ptr().cast(),
    )
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
pub unsafe fn cmd_set_tessellation_domain_origin_ext(
    command_buffer: &raw::CommandBuffer,
    domain_origin: TessellationDomainOrigin,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_tessellation_domain_origin_ext.get();
    vulkan_command(Some(command_buffer.borrow()), domain_origin)
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
pub unsafe fn cmd_set_rasterization_stream_ext(
    command_buffer: &raw::CommandBuffer,
    rasterization_stream: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_rasterization_stream_ext.get();
    vulkan_command(Some(command_buffer.borrow()), rasterization_stream)
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
pub unsafe fn cmd_set_conservative_rasterization_mode_ext(
    command_buffer: &raw::CommandBuffer,
    conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_conservative_rasterization_mode_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        conservative_rasterization_mode,
    )
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
pub unsafe fn cmd_set_extra_primitive_overestimation_size_ext(
    command_buffer: &raw::CommandBuffer,
    extra_primitive_overestimation_size: f32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_extra_primitive_overestimation_size_ext
        .get();
    vulkan_command(
        Some(command_buffer.borrow()),
        extra_primitive_overestimation_size,
    )
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
pub unsafe fn cmd_set_depth_clip_enable_ext(
    command_buffer: &raw::CommandBuffer,
    depth_clip_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_clip_enable_ext.get();
    vulkan_command(Some(command_buffer.borrow()), depth_clip_enable.into())
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
pub unsafe fn cmd_set_sample_locations_enable_ext(
    command_buffer: &raw::CommandBuffer,
    sample_locations_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_sample_locations_enable_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        sample_locations_enable.into(),
    )
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
pub unsafe fn cmd_set_color_blend_advanced_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    first_attachment: u32,
    p_color_blend_advanced: impl AsSlice<'a, ColorBlendAdvancedEXT>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_color_blend_advanced_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_attachment,
        p_color_blend_advanced.as_slice().len() as _,
        p_color_blend_advanced.as_slice().as_ptr().cast(),
    )
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
pub unsafe fn cmd_set_provoking_vertex_mode_ext(
    command_buffer: &raw::CommandBuffer,
    provoking_vertex_mode: ProvokingVertexModeEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_provoking_vertex_mode_ext.get();
    vulkan_command(Some(command_buffer.borrow()), provoking_vertex_mode)
}
#[cfg(any(
    all(
        feature = "ext_extended_dynamic_state3",
        any(feature = "version_1_4", feature = "ext_line_rasterization")
    ),
    all(
        feature = "ext_shader_object",
        any(feature = "version_1_4", feature = "ext_line_rasterization")
    )
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineRasterizationModeEXT.html>"]
#[doc(alias = "vkCmdSetLineRasterizationModeEXT")]
#[inline]
pub unsafe fn cmd_set_line_rasterization_mode_ext(
    command_buffer: &raw::CommandBuffer,
    line_rasterization_mode: LineRasterizationModeEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_line_rasterization_mode_ext.get();
    vulkan_command(Some(command_buffer.borrow()), line_rasterization_mode)
}
#[cfg(any(
    all(
        feature = "ext_extended_dynamic_state3",
        any(feature = "version_1_4", feature = "ext_line_rasterization")
    ),
    all(
        feature = "ext_shader_object",
        any(feature = "version_1_4", feature = "ext_line_rasterization")
    )
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEnableEXT.html>"]
#[doc(alias = "vkCmdSetLineStippleEnableEXT")]
#[inline]
pub unsafe fn cmd_set_line_stipple_enable_ext(
    command_buffer: &raw::CommandBuffer,
    stippled_line_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_line_stipple_enable_ext.get();
    vulkan_command(Some(command_buffer.borrow()), stippled_line_enable.into())
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
pub unsafe fn cmd_set_depth_clip_negative_one_to_one_ext(
    command_buffer: &raw::CommandBuffer,
    negative_one_to_one: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_clip_negative_one_to_one_ext.get();
    vulkan_command(Some(command_buffer.borrow()), negative_one_to_one.into())
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
pub unsafe fn cmd_set_viewport_wscaling_enable_nv(
    command_buffer: &raw::CommandBuffer,
    viewport_wscaling_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_viewport_wscaling_enable_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        viewport_wscaling_enable.into(),
    )
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
pub unsafe fn cmd_set_viewport_swizzle_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    first_viewport: u32,
    p_viewport_swizzles: impl AsSlice<'a, ViewportSwizzleNV>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_viewport_swizzle_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        first_viewport,
        p_viewport_swizzles.as_slice().len() as _,
        p_viewport_swizzles.as_slice().as_ptr().cast(),
    )
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
pub unsafe fn cmd_set_coverage_to_color_enable_nv(
    command_buffer: &raw::CommandBuffer,
    coverage_to_color_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_coverage_to_color_enable_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        coverage_to_color_enable.into(),
    )
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
pub unsafe fn cmd_set_coverage_to_color_location_nv(
    command_buffer: &raw::CommandBuffer,
    coverage_to_color_location: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_coverage_to_color_location_nv.get();
    vulkan_command(Some(command_buffer.borrow()), coverage_to_color_location)
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
pub unsafe fn cmd_set_coverage_modulation_mode_nv(
    command_buffer: &raw::CommandBuffer,
    coverage_modulation_mode: CoverageModulationModeNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_coverage_modulation_mode_nv.get();
    vulkan_command(Some(command_buffer.borrow()), coverage_modulation_mode)
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
pub unsafe fn cmd_set_coverage_modulation_table_enable_nv(
    command_buffer: &raw::CommandBuffer,
    coverage_modulation_table_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_coverage_modulation_table_enable_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        coverage_modulation_table_enable.into(),
    )
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
pub unsafe fn cmd_set_coverage_modulation_table_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    p_coverage_modulation_table: impl AsSlice<'a, f32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_coverage_modulation_table_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_coverage_modulation_table.as_slice().len() as _,
        p_coverage_modulation_table.as_slice().as_ptr().cast(),
    )
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
pub unsafe fn cmd_set_shading_rate_image_enable_nv(
    command_buffer: &raw::CommandBuffer,
    shading_rate_image_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_shading_rate_image_enable_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        shading_rate_image_enable.into(),
    )
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
pub unsafe fn cmd_set_representative_fragment_test_enable_nv(
    command_buffer: &raw::CommandBuffer,
    representative_fragment_test_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_representative_fragment_test_enable_nv
        .get();
    vulkan_command(
        Some(command_buffer.borrow()),
        representative_fragment_test_enable.into(),
    )
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
pub unsafe fn cmd_set_coverage_reduction_mode_nv(
    command_buffer: &raw::CommandBuffer,
    coverage_reduction_mode: CoverageReductionModeNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_coverage_reduction_mode_nv.get();
    vulkan_command(Some(command_buffer.borrow()), coverage_reduction_mode)
}
#[cfg(feature = "ext_tensors")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorARM.html>"]
#[doc(alias = "vkCreateTensorARM")]
pub unsafe fn create_tensor_arm(
    device: &raw::Device,
    p_create_info: &TensorCreateInfoARM,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<TensorARM> {
    let vulkan_command = dispatcher.create_tensor_arm.get();
    let mut p_tensor = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_tensor.as_mut_ptr(),
    );
    vk_status.map_success(|| p_tensor.assume_init())
}
#[cfg(feature = "ext_tensors")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorARM.html>"]
#[doc(alias = "vkDestroyTensorARM")]
#[inline]
pub unsafe fn destroy_tensor_arm(
    device: &raw::Device,
    tensor: Option<&raw::TensorARM>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_tensor_arm.get();
    vulkan_command(
        Some(device.borrow()),
        tensor.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_tensors")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorViewARM.html>"]
#[doc(alias = "vkCreateTensorViewARM")]
pub unsafe fn create_tensor_view_arm(
    device: &raw::Device,
    p_create_info: &TensorViewCreateInfoARM,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<TensorViewARM> {
    let vulkan_command = dispatcher.create_tensor_view_arm.get();
    let mut p_view = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_view.as_mut_ptr(),
    );
    vk_status.map_success(|| p_view.assume_init())
}
#[cfg(feature = "ext_tensors")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorViewARM.html>"]
#[doc(alias = "vkDestroyTensorViewARM")]
#[inline]
pub unsafe fn destroy_tensor_view_arm(
    device: &raw::Device,
    tensor_view: Option<&raw::TensorViewARM>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_tensor_view_arm.get();
    vulkan_command(
        Some(device.borrow()),
        tensor_view.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_tensors")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorMemoryRequirementsARM.html>"]
#[doc(alias = "vkGetTensorMemoryRequirementsARM")]
pub unsafe fn get_tensor_memory_requirements_arm<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &TensorMemoryRequirementsInfoARM,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_tensor_memory_requirements_arm.get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[cfg(feature = "ext_tensors")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindTensorMemoryARM.html>"]
#[doc(alias = "vkBindTensorMemoryARM")]
#[inline]
pub unsafe fn bind_tensor_memory_arm<'a>(
    device: &raw::Device,
    p_bind_infos: impl AsSlice<'a, BindTensorMemoryInfoARM<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.bind_tensor_memory_arm.get();
    vulkan_command(
        Some(device.borrow()),
        p_bind_infos.as_slice().len() as _,
        p_bind_infos.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_tensors")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceTensorMemoryRequirementsARM.html>"]
#[doc(alias = "vkGetDeviceTensorMemoryRequirementsARM")]
pub unsafe fn get_device_tensor_memory_requirements_arm<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &DeviceTensorMemoryRequirementsARM,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_device_tensor_memory_requirements_arm.get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[cfg(feature = "ext_tensors")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyTensorARM.html>"]
#[doc(alias = "vkCmdCopyTensorARM")]
#[inline]
pub unsafe fn cmd_copy_tensor_arm(
    command_buffer: &raw::CommandBuffer,
    p_copy_tensor_info: &CopyTensorInfoARM,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_tensor_arm.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_copy_tensor_info),
    )
}
#[cfg(feature = "ext_tensors")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalTensorPropertiesARM.html>"]
#[doc(alias = "vkGetPhysicalDeviceExternalTensorPropertiesARM")]
pub unsafe fn get_physical_device_external_tensor_properties_arm<
    S: StructureChainOut<ExternalTensorPropertiesARM<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_external_tensor_info: &PhysicalDeviceExternalTensorInfoARM,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_external_tensor_properties_arm
        .get();
    let mut p_external_tensor_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_external_tensor_properties);
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_external_tensor_info),
        S::get_uninit_head_ptr(p_external_tensor_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_external_tensor_properties.as_mut_ptr());
    p_external_tensor_properties.assume_init()
}
#[cfg(all(feature = "ext_tensors", feature = "ext_descriptor_buffer"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDescriptorDataARM.html>"]
#[doc(alias = "vkGetTensorOpaqueCaptureDescriptorDataARM")]
#[inline]
pub unsafe fn get_tensor_opaque_capture_descriptor_data_arm(
    device: &raw::Device,
    p_info: &TensorCaptureDescriptorDataInfoARM,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_tensor_opaque_capture_descriptor_data_arm
        .get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info), p_data).map_success(|| ())
}
#[cfg(all(feature = "ext_tensors", feature = "ext_descriptor_buffer"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorViewOpaqueCaptureDescriptorDataARM.html>"]
#[doc(alias = "vkGetTensorViewOpaqueCaptureDescriptorDataARM")]
#[inline]
pub unsafe fn get_tensor_view_opaque_capture_descriptor_data_arm(
    device: &raw::Device,
    p_info: &TensorViewCaptureDescriptorDataInfoARM,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_tensor_view_opaque_capture_descriptor_data_arm
        .get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info), p_data).map_success(|| ())
}
#[cfg(feature = "ext_shader_module_identifier")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleIdentifierEXT.html>"]
#[doc(alias = "vkGetShaderModuleIdentifierEXT")]
pub unsafe fn get_shader_module_identifier_ext<
    S: StructureChainOut<ShaderModuleIdentifierEXT<'static>>,
>(
    device: &raw::Device,
    shader_module: &raw::ShaderModule,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_shader_module_identifier_ext.get();
    let mut p_identifier = MaybeUninit::uninit();
    S::setup_uninit(&mut p_identifier);
    vulkan_command(
        Some(device.borrow()),
        Some(shader_module.borrow()),
        S::get_uninit_head_ptr(p_identifier.as_mut_ptr()),
    );
    S::setup_cleanup(p_identifier.as_mut_ptr());
    p_identifier.assume_init()
}
#[cfg(feature = "ext_shader_module_identifier")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleCreateInfoIdentifierEXT.html>"]
#[doc(alias = "vkGetShaderModuleCreateInfoIdentifierEXT")]
pub unsafe fn get_shader_module_create_info_identifier_ext<
    S: StructureChainOut<ShaderModuleIdentifierEXT<'static>>,
>(
    device: &raw::Device,
    p_create_info: &ShaderModuleCreateInfo,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_shader_module_create_info_identifier_ext
        .get();
    let mut p_identifier = MaybeUninit::uninit();
    S::setup_uninit(&mut p_identifier);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        S::get_uninit_head_ptr(p_identifier.as_mut_ptr()),
    );
    S::setup_cleanup(p_identifier.as_mut_ptr());
    p_identifier.assume_init()
}
#[cfg(feature = "ext_optical_flow")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html>"]
#[doc(alias = "vkGetPhysicalDeviceOpticalFlowImageFormatsNV")]
pub unsafe fn get_physical_device_optical_flow_image_formats_nv<
    R: DynamicArray<OpticalFlowImageFormatPropertiesNV<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_optical_flow_image_format_info: &OpticalFlowImageFormatInfoNV,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_optical_flow_image_formats_nv
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_format_count = vk_len.as_mut_ptr();
    let p_image_format_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_optical_flow_image_format_info),
        p_format_count,
        p_image_format_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_format_count = ptr::from_mut(&mut vk_len);
    let mut p_image_format_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            ptr::from_ref(p_optical_flow_image_format_info),
            p_format_count,
            p_image_format_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_format_count = ptr::from_mut(&mut vk_len);
        p_image_format_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_optical_flow")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateOpticalFlowSessionNV.html>"]
#[doc(alias = "vkCreateOpticalFlowSessionNV")]
pub unsafe fn create_optical_flow_session_nv(
    device: &raw::Device,
    p_create_info: &OpticalFlowSessionCreateInfoNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<OpticalFlowSessionNV> {
    let vulkan_command = dispatcher.create_optical_flow_session_nv.get();
    let mut p_session = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_session.as_mut_ptr(),
    );
    vk_status.map_success(|| p_session.assume_init())
}
#[cfg(feature = "ext_optical_flow")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyOpticalFlowSessionNV.html>"]
#[doc(alias = "vkDestroyOpticalFlowSessionNV")]
#[inline]
pub unsafe fn destroy_optical_flow_session_nv(
    device: &raw::Device,
    session: &raw::OpticalFlowSessionNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_optical_flow_session_nv.get();
    vulkan_command(
        Some(device.borrow()),
        Some(session.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_optical_flow")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindOpticalFlowSessionImageNV.html>"]
#[doc(alias = "vkBindOpticalFlowSessionImageNV")]
#[inline]
pub unsafe fn bind_optical_flow_session_image_nv(
    device: &raw::Device,
    session: &raw::OpticalFlowSessionNV,
    binding_point: OpticalFlowSessionBindingPointNV,
    view: Option<&raw::ImageView>,
    layout: ImageLayout,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.bind_optical_flow_session_image_nv.get();
    vulkan_command(
        Some(device.borrow()),
        Some(session.borrow()),
        binding_point,
        view.map(|v| v.borrow()),
        layout,
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_optical_flow")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdOpticalFlowExecuteNV.html>"]
#[doc(alias = "vkCmdOpticalFlowExecuteNV")]
#[inline]
pub unsafe fn cmd_optical_flow_execute_nv(
    command_buffer: &raw::CommandBuffer,
    session: &raw::OpticalFlowSessionNV,
    p_execute_info: &OpticalFlowExecuteInfoNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_optical_flow_execute_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(session.borrow()),
        ptr::from_ref(p_execute_info),
    )
}
#[cfg(feature = "ext_anti_lag")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAntiLagUpdateAMD.html>"]
#[doc(alias = "vkAntiLagUpdateAMD")]
#[inline]
pub unsafe fn anti_lag_update_amd(
    device: &raw::Device,
    p_data: &AntiLagDataAMD,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.anti_lag_update_amd.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_data))
}
#[cfg(feature = "ext_present_wait2")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresent2KHR.html>"]
#[doc(alias = "vkWaitForPresent2KHR")]
#[inline]
pub unsafe fn wait_for_present2_khr(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    p_present_wait2_info: &PresentWait2InfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher.wait_for_present2_khr.get();
    vulkan_command(
        Some(device.borrow()),
        Some(swapchain.borrow()),
        ptr::from_ref(p_present_wait2_info),
    )
    .into_result()
}
#[cfg(feature = "ext_shader_object")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShadersEXT.html>"]
#[doc(alias = "vkCreateShadersEXT")]
pub unsafe fn create_shaders_ext<'a, R: DynamicArray<ShaderEXT>>(
    device: &raw::Device,
    p_create_infos: impl AsSlice<'a, ShaderCreateInfoEXT<'a>>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, R)> {
    let vulkan_command = dispatcher.create_shaders_ext.get();
    let mut p_shaders = R::create_with_capacity(p_create_infos.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        p_create_infos.as_slice().len() as _,
        p_create_infos.as_slice().as_ptr().cast(),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_shaders.get_content_mut_ptr(),
    );
    vk_status.map_successes(|| {
        p_shaders.resize_with_len(p_create_infos.as_slice().len() as _);
        p_shaders
    })
}
#[cfg(feature = "ext_shader_object")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderEXT.html>"]
#[doc(alias = "vkDestroyShaderEXT")]
#[inline]
pub unsafe fn destroy_shader_ext(
    device: &raw::Device,
    shader: Option<&raw::ShaderEXT>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_shader_ext.get();
    vulkan_command(
        Some(device.borrow()),
        shader.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_shader_object")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderBinaryDataEXT.html>"]
#[doc(alias = "vkGetShaderBinaryDataEXT")]
pub unsafe fn get_shader_binary_data_ext(
    device: &raw::Device,
    shader: &raw::ShaderEXT,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<usize> {
    let vulkan_command = dispatcher.get_shader_binary_data_ext.get();
    let mut p_data_size = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(shader.borrow()),
        p_data_size.as_mut_ptr(),
        p_data,
    );
    vk_status.map_success(|| p_data_size.assume_init())
}
#[cfg(feature = "ext_shader_object")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadersEXT.html>"]
#[doc(alias = "vkCmdBindShadersEXT")]
#[inline]
pub unsafe fn cmd_bind_shaders_ext<'a, V3: Alias<raw::ShaderEXT> + 'a>(
    command_buffer: &raw::CommandBuffer,
    p_stages: impl AsSlice<'a, ShaderStageFlags>,
    p_shaders: impl AsSlice<'a, V3>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_shaders_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_shaders.as_slice().len() as _,
        p_stages.as_slice().as_ptr().cast(),
        p_shaders.as_slice().as_ptr().cast(),
    )
}
#[cfg(all(feature = "ext_shader_object", feature = "ext_depth_clamp_control"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampRangeEXT.html>"]
#[doc(alias = "vkCmdSetDepthClampRangeEXT")]
#[inline]
pub unsafe fn cmd_set_depth_clamp_range_ext(
    command_buffer: &raw::CommandBuffer,
    depth_clamp_mode: DepthClampModeEXT,
    p_depth_clamp_range: Option<&DepthClampRangeEXT>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_depth_clamp_range_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        depth_clamp_mode,
        p_depth_clamp_range
            .map(|v| ptr::from_ref(v))
            .unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_pipeline_binary")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineBinariesKHR.html>"]
#[doc(alias = "vkCreatePipelineBinariesKHR")]
pub unsafe fn create_pipeline_binaries_khr<
    S: StructureChainOut<PipelineBinaryHandlesInfoKHR<'static>>,
>(
    device: &raw::Device,
    p_create_info: &PipelineBinaryCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, S)> {
    let vulkan_command = dispatcher.create_pipeline_binaries_khr.get();
    let mut p_binaries = MaybeUninit::uninit();
    S::setup_uninit(&mut p_binaries);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        S::get_uninit_head_ptr(p_binaries.as_mut_ptr()),
    );
    vk_status.map_successes(|| {
        S::setup_cleanup(p_binaries.as_mut_ptr());
        p_binaries.assume_init()
    })
}
#[cfg(feature = "ext_pipeline_binary")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineBinaryKHR.html>"]
#[doc(alias = "vkDestroyPipelineBinaryKHR")]
#[inline]
pub unsafe fn destroy_pipeline_binary_khr(
    device: &raw::Device,
    pipeline_binary: Option<&raw::PipelineBinaryKHR>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_pipeline_binary_khr.get();
    vulkan_command(
        Some(device.borrow()),
        pipeline_binary.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_pipeline_binary")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineKeyKHR.html>"]
#[doc(alias = "vkGetPipelineKeyKHR")]
pub unsafe fn get_pipeline_key_khr<S: StructureChainOut<PipelineBinaryKeyKHR<'static>>>(
    device: &raw::Device,
    p_pipeline_create_info: Option<&PipelineCreateInfoKHR>,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.get_pipeline_key_khr.get();
    let mut p_pipeline_key = MaybeUninit::uninit();
    S::setup_uninit(&mut p_pipeline_key);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        p_pipeline_create_info
            .map(|v| ptr::from_ref(v))
            .unwrap_or(ptr::null()),
        S::get_uninit_head_ptr(p_pipeline_key.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_pipeline_key.as_mut_ptr());
        p_pipeline_key.assume_init()
    })
}
#[cfg(feature = "ext_pipeline_binary")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseCapturedPipelineDataKHR.html>"]
#[doc(alias = "vkReleaseCapturedPipelineDataKHR")]
#[inline]
pub unsafe fn release_captured_pipeline_data_khr(
    device: &raw::Device,
    p_info: &ReleaseCapturedPipelineDataInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.release_captured_pipeline_data_khr.get();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_tile_properties")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFramebufferTilePropertiesQCOM.html>"]
#[doc(alias = "vkGetFramebufferTilePropertiesQCOM")]
pub unsafe fn get_framebuffer_tile_properties_qcom<R: DynamicArray<TilePropertiesQCOM<'static>>>(
    device: &raw::Device,
    framebuffer: &raw::Framebuffer,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_framebuffer_tile_properties_qcom.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_properties_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(device.borrow()),
        Some(framebuffer.borrow()),
        p_properties_count,
        p_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_properties_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(device.borrow()),
            Some(framebuffer.borrow()),
            p_properties_count,
            p_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_properties_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_tile_properties")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDynamicRenderingTilePropertiesQCOM.html>"]
#[doc(alias = "vkGetDynamicRenderingTilePropertiesQCOM")]
pub unsafe fn get_dynamic_rendering_tile_properties_qcom<
    S: StructureChainOut<TilePropertiesQCOM<'static>>,
>(
    device: &raw::Device,
    p_rendering_info: &RenderingInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.get_dynamic_rendering_tile_properties_qcom.get();
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_rendering_info),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_properties.as_mut_ptr());
        p_properties.assume_init()
    })
}
#[cfg(feature = "ext_swapchain_maintenance1")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesKHR.html>"]
#[doc(alias = "vkReleaseSwapchainImagesKHR")]
#[inline]
pub unsafe fn release_swapchain_images_khr(
    device: &raw::Device,
    p_release_info: &ReleaseSwapchainImagesInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.release_swapchain_images_khr.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_release_info)).map_success(|| ())
}
#[cfg(feature = "ext_swapchain_maintenance1")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesEXT.html>"]
#[doc(alias = "vkReleaseSwapchainImagesEXT")]
#[inline]
pub unsafe fn release_swapchain_images_ext(
    device: &raw::Device,
    p_release_info: &ReleaseSwapchainImagesInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.release_swapchain_images_ext.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_release_info)).map_success(|| ())
}
#[cfg(feature = "ext_cooperative_vector")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeVectorPropertiesNV.html>"]
#[doc(alias = "vkGetPhysicalDeviceCooperativeVectorPropertiesNV")]
pub unsafe fn get_physical_device_cooperative_vector_properties_nv<
    R: DynamicArray<CooperativeVectorPropertiesNV<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_cooperative_vector_properties_nv
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_property_count,
        p_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_property_count,
            p_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_property_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_cooperative_vector")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkConvertCooperativeVectorMatrixNV.html>"]
#[doc(alias = "vkConvertCooperativeVectorMatrixNV")]
#[inline]
pub unsafe fn convert_cooperative_vector_matrix_nv(
    device: &raw::Device,
    p_info: &ConvertCooperativeVectorMatrixInfoNV,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.convert_cooperative_vector_matrix_nv.get();
    vulkan_command(Some(device.borrow()), ptr::from_ref(p_info)).map_success(|| ())
}
#[cfg(feature = "ext_cooperative_vector")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdConvertCooperativeVectorMatrixNV.html>"]
#[doc(alias = "vkCmdConvertCooperativeVectorMatrixNV")]
#[inline]
pub unsafe fn cmd_convert_cooperative_vector_matrix_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    p_infos: impl AsSlice<'a, ConvertCooperativeVectorMatrixInfoNV<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_convert_cooperative_vector_matrix_nv.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_infos.as_slice().len() as _,
        p_infos.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_low_latency2")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeNV.html>"]
#[doc(alias = "vkSetLatencySleepModeNV")]
#[inline]
pub unsafe fn set_latency_sleep_mode_nv(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    p_sleep_mode_info: &LatencySleepModeInfoNV,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.set_latency_sleep_mode_nv.get();
    vulkan_command(
        Some(device.borrow()),
        Some(swapchain.borrow()),
        ptr::from_ref(p_sleep_mode_info),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_low_latency2")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepNV.html>"]
#[doc(alias = "vkLatencySleepNV")]
#[inline]
pub unsafe fn latency_sleep_nv(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    p_sleep_info: &LatencySleepInfoNV,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.latency_sleep_nv.get();
    vulkan_command(
        Some(device.borrow()),
        Some(swapchain.borrow()),
        ptr::from_ref(p_sleep_info),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_low_latency2")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerNV.html>"]
#[doc(alias = "vkSetLatencyMarkerNV")]
#[inline]
pub unsafe fn set_latency_marker_nv(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    p_latency_marker_info: &SetLatencyMarkerInfoNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.set_latency_marker_nv.get();
    vulkan_command(
        Some(device.borrow()),
        Some(swapchain.borrow()),
        ptr::from_ref(p_latency_marker_info),
    )
}
#[cfg(feature = "ext_low_latency2")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsNV.html>"]
#[doc(alias = "vkGetLatencyTimingsNV")]
pub unsafe fn get_latency_timings_nv<S: StructureChainOut<GetLatencyMarkerInfoNV<'static>>>(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_latency_timings_nv.get();
    let mut p_latency_marker_info = MaybeUninit::uninit();
    S::setup_uninit(&mut p_latency_marker_info);
    vulkan_command(
        Some(device.borrow()),
        Some(swapchain.borrow()),
        S::get_uninit_head_ptr(p_latency_marker_info.as_mut_ptr()),
    );
    S::setup_cleanup(p_latency_marker_info.as_mut_ptr());
    p_latency_marker_info.assume_init()
}
#[cfg(feature = "ext_low_latency2")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandNV.html>"]
#[doc(alias = "vkQueueNotifyOutOfBandNV")]
#[inline]
pub unsafe fn queue_notify_out_of_band_nv(
    queue: &raw::Queue,
    p_queue_type_info: &OutOfBandQueueTypeInfoNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.queue_notify_out_of_band_nv.get();
    vulkan_command(Some(queue.borrow()), ptr::from_ref(p_queue_type_info))
}
#[cfg(feature = "ext_cooperative_matrix")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR")]
pub unsafe fn get_physical_device_cooperative_matrix_properties_khr<
    R: DynamicArray<CooperativeMatrixPropertiesKHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_cooperative_matrix_properties_khr
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_property_count,
        p_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_property_count,
            p_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_property_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_data_graph")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelinesARM.html>"]
#[doc(alias = "vkCreateDataGraphPipelinesARM")]
pub unsafe fn create_data_graph_pipelines_arm<'a, R: DynamicArray<Pipeline>>(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    pipeline_cache: Option<&raw::PipelineCache>,
    p_create_infos: impl AsSlice<'a, DataGraphPipelineCreateInfoARM<'a>>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, R)> {
    let vulkan_command = dispatcher.create_data_graph_pipelines_arm.get();
    let mut p_pipelines = R::create_with_capacity(p_create_infos.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        deferred_operation.map(|v| v.borrow()),
        pipeline_cache.map(|v| v.borrow()),
        p_create_infos.as_slice().len() as _,
        p_create_infos.as_slice().as_ptr().cast(),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_pipelines.get_content_mut_ptr(),
    );
    vk_status.map_successes(|| {
        p_pipelines.resize_with_len(p_create_infos.as_slice().len() as _);
        p_pipelines
    })
}
#[cfg(feature = "ext_data_graph")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelineSessionARM.html>"]
#[doc(alias = "vkCreateDataGraphPipelineSessionARM")]
pub unsafe fn create_data_graph_pipeline_session_arm(
    device: &raw::Device,
    p_create_info: &DataGraphPipelineSessionCreateInfoARM,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DataGraphPipelineSessionARM> {
    let vulkan_command = dispatcher.create_data_graph_pipeline_session_arm.get();
    let mut p_session = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_session.as_mut_ptr(),
    );
    vk_status.map_success(|| p_session.assume_init())
}
#[cfg(feature = "ext_data_graph")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionBindPointRequirementsARM.html>"]
#[doc(alias = "vkGetDataGraphPipelineSessionBindPointRequirementsARM")]
pub unsafe fn get_data_graph_pipeline_session_bind_point_requirements_arm<
    R: DynamicArray<DataGraphPipelineSessionBindPointRequirementARM<'static>>,
>(
    device: &raw::Device,
    p_info: &DataGraphPipelineSessionBindPointRequirementsInfoARM,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_data_graph_pipeline_session_bind_point_requirements_arm
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_bind_point_requirement_count = vk_len.as_mut_ptr();
    let p_bind_point_requirements = ptr::null_mut();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        p_bind_point_requirement_count,
        p_bind_point_requirements,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_bind_point_requirement_count = ptr::from_mut(&mut vk_len);
    let mut p_bind_point_requirements = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(device.borrow()),
            ptr::from_ref(p_info),
            p_bind_point_requirement_count,
            p_bind_point_requirements,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_bind_point_requirement_count = ptr::from_mut(&mut vk_len);
        p_bind_point_requirements = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_data_graph")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionMemoryRequirementsARM.html>"]
#[doc(alias = "vkGetDataGraphPipelineSessionMemoryRequirementsARM")]
pub unsafe fn get_data_graph_pipeline_session_memory_requirements_arm<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &DataGraphPipelineSessionMemoryRequirementsInfoARM,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_data_graph_pipeline_session_memory_requirements_arm
        .get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[cfg(feature = "ext_data_graph")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindDataGraphPipelineSessionMemoryARM.html>"]
#[doc(alias = "vkBindDataGraphPipelineSessionMemoryARM")]
#[inline]
pub unsafe fn bind_data_graph_pipeline_session_memory_arm<'a>(
    device: &raw::Device,
    p_bind_infos: impl AsSlice<'a, BindDataGraphPipelineSessionMemoryInfoARM<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher.bind_data_graph_pipeline_session_memory_arm.get();
    vulkan_command(
        Some(device.borrow()),
        p_bind_infos.as_slice().len() as _,
        p_bind_infos.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[cfg(feature = "ext_data_graph")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDataGraphPipelineSessionARM.html>"]
#[doc(alias = "vkDestroyDataGraphPipelineSessionARM")]
#[inline]
pub unsafe fn destroy_data_graph_pipeline_session_arm(
    device: &raw::Device,
    session: &raw::DataGraphPipelineSessionARM,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_data_graph_pipeline_session_arm.get();
    vulkan_command(
        Some(device.borrow()),
        Some(session.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_data_graph")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchDataGraphARM.html>"]
#[doc(alias = "vkCmdDispatchDataGraphARM")]
#[inline]
pub unsafe fn cmd_dispatch_data_graph_arm(
    command_buffer: &raw::CommandBuffer,
    session: &raw::DataGraphPipelineSessionARM,
    p_info: Option<&DataGraphPipelineDispatchInfoARM>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_dispatch_data_graph_arm.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(session.borrow()),
        p_info.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_data_graph")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineAvailablePropertiesARM.html>"]
#[doc(alias = "vkGetDataGraphPipelineAvailablePropertiesARM")]
pub unsafe fn get_data_graph_pipeline_available_properties_arm<
    R: DynamicArray<DataGraphPipelinePropertyARM>,
>(
    device: &raw::Device,
    p_pipeline_info: &DataGraphPipelineInfoARM,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_data_graph_pipeline_available_properties_arm
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_properties_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_pipeline_info),
        p_properties_count,
        p_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_properties_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(device.borrow()),
            ptr::from_ref(p_pipeline_info),
            p_properties_count,
            p_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_properties_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_data_graph")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelinePropertiesARM.html>"]
#[doc(alias = "vkGetDataGraphPipelinePropertiesARM")]
pub unsafe fn get_data_graph_pipeline_properties_arm<
    R: DynamicArray<DataGraphPipelinePropertyQueryResultARM<'static>>,
>(
    device: &raw::Device,
    p_pipeline_info: &DataGraphPipelineInfoARM,
    properties_count: u32,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher.get_data_graph_pipeline_properties_arm.get();
    let mut p_properties = R::create_with_capacity(properties_count as _);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_pipeline_info),
        properties_count,
        p_properties.get_content_mut_ptr(),
    );
    vk_status.map_success(|| {
        p_properties.resize_with_len(properties_count as _);
        p_properties
    })
}
#[cfg(feature = "ext_data_graph")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM.html>"]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM")]
pub unsafe fn get_physical_device_queue_family_data_graph_properties_arm<
    R: DynamicArray<QueueFamilyDataGraphPropertiesARM<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_queue_family_data_graph_properties_arm
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_queue_family_data_graph_property_count = vk_len.as_mut_ptr();
    let p_queue_family_data_graph_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        queue_family_index,
        p_queue_family_data_graph_property_count,
        p_queue_family_data_graph_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_queue_family_data_graph_property_count = ptr::from_mut(&mut vk_len);
    let mut p_queue_family_data_graph_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            queue_family_index,
            p_queue_family_data_graph_property_count,
            p_queue_family_data_graph_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_queue_family_data_graph_property_count = ptr::from_mut(&mut vk_len);
        p_queue_family_data_graph_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_data_graph")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM.html>"]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM")]
pub unsafe fn get_physical_device_queue_family_data_graph_processing_engine_properties_arm<
    S: StructureChainOut<QueueFamilyDataGraphProcessingEnginePropertiesARM<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_queue_family_data_graph_processing_engine_info : & PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_queue_family_data_graph_processing_engine_properties_arm
        .get();
    let mut p_queue_family_data_graph_processing_engine_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_queue_family_data_graph_processing_engine_properties);
    vulkan_command(
        Some(physical_device.borrow()),
        ptr::from_ref(p_queue_family_data_graph_processing_engine_info),
        S::get_uninit_head_ptr(p_queue_family_data_graph_processing_engine_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_queue_family_data_graph_processing_engine_properties.as_mut_ptr());
    p_queue_family_data_graph_processing_engine_properties.assume_init()
}
#[cfg(any(
    feature = "ext_data_graph_instruction_set_tosa",
    feature = "ext_data_graph_optical_flow"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM.html>"]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM")]
pub unsafe fn get_physical_device_queue_family_data_graph_engine_operation_properties_arm(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    p_queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM,
    dispatcher: &CommandsDispatcher,
) -> Result<BaseOutStructure<'static>> {
    let vulkan_command = dispatcher
        .get_physical_device_queue_family_data_graph_engine_operation_properties_arm
        .get();
    let mut p_properties = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(physical_device.borrow()),
        queue_family_index,
        ptr::from_ref(p_queue_family_data_graph_properties),
        p_properties.as_mut_ptr(),
    );
    vk_status.map_success(|| p_properties.assume_init())
}
#[cfg(feature = "ext_attachment_feedback_loop_dynamic_state")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAttachmentFeedbackLoopEnableEXT.html>"]
#[doc(alias = "vkCmdSetAttachmentFeedbackLoopEnableEXT")]
#[inline]
pub unsafe fn cmd_set_attachment_feedback_loop_enable_ext(
    command_buffer: &raw::CommandBuffer,
    aspect_mask: ImageAspectFlags,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_attachment_feedback_loop_enable_ext.get();
    vulkan_command(Some(command_buffer.borrow()), aspect_mask)
}
#[cfg(feature = "ext_external_memory_screen_buffer")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetScreenBufferPropertiesQNX.html>"]
#[doc(alias = "vkGetScreenBufferPropertiesQNX")]
pub unsafe fn get_screen_buffer_properties_qnx<
    S: StructureChainOut<ScreenBufferPropertiesQNX<'static>>,
>(
    device: &raw::Device,
    buffer: &VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.get_screen_buffer_properties_qnx.get();
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(buffer),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_properties.as_mut_ptr());
        p_properties.assume_init()
    })
}
#[cfg(feature = "ext_calibrated_timestamps")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsKHR")]
pub unsafe fn get_physical_device_calibrateable_time_domains_khr<R: DynamicArray<TimeDomainKHR>>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_calibrateable_time_domains_khr
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_time_domain_count = vk_len.as_mut_ptr();
    let p_time_domains = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_time_domain_count,
        p_time_domains,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_time_domain_count = ptr::from_mut(&mut vk_len);
    let mut p_time_domains = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_time_domain_count,
            p_time_domains,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_time_domain_count = ptr::from_mut(&mut vk_len);
        p_time_domains = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_calibrated_timestamps")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html>"]
#[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")]
pub unsafe fn get_physical_device_calibrateable_time_domains_ext<R: DynamicArray<TimeDomainKHR>>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_calibrateable_time_domains_ext
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_time_domain_count = vk_len.as_mut_ptr();
    let p_time_domains = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_time_domain_count,
        p_time_domains,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_time_domain_count = ptr::from_mut(&mut vk_len);
    let mut p_time_domains = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_time_domain_count,
            p_time_domains,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_time_domain_count = ptr::from_mut(&mut vk_len);
        p_time_domains = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(all(feature = "ext_maintenance6", feature = "ext_descriptor_buffer"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsets2EXT.html>"]
#[doc(alias = "vkCmdSetDescriptorBufferOffsets2EXT")]
#[inline]
pub unsafe fn cmd_set_descriptor_buffer_offsets2_ext(
    command_buffer: &raw::CommandBuffer,
    p_set_descriptor_buffer_offsets_info: &SetDescriptorBufferOffsetsInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_descriptor_buffer_offsets2_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_set_descriptor_buffer_offsets_info),
    )
}
#[cfg(all(feature = "ext_maintenance6", feature = "ext_descriptor_buffer"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html>"]
#[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplers2EXT")]
#[inline]
pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers2_ext(
    command_buffer: &raw::CommandBuffer,
    p_bind_descriptor_buffer_embedded_samplers_info: &BindDescriptorBufferEmbeddedSamplersInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_descriptor_buffer_embedded_samplers2_ext
        .get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_bind_descriptor_buffer_embedded_samplers_info),
    )
}
#[cfg(feature = "ext_tile_memory_heap")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTileMemoryQCOM.html>"]
#[doc(alias = "vkCmdBindTileMemoryQCOM")]
#[inline]
pub unsafe fn cmd_bind_tile_memory_qcom(
    command_buffer: &raw::CommandBuffer,
    p_tile_memory_bind_info: Option<&TileMemoryBindInfoQCOM>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_bind_tile_memory_qcom.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_tile_memory_bind_info
            .map(|v| ptr::from_ref(v))
            .unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_copy_memory_indirect")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectKHR.html>"]
#[doc(alias = "vkCmdCopyMemoryIndirectKHR")]
#[inline]
pub unsafe fn cmd_copy_memory_indirect_khr(
    command_buffer: &raw::CommandBuffer,
    p_copy_memory_indirect_info: &CopyMemoryIndirectInfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_memory_indirect_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_copy_memory_indirect_info),
    )
}
#[cfg(feature = "ext_copy_memory_indirect")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectKHR.html>"]
#[doc(alias = "vkCmdCopyMemoryToImageIndirectKHR")]
#[inline]
pub unsafe fn cmd_copy_memory_to_image_indirect_khr(
    command_buffer: &raw::CommandBuffer,
    p_copy_memory_to_image_indirect_info: &CopyMemoryToImageIndirectInfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_copy_memory_to_image_indirect_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_copy_memory_to_image_indirect_info),
    )
}
#[cfg(feature = "ext_memory_decompression")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryEXT.html>"]
#[doc(alias = "vkCmdDecompressMemoryEXT")]
#[inline]
pub unsafe fn cmd_decompress_memory_ext(
    command_buffer: &raw::CommandBuffer,
    p_decompress_memory_info_ext: &DecompressMemoryInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_decompress_memory_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_decompress_memory_info_ext),
    )
}
#[cfg(feature = "ext_memory_decompression")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountEXT.html>"]
#[doc(alias = "vkCmdDecompressMemoryIndirectCountEXT")]
#[inline]
pub unsafe fn cmd_decompress_memory_indirect_count_ext(
    command_buffer: &raw::CommandBuffer,
    decompression_method: MemoryDecompressionMethodFlagsEXT,
    indirect_commands_address: DeviceAddress,
    indirect_commands_count_address: DeviceAddress,
    max_decompression_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_decompress_memory_indirect_count_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        decompression_method,
        indirect_commands_address,
        indirect_commands_count_address,
        max_decompression_count,
        stride,
    )
}
#[cfg(feature = "ext_external_compute_queue")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExternalComputeQueueNV.html>"]
#[doc(alias = "vkCreateExternalComputeQueueNV")]
pub unsafe fn create_external_compute_queue_nv(
    device: &raw::Device,
    p_create_info: &ExternalComputeQueueCreateInfoNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<ExternalComputeQueueNV> {
    let vulkan_command = dispatcher.create_external_compute_queue_nv.get();
    let mut p_external_queue = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_external_queue.as_mut_ptr(),
    );
    vk_status.map_success(|| p_external_queue.assume_init())
}
#[cfg(feature = "ext_external_compute_queue")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyExternalComputeQueueNV.html>"]
#[doc(alias = "vkDestroyExternalComputeQueueNV")]
#[inline]
pub unsafe fn destroy_external_compute_queue_nv(
    device: &raw::Device,
    external_queue: &raw::ExternalComputeQueueNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_external_compute_queue_nv.get();
    vulkan_command(
        Some(device.borrow()),
        Some(external_queue.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_external_compute_queue")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExternalComputeQueueDataNV.html>"]
#[doc(alias = "vkGetExternalComputeQueueDataNV")]
pub unsafe fn get_external_compute_queue_data_nv<
    S: StructureChainOut<ExternalComputeQueueDataParamsNV<'static>>,
>(
    external_queue: &raw::ExternalComputeQueueNV,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher.get_external_compute_queue_data_nv.get();
    let mut params = MaybeUninit::uninit();
    S::setup_uninit(&mut params);
    vulkan_command(
        Some(external_queue.borrow()),
        S::get_uninit_head_ptr(params.as_mut_ptr()),
        p_data,
    );
    S::setup_cleanup(params.as_mut_ptr());
    params.assume_init()
}
#[cfg(feature = "ext_cluster_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetClusterAccelerationStructureBuildSizesNV.html>"]
#[doc(alias = "vkGetClusterAccelerationStructureBuildSizesNV")]
pub unsafe fn get_cluster_acceleration_structure_build_sizes_nv<
    S: StructureChainOut<AccelerationStructureBuildSizesInfoKHR<'static>>,
>(
    device: &raw::Device,
    p_info: &ClusterAccelerationStructureInputInfoNV,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_cluster_acceleration_structure_build_sizes_nv
        .get();
    let mut p_size_info = MaybeUninit::uninit();
    S::setup_uninit(&mut p_size_info);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_size_info.as_mut_ptr()),
    );
    S::setup_cleanup(p_size_info.as_mut_ptr());
    p_size_info.assume_init()
}
#[cfg(feature = "ext_cluster_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildClusterAccelerationStructureIndirectNV.html>"]
#[doc(alias = "vkCmdBuildClusterAccelerationStructureIndirectNV")]
#[inline]
pub unsafe fn cmd_build_cluster_acceleration_structure_indirect_nv(
    command_buffer: &raw::CommandBuffer,
    p_command_infos: &ClusterAccelerationStructureCommandsInfoNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_build_cluster_acceleration_structure_indirect_nv
        .get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_command_infos),
    )
}
#[cfg(feature = "ext_partitioned_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPartitionedAccelerationStructuresBuildSizesNV.html>"]
#[doc(alias = "vkGetPartitionedAccelerationStructuresBuildSizesNV")]
pub unsafe fn get_partitioned_acceleration_structures_build_sizes_nv<
    S: StructureChainOut<AccelerationStructureBuildSizesInfoKHR<'static>>,
>(
    device: &raw::Device,
    p_info: &PartitionedAccelerationStructureInstancesInputNV,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_partitioned_acceleration_structures_build_sizes_nv
        .get();
    let mut p_size_info = MaybeUninit::uninit();
    S::setup_uninit(&mut p_size_info);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_size_info.as_mut_ptr()),
    );
    S::setup_cleanup(p_size_info.as_mut_ptr());
    p_size_info.assume_init()
}
#[cfg(feature = "ext_partitioned_acceleration_structure")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildPartitionedAccelerationStructuresNV.html>"]
#[doc(alias = "vkCmdBuildPartitionedAccelerationStructuresNV")]
#[inline]
pub unsafe fn cmd_build_partitioned_acceleration_structures_nv(
    command_buffer: &raw::CommandBuffer,
    p_build_info: &BuildPartitionedAccelerationStructureInfoNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_build_partitioned_acceleration_structures_nv
        .get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_build_info))
}
#[cfg(feature = "ext_device_generated_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsEXT.html>"]
#[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsEXT")]
pub unsafe fn get_generated_commands_memory_requirements_ext<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &GeneratedCommandsMemoryRequirementsInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_generated_commands_memory_requirements_ext
        .get();
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[cfg(feature = "ext_device_generated_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsEXT.html>"]
#[doc(alias = "vkCmdPreprocessGeneratedCommandsEXT")]
#[inline]
pub unsafe fn cmd_preprocess_generated_commands_ext(
    command_buffer: &raw::CommandBuffer,
    p_generated_commands_info: &GeneratedCommandsInfoEXT,
    state_command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_preprocess_generated_commands_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        ptr::from_ref(p_generated_commands_info),
        Some(state_command_buffer.borrow()),
    )
}
#[cfg(feature = "ext_device_generated_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsEXT.html>"]
#[doc(alias = "vkCmdExecuteGeneratedCommandsEXT")]
#[inline]
pub unsafe fn cmd_execute_generated_commands_ext(
    command_buffer: &raw::CommandBuffer,
    is_preprocessed: impl Into<Bool32>,
    p_generated_commands_info: &GeneratedCommandsInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_execute_generated_commands_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        is_preprocessed.into(),
        ptr::from_ref(p_generated_commands_info),
    )
}
#[cfg(feature = "ext_device_generated_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutEXT.html>"]
#[doc(alias = "vkCreateIndirectCommandsLayoutEXT")]
pub unsafe fn create_indirect_commands_layout_ext(
    device: &raw::Device,
    p_create_info: &IndirectCommandsLayoutCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<IndirectCommandsLayoutEXT> {
    let vulkan_command = dispatcher.create_indirect_commands_layout_ext.get();
    let mut p_indirect_commands_layout = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_indirect_commands_layout.as_mut_ptr(),
    );
    vk_status.map_success(|| p_indirect_commands_layout.assume_init())
}
#[cfg(feature = "ext_device_generated_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutEXT.html>"]
#[doc(alias = "vkDestroyIndirectCommandsLayoutEXT")]
#[inline]
pub unsafe fn destroy_indirect_commands_layout_ext(
    device: &raw::Device,
    indirect_commands_layout: Option<&raw::IndirectCommandsLayoutEXT>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_indirect_commands_layout_ext.get();
    vulkan_command(
        Some(device.borrow()),
        indirect_commands_layout.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_device_generated_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectExecutionSetEXT.html>"]
#[doc(alias = "vkCreateIndirectExecutionSetEXT")]
pub unsafe fn create_indirect_execution_set_ext(
    device: &raw::Device,
    p_create_info: &IndirectExecutionSetCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<IndirectExecutionSetEXT> {
    let vulkan_command = dispatcher.create_indirect_execution_set_ext.get();
    let mut p_indirect_execution_set = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_indirect_execution_set.as_mut_ptr(),
    );
    vk_status.map_success(|| p_indirect_execution_set.assume_init())
}
#[cfg(feature = "ext_device_generated_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectExecutionSetEXT.html>"]
#[doc(alias = "vkDestroyIndirectExecutionSetEXT")]
#[inline]
pub unsafe fn destroy_indirect_execution_set_ext(
    device: &raw::Device,
    indirect_execution_set: Option<&raw::IndirectExecutionSetEXT>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_indirect_execution_set_ext.get();
    vulkan_command(
        Some(device.borrow()),
        indirect_execution_set.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_device_generated_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetPipelineEXT.html>"]
#[doc(alias = "vkUpdateIndirectExecutionSetPipelineEXT")]
#[inline]
pub unsafe fn update_indirect_execution_set_pipeline_ext<'a>(
    device: &raw::Device,
    indirect_execution_set: &raw::IndirectExecutionSetEXT,
    p_execution_set_writes: impl AsSlice<'a, WriteIndirectExecutionSetPipelineEXT<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.update_indirect_execution_set_pipeline_ext.get();
    vulkan_command(
        Some(device.borrow()),
        Some(indirect_execution_set.borrow()),
        p_execution_set_writes.as_slice().len() as _,
        p_execution_set_writes.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_device_generated_commands")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetShaderEXT.html>"]
#[doc(alias = "vkUpdateIndirectExecutionSetShaderEXT")]
#[inline]
pub unsafe fn update_indirect_execution_set_shader_ext<'a>(
    device: &raw::Device,
    indirect_execution_set: &raw::IndirectExecutionSetEXT,
    p_execution_set_writes: impl AsSlice<'a, WriteIndirectExecutionSetShaderEXT<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.update_indirect_execution_set_shader_ext.get();
    vulkan_command(
        Some(device.borrow()),
        Some(indirect_execution_set.borrow()),
        p_execution_set_writes.as_slice().len() as _,
        p_execution_set_writes.as_slice().as_ptr().cast(),
    )
}
#[cfg(feature = "ext_device_fault")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultReportsKHR.html>"]
#[doc(alias = "vkGetDeviceFaultReportsKHR")]
pub unsafe fn get_device_fault_reports_khr<R: DynamicArray<DeviceFaultInfoKHR<'static>>>(
    device: &raw::Device,
    timeout: u64,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, R)> {
    let vulkan_command = dispatcher.get_device_fault_reports_khr.get();
    let mut vk_len = MaybeUninit::uninit();
    let p_fault_counts = vk_len.as_mut_ptr();
    let p_fault_info = ptr::null_mut();
    vulkan_command(Some(device.borrow()), timeout, p_fault_counts, p_fault_info)
        .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_fault_counts = ptr::from_mut(&mut vk_len);
    let mut p_fault_info = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(Some(device.borrow()), timeout, p_fault_counts, p_fault_info);
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_fault_counts = ptr::from_mut(&mut vk_len);
        p_fault_info = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_successes(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_device_fault")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultDebugInfoKHR.html>"]
#[doc(alias = "vkGetDeviceFaultDebugInfoKHR")]
pub unsafe fn get_device_fault_debug_info_khr<
    S: StructureChainOut<DeviceFaultDebugInfoKHR<'static>>,
>(
    device: &raw::Device,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.get_device_fault_debug_info_khr.get();
    let mut p_debug_info = MaybeUninit::uninit();
    S::setup_uninit(&mut p_debug_info);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        S::get_uninit_head_ptr(p_debug_info.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_debug_info.as_mut_ptr());
        p_debug_info.assume_init()
    })
}
#[cfg(feature = "ext_ohos_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSurfaceOHOS.html>"]
#[doc(alias = "vkCreateSurfaceOHOS")]
pub unsafe fn create_surface_ohos(
    instance: &raw::Instance,
    p_create_info: &SurfaceCreateInfoOHOS,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_surface_ohos.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_cooperative_matrix2")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV.html>"]
#[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV")]
pub unsafe fn get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv<
    R: DynamicArray<CooperativeMatrixFlexibleDimensionsPropertiesNV<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_property_count,
        p_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_property_count,
            p_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_property_count = ptr::from_mut(&mut vk_len);
        p_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_external_memory_metal")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandleEXT.html>"]
#[doc(alias = "vkGetMemoryMetalHandleEXT")]
pub unsafe fn get_memory_metal_handle_ext(
    device: &raw::Device,
    p_get_metal_handle_info: &MemoryGetMetalHandleInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher.get_memory_metal_handle_ext.get();
    let mut p_handle = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_get_metal_handle_info),
        p_handle.as_mut_ptr(),
    );
    vk_status.map_success(|| p_handle.assume_init())
}
#[cfg(feature = "ext_external_memory_metal")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandlePropertiesEXT.html>"]
#[doc(alias = "vkGetMemoryMetalHandlePropertiesEXT")]
pub unsafe fn get_memory_metal_handle_properties_ext<
    S: StructureChainOut<MemoryMetalHandlePropertiesEXT<'static>>,
>(
    device: &raw::Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    p_handle: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher.get_memory_metal_handle_properties_ext.get();
    let mut p_memory_metal_handle_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_metal_handle_properties);
    let vk_status = vulkan_command(
        Some(device.borrow()),
        handle_type,
        p_handle,
        S::get_uninit_head_ptr(p_memory_metal_handle_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_memory_metal_handle_properties.as_mut_ptr());
        p_memory_metal_handle_properties.assume_init()
    })
}
#[cfg(feature = "ext_shader_instrumentation")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM.html>"]
#[doc(alias = "vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM")]
pub unsafe fn enumerate_physical_device_shader_instrumentation_metrics_arm<
    R: DynamicArray<ShaderInstrumentationMetricDescriptionARM<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .enumerate_physical_device_shader_instrumentation_metrics_arm
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_description_count = vk_len.as_mut_ptr();
    let p_descriptions = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        p_description_count,
        p_descriptions,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_description_count = ptr::from_mut(&mut vk_len);
    let mut p_descriptions = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            p_description_count,
            p_descriptions,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_description_count = ptr::from_mut(&mut vk_len);
        p_descriptions = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_shader_instrumentation")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderInstrumentationARM.html>"]
#[doc(alias = "vkCreateShaderInstrumentationARM")]
pub unsafe fn create_shader_instrumentation_arm(
    device: &raw::Device,
    p_create_info: &ShaderInstrumentationCreateInfoARM,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<ShaderInstrumentationARM> {
    let vulkan_command = dispatcher.create_shader_instrumentation_arm.get();
    let mut p_instrumentation = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_instrumentation.as_mut_ptr(),
    );
    vk_status.map_success(|| p_instrumentation.assume_init())
}
#[cfg(feature = "ext_shader_instrumentation")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderInstrumentationARM.html>"]
#[doc(alias = "vkDestroyShaderInstrumentationARM")]
#[inline]
pub unsafe fn destroy_shader_instrumentation_arm(
    device: &raw::Device,
    instrumentation: Option<&raw::ShaderInstrumentationARM>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.destroy_shader_instrumentation_arm.get();
    vulkan_command(
        Some(device.borrow()),
        instrumentation.map(|v| v.borrow()),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_shader_instrumentation")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginShaderInstrumentationARM.html>"]
#[doc(alias = "vkCmdBeginShaderInstrumentationARM")]
#[inline]
pub unsafe fn cmd_begin_shader_instrumentation_arm(
    command_buffer: &raw::CommandBuffer,
    instrumentation: &raw::ShaderInstrumentationARM,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_begin_shader_instrumentation_arm.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        Some(instrumentation.borrow()),
    )
}
#[cfg(feature = "ext_shader_instrumentation")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndShaderInstrumentationARM.html>"]
#[doc(alias = "vkCmdEndShaderInstrumentationARM")]
#[inline]
pub unsafe fn cmd_end_shader_instrumentation_arm(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_end_shader_instrumentation_arm.get();
    vulkan_command(Some(command_buffer.borrow()))
}
#[cfg(feature = "ext_shader_instrumentation")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInstrumentationValuesARM.html>"]
#[doc(alias = "vkGetShaderInstrumentationValuesARM")]
pub unsafe fn get_shader_instrumentation_values_arm(
    device: &raw::Device,
    instrumentation: &raw::ShaderInstrumentationARM,
    p_metric_values: VoidPtr,
    flags: u32,
    dispatcher: &CommandsDispatcher,
) -> Result<u32> {
    let vulkan_command = dispatcher.get_shader_instrumentation_values_arm.get();
    let mut p_metric_block_count = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(device.borrow()),
        Some(instrumentation.borrow()),
        p_metric_block_count.as_mut_ptr(),
        p_metric_values,
        flags,
    );
    vk_status.map_success(|| p_metric_block_count.assume_init())
}
#[cfg(feature = "ext_shader_instrumentation")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkClearShaderInstrumentationMetricsARM.html>"]
#[doc(alias = "vkClearShaderInstrumentationMetricsARM")]
#[inline]
pub unsafe fn clear_shader_instrumentation_metrics_arm(
    device: &raw::Device,
    instrumentation: &raw::ShaderInstrumentationARM,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.clear_shader_instrumentation_metrics_arm.get();
    vulkan_command(Some(device.borrow()), Some(instrumentation.borrow()))
}
#[cfg(all(
    feature = "ext_custom_resolve",
    any(feature = "ext_dynamic_rendering", feature = "version_1_3")
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginCustomResolveEXT.html>"]
#[doc(alias = "vkCmdBeginCustomResolveEXT")]
#[inline]
pub unsafe fn cmd_begin_custom_resolve_ext(
    command_buffer: &raw::CommandBuffer,
    p_begin_custom_resolve_info: Option<&BeginCustomResolveInfoEXT>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_begin_custom_resolve_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_begin_custom_resolve_info
            .map(|v| ptr::from_ref(v))
            .unwrap_or(ptr::null()),
    )
}
#[cfg(any(
    feature = "ext_fragment_density_map_offset",
    feature = "ext_maintenance10"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2KHR.html>"]
#[doc(alias = "vkCmdEndRendering2KHR")]
#[inline]
pub unsafe fn cmd_end_rendering2_khr(
    command_buffer: &raw::CommandBuffer,
    p_rendering_end_info: Option<&RenderingEndInfoKHR>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_end_rendering2_khr.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_rendering_end_info
            .map(|v| ptr::from_ref(v))
            .unwrap_or(ptr::null()),
    )
}
#[cfg(any(
    feature = "ext_fragment_density_map_offset",
    feature = "ext_maintenance10"
))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2EXT.html>"]
#[doc(alias = "vkCmdEndRendering2EXT")]
#[inline]
pub unsafe fn cmd_end_rendering2_ext(
    command_buffer: &raw::CommandBuffer,
    p_rendering_end_info: Option<&RenderingEndInfoKHR>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_end_rendering2_ext.get();
    vulkan_command(
        Some(command_buffer.borrow()),
        p_rendering_end_info
            .map(|v| ptr::from_ref(v))
            .unwrap_or(ptr::null()),
    )
}
#[cfg(feature = "ext_data_graph_optical_flow")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM.html>"]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM")]
pub unsafe fn get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm<
    R: DynamicArray<DataGraphOpticalFlowImageFormatPropertiesARM<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    p_queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM,
    p_optical_flow_image_format_info: &DataGraphOpticalFlowImageFormatInfoARM,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm
        .get();
    let mut vk_len = MaybeUninit::uninit();
    let p_format_count = vk_len.as_mut_ptr();
    let p_image_format_properties = ptr::null_mut();
    vulkan_command(
        Some(physical_device.borrow()),
        queue_family_index,
        ptr::from_ref(p_queue_family_data_graph_properties),
        ptr::from_ref(p_optical_flow_image_format_info),
        p_format_count,
        p_image_format_properties,
    )
    .map_success(|| ())?;
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_format_count = ptr::from_mut(&mut vk_len);
    let mut p_image_format_properties = vk_vec.get_content_mut_ptr();
    let vk_status = loop {
        let status = vulkan_command(
            Some(physical_device.borrow()),
            queue_family_index,
            ptr::from_ref(p_queue_family_data_graph_properties),
            ptr::from_ref(p_optical_flow_image_format_info),
            p_format_count,
            p_image_format_properties,
        );
        if status != Status::Incomplete {
            break status;
        }
        vk_vec.update_with_capacity(vk_len as _);
        p_format_count = ptr::from_mut(&mut vk_len);
        p_image_format_properties = vk_vec.get_content_mut_ptr();
    };
    vk_status.map_success(|| {
        vk_vec.resize_with_len(vk_len as _);
        vk_vec
    })
}
#[cfg(feature = "ext_compute_occupancy_priority")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetComputeOccupancyPriorityNV.html>"]
#[doc(alias = "vkCmdSetComputeOccupancyPriorityNV")]
#[inline]
pub unsafe fn cmd_set_compute_occupancy_priority_nv(
    command_buffer: &raw::CommandBuffer,
    p_parameters: &ComputeOccupancyPriorityParametersNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_compute_occupancy_priority_nv.get();
    vulkan_command(Some(command_buffer.borrow()), ptr::from_ref(p_parameters))
}
#[cfg(feature = "ext_ubm_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateUbmSurfaceSEC.html>"]
#[doc(alias = "vkCreateUbmSurfaceSEC")]
pub unsafe fn create_ubm_surface_sec(
    instance: &raw::Instance,
    p_create_info: &UbmSurfaceCreateInfoSEC,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher.create_ubm_surface_sec.get();
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(instance.borrow()),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[cfg(feature = "ext_ubm_surface")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceUbmPresentationSupportSEC.html>"]
#[doc(alias = "vkGetPhysicalDeviceUbmPresentationSupportSEC")]
#[inline]
pub unsafe fn get_physical_device_ubm_presentation_support_sec(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    device: &VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> bool {
    let vulkan_command = dispatcher
        .get_physical_device_ubm_presentation_support_sec
        .get();
    vulkan_command(
        Some(physical_device.borrow()),
        queue_family_index,
        ptr::from_ref(device),
    )
    .into()
}
#[cfg(feature = "ext_primitive_restart_index")]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartIndexEXT.html>"]
#[doc(alias = "vkCmdSetPrimitiveRestartIndexEXT")]
#[inline]
pub unsafe fn cmd_set_primitive_restart_index_ext(
    command_buffer: &raw::CommandBuffer,
    primitive_restart_index: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher.cmd_set_primitive_restart_index_ext.get();
    vulkan_command(Some(command_buffer.borrow()), primitive_restart_index)
}
