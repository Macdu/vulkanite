#![allow(unused_unsafe)]
#![allow(unused_mut)]
use crate::vk::raw::{self, *};
use crate::vk::*;
use crate::*;
use std::ffi::{c_int, CStr};
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html>"]
#[doc(alias = "vkCreateInstance")]
pub unsafe fn create_instance(
    p_create_info: &InstanceCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Instance> {
    let vulkan_command = dispatcher
        .create_instance
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_instance = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_instance.as_mut_ptr(),
    );
    vk_status.map_success(|| p_instance.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html>"]
#[doc(alias = "vkDestroyInstance")]
pub unsafe fn destroy_instance(
    instance: Option<&raw::Instance>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_instance
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        instance.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html>"]
#[doc(alias = "vkEnumeratePhysicalDevices")]
pub unsafe fn enumerate_physical_devices<R: DynamicArray<PhysicalDevice>>(
    instance: &raw::Instance,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .enumerate_physical_devices
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_physical_device_count = vk_len.as_mut_ptr();
    let p_physical_devices = ptr::null_mut();
    vulkan_command(
        Some(unsafe { instance.clone() }),
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
            Some(unsafe { instance.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures.html>"]
#[doc(alias = "vkGetPhysicalDeviceFeatures")]
pub unsafe fn get_physical_device_features(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> PhysicalDeviceFeatures {
    let vulkan_command = dispatcher
        .get_physical_device_features
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_features = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        p_features.as_mut_ptr(),
    );
    p_features.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html>"]
#[doc(alias = "vkGetPhysicalDeviceFormatProperties")]
pub unsafe fn get_physical_device_format_properties(
    physical_device: &raw::PhysicalDevice,
    format: Format,
    dispatcher: &CommandsDispatcher,
) -> FormatProperties {
    let vulkan_command = dispatcher
        .get_physical_device_format_properties
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_format_properties = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        format,
        p_format_properties.as_mut_ptr(),
    );
    p_format_properties.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html>"]
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
    let vulkan_command = dispatcher
        .get_physical_device_image_format_properties
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_image_format_properties = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { physical_device.clone() }),
        format,
        ty,
        tiling,
        usage,
        flags,
        p_image_format_properties.as_mut_ptr(),
    );
    vk_status.map_success(|| p_image_format_properties.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties.html>"]
#[doc(alias = "vkGetPhysicalDeviceProperties")]
pub unsafe fn get_physical_device_properties(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> PhysicalDeviceProperties {
    let vulkan_command = dispatcher
        .get_physical_device_properties
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_properties = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        p_properties.as_mut_ptr(),
    );
    p_properties.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html>"]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties")]
pub unsafe fn get_physical_device_queue_family_properties<
    R: DynamicArray<QueueFamilyProperties>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher
        .get_physical_device_queue_family_properties
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_queue_family_property_count = vk_len.as_mut_ptr();
    let p_queue_family_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        p_queue_family_property_count,
        p_queue_family_properties,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_queue_family_property_count = ptr::from_mut(&mut vk_len);
    let mut p_queue_family_properties = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        p_queue_family_property_count,
        p_queue_family_properties,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html>"]
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties")]
pub unsafe fn get_physical_device_memory_properties(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> PhysicalDeviceMemoryProperties {
    let vulkan_command = dispatcher
        .get_physical_device_memory_properties
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_properties = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        p_memory_properties.as_mut_ptr(),
    );
    p_memory_properties.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html>"]
#[doc(alias = "vkGetInstanceProcAddr")]
pub unsafe fn get_instance_proc_addr(
    instance: Option<&raw::Instance>,
    p_name: &CStr,
    dispatcher: &CommandsDispatcher,
) -> FuncPtr {
    let vulkan_command = dispatcher
        .get_instance_proc_addr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(instance.map(|v| unsafe { v.clone() }), p_name.as_ptr())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceProcAddr.html>"]
#[doc(alias = "vkGetDeviceProcAddr")]
pub unsafe fn get_device_proc_addr(
    device: &raw::Device,
    p_name: &CStr,
    dispatcher: &CommandsDispatcher,
) -> FuncPtr {
    let vulkan_command = dispatcher
        .get_device_proc_addr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), p_name.as_ptr())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html>"]
#[doc(alias = "vkCreateDevice")]
pub unsafe fn create_device(
    physical_device: &raw::PhysicalDevice,
    p_create_info: &DeviceCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Device> {
    let vulkan_command = dispatcher
        .create_device
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_device = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_device.as_mut_ptr(),
    );
    vk_status.map_success(|| p_device.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDevice.html>"]
#[doc(alias = "vkDestroyDevice")]
pub unsafe fn destroy_device(
    device: Option<&raw::Device>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_device
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        device.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html>"]
#[doc(alias = "vkEnumerateInstanceExtensionProperties")]
pub unsafe fn enumerate_instance_extension_properties<R: DynamicArray<ExtensionProperties>>(
    p_layer_name: Option<&CStr>,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .enumerate_instance_extension_properties
        .get()
        .expect("Vulkan command not loaded.");
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceExtensionProperties.html>"]
#[doc(alias = "vkEnumerateDeviceExtensionProperties")]
pub unsafe fn enumerate_device_extension_properties<R: DynamicArray<ExtensionProperties>>(
    physical_device: &raw::PhysicalDevice,
    p_layer_name: Option<&CStr>,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .enumerate_device_extension_properties
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html>"]
#[doc(alias = "vkEnumerateInstanceLayerProperties")]
pub unsafe fn enumerate_instance_layer_properties<R: DynamicArray<LayerProperties>>(
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .enumerate_instance_layer_properties
        .get()
        .expect("Vulkan command not loaded.");
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateDeviceLayerProperties.html>"]
#[doc(alias = "vkEnumerateDeviceLayerProperties")]
pub unsafe fn enumerate_device_layer_properties<R: DynamicArray<LayerProperties>>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .enumerate_device_layer_properties
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue.html>"]
#[doc(alias = "vkGetDeviceQueue")]
pub unsafe fn get_device_queue(
    device: &raw::Device,
    queue_family_index: u32,
    queue_index: u32,
    dispatcher: &CommandsDispatcher,
) -> Queue {
    let vulkan_command = dispatcher
        .get_device_queue
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_queue = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        queue_family_index,
        queue_index,
        p_queue.as_mut_ptr(),
    );
    p_queue.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit.html>"]
#[doc(alias = "vkQueueSubmit")]
pub unsafe fn queue_submit<'a>(
    queue: &raw::Queue,
    p_submits: impl AsSlice<'a, SubmitInfo<'a>>,
    fence: Option<&raw::Fence>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .queue_submit
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { queue.clone() }),
        p_submits.as_slice().len() as _,
        p_submits.as_slice().as_ptr().cast(),
        fence.map(|v| unsafe { v.clone() }),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueWaitIdle.html>"]
#[doc(alias = "vkQueueWaitIdle")]
pub unsafe fn queue_wait_idle(queue: &raw::Queue, dispatcher: &CommandsDispatcher) -> Result<()> {
    let vulkan_command = dispatcher
        .queue_wait_idle
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { queue.clone() })).map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeviceWaitIdle.html>"]
#[doc(alias = "vkDeviceWaitIdle")]
pub unsafe fn device_wait_idle(
    device: &raw::Device,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .device_wait_idle
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() })).map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateMemory.html>"]
#[doc(alias = "vkAllocateMemory")]
pub unsafe fn allocate_memory(
    device: &raw::Device,
    p_allocate_info: &MemoryAllocateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DeviceMemory> {
    let vulkan_command = dispatcher
        .allocate_memory
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_allocate_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_memory.as_mut_ptr(),
    );
    vk_status.map_success(|| p_memory.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeMemory.html>"]
#[doc(alias = "vkFreeMemory")]
pub unsafe fn free_memory(
    device: &raw::Device,
    memory: Option<&raw::DeviceMemory>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .free_memory
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        memory.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMapMemory.html>"]
#[doc(alias = "vkMapMemory")]
pub unsafe fn map_memory(
    device: &raw::Device,
    memory: &raw::DeviceMemory,
    offset: DeviceSize,
    size: DeviceSize,
    flags: MemoryMapFlags,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher
        .map_memory
        .get()
        .expect("Vulkan command not loaded.");
    let mut pp_data = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { memory.clone() }),
        offset,
        size,
        flags,
        pp_data.as_mut_ptr(),
    );
    vk_status.map_success(|| pp_data.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory.html>"]
#[doc(alias = "vkUnmapMemory")]
pub unsafe fn unmap_memory(
    device: &raw::Device,
    memory: &raw::DeviceMemory,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .unmap_memory
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { memory.clone() }),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFlushMappedMemoryRanges.html>"]
#[doc(alias = "vkFlushMappedMemoryRanges")]
pub unsafe fn flush_mapped_memory_ranges<'a>(
    device: &raw::Device,
    p_memory_ranges: impl AsSlice<'a, MappedMemoryRange<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .flush_mapped_memory_ranges
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        p_memory_ranges.as_slice().len() as _,
        p_memory_ranges.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkInvalidateMappedMemoryRanges.html>"]
#[doc(alias = "vkInvalidateMappedMemoryRanges")]
pub unsafe fn invalidate_mapped_memory_ranges<'a>(
    device: &raw::Device,
    p_memory_ranges: impl AsSlice<'a, MappedMemoryRange<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .invalidate_mapped_memory_ranges
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        p_memory_ranges.as_slice().len() as _,
        p_memory_ranges.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryCommitment.html>"]
#[doc(alias = "vkGetDeviceMemoryCommitment")]
pub unsafe fn get_device_memory_commitment(
    device: &raw::Device,
    memory: &raw::DeviceMemory,
    dispatcher: &CommandsDispatcher,
) -> DeviceSize {
    let vulkan_command = dispatcher
        .get_device_memory_commitment
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_committed_memory_in_bytes = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { memory.clone() }),
        p_committed_memory_in_bytes.as_mut_ptr(),
    );
    p_committed_memory_in_bytes.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory.html>"]
#[doc(alias = "vkBindBufferMemory")]
pub unsafe fn bind_buffer_memory(
    device: &raw::Device,
    buffer: &raw::Buffer,
    memory: &raw::DeviceMemory,
    memory_offset: DeviceSize,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .bind_buffer_memory
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { buffer.clone() }),
        Some(unsafe { memory.clone() }),
        memory_offset,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory.html>"]
#[doc(alias = "vkBindImageMemory")]
pub unsafe fn bind_image_memory(
    device: &raw::Device,
    image: &raw::Image,
    memory: &raw::DeviceMemory,
    memory_offset: DeviceSize,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .bind_image_memory
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { image.clone() }),
        Some(unsafe { memory.clone() }),
        memory_offset,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements.html>"]
#[doc(alias = "vkGetBufferMemoryRequirements")]
pub unsafe fn get_buffer_memory_requirements(
    device: &raw::Device,
    buffer: &raw::Buffer,
    dispatcher: &CommandsDispatcher,
) -> MemoryRequirements {
    let vulkan_command = dispatcher
        .get_buffer_memory_requirements
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_requirements = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { buffer.clone() }),
        p_memory_requirements.as_mut_ptr(),
    );
    p_memory_requirements.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements.html>"]
#[doc(alias = "vkGetImageMemoryRequirements")]
pub unsafe fn get_image_memory_requirements(
    device: &raw::Device,
    image: &raw::Image,
    dispatcher: &CommandsDispatcher,
) -> MemoryRequirements {
    let vulkan_command = dispatcher
        .get_image_memory_requirements
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_requirements = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { image.clone() }),
        p_memory_requirements.as_mut_ptr(),
    );
    p_memory_requirements.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements.html>"]
#[doc(alias = "vkGetImageSparseMemoryRequirements")]
pub unsafe fn get_image_sparse_memory_requirements<
    R: DynamicArray<SparseImageMemoryRequirements>,
>(
    device: &raw::Device,
    image: &raw::Image,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher
        .get_image_sparse_memory_requirements
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_sparse_memory_requirement_count = vk_len.as_mut_ptr();
    let p_sparse_memory_requirements = ptr::null_mut();
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { image.clone() }),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_sparse_memory_requirement_count = ptr::from_mut(&mut vk_len);
    let mut p_sparse_memory_requirements = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { image.clone() }),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
        Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueBindSparse.html>"]
#[doc(alias = "vkQueueBindSparse")]
pub unsafe fn queue_bind_sparse<'a>(
    queue: &raw::Queue,
    p_bind_info: impl AsSlice<'a, BindSparseInfo<'a>>,
    fence: Option<&raw::Fence>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .queue_bind_sparse
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { queue.clone() }),
        p_bind_info.as_slice().len() as _,
        p_bind_info.as_slice().as_ptr().cast(),
        fence.map(|v| unsafe { v.clone() }),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateFence.html>"]
#[doc(alias = "vkCreateFence")]
pub unsafe fn create_fence(
    device: &raw::Device,
    p_create_info: &FenceCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Fence> {
    let vulkan_command = dispatcher
        .create_fence
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_fence = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_fence.as_mut_ptr(),
    );
    vk_status.map_success(|| p_fence.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyFence.html>"]
#[doc(alias = "vkDestroyFence")]
pub unsafe fn destroy_fence(
    device: &raw::Device,
    fence: Option<&raw::Fence>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_fence
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        fence.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetFences.html>"]
#[doc(alias = "vkResetFences")]
pub unsafe fn reset_fences<'a, V2: Alias<raw::Fence> + 'a>(
    device: &raw::Device,
    p_fences: impl AsSlice<'a, V2>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .reset_fences
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        p_fences.as_slice().len() as _,
        p_fences.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceStatus.html>"]
#[doc(alias = "vkGetFenceStatus")]
pub unsafe fn get_fence_status(
    device: &raw::Device,
    fence: &raw::Fence,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .get_fence_status
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { fence.clone() }),
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitForFences.html>"]
#[doc(alias = "vkWaitForFences")]
pub unsafe fn wait_for_fences<'a, V2: Alias<raw::Fence> + 'a>(
    device: &raw::Device,
    p_fences: impl AsSlice<'a, V2>,
    wait_all: impl Into<Bool32>,
    timeout: u64,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .wait_for_fences
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        p_fences.as_slice().len() as _,
        p_fences.as_slice().as_ptr().cast(),
        wait_all.into(),
        timeout,
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSemaphore.html>"]
#[doc(alias = "vkCreateSemaphore")]
pub unsafe fn create_semaphore(
    device: &raw::Device,
    p_create_info: &SemaphoreCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Semaphore> {
    let vulkan_command = dispatcher
        .create_semaphore
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_semaphore = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_semaphore.as_mut_ptr(),
    );
    vk_status.map_success(|| p_semaphore.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySemaphore.html>"]
#[doc(alias = "vkDestroySemaphore")]
pub unsafe fn destroy_semaphore(
    device: &raw::Device,
    semaphore: Option<&raw::Semaphore>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_semaphore
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        semaphore.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateEvent.html>"]
#[doc(alias = "vkCreateEvent")]
pub unsafe fn create_event(
    device: &raw::Device,
    p_create_info: &EventCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Event> {
    let vulkan_command = dispatcher
        .create_event
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_event = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_event.as_mut_ptr(),
    );
    vk_status.map_success(|| p_event.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyEvent.html>"]
#[doc(alias = "vkDestroyEvent")]
pub unsafe fn destroy_event(
    device: &raw::Device,
    event: Option<&raw::Event>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_event
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        event.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetEventStatus.html>"]
#[doc(alias = "vkGetEventStatus")]
pub unsafe fn get_event_status(
    device: &raw::Device,
    event: &raw::Event,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .get_event_status
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { event.clone() }),
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetEvent.html>"]
#[doc(alias = "vkSetEvent")]
pub unsafe fn set_event(
    device: &raw::Device,
    event: &raw::Event,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .set_event
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { event.clone() }),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetEvent.html>"]
#[doc(alias = "vkResetEvent")]
pub unsafe fn reset_event(
    device: &raw::Device,
    event: &raw::Event,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .reset_event
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { event.clone() }),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateQueryPool.html>"]
#[doc(alias = "vkCreateQueryPool")]
pub unsafe fn create_query_pool(
    device: &raw::Device,
    p_create_info: &QueryPoolCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<QueryPool> {
    let vulkan_command = dispatcher
        .create_query_pool
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_query_pool = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_query_pool.as_mut_ptr(),
    );
    vk_status.map_success(|| p_query_pool.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyQueryPool.html>"]
#[doc(alias = "vkDestroyQueryPool")]
pub unsafe fn destroy_query_pool(
    device: &raw::Device,
    query_pool: Option<&raw::QueryPool>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_query_pool
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        query_pool.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueryPoolResults.html>"]
#[doc(alias = "vkGetQueryPoolResults")]
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
    let vulkan_command = dispatcher
        .get_query_pool_results
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { query_pool.clone() }),
        first_query,
        query_count,
        data_size,
        p_data,
        stride,
        flags,
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBuffer.html>"]
#[doc(alias = "vkCreateBuffer")]
pub unsafe fn create_buffer(
    device: &raw::Device,
    p_create_info: &BufferCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Buffer> {
    let vulkan_command = dispatcher
        .create_buffer
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_buffer = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_buffer.as_mut_ptr(),
    );
    vk_status.map_success(|| p_buffer.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBuffer.html>"]
#[doc(alias = "vkDestroyBuffer")]
pub unsafe fn destroy_buffer(
    device: &raw::Device,
    buffer: Option<&raw::Buffer>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_buffer
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        buffer.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBufferView.html>"]
#[doc(alias = "vkCreateBufferView")]
pub unsafe fn create_buffer_view(
    device: &raw::Device,
    p_create_info: &BufferViewCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<BufferView> {
    let vulkan_command = dispatcher
        .create_buffer_view
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_view = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_view.as_mut_ptr(),
    );
    vk_status.map_success(|| p_view.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferView.html>"]
#[doc(alias = "vkDestroyBufferView")]
pub unsafe fn destroy_buffer_view(
    device: &raw::Device,
    buffer_view: Option<&raw::BufferView>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_buffer_view
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        buffer_view.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImage.html>"]
#[doc(alias = "vkCreateImage")]
pub unsafe fn create_image(
    device: &raw::Device,
    p_create_info: &ImageCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Image> {
    let vulkan_command = dispatcher
        .create_image
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_image = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_image.as_mut_ptr(),
    );
    vk_status.map_success(|| p_image.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImage.html>"]
#[doc(alias = "vkDestroyImage")]
pub unsafe fn destroy_image(
    device: &raw::Device,
    image: Option<&raw::Image>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_image
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        image.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout.html>"]
#[doc(alias = "vkGetImageSubresourceLayout")]
pub unsafe fn get_image_subresource_layout(
    device: &raw::Device,
    image: &raw::Image,
    p_subresource: &ImageSubresource,
    dispatcher: &CommandsDispatcher,
) -> SubresourceLayout {
    let vulkan_command = dispatcher
        .get_image_subresource_layout
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_layout = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { image.clone() }),
        ptr::from_ref(p_subresource),
        p_layout.as_mut_ptr(),
    );
    p_layout.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImageView.html>"]
#[doc(alias = "vkCreateImageView")]
pub unsafe fn create_image_view(
    device: &raw::Device,
    p_create_info: &ImageViewCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<ImageView> {
    let vulkan_command = dispatcher
        .create_image_view
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_view = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_view.as_mut_ptr(),
    );
    vk_status.map_success(|| p_view.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImageView.html>"]
#[doc(alias = "vkDestroyImageView")]
pub unsafe fn destroy_image_view(
    device: &raw::Device,
    image_view: Option<&raw::ImageView>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_image_view
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        image_view.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateShaderModule.html>"]
#[doc(alias = "vkCreateShaderModule")]
pub unsafe fn create_shader_module(
    device: &raw::Device,
    p_create_info: &ShaderModuleCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<ShaderModule> {
    let vulkan_command = dispatcher
        .create_shader_module
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_shader_module = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_shader_module.as_mut_ptr(),
    );
    vk_status.map_success(|| p_shader_module.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderModule.html>"]
#[doc(alias = "vkDestroyShaderModule")]
pub unsafe fn destroy_shader_module(
    device: &raw::Device,
    shader_module: Option<&raw::ShaderModule>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_shader_module
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        shader_module.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineCache.html>"]
#[doc(alias = "vkCreatePipelineCache")]
pub unsafe fn create_pipeline_cache(
    device: &raw::Device,
    p_create_info: &PipelineCacheCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<PipelineCache> {
    let vulkan_command = dispatcher
        .create_pipeline_cache
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_pipeline_cache = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_pipeline_cache.as_mut_ptr(),
    );
    vk_status.map_success(|| p_pipeline_cache.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineCache.html>"]
#[doc(alias = "vkDestroyPipelineCache")]
pub unsafe fn destroy_pipeline_cache(
    device: &raw::Device,
    pipeline_cache: Option<&raw::PipelineCache>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_pipeline_cache
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        pipeline_cache.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineCacheData.html>"]
#[doc(alias = "vkGetPipelineCacheData")]
pub unsafe fn get_pipeline_cache_data(
    device: &raw::Device,
    pipeline_cache: &raw::PipelineCache,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<usize> {
    let vulkan_command = dispatcher
        .get_pipeline_cache_data
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_data_size = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { pipeline_cache.clone() }),
        p_data_size.as_mut_ptr(),
        p_data,
    );
    vk_status.map_success(|| p_data_size.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMergePipelineCaches.html>"]
#[doc(alias = "vkMergePipelineCaches")]
pub unsafe fn merge_pipeline_caches<'a, V3: Alias<raw::PipelineCache> + 'a>(
    device: &raw::Device,
    dst_cache: &raw::PipelineCache,
    p_src_caches: impl AsSlice<'a, V3>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .merge_pipeline_caches
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { dst_cache.clone() }),
        p_src_caches.as_slice().len() as _,
        p_src_caches.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateGraphicsPipelines.html>"]
#[doc(alias = "vkCreateGraphicsPipelines")]
pub unsafe fn create_graphics_pipelines<'a, R: DynamicArray<Pipeline>>(
    device: &raw::Device,
    pipeline_cache: Option<&raw::PipelineCache>,
    p_create_infos: impl AsSlice<'a, GraphicsPipelineCreateInfo<'a>>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, R)> {
    let vulkan_command = dispatcher
        .create_graphics_pipelines
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_pipelines = R::create_with_capacity(p_create_infos.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        pipeline_cache.map(|v| unsafe { v.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateComputePipelines.html>"]
#[doc(alias = "vkCreateComputePipelines")]
pub unsafe fn create_compute_pipelines<'a, R: DynamicArray<Pipeline>>(
    device: &raw::Device,
    pipeline_cache: Option<&raw::PipelineCache>,
    p_create_infos: impl AsSlice<'a, ComputePipelineCreateInfo<'a>>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, R)> {
    let vulkan_command = dispatcher
        .create_compute_pipelines
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_pipelines = R::create_with_capacity(p_create_infos.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        pipeline_cache.map(|v| unsafe { v.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipeline.html>"]
#[doc(alias = "vkDestroyPipeline")]
pub unsafe fn destroy_pipeline(
    device: &raw::Device,
    pipeline: Option<&raw::Pipeline>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_pipeline
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        pipeline.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineLayout.html>"]
#[doc(alias = "vkCreatePipelineLayout")]
pub unsafe fn create_pipeline_layout(
    device: &raw::Device,
    p_create_info: &PipelineLayoutCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<PipelineLayout> {
    let vulkan_command = dispatcher
        .create_pipeline_layout
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_pipeline_layout = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_pipeline_layout.as_mut_ptr(),
    );
    vk_status.map_success(|| p_pipeline_layout.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineLayout.html>"]
#[doc(alias = "vkDestroyPipelineLayout")]
pub unsafe fn destroy_pipeline_layout(
    device: &raw::Device,
    pipeline_layout: Option<&raw::PipelineLayout>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_pipeline_layout
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        pipeline_layout.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSampler.html>"]
#[doc(alias = "vkCreateSampler")]
pub unsafe fn create_sampler(
    device: &raw::Device,
    p_create_info: &SamplerCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Sampler> {
    let vulkan_command = dispatcher
        .create_sampler
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_sampler = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_sampler.as_mut_ptr(),
    );
    vk_status.map_success(|| p_sampler.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySampler.html>"]
#[doc(alias = "vkDestroySampler")]
pub unsafe fn destroy_sampler(
    device: &raw::Device,
    sampler: Option<&raw::Sampler>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_sampler
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        sampler.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorSetLayout.html>"]
#[doc(alias = "vkCreateDescriptorSetLayout")]
pub unsafe fn create_descriptor_set_layout(
    device: &raw::Device,
    p_create_info: &DescriptorSetLayoutCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DescriptorSetLayout> {
    let vulkan_command = dispatcher
        .create_descriptor_set_layout
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_set_layout = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_set_layout.as_mut_ptr(),
    );
    vk_status.map_success(|| p_set_layout.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorSetLayout.html>"]
#[doc(alias = "vkDestroyDescriptorSetLayout")]
pub unsafe fn destroy_descriptor_set_layout(
    device: &raw::Device,
    descriptor_set_layout: Option<&raw::DescriptorSetLayout>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_descriptor_set_layout
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        descriptor_set_layout.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorPool.html>"]
#[doc(alias = "vkCreateDescriptorPool")]
pub unsafe fn create_descriptor_pool(
    device: &raw::Device,
    p_create_info: &DescriptorPoolCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DescriptorPool> {
    let vulkan_command = dispatcher
        .create_descriptor_pool
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_descriptor_pool = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_descriptor_pool.as_mut_ptr(),
    );
    vk_status.map_success(|| p_descriptor_pool.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorPool.html>"]
#[doc(alias = "vkDestroyDescriptorPool")]
pub unsafe fn destroy_descriptor_pool(
    device: &raw::Device,
    descriptor_pool: Option<&raw::DescriptorPool>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_descriptor_pool
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        descriptor_pool.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetDescriptorPool.html>"]
#[doc(alias = "vkResetDescriptorPool")]
pub unsafe fn reset_descriptor_pool(
    device: &raw::Device,
    descriptor_pool: &raw::DescriptorPool,
    flags: u32,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .reset_descriptor_pool
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { descriptor_pool.clone() }),
        flags,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateDescriptorSets.html>"]
#[doc(alias = "vkAllocateDescriptorSets")]
pub unsafe fn allocate_descriptor_sets<R: DynamicArray<DescriptorSet>>(
    device: &raw::Device,
    p_allocate_info: &DescriptorSetAllocateInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .allocate_descriptor_sets
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_descriptor_sets = R::create_with_capacity(p_allocate_info.descriptor_set_count as _);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_allocate_info),
        p_descriptor_sets.get_content_mut_ptr(),
    );
    vk_status.map_success(|| {
        p_descriptor_sets.resize_with_len(p_allocate_info.descriptor_set_count as _);
        p_descriptor_sets
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeDescriptorSets.html>"]
#[doc(alias = "vkFreeDescriptorSets")]
pub unsafe fn free_descriptor_sets<'a, V3: Alias<raw::DescriptorSet> + 'a>(
    device: &raw::Device,
    descriptor_pool: &raw::DescriptorPool,
    p_descriptor_sets: impl AsSlice<'a, V3>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .free_descriptor_sets
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { descriptor_pool.clone() }),
        p_descriptor_sets.as_slice().len() as _,
        p_descriptor_sets.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSets.html>"]
#[doc(alias = "vkUpdateDescriptorSets")]
pub unsafe fn update_descriptor_sets<'a>(
    device: &raw::Device,
    p_descriptor_writes: impl AsSlice<'a, WriteDescriptorSet<'a>>,
    p_descriptor_copies: impl AsSlice<'a, CopyDescriptorSet<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .update_descriptor_sets
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        p_descriptor_writes.as_slice().len() as _,
        p_descriptor_writes.as_slice().as_ptr().cast(),
        p_descriptor_copies.as_slice().len() as _,
        p_descriptor_copies.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateFramebuffer.html>"]
#[doc(alias = "vkCreateFramebuffer")]
pub unsafe fn create_framebuffer(
    device: &raw::Device,
    p_create_info: &FramebufferCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Framebuffer> {
    let vulkan_command = dispatcher
        .create_framebuffer
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_framebuffer = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_framebuffer.as_mut_ptr(),
    );
    vk_status.map_success(|| p_framebuffer.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyFramebuffer.html>"]
#[doc(alias = "vkDestroyFramebuffer")]
pub unsafe fn destroy_framebuffer(
    device: &raw::Device,
    framebuffer: Option<&raw::Framebuffer>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_framebuffer
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        framebuffer.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass.html>"]
#[doc(alias = "vkCreateRenderPass")]
pub unsafe fn create_render_pass(
    device: &raw::Device,
    p_create_info: &RenderPassCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<RenderPass> {
    let vulkan_command = dispatcher
        .create_render_pass
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_render_pass = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_render_pass.as_mut_ptr(),
    );
    vk_status.map_success(|| p_render_pass.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyRenderPass.html>"]
#[doc(alias = "vkDestroyRenderPass")]
pub unsafe fn destroy_render_pass(
    device: &raw::Device,
    render_pass: Option<&raw::RenderPass>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_render_pass
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        render_pass.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRenderAreaGranularity.html>"]
#[doc(alias = "vkGetRenderAreaGranularity")]
pub unsafe fn get_render_area_granularity(
    device: &raw::Device,
    render_pass: &raw::RenderPass,
    dispatcher: &CommandsDispatcher,
) -> Extent2D {
    let vulkan_command = dispatcher
        .get_render_area_granularity
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_granularity = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { render_pass.clone() }),
        p_granularity.as_mut_ptr(),
    );
    p_granularity.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCommandPool.html>"]
#[doc(alias = "vkCreateCommandPool")]
pub unsafe fn create_command_pool(
    device: &raw::Device,
    p_create_info: &CommandPoolCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<CommandPool> {
    let vulkan_command = dispatcher
        .create_command_pool
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_command_pool = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_command_pool.as_mut_ptr(),
    );
    vk_status.map_success(|| p_command_pool.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCommandPool.html>"]
#[doc(alias = "vkDestroyCommandPool")]
pub unsafe fn destroy_command_pool(
    device: &raw::Device,
    command_pool: Option<&raw::CommandPool>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_command_pool
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        command_pool.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandPool.html>"]
#[doc(alias = "vkResetCommandPool")]
pub unsafe fn reset_command_pool(
    device: &raw::Device,
    command_pool: &raw::CommandPool,
    flags: CommandPoolResetFlags,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .reset_command_pool
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { command_pool.clone() }),
        flags,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateCommandBuffers.html>"]
#[doc(alias = "vkAllocateCommandBuffers")]
pub unsafe fn allocate_command_buffers<R: DynamicArray<CommandBuffer>>(
    device: &raw::Device,
    p_allocate_info: &CommandBufferAllocateInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .allocate_command_buffers
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_command_buffers = R::create_with_capacity(p_allocate_info.command_buffer_count as _);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_allocate_info),
        p_command_buffers.get_content_mut_ptr(),
    );
    vk_status.map_success(|| {
        p_command_buffers.resize_with_len(p_allocate_info.command_buffer_count as _);
        p_command_buffers
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeCommandBuffers.html>"]
#[doc(alias = "vkFreeCommandBuffers")]
pub unsafe fn free_command_buffers<'a, V3: Alias<raw::CommandBuffer> + 'a>(
    device: &raw::Device,
    command_pool: &raw::CommandPool,
    p_command_buffers: impl AsSlice<'a, V3>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .free_command_buffers
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { command_pool.clone() }),
        p_command_buffers.as_slice().len() as _,
        p_command_buffers.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBeginCommandBuffer.html>"]
#[doc(alias = "vkBeginCommandBuffer")]
pub unsafe fn begin_command_buffer(
    command_buffer: &raw::CommandBuffer,
    p_begin_info: &CommandBufferBeginInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .begin_command_buffer
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_begin_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEndCommandBuffer.html>"]
#[doc(alias = "vkEndCommandBuffer")]
pub unsafe fn end_command_buffer(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .end_command_buffer
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() })).map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandBuffer.html>"]
#[doc(alias = "vkResetCommandBuffer")]
pub unsafe fn reset_command_buffer(
    command_buffer: &raw::CommandBuffer,
    flags: CommandBufferResetFlags,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .reset_command_buffer
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), flags).map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipeline.html>"]
#[doc(alias = "vkCmdBindPipeline")]
pub unsafe fn cmd_bind_pipeline(
    command_buffer: &raw::CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: &raw::Pipeline,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_pipeline
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        pipeline_bind_point,
        Some(unsafe { pipeline.clone() }),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewport.html>"]
#[doc(alias = "vkCmdSetViewport")]
pub unsafe fn cmd_set_viewport<'a>(
    command_buffer: &raw::CommandBuffer,
    first_viewport: u32,
    p_viewports: impl AsSlice<'a, Viewport>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_viewport
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        first_viewport,
        p_viewports.as_slice().len() as _,
        p_viewports.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissor.html>"]
#[doc(alias = "vkCmdSetScissor")]
pub unsafe fn cmd_set_scissor<'a>(
    command_buffer: &raw::CommandBuffer,
    first_scissor: u32,
    p_scissors: impl AsSlice<'a, Rect2D>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_scissor
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        first_scissor,
        p_scissors.as_slice().len() as _,
        p_scissors.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineWidth.html>"]
#[doc(alias = "vkCmdSetLineWidth")]
pub unsafe fn cmd_set_line_width(
    command_buffer: &raw::CommandBuffer,
    line_width: f32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_line_width
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), line_width)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBias.html>"]
#[doc(alias = "vkCmdSetDepthBias")]
pub unsafe fn cmd_set_depth_bias(
    command_buffer: &raw::CommandBuffer,
    depth_bias_constant_factor: f32,
    depth_bias_clamp: f32,
    depth_bias_slope_factor: f32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_bias
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        depth_bias_constant_factor,
        depth_bias_clamp,
        depth_bias_slope_factor,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetBlendConstants.html>"]
#[doc(alias = "vkCmdSetBlendConstants")]
pub unsafe fn cmd_set_blend_constants(
    command_buffer: &raw::CommandBuffer,
    blend_constants: [f32; 4u16 as _],
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_blend_constants
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), blend_constants)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBounds.html>"]
#[doc(alias = "vkCmdSetDepthBounds")]
pub unsafe fn cmd_set_depth_bounds(
    command_buffer: &raw::CommandBuffer,
    min_depth_bounds: f32,
    max_depth_bounds: f32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_bounds
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        min_depth_bounds,
        max_depth_bounds,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilCompareMask.html>"]
#[doc(alias = "vkCmdSetStencilCompareMask")]
pub unsafe fn cmd_set_stencil_compare_mask(
    command_buffer: &raw::CommandBuffer,
    face_mask: StencilFaceFlags,
    compare_mask: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_stencil_compare_mask
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        face_mask,
        compare_mask,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilWriteMask.html>"]
#[doc(alias = "vkCmdSetStencilWriteMask")]
pub unsafe fn cmd_set_stencil_write_mask(
    command_buffer: &raw::CommandBuffer,
    face_mask: StencilFaceFlags,
    write_mask: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_stencil_write_mask
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        face_mask,
        write_mask,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilReference.html>"]
#[doc(alias = "vkCmdSetStencilReference")]
pub unsafe fn cmd_set_stencil_reference(
    command_buffer: &raw::CommandBuffer,
    face_mask: StencilFaceFlags,
    reference: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_stencil_reference
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        face_mask,
        reference,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorSets.html>"]
#[doc(alias = "vkCmdBindDescriptorSets")]
pub unsafe fn cmd_bind_descriptor_sets<'a, V5: Alias<raw::DescriptorSet> + 'a>(
    command_buffer: &raw::CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: &raw::PipelineLayout,
    first_set: u32,
    p_descriptor_sets: impl AsSlice<'a, V5>,
    p_dynamic_offsets: impl AsSlice<'a, u32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_descriptor_sets
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        pipeline_bind_point,
        Some(unsafe { layout.clone() }),
        first_set,
        p_descriptor_sets.as_slice().len() as _,
        p_descriptor_sets.as_slice().as_ptr().cast(),
        p_dynamic_offsets.as_slice().len() as _,
        p_dynamic_offsets.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindIndexBuffer.html>"]
#[doc(alias = "vkCmdBindIndexBuffer")]
pub unsafe fn cmd_bind_index_buffer(
    command_buffer: &raw::CommandBuffer,
    buffer: Option<&raw::Buffer>,
    offset: DeviceSize,
    index_type: IndexType,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_index_buffer
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        buffer.map(|v| unsafe { v.clone() }),
        offset,
        index_type,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers.html>"]
#[doc(alias = "vkCmdBindVertexBuffers")]
pub unsafe fn cmd_bind_vertex_buffers<'a, V3: Alias<raw::Buffer> + 'a>(
    command_buffer: &raw::CommandBuffer,
    first_binding: u32,
    p_buffers: impl AsSlice<'a, V3>,
    p_offsets: impl AsSlice<'a, DeviceSize>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_vertex_buffers
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        first_binding,
        p_offsets.as_slice().len() as _,
        p_buffers.as_slice().as_ptr().cast(),
        p_offsets.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDraw.html>"]
#[doc(alias = "vkCmdDraw")]
pub unsafe fn cmd_draw(
    command_buffer: &raw::CommandBuffer,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_draw
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        vertex_count,
        instance_count,
        first_vertex,
        first_instance,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexed.html>"]
#[doc(alias = "vkCmdDrawIndexed")]
pub unsafe fn cmd_draw_indexed(
    command_buffer: &raw::CommandBuffer,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_draw_indexed
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        index_count,
        instance_count,
        first_index,
        vertex_offset,
        first_instance,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirect.html>"]
#[doc(alias = "vkCmdDrawIndirect")]
pub unsafe fn cmd_draw_indirect(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_draw_indirect
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { buffer.clone() }),
        offset,
        draw_count,
        stride,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirect.html>"]
#[doc(alias = "vkCmdDrawIndexedIndirect")]
pub unsafe fn cmd_draw_indexed_indirect(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_draw_indexed_indirect
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { buffer.clone() }),
        offset,
        draw_count,
        stride,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatch.html>"]
#[doc(alias = "vkCmdDispatch")]
pub unsafe fn cmd_dispatch(
    command_buffer: &raw::CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_dispatch
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        group_count_x,
        group_count_y,
        group_count_z,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchIndirect.html>"]
#[doc(alias = "vkCmdDispatchIndirect")]
pub unsafe fn cmd_dispatch_indirect(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_dispatch_indirect
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { buffer.clone() }),
        offset,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer.html>"]
#[doc(alias = "vkCmdCopyBuffer")]
pub unsafe fn cmd_copy_buffer<'a>(
    command_buffer: &raw::CommandBuffer,
    src_buffer: &raw::Buffer,
    dst_buffer: &raw::Buffer,
    p_regions: impl AsSlice<'a, BufferCopy>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_buffer
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { src_buffer.clone() }),
        Some(unsafe { dst_buffer.clone() }),
        p_regions.as_slice().len() as _,
        p_regions.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage.html>"]
#[doc(alias = "vkCmdCopyImage")]
pub unsafe fn cmd_copy_image<'a>(
    command_buffer: &raw::CommandBuffer,
    src_image: &raw::Image,
    src_image_layout: ImageLayout,
    dst_image: &raw::Image,
    dst_image_layout: ImageLayout,
    p_regions: impl AsSlice<'a, ImageCopy>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_image
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { src_image.clone() }),
        src_image_layout,
        Some(unsafe { dst_image.clone() }),
        dst_image_layout,
        p_regions.as_slice().len() as _,
        p_regions.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage.html>"]
#[doc(alias = "vkCmdBlitImage")]
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
    let vulkan_command = dispatcher
        .cmd_blit_image
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { src_image.clone() }),
        src_image_layout,
        Some(unsafe { dst_image.clone() }),
        dst_image_layout,
        p_regions.as_slice().len() as _,
        p_regions.as_slice().as_ptr().cast(),
        filter,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage.html>"]
#[doc(alias = "vkCmdCopyBufferToImage")]
pub unsafe fn cmd_copy_buffer_to_image<'a>(
    command_buffer: &raw::CommandBuffer,
    src_buffer: &raw::Buffer,
    dst_image: &raw::Image,
    dst_image_layout: ImageLayout,
    p_regions: impl AsSlice<'a, BufferImageCopy>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_buffer_to_image
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { src_buffer.clone() }),
        Some(unsafe { dst_image.clone() }),
        dst_image_layout,
        p_regions.as_slice().len() as _,
        p_regions.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer.html>"]
#[doc(alias = "vkCmdCopyImageToBuffer")]
pub unsafe fn cmd_copy_image_to_buffer<'a>(
    command_buffer: &raw::CommandBuffer,
    src_image: &raw::Image,
    src_image_layout: ImageLayout,
    dst_buffer: &raw::Buffer,
    p_regions: impl AsSlice<'a, BufferImageCopy>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_image_to_buffer
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { src_image.clone() }),
        src_image_layout,
        Some(unsafe { dst_buffer.clone() }),
        p_regions.as_slice().len() as _,
        p_regions.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdUpdateBuffer.html>"]
#[doc(alias = "vkCmdUpdateBuffer")]
pub unsafe fn cmd_update_buffer(
    command_buffer: &raw::CommandBuffer,
    dst_buffer: &raw::Buffer,
    dst_offset: DeviceSize,
    data_size: DeviceSize,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_update_buffer
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { dst_buffer.clone() }),
        dst_offset,
        data_size,
        p_data,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdFillBuffer.html>"]
#[doc(alias = "vkCmdFillBuffer")]
pub unsafe fn cmd_fill_buffer(
    command_buffer: &raw::CommandBuffer,
    dst_buffer: &raw::Buffer,
    dst_offset: DeviceSize,
    size: DeviceSize,
    data: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_fill_buffer
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { dst_buffer.clone() }),
        dst_offset,
        size,
        data,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearColorImage.html>"]
#[doc(alias = "vkCmdClearColorImage")]
pub unsafe fn cmd_clear_color_image<'a>(
    command_buffer: &raw::CommandBuffer,
    image: &raw::Image,
    image_layout: ImageLayout,
    p_color: &ClearColorValue,
    p_ranges: impl AsSlice<'a, ImageSubresourceRange>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_clear_color_image
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { image.clone() }),
        image_layout,
        ptr::from_ref(p_color),
        p_ranges.as_slice().len() as _,
        p_ranges.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearDepthStencilImage.html>"]
#[doc(alias = "vkCmdClearDepthStencilImage")]
pub unsafe fn cmd_clear_depth_stencil_image<'a>(
    command_buffer: &raw::CommandBuffer,
    image: &raw::Image,
    image_layout: ImageLayout,
    p_depth_stencil: &ClearDepthStencilValue,
    p_ranges: impl AsSlice<'a, ImageSubresourceRange>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_clear_depth_stencil_image
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { image.clone() }),
        image_layout,
        ptr::from_ref(p_depth_stencil),
        p_ranges.as_slice().len() as _,
        p_ranges.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearAttachments.html>"]
#[doc(alias = "vkCmdClearAttachments")]
pub unsafe fn cmd_clear_attachments<'a>(
    command_buffer: &raw::CommandBuffer,
    p_attachments: impl AsSlice<'a, ClearAttachment>,
    p_rects: impl AsSlice<'a, ClearRect>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_clear_attachments
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_attachments.as_slice().len() as _,
        p_attachments.as_slice().as_ptr().cast(),
        p_rects.as_slice().len() as _,
        p_rects.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage.html>"]
#[doc(alias = "vkCmdResolveImage")]
pub unsafe fn cmd_resolve_image<'a>(
    command_buffer: &raw::CommandBuffer,
    src_image: &raw::Image,
    src_image_layout: ImageLayout,
    dst_image: &raw::Image,
    dst_image_layout: ImageLayout,
    p_regions: impl AsSlice<'a, ImageResolve>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_resolve_image
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { src_image.clone() }),
        src_image_layout,
        Some(unsafe { dst_image.clone() }),
        dst_image_layout,
        p_regions.as_slice().len() as _,
        p_regions.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent.html>"]
#[doc(alias = "vkCmdSetEvent")]
pub unsafe fn cmd_set_event(
    command_buffer: &raw::CommandBuffer,
    event: &raw::Event,
    stage_mask: PipelineStageFlags,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_event
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { event.clone() }),
        stage_mask,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent.html>"]
#[doc(alias = "vkCmdResetEvent")]
pub unsafe fn cmd_reset_event(
    command_buffer: &raw::CommandBuffer,
    event: &raw::Event,
    stage_mask: PipelineStageFlags,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_reset_event
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { event.clone() }),
        stage_mask,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents.html>"]
#[doc(alias = "vkCmdWaitEvents")]
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
    let vulkan_command = dispatcher
        .cmd_wait_events
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier.html>"]
#[doc(alias = "vkCmdPipelineBarrier")]
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
    let vulkan_command = dispatcher
        .cmd_pipeline_barrier
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQuery.html>"]
#[doc(alias = "vkCmdBeginQuery")]
pub unsafe fn cmd_begin_query(
    command_buffer: &raw::CommandBuffer,
    query_pool: &raw::QueryPool,
    query: u32,
    flags: QueryControlFlags,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_begin_query
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { query_pool.clone() }),
        query,
        flags,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQuery.html>"]
#[doc(alias = "vkCmdEndQuery")]
pub unsafe fn cmd_end_query(
    command_buffer: &raw::CommandBuffer,
    query_pool: &raw::QueryPool,
    query: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_end_query
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { query_pool.clone() }),
        query,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetQueryPool.html>"]
#[doc(alias = "vkCmdResetQueryPool")]
pub unsafe fn cmd_reset_query_pool(
    command_buffer: &raw::CommandBuffer,
    query_pool: &raw::QueryPool,
    first_query: u32,
    query_count: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_reset_query_pool
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { query_pool.clone() }),
        first_query,
        query_count,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp.html>"]
#[doc(alias = "vkCmdWriteTimestamp")]
pub unsafe fn cmd_write_timestamp(
    command_buffer: &raw::CommandBuffer,
    pipeline_stage: PipelineStageFlags,
    query_pool: &raw::QueryPool,
    query: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_write_timestamp
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        pipeline_stage,
        Some(unsafe { query_pool.clone() }),
        query,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyQueryPoolResults.html>"]
#[doc(alias = "vkCmdCopyQueryPoolResults")]
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
    let vulkan_command = dispatcher
        .cmd_copy_query_pool_results
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { query_pool.clone() }),
        first_query,
        query_count,
        Some(unsafe { dst_buffer.clone() }),
        dst_offset,
        stride,
        flags,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants.html>"]
#[doc(alias = "vkCmdPushConstants")]
pub unsafe fn cmd_push_constants(
    command_buffer: &raw::CommandBuffer,
    layout: &raw::PipelineLayout,
    stage_flags: ShaderStageFlags,
    offset: u32,
    size: u32,
    p_values: VoidPtr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_push_constants
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { layout.clone() }),
        stage_flags,
        offset,
        size,
        p_values,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass.html>"]
#[doc(alias = "vkCmdBeginRenderPass")]
pub unsafe fn cmd_begin_render_pass(
    command_buffer: &raw::CommandBuffer,
    p_render_pass_begin: &RenderPassBeginInfo,
    contents: SubpassContents,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_begin_render_pass
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_render_pass_begin),
        contents,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass.html>"]
#[doc(alias = "vkCmdNextSubpass")]
pub unsafe fn cmd_next_subpass(
    command_buffer: &raw::CommandBuffer,
    contents: SubpassContents,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_next_subpass
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), contents)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass.html>"]
#[doc(alias = "vkCmdEndRenderPass")]
pub unsafe fn cmd_end_render_pass(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_end_render_pass
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteCommands.html>"]
#[doc(alias = "vkCmdExecuteCommands")]
pub unsafe fn cmd_execute_commands<'a, V2: Alias<raw::CommandBuffer> + 'a>(
    command_buffer: &raw::CommandBuffer,
    p_command_buffers: impl AsSlice<'a, V2>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_execute_commands
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_command_buffers.as_slice().len() as _,
        p_command_buffers.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceVersion.html>"]
#[doc(alias = "vkEnumerateInstanceVersion")]
pub unsafe fn enumerate_instance_version(dispatcher: &CommandsDispatcher) -> Result<u32> {
    let vulkan_command = dispatcher
        .enumerate_instance_version
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_api_version = MaybeUninit::uninit();
    let vk_status = vulkan_command(p_api_version.as_mut_ptr());
    vk_status.map_success(|| p_api_version.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2.html>"]
#[doc(alias = "vkBindBufferMemory2")]
pub unsafe fn bind_buffer_memory2<'a>(
    device: &raw::Device,
    p_bind_infos: impl AsSlice<'a, BindBufferMemoryInfo<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .bind_buffer_memory2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        p_bind_infos.as_slice().len() as _,
        p_bind_infos.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindBufferMemory2KHR.html>"]
#[doc(alias = "vkBindBufferMemory2KHR")]
pub unsafe fn bind_buffer_memory2_khr<'a>(
    device: &raw::Device,
    p_bind_infos: impl AsSlice<'a, BindBufferMemoryInfo<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .bind_buffer_memory2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        p_bind_infos.as_slice().len() as _,
        p_bind_infos.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2.html>"]
#[doc(alias = "vkBindImageMemory2")]
pub unsafe fn bind_image_memory2<'a>(
    device: &raw::Device,
    p_bind_infos: impl AsSlice<'a, BindImageMemoryInfo<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .bind_image_memory2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        p_bind_infos.as_slice().len() as _,
        p_bind_infos.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindImageMemory2KHR.html>"]
#[doc(alias = "vkBindImageMemory2KHR")]
pub unsafe fn bind_image_memory2_khr<'a>(
    device: &raw::Device,
    p_bind_infos: impl AsSlice<'a, BindImageMemoryInfo<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .bind_image_memory2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        p_bind_infos.as_slice().len() as _,
        p_bind_infos.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html>"]
#[doc(alias = "vkGetDeviceGroupPeerMemoryFeatures")]
pub unsafe fn get_device_group_peer_memory_features(
    device: &raw::Device,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    dispatcher: &CommandsDispatcher,
) -> PeerMemoryFeatureFlags {
    let vulkan_command = dispatcher
        .get_device_group_peer_memory_features
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_peer_memory_features = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        heap_index,
        local_device_index,
        remote_device_index,
        p_peer_memory_features.as_mut_ptr(),
    );
    p_peer_memory_features.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPeerMemoryFeaturesKHR.html>"]
#[doc(alias = "vkGetDeviceGroupPeerMemoryFeaturesKHR")]
pub unsafe fn get_device_group_peer_memory_features_khr(
    device: &raw::Device,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    dispatcher: &CommandsDispatcher,
) -> PeerMemoryFeatureFlags {
    let vulkan_command = dispatcher
        .get_device_group_peer_memory_features_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_peer_memory_features = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        heap_index,
        local_device_index,
        remote_device_index,
        p_peer_memory_features.as_mut_ptr(),
    );
    p_peer_memory_features.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMask.html>"]
#[doc(alias = "vkCmdSetDeviceMask")]
pub unsafe fn cmd_set_device_mask(
    command_buffer: &raw::CommandBuffer,
    device_mask: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_device_mask
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), device_mask)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMaskKHR.html>"]
#[doc(alias = "vkCmdSetDeviceMaskKHR")]
pub unsafe fn cmd_set_device_mask_khr(
    command_buffer: &raw::CommandBuffer,
    device_mask: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_device_mask_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), device_mask)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchBase.html>"]
#[doc(alias = "vkCmdDispatchBase")]
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
    let vulkan_command = dispatcher
        .cmd_dispatch_base
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        base_group_x,
        base_group_y,
        base_group_z,
        group_count_x,
        group_count_y,
        group_count_z,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchBaseKHR.html>"]
#[doc(alias = "vkCmdDispatchBaseKHR")]
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
    let vulkan_command = dispatcher
        .cmd_dispatch_base_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        base_group_x,
        base_group_y,
        base_group_z,
        group_count_x,
        group_count_y,
        group_count_z,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html>"]
#[doc(alias = "vkEnumeratePhysicalDeviceGroups")]
pub unsafe fn enumerate_physical_device_groups<
    R: DynamicArray<PhysicalDeviceGroupProperties<'static>>,
>(
    instance: &raw::Instance,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .enumerate_physical_device_groups
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_physical_device_group_count = vk_len.as_mut_ptr();
    let p_physical_device_group_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { instance.clone() }),
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
            Some(unsafe { instance.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroupsKHR.html>"]
#[doc(alias = "vkEnumeratePhysicalDeviceGroupsKHR")]
pub unsafe fn enumerate_physical_device_groups_khr<
    R: DynamicArray<PhysicalDeviceGroupProperties<'static>>,
>(
    instance: &raw::Instance,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .enumerate_physical_device_groups_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_physical_device_group_count = vk_len.as_mut_ptr();
    let p_physical_device_group_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { instance.clone() }),
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
            Some(unsafe { instance.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2.html>"]
#[doc(alias = "vkGetImageMemoryRequirements2")]
pub unsafe fn get_image_memory_requirements2<S: StructureChainOut<MemoryRequirements2<'static>>>(
    device: &raw::Device,
    p_info: &ImageMemoryRequirementsInfo2,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_image_memory_requirements2
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2KHR.html>"]
#[doc(alias = "vkGetImageMemoryRequirements2KHR")]
pub unsafe fn get_image_memory_requirements2_khr<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &ImageMemoryRequirementsInfo2,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_image_memory_requirements2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2.html>"]
#[doc(alias = "vkGetBufferMemoryRequirements2")]
pub unsafe fn get_buffer_memory_requirements2<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &BufferMemoryRequirementsInfo2,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_buffer_memory_requirements2
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2KHR.html>"]
#[doc(alias = "vkGetBufferMemoryRequirements2KHR")]
pub unsafe fn get_buffer_memory_requirements2_khr<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &BufferMemoryRequirementsInfo2,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_buffer_memory_requirements2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2.html>"]
#[doc(alias = "vkGetImageSparseMemoryRequirements2")]
pub unsafe fn get_image_sparse_memory_requirements2<
    R: DynamicArray<SparseImageMemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &ImageSparseMemoryRequirementsInfo2,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher
        .get_image_sparse_memory_requirements2
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_sparse_memory_requirement_count = vk_len.as_mut_ptr();
    let p_sparse_memory_requirements = ptr::null_mut();
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_sparse_memory_requirement_count = ptr::from_mut(&mut vk_len);
    let mut p_sparse_memory_requirements = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2KHR.html>"]
#[doc(alias = "vkGetImageSparseMemoryRequirements2KHR")]
pub unsafe fn get_image_sparse_memory_requirements2_khr<
    R: DynamicArray<SparseImageMemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &ImageSparseMemoryRequirementsInfo2,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher
        .get_image_sparse_memory_requirements2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_sparse_memory_requirement_count = vk_len.as_mut_ptr();
    let p_sparse_memory_requirements = ptr::null_mut();
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_sparse_memory_requirement_count = ptr::from_mut(&mut vk_len);
    let mut p_sparse_memory_requirements = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2.html>"]
#[doc(alias = "vkGetPhysicalDeviceFeatures2")]
pub unsafe fn get_physical_device_features2<
    S: StructureChainOut<PhysicalDeviceFeatures2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_features2
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_features = MaybeUninit::uninit();
    S::setup_uninit(&mut p_features);
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        S::get_uninit_head_ptr(p_features.as_mut_ptr()),
    );
    S::setup_cleanup(p_features.as_mut_ptr());
    p_features.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceFeatures2KHR")]
pub unsafe fn get_physical_device_features2_khr<
    S: StructureChainOut<PhysicalDeviceFeatures2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_features2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_features = MaybeUninit::uninit();
    S::setup_uninit(&mut p_features);
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        S::get_uninit_head_ptr(p_features.as_mut_ptr()),
    );
    S::setup_cleanup(p_features.as_mut_ptr());
    p_features.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2.html>"]
#[doc(alias = "vkGetPhysicalDeviceProperties2")]
pub unsafe fn get_physical_device_properties2<
    S: StructureChainOut<PhysicalDeviceProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_properties2
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_properties.as_mut_ptr());
    p_properties.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceProperties2KHR")]
pub unsafe fn get_physical_device_properties2_khr<
    S: StructureChainOut<PhysicalDeviceProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_properties2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_properties.as_mut_ptr());
    p_properties.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html>"]
#[doc(alias = "vkGetPhysicalDeviceFormatProperties2")]
pub unsafe fn get_physical_device_format_properties2<
    S: StructureChainOut<FormatProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    format: Format,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_format_properties2
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_format_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_format_properties);
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        format,
        S::get_uninit_head_ptr(p_format_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_format_properties.as_mut_ptr());
    p_format_properties.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceFormatProperties2KHR")]
pub unsafe fn get_physical_device_format_properties2_khr<
    S: StructureChainOut<FormatProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    format: Format,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_format_properties2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_format_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_format_properties);
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        format,
        S::get_uninit_head_ptr(p_format_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_format_properties.as_mut_ptr());
    p_format_properties.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_image_format_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_image_format_properties);
    let vk_status = vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(p_image_format_info),
        S::get_uninit_head_ptr(p_image_format_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_image_format_properties.as_mut_ptr());
        p_image_format_properties.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2KHR.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_image_format_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_image_format_properties);
    let vk_status = vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(p_image_format_info),
        S::get_uninit_head_ptr(p_image_format_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_image_format_properties.as_mut_ptr());
        p_image_format_properties.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html>"]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2")]
pub unsafe fn get_physical_device_queue_family_properties2<
    R: DynamicArray<QueueFamilyProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher
        .get_physical_device_queue_family_properties2
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_queue_family_property_count = vk_len.as_mut_ptr();
    let p_queue_family_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        p_queue_family_property_count,
        p_queue_family_properties,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_queue_family_property_count = ptr::from_mut(&mut vk_len);
    let mut p_queue_family_properties = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        p_queue_family_property_count,
        p_queue_family_properties,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2KHR")]
pub unsafe fn get_physical_device_queue_family_properties2_khr<
    R: DynamicArray<QueueFamilyProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher
        .get_physical_device_queue_family_properties2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_queue_family_property_count = vk_len.as_mut_ptr();
    let p_queue_family_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        p_queue_family_property_count,
        p_queue_family_properties,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_queue_family_property_count = ptr::from_mut(&mut vk_len);
    let mut p_queue_family_properties = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        p_queue_family_property_count,
        p_queue_family_properties,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html>"]
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties2")]
pub unsafe fn get_physical_device_memory_properties2<
    S: StructureChainOut<PhysicalDeviceMemoryProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_memory_properties2
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_properties);
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        S::get_uninit_head_ptr(p_memory_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_properties.as_mut_ptr());
    p_memory_properties.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties2KHR")]
pub unsafe fn get_physical_device_memory_properties2_khr<
    S: StructureChainOut<PhysicalDeviceMemoryProperties2<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_physical_device_memory_properties2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_properties);
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        S::get_uninit_head_ptr(p_memory_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_properties.as_mut_ptr());
    p_memory_properties.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(p_format_info),
        p_property_count,
        p_properties,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(p_format_info),
        p_property_count,
        p_properties,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(p_format_info),
        p_property_count,
        p_properties,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_property_count = ptr::from_mut(&mut vk_len);
    let mut p_properties = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(p_format_info),
        p_property_count,
        p_properties,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPool.html>"]
#[doc(alias = "vkTrimCommandPool")]
pub unsafe fn trim_command_pool(
    device: &raw::Device,
    command_pool: &raw::CommandPool,
    flags: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .trim_command_pool
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { command_pool.clone() }),
        flags,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPoolKHR.html>"]
#[doc(alias = "vkTrimCommandPoolKHR")]
pub unsafe fn trim_command_pool_khr(
    device: &raw::Device,
    command_pool: &raw::CommandPool,
    flags: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .trim_command_pool_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { command_pool.clone() }),
        flags,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue2.html>"]
#[doc(alias = "vkGetDeviceQueue2")]
pub unsafe fn get_device_queue2(
    device: &raw::Device,
    p_queue_info: &DeviceQueueInfo2,
    dispatcher: &CommandsDispatcher,
) -> Queue {
    let vulkan_command = dispatcher
        .get_device_queue2
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_queue = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_queue_info),
        p_queue.as_mut_ptr(),
    );
    p_queue.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversion.html>"]
#[doc(alias = "vkCreateSamplerYcbcrConversion")]
pub unsafe fn create_sampler_ycbcr_conversion(
    device: &raw::Device,
    p_create_info: &SamplerYcbcrConversionCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SamplerYcbcrConversion> {
    let vulkan_command = dispatcher
        .create_sampler_ycbcr_conversion
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_ycbcr_conversion = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_ycbcr_conversion.as_mut_ptr(),
    );
    vk_status.map_success(|| p_ycbcr_conversion.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSamplerYcbcrConversionKHR.html>"]
#[doc(alias = "vkCreateSamplerYcbcrConversionKHR")]
pub unsafe fn create_sampler_ycbcr_conversion_khr(
    device: &raw::Device,
    p_create_info: &SamplerYcbcrConversionCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SamplerYcbcrConversion> {
    let vulkan_command = dispatcher
        .create_sampler_ycbcr_conversion_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_ycbcr_conversion = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_ycbcr_conversion.as_mut_ptr(),
    );
    vk_status.map_success(|| p_ycbcr_conversion.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversion.html>"]
#[doc(alias = "vkDestroySamplerYcbcrConversion")]
pub unsafe fn destroy_sampler_ycbcr_conversion(
    device: &raw::Device,
    ycbcr_conversion: Option<&raw::SamplerYcbcrConversion>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_sampler_ycbcr_conversion
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ycbcr_conversion.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySamplerYcbcrConversionKHR.html>"]
#[doc(alias = "vkDestroySamplerYcbcrConversionKHR")]
pub unsafe fn destroy_sampler_ycbcr_conversion_khr(
    device: &raw::Device,
    ycbcr_conversion: Option<&raw::SamplerYcbcrConversion>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_sampler_ycbcr_conversion_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ycbcr_conversion.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplate.html>"]
#[doc(alias = "vkCreateDescriptorUpdateTemplate")]
pub unsafe fn create_descriptor_update_template(
    device: &raw::Device,
    p_create_info: &DescriptorUpdateTemplateCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DescriptorUpdateTemplate> {
    let vulkan_command = dispatcher
        .create_descriptor_update_template
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_descriptor_update_template = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_descriptor_update_template.as_mut_ptr(),
    );
    vk_status.map_success(|| p_descriptor_update_template.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorUpdateTemplateKHR.html>"]
#[doc(alias = "vkCreateDescriptorUpdateTemplateKHR")]
pub unsafe fn create_descriptor_update_template_khr(
    device: &raw::Device,
    p_create_info: &DescriptorUpdateTemplateCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DescriptorUpdateTemplate> {
    let vulkan_command = dispatcher
        .create_descriptor_update_template_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_descriptor_update_template = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_descriptor_update_template.as_mut_ptr(),
    );
    vk_status.map_success(|| p_descriptor_update_template.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html>"]
#[doc(alias = "vkDestroyDescriptorUpdateTemplate")]
pub unsafe fn destroy_descriptor_update_template(
    device: &raw::Device,
    descriptor_update_template: Option<&raw::DescriptorUpdateTemplate>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_descriptor_update_template
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        descriptor_update_template.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorUpdateTemplateKHR.html>"]
#[doc(alias = "vkDestroyDescriptorUpdateTemplateKHR")]
pub unsafe fn destroy_descriptor_update_template_khr(
    device: &raw::Device,
    descriptor_update_template: Option<&raw::DescriptorUpdateTemplate>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_descriptor_update_template_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        descriptor_update_template.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html>"]
#[doc(alias = "vkUpdateDescriptorSetWithTemplate")]
pub unsafe fn update_descriptor_set_with_template(
    device: &raw::Device,
    descriptor_set: &raw::DescriptorSet,
    descriptor_update_template: &raw::DescriptorUpdateTemplate,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .update_descriptor_set_with_template
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { descriptor_set.clone() }),
        Some(unsafe { descriptor_update_template.clone() }),
        p_data,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSetWithTemplateKHR.html>"]
#[doc(alias = "vkUpdateDescriptorSetWithTemplateKHR")]
pub unsafe fn update_descriptor_set_with_template_khr(
    device: &raw::Device,
    descriptor_set: &raw::DescriptorSet,
    descriptor_update_template: &raw::DescriptorUpdateTemplate,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .update_descriptor_set_with_template_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { descriptor_set.clone() }),
        Some(unsafe { descriptor_update_template.clone() }),
        p_data,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_external_buffer_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_external_buffer_properties);
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(p_external_buffer_info),
        S::get_uninit_head_ptr(p_external_buffer_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_external_buffer_properties.as_mut_ptr());
    p_external_buffer_properties.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_external_buffer_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_external_buffer_properties);
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(p_external_buffer_info),
        S::get_uninit_head_ptr(p_external_buffer_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_external_buffer_properties.as_mut_ptr());
    p_external_buffer_properties.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_external_fence_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_external_fence_properties);
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(p_external_fence_info),
        S::get_uninit_head_ptr(p_external_fence_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_external_fence_properties.as_mut_ptr());
    p_external_fence_properties.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFencePropertiesKHR.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_external_fence_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_external_fence_properties);
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(p_external_fence_info),
        S::get_uninit_head_ptr(p_external_fence_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_external_fence_properties.as_mut_ptr());
    p_external_fence_properties.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_external_semaphore_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_external_semaphore_properties);
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(p_external_semaphore_info),
        S::get_uninit_head_ptr(p_external_semaphore_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_external_semaphore_properties.as_mut_ptr());
    p_external_semaphore_properties.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_external_semaphore_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_external_semaphore_properties);
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(p_external_semaphore_info),
        S::get_uninit_head_ptr(p_external_semaphore_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_external_semaphore_properties.as_mut_ptr());
    p_external_semaphore_properties.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupport.html>"]
#[doc(alias = "vkGetDescriptorSetLayoutSupport")]
pub unsafe fn get_descriptor_set_layout_support<
    S: StructureChainOut<DescriptorSetLayoutSupport<'static>>,
>(
    device: &raw::Device,
    p_create_info: &DescriptorSetLayoutCreateInfo,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_descriptor_set_layout_support
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_support = MaybeUninit::uninit();
    S::setup_uninit(&mut p_support);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        S::get_uninit_head_ptr(p_support.as_mut_ptr()),
    );
    S::setup_cleanup(p_support.as_mut_ptr());
    p_support.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupportKHR.html>"]
#[doc(alias = "vkGetDescriptorSetLayoutSupportKHR")]
pub unsafe fn get_descriptor_set_layout_support_khr<
    S: StructureChainOut<DescriptorSetLayoutSupport<'static>>,
>(
    device: &raw::Device,
    p_create_info: &DescriptorSetLayoutCreateInfo,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_descriptor_set_layout_support_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_support = MaybeUninit::uninit();
    S::setup_uninit(&mut p_support);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        S::get_uninit_head_ptr(p_support.as_mut_ptr()),
    );
    S::setup_cleanup(p_support.as_mut_ptr());
    p_support.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCount.html>"]
#[doc(alias = "vkCmdDrawIndirectCount")]
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
    let vulkan_command = dispatcher
        .cmd_draw_indirect_count
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { buffer.clone() }),
        offset,
        Some(unsafe { count_buffer.clone() }),
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCountKHR.html>"]
#[doc(alias = "vkCmdDrawIndirectCountKHR")]
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
    let vulkan_command = dispatcher
        .cmd_draw_indirect_count_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { buffer.clone() }),
        offset,
        Some(unsafe { count_buffer.clone() }),
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCountAMD.html>"]
#[doc(alias = "vkCmdDrawIndirectCountAMD")]
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
    let vulkan_command = dispatcher
        .cmd_draw_indirect_count_amd
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { buffer.clone() }),
        offset,
        Some(unsafe { count_buffer.clone() }),
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCount.html>"]
#[doc(alias = "vkCmdDrawIndexedIndirectCount")]
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
    let vulkan_command = dispatcher
        .cmd_draw_indexed_indirect_count
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { buffer.clone() }),
        offset,
        Some(unsafe { count_buffer.clone() }),
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCountKHR.html>"]
#[doc(alias = "vkCmdDrawIndexedIndirectCountKHR")]
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
    let vulkan_command = dispatcher
        .cmd_draw_indexed_indirect_count_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { buffer.clone() }),
        offset,
        Some(unsafe { count_buffer.clone() }),
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCountAMD.html>"]
#[doc(alias = "vkCmdDrawIndexedIndirectCountAMD")]
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
    let vulkan_command = dispatcher
        .cmd_draw_indexed_indirect_count_amd
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { buffer.clone() }),
        offset,
        Some(unsafe { count_buffer.clone() }),
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2.html>"]
#[doc(alias = "vkCreateRenderPass2")]
pub unsafe fn create_render_pass2(
    device: &raw::Device,
    p_create_info: &RenderPassCreateInfo2,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<RenderPass> {
    let vulkan_command = dispatcher
        .create_render_pass2
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_render_pass = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_render_pass.as_mut_ptr(),
    );
    vk_status.map_success(|| p_render_pass.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2KHR.html>"]
#[doc(alias = "vkCreateRenderPass2KHR")]
pub unsafe fn create_render_pass2_khr(
    device: &raw::Device,
    p_create_info: &RenderPassCreateInfo2,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<RenderPass> {
    let vulkan_command = dispatcher
        .create_render_pass2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_render_pass = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_render_pass.as_mut_ptr(),
    );
    vk_status.map_success(|| p_render_pass.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2.html>"]
#[doc(alias = "vkCmdBeginRenderPass2")]
pub unsafe fn cmd_begin_render_pass2(
    command_buffer: &raw::CommandBuffer,
    p_render_pass_begin: &RenderPassBeginInfo,
    p_subpass_begin_info: &SubpassBeginInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_begin_render_pass2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_render_pass_begin),
        ptr::from_ref(p_subpass_begin_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2KHR.html>"]
#[doc(alias = "vkCmdBeginRenderPass2KHR")]
pub unsafe fn cmd_begin_render_pass2_khr(
    command_buffer: &raw::CommandBuffer,
    p_render_pass_begin: &RenderPassBeginInfo,
    p_subpass_begin_info: &SubpassBeginInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_begin_render_pass2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_render_pass_begin),
        ptr::from_ref(p_subpass_begin_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2.html>"]
#[doc(alias = "vkCmdNextSubpass2")]
pub unsafe fn cmd_next_subpass2(
    command_buffer: &raw::CommandBuffer,
    p_subpass_begin_info: &SubpassBeginInfo,
    p_subpass_end_info: &SubpassEndInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_next_subpass2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_subpass_begin_info),
        ptr::from_ref(p_subpass_end_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2KHR.html>"]
#[doc(alias = "vkCmdNextSubpass2KHR")]
pub unsafe fn cmd_next_subpass2_khr(
    command_buffer: &raw::CommandBuffer,
    p_subpass_begin_info: &SubpassBeginInfo,
    p_subpass_end_info: &SubpassEndInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_next_subpass2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_subpass_begin_info),
        ptr::from_ref(p_subpass_end_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2.html>"]
#[doc(alias = "vkCmdEndRenderPass2")]
pub unsafe fn cmd_end_render_pass2(
    command_buffer: &raw::CommandBuffer,
    p_subpass_end_info: &SubpassEndInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_end_render_pass2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_subpass_end_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2KHR.html>"]
#[doc(alias = "vkCmdEndRenderPass2KHR")]
pub unsafe fn cmd_end_render_pass2_khr(
    command_buffer: &raw::CommandBuffer,
    p_subpass_end_info: &SubpassEndInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_end_render_pass2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_subpass_end_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetQueryPool.html>"]
#[doc(alias = "vkResetQueryPool")]
pub unsafe fn reset_query_pool(
    device: &raw::Device,
    query_pool: &raw::QueryPool,
    first_query: u32,
    query_count: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .reset_query_pool
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { query_pool.clone() }),
        first_query,
        query_count,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetQueryPoolEXT.html>"]
#[doc(alias = "vkResetQueryPoolEXT")]
pub unsafe fn reset_query_pool_ext(
    device: &raw::Device,
    query_pool: &raw::QueryPool,
    first_query: u32,
    query_count: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .reset_query_pool_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { query_pool.clone() }),
        first_query,
        query_count,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValue.html>"]
#[doc(alias = "vkGetSemaphoreCounterValue")]
pub unsafe fn get_semaphore_counter_value(
    device: &raw::Device,
    semaphore: &raw::Semaphore,
    dispatcher: &CommandsDispatcher,
) -> Result<u64> {
    let vulkan_command = dispatcher
        .get_semaphore_counter_value
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_value = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { semaphore.clone() }),
        p_value.as_mut_ptr(),
    );
    vk_status.map_success(|| p_value.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValueKHR.html>"]
#[doc(alias = "vkGetSemaphoreCounterValueKHR")]
pub unsafe fn get_semaphore_counter_value_khr(
    device: &raw::Device,
    semaphore: &raw::Semaphore,
    dispatcher: &CommandsDispatcher,
) -> Result<u64> {
    let vulkan_command = dispatcher
        .get_semaphore_counter_value_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_value = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { semaphore.clone() }),
        p_value.as_mut_ptr(),
    );
    vk_status.map_success(|| p_value.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphores.html>"]
#[doc(alias = "vkWaitSemaphores")]
pub unsafe fn wait_semaphores(
    device: &raw::Device,
    p_wait_info: &SemaphoreWaitInfo,
    timeout: u64,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .wait_semaphores
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_wait_info),
        timeout,
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphoresKHR.html>"]
#[doc(alias = "vkWaitSemaphoresKHR")]
pub unsafe fn wait_semaphores_khr(
    device: &raw::Device,
    p_wait_info: &SemaphoreWaitInfo,
    timeout: u64,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .wait_semaphores_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_wait_info),
        timeout,
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphore.html>"]
#[doc(alias = "vkSignalSemaphore")]
pub unsafe fn signal_semaphore(
    device: &raw::Device,
    p_signal_info: &SemaphoreSignalInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .signal_semaphore
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_signal_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphoreKHR.html>"]
#[doc(alias = "vkSignalSemaphoreKHR")]
pub unsafe fn signal_semaphore_khr(
    device: &raw::Device,
    p_signal_info: &SemaphoreSignalInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .signal_semaphore_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_signal_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddress.html>"]
#[doc(alias = "vkGetBufferDeviceAddress")]
pub unsafe fn get_buffer_device_address(
    device: &raw::Device,
    p_info: &BufferDeviceAddressInfo,
    dispatcher: &CommandsDispatcher,
) -> DeviceAddress {
    let vulkan_command = dispatcher
        .get_buffer_device_address
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), ptr::from_ref(p_info))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddressKHR.html>"]
#[doc(alias = "vkGetBufferDeviceAddressKHR")]
pub unsafe fn get_buffer_device_address_khr(
    device: &raw::Device,
    p_info: &BufferDeviceAddressInfo,
    dispatcher: &CommandsDispatcher,
) -> DeviceAddress {
    let vulkan_command = dispatcher
        .get_buffer_device_address_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), ptr::from_ref(p_info))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddressEXT.html>"]
#[doc(alias = "vkGetBufferDeviceAddressEXT")]
pub unsafe fn get_buffer_device_address_ext(
    device: &raw::Device,
    p_info: &BufferDeviceAddressInfo,
    dispatcher: &CommandsDispatcher,
) -> DeviceAddress {
    let vulkan_command = dispatcher
        .get_buffer_device_address_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), ptr::from_ref(p_info))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureAddress.html>"]
#[doc(alias = "vkGetBufferOpaqueCaptureAddress")]
pub unsafe fn get_buffer_opaque_capture_address(
    device: &raw::Device,
    p_info: &BufferDeviceAddressInfo,
    dispatcher: &CommandsDispatcher,
) -> u64 {
    let vulkan_command = dispatcher
        .get_buffer_opaque_capture_address
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), ptr::from_ref(p_info))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureAddressKHR.html>"]
#[doc(alias = "vkGetBufferOpaqueCaptureAddressKHR")]
pub unsafe fn get_buffer_opaque_capture_address_khr(
    device: &raw::Device,
    p_info: &BufferDeviceAddressInfo,
    dispatcher: &CommandsDispatcher,
) -> u64 {
    let vulkan_command = dispatcher
        .get_buffer_opaque_capture_address_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), ptr::from_ref(p_info))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddress.html>"]
#[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddress")]
pub unsafe fn get_device_memory_opaque_capture_address(
    device: &raw::Device,
    p_info: &DeviceMemoryOpaqueCaptureAddressInfo,
    dispatcher: &CommandsDispatcher,
) -> u64 {
    let vulkan_command = dispatcher
        .get_device_memory_opaque_capture_address
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), ptr::from_ref(p_info))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html>"]
#[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddressKHR")]
pub unsafe fn get_device_memory_opaque_capture_address_khr(
    device: &raw::Device,
    p_info: &DeviceMemoryOpaqueCaptureAddressInfo,
    dispatcher: &CommandsDispatcher,
) -> u64 {
    let vulkan_command = dispatcher
        .get_device_memory_opaque_capture_address_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), ptr::from_ref(p_info))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolProperties.html>"]
#[doc(alias = "vkGetPhysicalDeviceToolProperties")]
pub unsafe fn get_physical_device_tool_properties<
    R: DynamicArray<PhysicalDeviceToolProperties<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_tool_properties
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_tool_count = vk_len.as_mut_ptr();
    let p_tool_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html>"]
#[doc(alias = "vkGetPhysicalDeviceToolPropertiesEXT")]
pub unsafe fn get_physical_device_tool_properties_ext<
    R: DynamicArray<PhysicalDeviceToolProperties<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_tool_properties_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_tool_count = vk_len.as_mut_ptr();
    let p_tool_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePrivateDataSlot.html>"]
#[doc(alias = "vkCreatePrivateDataSlot")]
pub unsafe fn create_private_data_slot(
    device: &raw::Device,
    p_create_info: &PrivateDataSlotCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<PrivateDataSlot> {
    let vulkan_command = dispatcher
        .create_private_data_slot
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_private_data_slot = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_private_data_slot.as_mut_ptr(),
    );
    vk_status.map_success(|| p_private_data_slot.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePrivateDataSlotEXT.html>"]
#[doc(alias = "vkCreatePrivateDataSlotEXT")]
pub unsafe fn create_private_data_slot_ext(
    device: &raw::Device,
    p_create_info: &PrivateDataSlotCreateInfo,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<PrivateDataSlot> {
    let vulkan_command = dispatcher
        .create_private_data_slot_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_private_data_slot = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_private_data_slot.as_mut_ptr(),
    );
    vk_status.map_success(|| p_private_data_slot.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPrivateDataSlot.html>"]
#[doc(alias = "vkDestroyPrivateDataSlot")]
pub unsafe fn destroy_private_data_slot(
    device: &raw::Device,
    private_data_slot: Option<&raw::PrivateDataSlot>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_private_data_slot
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        private_data_slot.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPrivateDataSlotEXT.html>"]
#[doc(alias = "vkDestroyPrivateDataSlotEXT")]
pub unsafe fn destroy_private_data_slot_ext(
    device: &raw::Device,
    private_data_slot: Option<&raw::PrivateDataSlot>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_private_data_slot_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        private_data_slot.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetPrivateData.html>"]
#[doc(alias = "vkSetPrivateData")]
pub unsafe fn set_private_data(
    device: &raw::Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: &raw::PrivateDataSlot,
    data: u64,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .set_private_data
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        object_type,
        object_handle,
        Some(unsafe { private_data_slot.clone() }),
        data,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetPrivateDataEXT.html>"]
#[doc(alias = "vkSetPrivateDataEXT")]
pub unsafe fn set_private_data_ext(
    device: &raw::Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: &raw::PrivateDataSlot,
    data: u64,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .set_private_data_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        object_type,
        object_handle,
        Some(unsafe { private_data_slot.clone() }),
        data,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPrivateData.html>"]
#[doc(alias = "vkGetPrivateData")]
pub unsafe fn get_private_data(
    device: &raw::Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: &raw::PrivateDataSlot,
    dispatcher: &CommandsDispatcher,
) -> u64 {
    let vulkan_command = dispatcher
        .get_private_data
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_data = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        object_type,
        object_handle,
        Some(unsafe { private_data_slot.clone() }),
        p_data.as_mut_ptr(),
    );
    p_data.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPrivateDataEXT.html>"]
#[doc(alias = "vkGetPrivateDataEXT")]
pub unsafe fn get_private_data_ext(
    device: &raw::Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: &raw::PrivateDataSlot,
    dispatcher: &CommandsDispatcher,
) -> u64 {
    let vulkan_command = dispatcher
        .get_private_data_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_data = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        object_type,
        object_handle,
        Some(unsafe { private_data_slot.clone() }),
        p_data.as_mut_ptr(),
    );
    p_data.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2.html>"]
#[doc(alias = "vkCmdSetEvent2")]
pub unsafe fn cmd_set_event2(
    command_buffer: &raw::CommandBuffer,
    event: &raw::Event,
    p_dependency_info: &DependencyInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_event2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { event.clone() }),
        ptr::from_ref(p_dependency_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2KHR.html>"]
#[doc(alias = "vkCmdSetEvent2KHR")]
pub unsafe fn cmd_set_event2_khr(
    command_buffer: &raw::CommandBuffer,
    event: &raw::Event,
    p_dependency_info: &DependencyInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_event2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { event.clone() }),
        ptr::from_ref(p_dependency_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2.html>"]
#[doc(alias = "vkCmdResetEvent2")]
pub unsafe fn cmd_reset_event2(
    command_buffer: &raw::CommandBuffer,
    event: &raw::Event,
    stage_mask: PipelineStageFlags2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_reset_event2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { event.clone() }),
        stage_mask,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetEvent2KHR.html>"]
#[doc(alias = "vkCmdResetEvent2KHR")]
pub unsafe fn cmd_reset_event2_khr(
    command_buffer: &raw::CommandBuffer,
    event: &raw::Event,
    stage_mask: PipelineStageFlags2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_reset_event2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { event.clone() }),
        stage_mask,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2.html>"]
#[doc(alias = "vkCmdWaitEvents2")]
pub unsafe fn cmd_wait_events2<'a, V2: Alias<raw::Event> + 'a>(
    command_buffer: &raw::CommandBuffer,
    p_events: impl AsSlice<'a, V2>,
    p_dependency_infos: impl AsSlice<'a, DependencyInfo<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_wait_events2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_dependency_infos.as_slice().len() as _,
        p_events.as_slice().as_ptr().cast(),
        p_dependency_infos.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2KHR.html>"]
#[doc(alias = "vkCmdWaitEvents2KHR")]
pub unsafe fn cmd_wait_events2_khr<'a, V2: Alias<raw::Event> + 'a>(
    command_buffer: &raw::CommandBuffer,
    p_events: impl AsSlice<'a, V2>,
    p_dependency_infos: impl AsSlice<'a, DependencyInfo<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_wait_events2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_dependency_infos.as_slice().len() as _,
        p_events.as_slice().as_ptr().cast(),
        p_dependency_infos.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2.html>"]
#[doc(alias = "vkCmdPipelineBarrier2")]
pub unsafe fn cmd_pipeline_barrier2(
    command_buffer: &raw::CommandBuffer,
    p_dependency_info: &DependencyInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_pipeline_barrier2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_dependency_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPipelineBarrier2KHR.html>"]
#[doc(alias = "vkCmdPipelineBarrier2KHR")]
pub unsafe fn cmd_pipeline_barrier2_khr(
    command_buffer: &raw::CommandBuffer,
    p_dependency_info: &DependencyInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_pipeline_barrier2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_dependency_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2.html>"]
#[doc(alias = "vkCmdWriteTimestamp2")]
pub unsafe fn cmd_write_timestamp2(
    command_buffer: &raw::CommandBuffer,
    stage: PipelineStageFlags2,
    query_pool: &raw::QueryPool,
    query: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_write_timestamp2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        stage,
        Some(unsafe { query_pool.clone() }),
        query,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteTimestamp2KHR.html>"]
#[doc(alias = "vkCmdWriteTimestamp2KHR")]
pub unsafe fn cmd_write_timestamp2_khr(
    command_buffer: &raw::CommandBuffer,
    stage: PipelineStageFlags2,
    query_pool: &raw::QueryPool,
    query: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_write_timestamp2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        stage,
        Some(unsafe { query_pool.clone() }),
        query,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2.html>"]
#[doc(alias = "vkQueueSubmit2")]
pub unsafe fn queue_submit2<'a>(
    queue: &raw::Queue,
    p_submits: impl AsSlice<'a, SubmitInfo2<'a>>,
    fence: Option<&raw::Fence>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .queue_submit2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { queue.clone() }),
        p_submits.as_slice().len() as _,
        p_submits.as_slice().as_ptr().cast(),
        fence.map(|v| unsafe { v.clone() }),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2KHR.html>"]
#[doc(alias = "vkQueueSubmit2KHR")]
pub unsafe fn queue_submit2_khr<'a>(
    queue: &raw::Queue,
    p_submits: impl AsSlice<'a, SubmitInfo2<'a>>,
    fence: Option<&raw::Fence>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .queue_submit2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { queue.clone() }),
        p_submits.as_slice().len() as _,
        p_submits.as_slice().as_ptr().cast(),
        fence.map(|v| unsafe { v.clone() }),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer2.html>"]
#[doc(alias = "vkCmdCopyBuffer2")]
pub unsafe fn cmd_copy_buffer2(
    command_buffer: &raw::CommandBuffer,
    p_copy_buffer_info: &CopyBufferInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_buffer2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_copy_buffer_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer2KHR.html>"]
#[doc(alias = "vkCmdCopyBuffer2KHR")]
pub unsafe fn cmd_copy_buffer2_khr(
    command_buffer: &raw::CommandBuffer,
    p_copy_buffer_info: &CopyBufferInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_buffer2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_copy_buffer_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage2.html>"]
#[doc(alias = "vkCmdCopyImage2")]
pub unsafe fn cmd_copy_image2(
    command_buffer: &raw::CommandBuffer,
    p_copy_image_info: &CopyImageInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_image2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_copy_image_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage2KHR.html>"]
#[doc(alias = "vkCmdCopyImage2KHR")]
pub unsafe fn cmd_copy_image2_khr(
    command_buffer: &raw::CommandBuffer,
    p_copy_image_info: &CopyImageInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_image2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_copy_image_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2.html>"]
#[doc(alias = "vkCmdCopyBufferToImage2")]
pub unsafe fn cmd_copy_buffer_to_image2(
    command_buffer: &raw::CommandBuffer,
    p_copy_buffer_to_image_info: &CopyBufferToImageInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_buffer_to_image2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_copy_buffer_to_image_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2KHR.html>"]
#[doc(alias = "vkCmdCopyBufferToImage2KHR")]
pub unsafe fn cmd_copy_buffer_to_image2_khr(
    command_buffer: &raw::CommandBuffer,
    p_copy_buffer_to_image_info: &CopyBufferToImageInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_buffer_to_image2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_copy_buffer_to_image_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2.html>"]
#[doc(alias = "vkCmdCopyImageToBuffer2")]
pub unsafe fn cmd_copy_image_to_buffer2(
    command_buffer: &raw::CommandBuffer,
    p_copy_image_to_buffer_info: &CopyImageToBufferInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_image_to_buffer2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_copy_image_to_buffer_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer2KHR.html>"]
#[doc(alias = "vkCmdCopyImageToBuffer2KHR")]
pub unsafe fn cmd_copy_image_to_buffer2_khr(
    command_buffer: &raw::CommandBuffer,
    p_copy_image_to_buffer_info: &CopyImageToBufferInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_image_to_buffer2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_copy_image_to_buffer_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage2.html>"]
#[doc(alias = "vkCmdBlitImage2")]
pub unsafe fn cmd_blit_image2(
    command_buffer: &raw::CommandBuffer,
    p_blit_image_info: &BlitImageInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_blit_image2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_blit_image_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage2KHR.html>"]
#[doc(alias = "vkCmdBlitImage2KHR")]
pub unsafe fn cmd_blit_image2_khr(
    command_buffer: &raw::CommandBuffer,
    p_blit_image_info: &BlitImageInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_blit_image2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_blit_image_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage2.html>"]
#[doc(alias = "vkCmdResolveImage2")]
pub unsafe fn cmd_resolve_image2(
    command_buffer: &raw::CommandBuffer,
    p_resolve_image_info: &ResolveImageInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_resolve_image2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_resolve_image_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage2KHR.html>"]
#[doc(alias = "vkCmdResolveImage2KHR")]
pub unsafe fn cmd_resolve_image2_khr(
    command_buffer: &raw::CommandBuffer,
    p_resolve_image_info: &ResolveImageInfo2,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_resolve_image2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_resolve_image_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRendering.html>"]
#[doc(alias = "vkCmdBeginRendering")]
pub unsafe fn cmd_begin_rendering(
    command_buffer: &raw::CommandBuffer,
    p_rendering_info: &RenderingInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_begin_rendering
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_rendering_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderingKHR.html>"]
#[doc(alias = "vkCmdBeginRenderingKHR")]
pub unsafe fn cmd_begin_rendering_khr(
    command_buffer: &raw::CommandBuffer,
    p_rendering_info: &RenderingInfo,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_begin_rendering_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_rendering_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRendering.html>"]
#[doc(alias = "vkCmdEndRendering")]
pub unsafe fn cmd_end_rendering(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_end_rendering
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderingKHR.html>"]
#[doc(alias = "vkCmdEndRenderingKHR")]
pub unsafe fn cmd_end_rendering_khr(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_end_rendering_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullMode.html>"]
#[doc(alias = "vkCmdSetCullMode")]
pub unsafe fn cmd_set_cull_mode(
    command_buffer: &raw::CommandBuffer,
    cull_mode: CullModeFlags,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_cull_mode
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), cull_mode)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullModeEXT.html>"]
#[doc(alias = "vkCmdSetCullModeEXT")]
pub unsafe fn cmd_set_cull_mode_ext(
    command_buffer: &raw::CommandBuffer,
    cull_mode: CullModeFlags,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_cull_mode_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), cull_mode)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFace.html>"]
#[doc(alias = "vkCmdSetFrontFace")]
pub unsafe fn cmd_set_front_face(
    command_buffer: &raw::CommandBuffer,
    front_face: FrontFace,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_front_face
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), front_face)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFaceEXT.html>"]
#[doc(alias = "vkCmdSetFrontFaceEXT")]
pub unsafe fn cmd_set_front_face_ext(
    command_buffer: &raw::CommandBuffer,
    front_face: FrontFace,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_front_face_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), front_face)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopology.html>"]
#[doc(alias = "vkCmdSetPrimitiveTopology")]
pub unsafe fn cmd_set_primitive_topology(
    command_buffer: &raw::CommandBuffer,
    primitive_topology: PrimitiveTopology,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_primitive_topology
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), primitive_topology)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopologyEXT.html>"]
#[doc(alias = "vkCmdSetPrimitiveTopologyEXT")]
pub unsafe fn cmd_set_primitive_topology_ext(
    command_buffer: &raw::CommandBuffer,
    primitive_topology: PrimitiveTopology,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_primitive_topology_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), primitive_topology)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCount.html>"]
#[doc(alias = "vkCmdSetViewportWithCount")]
pub unsafe fn cmd_set_viewport_with_count<'a>(
    command_buffer: &raw::CommandBuffer,
    p_viewports: impl AsSlice<'a, Viewport>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_viewport_with_count
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_viewports.as_slice().len() as _,
        p_viewports.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCountEXT.html>"]
#[doc(alias = "vkCmdSetViewportWithCountEXT")]
pub unsafe fn cmd_set_viewport_with_count_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    p_viewports: impl AsSlice<'a, Viewport>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_viewport_with_count_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_viewports.as_slice().len() as _,
        p_viewports.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCount.html>"]
#[doc(alias = "vkCmdSetScissorWithCount")]
pub unsafe fn cmd_set_scissor_with_count<'a>(
    command_buffer: &raw::CommandBuffer,
    p_scissors: impl AsSlice<'a, Rect2D>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_scissor_with_count
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_scissors.as_slice().len() as _,
        p_scissors.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCountEXT.html>"]
#[doc(alias = "vkCmdSetScissorWithCountEXT")]
pub unsafe fn cmd_set_scissor_with_count_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    p_scissors: impl AsSlice<'a, Rect2D>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_scissor_with_count_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_scissors.as_slice().len() as _,
        p_scissors.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2.html>"]
#[doc(alias = "vkCmdBindVertexBuffers2")]
pub unsafe fn cmd_bind_vertex_buffers2<'a, V3: Alias<raw::Buffer> + 'a>(
    command_buffer: &raw::CommandBuffer,
    first_binding: u32,
    p_buffers: impl AsSlice<'a, V3>,
    p_offsets: impl AsSlice<'a, DeviceSize>,
    p_sizes: Option<impl AsSlice<'a, DeviceSize>>,
    p_strides: Option<impl AsSlice<'a, DeviceSize>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_vertex_buffers2
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2EXT.html>"]
#[doc(alias = "vkCmdBindVertexBuffers2EXT")]
pub unsafe fn cmd_bind_vertex_buffers2_ext<'a, V3: Alias<raw::Buffer> + 'a>(
    command_buffer: &raw::CommandBuffer,
    first_binding: u32,
    p_buffers: impl AsSlice<'a, V3>,
    p_offsets: impl AsSlice<'a, DeviceSize>,
    p_sizes: Option<impl AsSlice<'a, DeviceSize>>,
    p_strides: Option<impl AsSlice<'a, DeviceSize>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_vertex_buffers2_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnable.html>"]
#[doc(alias = "vkCmdSetDepthTestEnable")]
pub unsafe fn cmd_set_depth_test_enable(
    command_buffer: &raw::CommandBuffer,
    depth_test_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_test_enable
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        depth_test_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnableEXT.html>"]
#[doc(alias = "vkCmdSetDepthTestEnableEXT")]
pub unsafe fn cmd_set_depth_test_enable_ext(
    command_buffer: &raw::CommandBuffer,
    depth_test_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_test_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        depth_test_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnable.html>"]
#[doc(alias = "vkCmdSetDepthWriteEnable")]
pub unsafe fn cmd_set_depth_write_enable(
    command_buffer: &raw::CommandBuffer,
    depth_write_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_write_enable
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        depth_write_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnableEXT.html>"]
#[doc(alias = "vkCmdSetDepthWriteEnableEXT")]
pub unsafe fn cmd_set_depth_write_enable_ext(
    command_buffer: &raw::CommandBuffer,
    depth_write_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_write_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        depth_write_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOp.html>"]
#[doc(alias = "vkCmdSetDepthCompareOp")]
pub unsafe fn cmd_set_depth_compare_op(
    command_buffer: &raw::CommandBuffer,
    depth_compare_op: CompareOp,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_compare_op
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), depth_compare_op)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOpEXT.html>"]
#[doc(alias = "vkCmdSetDepthCompareOpEXT")]
pub unsafe fn cmd_set_depth_compare_op_ext(
    command_buffer: &raw::CommandBuffer,
    depth_compare_op: CompareOp,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_compare_op_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), depth_compare_op)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnable.html>"]
#[doc(alias = "vkCmdSetDepthBoundsTestEnable")]
pub unsafe fn cmd_set_depth_bounds_test_enable(
    command_buffer: &raw::CommandBuffer,
    depth_bounds_test_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_bounds_test_enable
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        depth_bounds_test_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnableEXT.html>"]
#[doc(alias = "vkCmdSetDepthBoundsTestEnableEXT")]
pub unsafe fn cmd_set_depth_bounds_test_enable_ext(
    command_buffer: &raw::CommandBuffer,
    depth_bounds_test_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_bounds_test_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        depth_bounds_test_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilTestEnable.html>"]
#[doc(alias = "vkCmdSetStencilTestEnable")]
pub unsafe fn cmd_set_stencil_test_enable(
    command_buffer: &raw::CommandBuffer,
    stencil_test_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_stencil_test_enable
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        stencil_test_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilTestEnableEXT.html>"]
#[doc(alias = "vkCmdSetStencilTestEnableEXT")]
pub unsafe fn cmd_set_stencil_test_enable_ext(
    command_buffer: &raw::CommandBuffer,
    stencil_test_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_stencil_test_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        stencil_test_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilOp.html>"]
#[doc(alias = "vkCmdSetStencilOp")]
pub unsafe fn cmd_set_stencil_op(
    command_buffer: &raw::CommandBuffer,
    face_mask: StencilFaceFlags,
    fail_op: StencilOp,
    pass_op: StencilOp,
    depth_fail_op: StencilOp,
    compare_op: CompareOp,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_stencil_op
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        face_mask,
        fail_op,
        pass_op,
        depth_fail_op,
        compare_op,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilOpEXT.html>"]
#[doc(alias = "vkCmdSetStencilOpEXT")]
pub unsafe fn cmd_set_stencil_op_ext(
    command_buffer: &raw::CommandBuffer,
    face_mask: StencilFaceFlags,
    fail_op: StencilOp,
    pass_op: StencilOp,
    depth_fail_op: StencilOp,
    compare_op: CompareOp,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_stencil_op_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        face_mask,
        fail_op,
        pass_op,
        depth_fail_op,
        compare_op,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnable.html>"]
#[doc(alias = "vkCmdSetRasterizerDiscardEnable")]
pub unsafe fn cmd_set_rasterizer_discard_enable(
    command_buffer: &raw::CommandBuffer,
    rasterizer_discard_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_rasterizer_discard_enable
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        rasterizer_discard_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnableEXT.html>"]
#[doc(alias = "vkCmdSetRasterizerDiscardEnableEXT")]
pub unsafe fn cmd_set_rasterizer_discard_enable_ext(
    command_buffer: &raw::CommandBuffer,
    rasterizer_discard_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_rasterizer_discard_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        rasterizer_discard_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnable.html>"]
#[doc(alias = "vkCmdSetDepthBiasEnable")]
pub unsafe fn cmd_set_depth_bias_enable(
    command_buffer: &raw::CommandBuffer,
    depth_bias_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_bias_enable
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        depth_bias_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnableEXT.html>"]
#[doc(alias = "vkCmdSetDepthBiasEnableEXT")]
pub unsafe fn cmd_set_depth_bias_enable_ext(
    command_buffer: &raw::CommandBuffer,
    depth_bias_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_bias_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        depth_bias_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnable.html>"]
#[doc(alias = "vkCmdSetPrimitiveRestartEnable")]
pub unsafe fn cmd_set_primitive_restart_enable(
    command_buffer: &raw::CommandBuffer,
    primitive_restart_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_primitive_restart_enable
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        primitive_restart_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnableEXT.html>"]
#[doc(alias = "vkCmdSetPrimitiveRestartEnableEXT")]
pub unsafe fn cmd_set_primitive_restart_enable_ext(
    command_buffer: &raw::CommandBuffer,
    primitive_restart_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_primitive_restart_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        primitive_restart_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirements.html>"]
#[doc(alias = "vkGetDeviceBufferMemoryRequirements")]
pub unsafe fn get_device_buffer_memory_requirements<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &DeviceBufferMemoryRequirements,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_device_buffer_memory_requirements
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirementsKHR.html>"]
#[doc(alias = "vkGetDeviceBufferMemoryRequirementsKHR")]
pub unsafe fn get_device_buffer_memory_requirements_khr<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &DeviceBufferMemoryRequirements,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_device_buffer_memory_requirements_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirements.html>"]
#[doc(alias = "vkGetDeviceImageMemoryRequirements")]
pub unsafe fn get_device_image_memory_requirements<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &DeviceImageMemoryRequirements,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_device_image_memory_requirements
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirementsKHR.html>"]
#[doc(alias = "vkGetDeviceImageMemoryRequirementsKHR")]
pub unsafe fn get_device_image_memory_requirements_khr<
    S: StructureChainOut<MemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &DeviceImageMemoryRequirements,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_device_image_memory_requirements_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSparseMemoryRequirements.html>"]
#[doc(alias = "vkGetDeviceImageSparseMemoryRequirements")]
pub unsafe fn get_device_image_sparse_memory_requirements<
    R: DynamicArray<SparseImageMemoryRequirements2<'static>>,
>(
    device: &raw::Device,
    p_info: &DeviceImageMemoryRequirements,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher
        .get_device_image_sparse_memory_requirements
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_sparse_memory_requirement_count = vk_len.as_mut_ptr();
    let p_sparse_memory_requirements = ptr::null_mut();
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_sparse_memory_requirement_count = ptr::from_mut(&mut vk_len);
    let mut p_sparse_memory_requirements = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSparseMemoryRequirementsKHR.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_sparse_memory_requirement_count = vk_len.as_mut_ptr();
    let p_sparse_memory_requirements = ptr::null_mut();
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_sparse_memory_requirement_count = ptr::from_mut(&mut vk_len);
    let mut p_sparse_memory_requirements = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        p_sparse_memory_requirement_count,
        p_sparse_memory_requirements,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html>"]
#[doc(alias = "vkDestroySurfaceKHR")]
pub unsafe fn destroy_surface_khr(
    instance: &raw::Instance,
    surface: Option<&raw::SurfaceKHR>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_surface_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { instance.clone() }),
        surface.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
pub unsafe fn get_physical_device_surface_support_khr(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    surface: &raw::SurfaceKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<bool> {
    let vulkan_command = dispatcher
        .get_physical_device_surface_support_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_supported = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { physical_device.clone() }),
        queue_family_index,
        Some(unsafe { surface.clone() }),
        p_supported.as_mut_ptr(),
    );
    vk_status.map_success(|| p_supported.assume_init().into())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
pub unsafe fn get_physical_device_surface_capabilities_khr(
    physical_device: &raw::PhysicalDevice,
    surface: &raw::SurfaceKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceCapabilitiesKHR> {
    let vulkan_command = dispatcher
        .get_physical_device_surface_capabilities_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface_capabilities = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { physical_device.clone() }),
        Some(unsafe { surface.clone() }),
        p_surface_capabilities.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface_capabilities.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
pub unsafe fn get_physical_device_surface_formats_khr<R: DynamicArray<SurfaceFormatKHR>>(
    physical_device: &raw::PhysicalDevice,
    surface: Option<&raw::SurfaceKHR>,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_surface_formats_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_surface_format_count = vk_len.as_mut_ptr();
    let p_surface_formats = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        surface.map(|v| unsafe { v.clone() }),
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
            Some(unsafe { physical_device.clone() }),
            surface.map(|v| unsafe { v.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
pub unsafe fn get_physical_device_surface_present_modes_khr<R: DynamicArray<PresentModeKHR>>(
    physical_device: &raw::PhysicalDevice,
    surface: Option<&raw::SurfaceKHR>,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_surface_present_modes_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_present_mode_count = vk_len.as_mut_ptr();
    let p_present_modes = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        surface.map(|v| unsafe { v.clone() }),
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
            Some(unsafe { physical_device.clone() }),
            surface.map(|v| unsafe { v.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html>"]
#[doc(alias = "vkCreateSwapchainKHR")]
pub unsafe fn create_swapchain_khr(
    device: &raw::Device,
    p_create_info: &SwapchainCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SwapchainKHR> {
    let vulkan_command = dispatcher
        .create_swapchain_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_swapchain = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_swapchain.as_mut_ptr(),
    );
    vk_status.map_success(|| p_swapchain.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html>"]
#[doc(alias = "vkDestroySwapchainKHR")]
pub unsafe fn destroy_swapchain_khr(
    device: &raw::Device,
    swapchain: Option<&raw::SwapchainKHR>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_swapchain_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        swapchain.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html>"]
#[doc(alias = "vkGetSwapchainImagesKHR")]
pub unsafe fn get_swapchain_images_khr<R: DynamicArray<Image>>(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_swapchain_images_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_swapchain_image_count = vk_len.as_mut_ptr();
    let p_swapchain_images = ptr::null_mut();
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { swapchain.clone() }),
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
            Some(unsafe { device.clone() }),
            Some(unsafe { swapchain.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html>"]
#[doc(alias = "vkAcquireNextImageKHR")]
pub unsafe fn acquire_next_image_khr(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    timeout: u64,
    semaphore: Option<&raw::Semaphore>,
    fence: Option<&raw::Fence>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, u32)> {
    let vulkan_command = dispatcher
        .acquire_next_image_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_image_index = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { swapchain.clone() }),
        timeout,
        semaphore.map(|v| unsafe { v.clone() }),
        fence.map(|v| unsafe { v.clone() }),
        p_image_index.as_mut_ptr(),
    );
    vk_status.map_successes(|| p_image_index.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html>"]
#[doc(alias = "vkQueuePresentKHR")]
pub unsafe fn queue_present_khr(
    queue: &raw::Queue,
    p_present_info: &PresentInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .queue_present_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { queue.clone() }),
        ptr::from_ref(p_present_info),
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html>"]
#[doc(alias = "vkGetDeviceGroupPresentCapabilitiesKHR")]
pub unsafe fn get_device_group_present_capabilities_khr<
    S: StructureChainOut<DeviceGroupPresentCapabilitiesKHR<'static>>,
>(
    device: &raw::Device,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_device_group_present_capabilities_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_device_group_present_capabilities = MaybeUninit::uninit();
    S::setup_uninit(&mut p_device_group_present_capabilities);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        S::get_uninit_head_ptr(p_device_group_present_capabilities.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_device_group_present_capabilities.as_mut_ptr());
        p_device_group_present_capabilities.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html>"]
#[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
pub unsafe fn get_device_group_surface_present_modes_khr(
    device: &raw::Device,
    surface: &raw::SurfaceKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<DeviceGroupPresentModeFlagsKHR> {
    let vulkan_command = dispatcher
        .get_device_group_surface_present_modes_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_modes = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { surface.clone() }),
        p_modes.as_mut_ptr(),
    );
    vk_status.map_success(|| p_modes.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html>"]
#[doc(alias = "vkGetPhysicalDevicePresentRectanglesKHR")]
pub unsafe fn get_physical_device_present_rectangles_khr<R: DynamicArray<Rect2D>>(
    physical_device: &raw::PhysicalDevice,
    surface: &raw::SurfaceKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_present_rectangles_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_rect_count = vk_len.as_mut_ptr();
    let p_rects = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        Some(unsafe { surface.clone() }),
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
            Some(unsafe { physical_device.clone() }),
            Some(unsafe { surface.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html>"]
#[doc(alias = "vkAcquireNextImage2KHR")]
pub unsafe fn acquire_next_image2_khr(
    device: &raw::Device,
    p_acquire_info: &AcquireNextImageInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, u32)> {
    let vulkan_command = dispatcher
        .acquire_next_image2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_image_index = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_acquire_info),
        p_image_index.as_mut_ptr(),
    );
    vk_status.map_successes(|| p_image_index.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceDisplayPropertiesKHR")]
pub unsafe fn get_physical_device_display_properties_khr<
    R: DynamicArray<DisplayPropertiesKHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_display_properties_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")]
pub unsafe fn get_physical_device_display_plane_properties_khr<
    R: DynamicArray<DisplayPlanePropertiesKHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_display_plane_properties_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html>"]
#[doc(alias = "vkGetDisplayPlaneSupportedDisplaysKHR")]
pub unsafe fn get_display_plane_supported_displays_khr<R: DynamicArray<DisplayKHR>>(
    physical_device: &raw::PhysicalDevice,
    plane_index: u32,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_display_plane_supported_displays_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_display_count = vk_len.as_mut_ptr();
    let p_displays = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModePropertiesKHR.html>"]
#[doc(alias = "vkGetDisplayModePropertiesKHR")]
pub unsafe fn get_display_mode_properties_khr<
    R: DynamicArray<DisplayModePropertiesKHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    display: &raw::DisplayKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_display_mode_properties_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        Some(unsafe { display.clone() }),
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
            Some(unsafe { physical_device.clone() }),
            Some(unsafe { display.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayModeKHR.html>"]
#[doc(alias = "vkCreateDisplayModeKHR")]
pub unsafe fn create_display_mode_khr(
    physical_device: &raw::PhysicalDevice,
    display: &raw::DisplayKHR,
    p_create_info: &DisplayModeCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DisplayModeKHR> {
    let vulkan_command = dispatcher
        .create_display_mode_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_mode = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { physical_device.clone() }),
        Some(unsafe { display.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_mode.as_mut_ptr(),
    );
    vk_status.map_success(|| p_mode.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html>"]
#[doc(alias = "vkGetDisplayPlaneCapabilitiesKHR")]
pub unsafe fn get_display_plane_capabilities_khr(
    physical_device: &raw::PhysicalDevice,
    mode: &raw::DisplayModeKHR,
    plane_index: u32,
    dispatcher: &CommandsDispatcher,
) -> Result<DisplayPlaneCapabilitiesKHR> {
    let vulkan_command = dispatcher
        .get_display_plane_capabilities_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_capabilities = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { physical_device.clone() }),
        Some(unsafe { mode.clone() }),
        plane_index,
        p_capabilities.as_mut_ptr(),
    );
    vk_status.map_success(|| p_capabilities.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html>"]
#[doc(alias = "vkCreateDisplayPlaneSurfaceKHR")]
pub unsafe fn create_display_plane_surface_khr(
    instance: &raw::Instance,
    p_create_info: &DisplaySurfaceCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher
        .create_display_plane_surface_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSharedSwapchainsKHR.html>"]
#[doc(alias = "vkCreateSharedSwapchainsKHR")]
pub unsafe fn create_shared_swapchains_khr<'a, R: DynamicArray<SwapchainKHR>>(
    device: &raw::Device,
    p_create_infos: impl AsSlice<'a, SwapchainCreateInfoKHR<'a>>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .create_shared_swapchains_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_swapchains = R::create_with_capacity(p_create_infos.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateXlibSurfaceKHR.html>"]
#[doc(alias = "vkCreateXlibSurfaceKHR")]
pub unsafe fn create_xlib_surface_khr(
    instance: &raw::Instance,
    p_create_info: &XlibSurfaceCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher
        .create_xlib_surface_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceXlibPresentationSupportKHR")]
pub unsafe fn get_physical_device_xlib_presentation_support_khr(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    dpy: &VoidPtr,
    visual_id: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> bool {
    let vulkan_command = dispatcher
        .get_physical_device_xlib_presentation_support_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        queue_family_index,
        ptr::from_ref(dpy),
        visual_id,
    )
    .into()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateXcbSurfaceKHR.html>"]
#[doc(alias = "vkCreateXcbSurfaceKHR")]
pub unsafe fn create_xcb_surface_khr(
    instance: &raw::Instance,
    p_create_info: &XcbSurfaceCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher
        .create_xcb_surface_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceXcbPresentationSupportKHR")]
pub unsafe fn get_physical_device_xcb_presentation_support_khr(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    connection: &VoidPtr,
    visualid: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> bool {
    let vulkan_command = dispatcher
        .get_physical_device_xcb_presentation_support_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        queue_family_index,
        ptr::from_ref(connection),
        visualid,
    )
    .into()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateWaylandSurfaceKHR.html>"]
#[doc(alias = "vkCreateWaylandSurfaceKHR")]
pub unsafe fn create_wayland_surface_khr(
    instance: &raw::Instance,
    p_create_info: &WaylandSurfaceCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher
        .create_wayland_surface_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceWaylandPresentationSupportKHR")]
pub unsafe fn get_physical_device_wayland_presentation_support_khr(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    display: &VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> bool {
    let vulkan_command = dispatcher
        .get_physical_device_wayland_presentation_support_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        queue_family_index,
        ptr::from_ref(display),
    )
    .into()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAndroidSurfaceKHR.html>"]
#[doc(alias = "vkCreateAndroidSurfaceKHR")]
pub unsafe fn create_android_surface_khr(
    instance: &raw::Instance,
    p_create_info: &AndroidSurfaceCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher
        .create_android_surface_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateWin32SurfaceKHR.html>"]
#[doc(alias = "vkCreateWin32SurfaceKHR")]
pub unsafe fn create_win32_surface_khr(
    instance: &raw::Instance,
    p_create_info: &Win32SurfaceCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher
        .create_win32_surface_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceWin32PresentationSupportKHR")]
pub unsafe fn get_physical_device_win32_presentation_support_khr(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    dispatcher: &CommandsDispatcher,
) -> bool {
    let vulkan_command = dispatcher
        .get_physical_device_win32_presentation_support_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { physical_device.clone() }), queue_family_index).into()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugReportCallbackEXT.html>"]
#[doc(alias = "vkCreateDebugReportCallbackEXT")]
pub unsafe fn create_debug_report_callback_ext(
    instance: &raw::Instance,
    p_create_info: &DebugReportCallbackCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DebugReportCallbackEXT> {
    let vulkan_command = dispatcher
        .create_debug_report_callback_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_callback = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_callback.as_mut_ptr(),
    );
    vk_status.map_success(|| p_callback.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugReportCallbackEXT.html>"]
#[doc(alias = "vkDestroyDebugReportCallbackEXT")]
pub unsafe fn destroy_debug_report_callback_ext(
    instance: &raw::Instance,
    callback: Option<&raw::DebugReportCallbackEXT>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_debug_report_callback_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { instance.clone() }),
        callback.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugReportMessageEXT.html>"]
#[doc(alias = "vkDebugReportMessageEXT")]
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
    let vulkan_command = dispatcher
        .debug_report_message_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { instance.clone() }),
        flags,
        object_type,
        object,
        location,
        message_code,
        p_layer_prefix.as_ptr(),
        p_message.as_ptr(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html>"]
#[doc(alias = "vkDebugMarkerSetObjectTagEXT")]
pub unsafe fn debug_marker_set_object_tag_ext(
    device: &raw::Device,
    p_tag_info: &DebugMarkerObjectTagInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .debug_marker_set_object_tag_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), ptr::from_ref(p_tag_info)).map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html>"]
#[doc(alias = "vkDebugMarkerSetObjectNameEXT")]
pub unsafe fn debug_marker_set_object_name_ext(
    device: &raw::Device,
    p_name_info: &DebugMarkerObjectNameInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .debug_marker_set_object_name_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), ptr::from_ref(p_name_info)).map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerBeginEXT.html>"]
#[doc(alias = "vkCmdDebugMarkerBeginEXT")]
pub unsafe fn cmd_debug_marker_begin_ext(
    command_buffer: &raw::CommandBuffer,
    p_marker_info: &DebugMarkerMarkerInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_debug_marker_begin_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_marker_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerEndEXT.html>"]
#[doc(alias = "vkCmdDebugMarkerEndEXT")]
pub unsafe fn cmd_debug_marker_end_ext(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_debug_marker_end_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerInsertEXT.html>"]
#[doc(alias = "vkCmdDebugMarkerInsertEXT")]
pub unsafe fn cmd_debug_marker_insert_ext(
    command_buffer: &raw::CommandBuffer,
    p_marker_info: &DebugMarkerMarkerInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_debug_marker_insert_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_marker_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindTransformFeedbackBuffersEXT.html>"]
#[doc(alias = "vkCmdBindTransformFeedbackBuffersEXT")]
pub unsafe fn cmd_bind_transform_feedback_buffers_ext<'a, V3: Alias<raw::Buffer> + 'a>(
    command_buffer: &raw::CommandBuffer,
    first_binding: u32,
    p_buffers: impl AsSlice<'a, V3>,
    p_offsets: impl AsSlice<'a, DeviceSize>,
    p_sizes: Option<impl AsSlice<'a, DeviceSize>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_transform_feedback_buffers_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        first_binding,
        p_offsets.as_slice().len() as _,
        p_buffers.as_slice().as_ptr().cast(),
        p_offsets.as_slice().as_ptr().cast(),
        p_sizes
            .map(|p| p.as_slice().as_ptr().cast())
            .unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html>"]
#[doc(alias = "vkCmdBeginTransformFeedbackEXT")]
pub unsafe fn cmd_begin_transform_feedback_ext<'a, V3: Alias<raw::Buffer> + 'a>(
    command_buffer: &raw::CommandBuffer,
    first_counter_buffer: u32,
    p_counter_buffers: impl AsSlice<'a, V3>,
    p_counter_buffer_offsets: Option<impl AsSlice<'a, DeviceSize>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_begin_transform_feedback_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        first_counter_buffer,
        p_counter_buffers.as_slice().len() as _,
        p_counter_buffers.as_slice().as_ptr().cast(),
        p_counter_buffer_offsets
            .map(|p| p.as_slice().as_ptr().cast())
            .unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndTransformFeedbackEXT.html>"]
#[doc(alias = "vkCmdEndTransformFeedbackEXT")]
pub unsafe fn cmd_end_transform_feedback_ext<'a, V3: Alias<raw::Buffer> + 'a>(
    command_buffer: &raw::CommandBuffer,
    first_counter_buffer: u32,
    p_counter_buffers: impl AsSlice<'a, V3>,
    p_counter_buffer_offsets: Option<impl AsSlice<'a, DeviceSize>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_end_transform_feedback_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        first_counter_buffer,
        p_counter_buffers.as_slice().len() as _,
        p_counter_buffers.as_slice().as_ptr().cast(),
        p_counter_buffer_offsets
            .map(|p| p.as_slice().as_ptr().cast())
            .unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQueryIndexedEXT.html>"]
#[doc(alias = "vkCmdBeginQueryIndexedEXT")]
pub unsafe fn cmd_begin_query_indexed_ext(
    command_buffer: &raw::CommandBuffer,
    query_pool: &raw::QueryPool,
    query: u32,
    flags: QueryControlFlags,
    index: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_begin_query_indexed_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { query_pool.clone() }),
        query,
        flags,
        index,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQueryIndexedEXT.html>"]
#[doc(alias = "vkCmdEndQueryIndexedEXT")]
pub unsafe fn cmd_end_query_indexed_ext(
    command_buffer: &raw::CommandBuffer,
    query_pool: &raw::QueryPool,
    query: u32,
    index: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_end_query_indexed_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { query_pool.clone() }),
        query,
        index,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectByteCountEXT.html>"]
#[doc(alias = "vkCmdDrawIndirectByteCountEXT")]
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
    let vulkan_command = dispatcher
        .cmd_draw_indirect_byte_count_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        instance_count,
        first_instance,
        Some(unsafe { counter_buffer.clone() }),
        counter_buffer_offset,
        counter_offset,
        vertex_stride,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCuModuleNVX.html>"]
#[doc(alias = "vkCreateCuModuleNVX")]
pub unsafe fn create_cu_module_nvx(
    device: &raw::Device,
    p_create_info: &CuModuleCreateInfoNVX,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<CuModuleNVX> {
    let vulkan_command = dispatcher
        .create_cu_module_nvx
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_module = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_module.as_mut_ptr(),
    );
    vk_status.map_success(|| p_module.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCuFunctionNVX.html>"]
#[doc(alias = "vkCreateCuFunctionNVX")]
pub unsafe fn create_cu_function_nvx(
    device: &raw::Device,
    p_create_info: &CuFunctionCreateInfoNVX,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<CuFunctionNVX> {
    let vulkan_command = dispatcher
        .create_cu_function_nvx
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_function = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_function.as_mut_ptr(),
    );
    vk_status.map_success(|| p_function.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCuModuleNVX.html>"]
#[doc(alias = "vkDestroyCuModuleNVX")]
pub unsafe fn destroy_cu_module_nvx(
    device: &raw::Device,
    module: &raw::CuModuleNVX,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_cu_module_nvx
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { module.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCuFunctionNVX.html>"]
#[doc(alias = "vkDestroyCuFunctionNVX")]
pub unsafe fn destroy_cu_function_nvx(
    device: &raw::Device,
    function: &raw::CuFunctionNVX,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_cu_function_nvx
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { function.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCuLaunchKernelNVX.html>"]
#[doc(alias = "vkCmdCuLaunchKernelNVX")]
pub unsafe fn cmd_cu_launch_kernel_nvx(
    command_buffer: &raw::CommandBuffer,
    p_launch_info: &CuLaunchInfoNVX,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_cu_launch_kernel_nvx
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_launch_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewHandleNVX.html>"]
#[doc(alias = "vkGetImageViewHandleNVX")]
pub unsafe fn get_image_view_handle_nvx(
    device: &raw::Device,
    p_info: &ImageViewHandleInfoNVX,
    dispatcher: &CommandsDispatcher,
) -> u32 {
    let vulkan_command = dispatcher
        .get_image_view_handle_nvx
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), ptr::from_ref(p_info))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewAddressNVX.html>"]
#[doc(alias = "vkGetImageViewAddressNVX")]
pub unsafe fn get_image_view_address_nvx<
    S: StructureChainOut<ImageViewAddressPropertiesNVX<'static>>,
>(
    device: &raw::Device,
    image_view: &raw::ImageView,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_image_view_address_nvx
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { image_view.clone() }),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_properties.as_mut_ptr());
        p_properties.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderInfoAMD.html>"]
#[doc(alias = "vkGetShaderInfoAMD")]
pub unsafe fn get_shader_info_amd(
    device: &raw::Device,
    pipeline: &raw::Pipeline,
    shader_stage: ShaderStageFlags,
    info_type: ShaderInfoTypeAMD,
    p_info: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<usize> {
    let vulkan_command = dispatcher
        .get_shader_info_amd
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_info_size = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { pipeline.clone() }),
        shader_stage,
        info_type,
        p_info_size.as_mut_ptr(),
        p_info,
    );
    vk_status.map_success(|| p_info_size.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateStreamDescriptorSurfaceGGP.html>"]
#[doc(alias = "vkCreateStreamDescriptorSurfaceGGP")]
pub unsafe fn create_stream_descriptor_surface_ggp(
    instance: &raw::Instance,
    p_create_info: &StreamDescriptorSurfaceCreateInfoGGP,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher
        .create_stream_descriptor_surface_ggp
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_external_image_format_properties = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleNV.html>"]
#[doc(alias = "vkGetMemoryWin32HandleNV")]
pub unsafe fn get_memory_win32_handle_nv(
    device: &raw::Device,
    memory: &raw::DeviceMemory,
    handle_type: ExternalMemoryHandleTypeFlagsNV,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher
        .get_memory_win32_handle_nv
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_handle = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { memory.clone() }),
        handle_type,
        p_handle.as_mut_ptr(),
    );
    vk_status.map_success(|| p_handle.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateViSurfaceNN.html>"]
#[doc(alias = "vkCreateViSurfaceNN")]
pub unsafe fn create_vi_surface_nn(
    instance: &raw::Instance,
    p_create_info: &ViSurfaceCreateInfoNN,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher
        .create_vi_surface_nn
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleKHR.html>"]
#[doc(alias = "vkGetMemoryWin32HandleKHR")]
pub unsafe fn get_memory_win32_handle_khr(
    device: &raw::Device,
    p_get_win32_handle_info: &MemoryGetWin32HandleInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher
        .get_memory_win32_handle_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_handle = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_get_win32_handle_info),
        p_handle.as_mut_ptr(),
    );
    vk_status.map_success(|| p_handle.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html>"]
#[doc(alias = "vkGetMemoryWin32HandlePropertiesKHR")]
pub unsafe fn get_memory_win32_handle_properties_khr<
    S: StructureChainOut<MemoryWin32HandlePropertiesKHR<'static>>,
>(
    device: &raw::Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    handle: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_memory_win32_handle_properties_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_win32_handle_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_win32_handle_properties);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        handle_type,
        handle,
        S::get_uninit_head_ptr(p_memory_win32_handle_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_memory_win32_handle_properties.as_mut_ptr());
        p_memory_win32_handle_properties.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdKHR.html>"]
#[doc(alias = "vkGetMemoryFdKHR")]
pub unsafe fn get_memory_fd_khr(
    device: &raw::Device,
    p_get_fd_info: &MemoryGetFdInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<c_int> {
    let vulkan_command = dispatcher
        .get_memory_fd_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_fd = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_get_fd_info),
        p_fd.as_mut_ptr(),
    );
    vk_status.map_success(|| p_fd.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdPropertiesKHR.html>"]
#[doc(alias = "vkGetMemoryFdPropertiesKHR")]
pub unsafe fn get_memory_fd_properties_khr<S: StructureChainOut<MemoryFdPropertiesKHR<'static>>>(
    device: &raw::Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    fd: c_int,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_memory_fd_properties_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_fd_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_fd_properties);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        handle_type,
        fd,
        S::get_uninit_head_ptr(p_memory_fd_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_memory_fd_properties.as_mut_ptr());
        p_memory_fd_properties.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html>"]
#[doc(alias = "vkImportSemaphoreWin32HandleKHR")]
pub unsafe fn import_semaphore_win32_handle_khr(
    device: &raw::Device,
    p_import_semaphore_win32_handle_info: &ImportSemaphoreWin32HandleInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .import_semaphore_win32_handle_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_import_semaphore_win32_handle_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html>"]
#[doc(alias = "vkGetSemaphoreWin32HandleKHR")]
pub unsafe fn get_semaphore_win32_handle_khr(
    device: &raw::Device,
    p_get_win32_handle_info: &SemaphoreGetWin32HandleInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher
        .get_semaphore_win32_handle_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_handle = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_get_win32_handle_info),
        p_handle.as_mut_ptr(),
    );
    vk_status.map_success(|| p_handle.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreFdKHR.html>"]
#[doc(alias = "vkImportSemaphoreFdKHR")]
pub unsafe fn import_semaphore_fd_khr(
    device: &raw::Device,
    p_import_semaphore_fd_info: &ImportSemaphoreFdInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .import_semaphore_fd_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_import_semaphore_fd_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreFdKHR.html>"]
#[doc(alias = "vkGetSemaphoreFdKHR")]
pub unsafe fn get_semaphore_fd_khr(
    device: &raw::Device,
    p_get_fd_info: &SemaphoreGetFdInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<c_int> {
    let vulkan_command = dispatcher
        .get_semaphore_fd_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_fd = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_get_fd_info),
        p_fd.as_mut_ptr(),
    );
    vk_status.map_success(|| p_fd.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetKHR.html>"]
#[doc(alias = "vkCmdPushDescriptorSetKHR")]
pub unsafe fn cmd_push_descriptor_set_khr<'a>(
    command_buffer: &raw::CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: &raw::PipelineLayout,
    set: u32,
    p_descriptor_writes: impl AsSlice<'a, WriteDescriptorSet<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_push_descriptor_set_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        pipeline_bind_point,
        Some(unsafe { layout.clone() }),
        set,
        p_descriptor_writes.as_slice().len() as _,
        p_descriptor_writes.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html>"]
#[doc(alias = "vkCmdPushDescriptorSetWithTemplateKHR")]
pub unsafe fn cmd_push_descriptor_set_with_template_khr(
    command_buffer: &raw::CommandBuffer,
    descriptor_update_template: &raw::DescriptorUpdateTemplate,
    layout: &raw::PipelineLayout,
    set: u32,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_push_descriptor_set_with_template_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { descriptor_update_template.clone() }),
        Some(unsafe { layout.clone() }),
        set,
        p_data,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html>"]
#[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
pub unsafe fn cmd_begin_conditional_rendering_ext(
    command_buffer: &raw::CommandBuffer,
    p_conditional_rendering_begin: &ConditionalRenderingBeginInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_begin_conditional_rendering_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_conditional_rendering_begin),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndConditionalRenderingEXT.html>"]
#[doc(alias = "vkCmdEndConditionalRenderingEXT")]
pub unsafe fn cmd_end_conditional_rendering_ext(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_end_conditional_rendering_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingNV.html>"]
#[doc(alias = "vkCmdSetViewportWScalingNV")]
pub unsafe fn cmd_set_viewport_wscaling_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    first_viewport: u32,
    p_viewport_wscalings: impl AsSlice<'a, ViewportWScalingNV>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_viewport_wscaling_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        first_viewport,
        p_viewport_wscalings.as_slice().len() as _,
        p_viewport_wscalings.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseDisplayEXT.html>"]
#[doc(alias = "vkReleaseDisplayEXT")]
pub unsafe fn release_display_ext(
    physical_device: &raw::PhysicalDevice,
    display: &raw::DisplayKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .release_display_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        Some(unsafe { display.clone() }),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireXlibDisplayEXT.html>"]
#[doc(alias = "vkAcquireXlibDisplayEXT")]
pub unsafe fn acquire_xlib_display_ext(
    physical_device: &raw::PhysicalDevice,
    dpy: &VoidPtr,
    display: &raw::DisplayKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .acquire_xlib_display_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(dpy),
        Some(unsafe { display.clone() }),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRandROutputDisplayEXT.html>"]
#[doc(alias = "vkGetRandROutputDisplayEXT")]
pub unsafe fn get_rand_routput_display_ext(
    physical_device: &raw::PhysicalDevice,
    dpy: &VoidPtr,
    rr_output: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<DisplayKHR> {
    let vulkan_command = dispatcher
        .get_rand_routput_display_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_display = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(dpy),
        rr_output,
        p_display.as_mut_ptr(),
    );
    vk_status.map_success(|| p_display.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface_capabilities = MaybeUninit::uninit();
    S::setup_uninit(&mut p_surface_capabilities);
    let vk_status = vulkan_command(
        Some(unsafe { physical_device.clone() }),
        Some(unsafe { surface.clone() }),
        S::get_uninit_head_ptr(p_surface_capabilities.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_surface_capabilities.as_mut_ptr());
        p_surface_capabilities.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDisplayPowerControlEXT.html>"]
#[doc(alias = "vkDisplayPowerControlEXT")]
pub unsafe fn display_power_control_ext(
    device: &raw::Device,
    display: &raw::DisplayKHR,
    p_display_power_info: &DisplayPowerInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .display_power_control_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { display.clone() }),
        ptr::from_ref(p_display_power_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkRegisterDeviceEventEXT.html>"]
#[doc(alias = "vkRegisterDeviceEventEXT")]
pub unsafe fn register_device_event_ext(
    device: &raw::Device,
    p_device_event_info: &DeviceEventInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Fence> {
    let vulkan_command = dispatcher
        .register_device_event_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_fence = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_device_event_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_fence.as_mut_ptr(),
    );
    vk_status.map_success(|| p_fence.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkRegisterDisplayEventEXT.html>"]
#[doc(alias = "vkRegisterDisplayEventEXT")]
pub unsafe fn register_display_event_ext(
    device: &raw::Device,
    display: &raw::DisplayKHR,
    p_display_event_info: &DisplayEventInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<Fence> {
    let vulkan_command = dispatcher
        .register_display_event_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_fence = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { display.clone() }),
        ptr::from_ref(p_display_event_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_fence.as_mut_ptr(),
    );
    vk_status.map_success(|| p_fence.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainCounterEXT.html>"]
#[doc(alias = "vkGetSwapchainCounterEXT")]
pub unsafe fn get_swapchain_counter_ext(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    counter: SurfaceCounterFlagsEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<u64> {
    let vulkan_command = dispatcher
        .get_swapchain_counter_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_counter_value = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { swapchain.clone() }),
        counter,
        p_counter_value.as_mut_ptr(),
    );
    vk_status.map_success(|| p_counter_value.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html>"]
#[doc(alias = "vkGetRefreshCycleDurationGOOGLE")]
pub unsafe fn get_refresh_cycle_duration_google(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<RefreshCycleDurationGOOGLE> {
    let vulkan_command = dispatcher
        .get_refresh_cycle_duration_google
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_display_timing_properties = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { swapchain.clone() }),
        p_display_timing_properties.as_mut_ptr(),
    );
    vk_status.map_success(|| p_display_timing_properties.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html>"]
#[doc(alias = "vkGetPastPresentationTimingGOOGLE")]
pub unsafe fn get_past_presentation_timing_google<R: DynamicArray<PastPresentationTimingGOOGLE>>(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_past_presentation_timing_google
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_presentation_timing_count = vk_len.as_mut_ptr();
    let p_presentation_timings = ptr::null_mut();
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { swapchain.clone() }),
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
            Some(unsafe { device.clone() }),
            Some(unsafe { swapchain.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEXT.html>"]
#[doc(alias = "vkCmdSetDiscardRectangleEXT")]
pub unsafe fn cmd_set_discard_rectangle_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    first_discard_rectangle: u32,
    p_discard_rectangles: impl AsSlice<'a, Rect2D>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_discard_rectangle_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        first_discard_rectangle,
        p_discard_rectangles.as_slice().len() as _,
        p_discard_rectangles.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEnableEXT.html>"]
#[doc(alias = "vkCmdSetDiscardRectangleEnableEXT")]
pub unsafe fn cmd_set_discard_rectangle_enable_ext(
    command_buffer: &raw::CommandBuffer,
    discard_rectangle_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_discard_rectangle_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        discard_rectangle_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleModeEXT.html>"]
#[doc(alias = "vkCmdSetDiscardRectangleModeEXT")]
pub unsafe fn cmd_set_discard_rectangle_mode_ext(
    command_buffer: &raw::CommandBuffer,
    discard_rectangle_mode: DiscardRectangleModeEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_discard_rectangle_mode_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        discard_rectangle_mode,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetHdrMetadataEXT.html>"]
#[doc(alias = "vkSetHdrMetadataEXT")]
pub unsafe fn set_hdr_metadata_ext<'a, V2: Alias<raw::SwapchainKHR> + 'a>(
    device: &raw::Device,
    p_swapchains: impl AsSlice<'a, V2>,
    p_metadata: impl AsSlice<'a, HdrMetadataEXT<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .set_hdr_metadata_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        p_metadata.as_slice().len() as _,
        p_swapchains.as_slice().as_ptr().cast(),
        p_metadata.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainStatusKHR.html>"]
#[doc(alias = "vkGetSwapchainStatusKHR")]
pub unsafe fn get_swapchain_status_khr(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .get_swapchain_status_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { swapchain.clone() }),
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceWin32HandleKHR.html>"]
#[doc(alias = "vkImportFenceWin32HandleKHR")]
pub unsafe fn import_fence_win32_handle_khr(
    device: &raw::Device,
    p_import_fence_win32_handle_info: &ImportFenceWin32HandleInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .import_fence_win32_handle_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_import_fence_win32_handle_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceWin32HandleKHR.html>"]
#[doc(alias = "vkGetFenceWin32HandleKHR")]
pub unsafe fn get_fence_win32_handle_khr(
    device: &raw::Device,
    p_get_win32_handle_info: &FenceGetWin32HandleInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher
        .get_fence_win32_handle_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_handle = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_get_win32_handle_info),
        p_handle.as_mut_ptr(),
    );
    vk_status.map_success(|| p_handle.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceFdKHR.html>"]
#[doc(alias = "vkImportFenceFdKHR")]
pub unsafe fn import_fence_fd_khr(
    device: &raw::Device,
    p_import_fence_fd_info: &ImportFenceFdInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .import_fence_fd_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_import_fence_fd_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceFdKHR.html>"]
#[doc(alias = "vkGetFenceFdKHR")]
pub unsafe fn get_fence_fd_khr(
    device: &raw::Device,
    p_get_fd_info: &FenceGetFdInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<c_int> {
    let vulkan_command = dispatcher
        .get_fence_fd_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_fd = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_get_fd_info),
        p_fd.as_mut_ptr(),
    );
    vk_status.map_success(|| p_fd.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")]
pub unsafe fn get_physical_device_queue_family_performance_query_passes_khr(
    physical_device: &raw::PhysicalDevice,
    p_performance_query_create_info: &QueryPoolPerformanceCreateInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> u32 {
    let vulkan_command = dispatcher
        .get_physical_device_queue_family_performance_query_passes_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_num_passes = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(p_performance_query_create_info),
        p_num_passes.as_mut_ptr(),
    );
    p_num_passes.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireProfilingLockKHR.html>"]
#[doc(alias = "vkAcquireProfilingLockKHR")]
pub unsafe fn acquire_profiling_lock_khr(
    device: &raw::Device,
    p_info: &AcquireProfilingLockInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .acquire_profiling_lock_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), ptr::from_ref(p_info)).map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseProfilingLockKHR.html>"]
#[doc(alias = "vkReleaseProfilingLockKHR")]
pub unsafe fn release_profiling_lock_khr(device: &raw::Device, dispatcher: &CommandsDispatcher) {
    let vulkan_command = dispatcher
        .release_profiling_lock_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface_capabilities = MaybeUninit::uninit();
    S::setup_uninit(&mut p_surface_capabilities);
    let vk_status = vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(p_surface_info),
        S::get_uninit_head_ptr(p_surface_capabilities.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_surface_capabilities.as_mut_ptr());
        p_surface_capabilities.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceSurfaceFormats2KHR")]
pub unsafe fn get_physical_device_surface_formats2_khr<
    R: DynamicArray<SurfaceFormat2KHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_surface_formats2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_surface_format_count = vk_len.as_mut_ptr();
    let p_surface_formats = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
pub unsafe fn get_physical_device_display_properties2_khr<
    R: DynamicArray<DisplayProperties2KHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_display_properties2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
pub unsafe fn get_physical_device_display_plane_properties2_khr<
    R: DynamicArray<DisplayPlaneProperties2KHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_display_plane_properties2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModeProperties2KHR.html>"]
#[doc(alias = "vkGetDisplayModeProperties2KHR")]
pub unsafe fn get_display_mode_properties2_khr<
    R: DynamicArray<DisplayModeProperties2KHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    display: &raw::DisplayKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_display_mode_properties2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        Some(unsafe { display.clone() }),
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
            Some(unsafe { physical_device.clone() }),
            Some(unsafe { display.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html>"]
#[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
pub unsafe fn get_display_plane_capabilities2_khr<
    S: StructureChainOut<DisplayPlaneCapabilities2KHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    p_display_plane_info: &DisplayPlaneInfo2KHR,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_display_plane_capabilities2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_capabilities = MaybeUninit::uninit();
    S::setup_uninit(&mut p_capabilities);
    let vk_status = vulkan_command(
        Some(unsafe { physical_device.clone() }),
        ptr::from_ref(p_display_plane_info),
        S::get_uninit_head_ptr(p_capabilities.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_capabilities.as_mut_ptr());
        p_capabilities.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateIOSSurfaceMVK.html>"]
#[doc(alias = "vkCreateIOSSurfaceMVK")]
pub unsafe fn create_iossurface_mvk(
    instance: &raw::Instance,
    p_create_info: &IOSSurfaceCreateInfoMVK,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher
        .create_iossurface_mvk
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMacOSSurfaceMVK.html>"]
#[doc(alias = "vkCreateMacOSSurfaceMVK")]
pub unsafe fn create_mac_ossurface_mvk(
    instance: &raw::Instance,
    p_create_info: &MacOSSurfaceCreateInfoMVK,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher
        .create_mac_ossurface_mvk
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html>"]
#[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
pub unsafe fn set_debug_utils_object_name_ext(
    device: &raw::Device,
    p_name_info: &DebugUtilsObjectNameInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .set_debug_utils_object_name_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), ptr::from_ref(p_name_info)).map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html>"]
#[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
pub unsafe fn set_debug_utils_object_tag_ext(
    device: &raw::Device,
    p_tag_info: &DebugUtilsObjectTagInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .set_debug_utils_object_tag_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), ptr::from_ref(p_tag_info)).map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html>"]
#[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
pub unsafe fn queue_begin_debug_utils_label_ext(
    queue: &raw::Queue,
    p_label_info: &DebugUtilsLabelEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .queue_begin_debug_utils_label_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { queue.clone() }), ptr::from_ref(p_label_info))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html>"]
#[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
pub unsafe fn queue_end_debug_utils_label_ext(queue: &raw::Queue, dispatcher: &CommandsDispatcher) {
    let vulkan_command = dispatcher
        .queue_end_debug_utils_label_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { queue.clone() }))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html>"]
#[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
pub unsafe fn queue_insert_debug_utils_label_ext(
    queue: &raw::Queue,
    p_label_info: &DebugUtilsLabelEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .queue_insert_debug_utils_label_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { queue.clone() }), ptr::from_ref(p_label_info))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html>"]
#[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
pub unsafe fn cmd_begin_debug_utils_label_ext(
    command_buffer: &raw::CommandBuffer,
    p_label_info: &DebugUtilsLabelEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_begin_debug_utils_label_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_label_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html>"]
#[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
pub unsafe fn cmd_end_debug_utils_label_ext(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_end_debug_utils_label_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html>"]
#[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
pub unsafe fn cmd_insert_debug_utils_label_ext(
    command_buffer: &raw::CommandBuffer,
    p_label_info: &DebugUtilsLabelEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_insert_debug_utils_label_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_label_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html>"]
#[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
pub unsafe fn create_debug_utils_messenger_ext(
    instance: &raw::Instance,
    p_create_info: &DebugUtilsMessengerCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DebugUtilsMessengerEXT> {
    let vulkan_command = dispatcher
        .create_debug_utils_messenger_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_messenger = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_messenger.as_mut_ptr(),
    );
    vk_status.map_success(|| p_messenger.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html>"]
#[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
pub unsafe fn destroy_debug_utils_messenger_ext(
    instance: &raw::Instance,
    messenger: Option<&raw::DebugUtilsMessengerEXT>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_debug_utils_messenger_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { instance.clone() }),
        messenger.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html>"]
#[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
pub unsafe fn submit_debug_utils_message_ext(
    instance: &raw::Instance,
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: &DebugUtilsMessengerCallbackDataEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .submit_debug_utils_message_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { instance.clone() }),
        message_severity,
        message_types,
        ptr::from_ref(p_callback_data),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(buffer),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_properties.as_mut_ptr());
        p_properties.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html>"]
#[doc(alias = "vkGetMemoryAndroidHardwareBufferANDROID")]
pub unsafe fn get_memory_android_hardware_buffer_android(
    device: &raw::Device,
    p_info: &MemoryGetAndroidHardwareBufferInfoANDROID,
    p_buffer: &&AHardwareBuffer,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_memory_android_hardware_buffer_android
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        ptr::from_ref(p_buffer).cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateExecutionGraphPipelinesAMDX.html>"]
#[doc(alias = "vkCreateExecutionGraphPipelinesAMDX")]
pub unsafe fn create_execution_graph_pipelines_amdx<'a, R: DynamicArray<Pipeline>>(
    device: &raw::Device,
    pipeline_cache: Option<&raw::PipelineCache>,
    p_create_infos: impl AsSlice<'a, ExecutionGraphPipelineCreateInfoAMDX<'a>>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, R)> {
    let vulkan_command = dispatcher
        .create_execution_graph_pipelines_amdx
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_pipelines = R::create_with_capacity(p_create_infos.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        pipeline_cache.map(|v| unsafe { v.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetExecutionGraphPipelineScratchSizeAMDX.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_size_info = MaybeUninit::uninit();
    S::setup_uninit(&mut p_size_info);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { execution_graph.clone() }),
        S::get_uninit_head_ptr(p_size_info.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_size_info.as_mut_ptr());
        p_size_info.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetExecutionGraphPipelineNodeIndexAMDX.html>"]
#[doc(alias = "vkGetExecutionGraphPipelineNodeIndexAMDX")]
pub unsafe fn get_execution_graph_pipeline_node_index_amdx(
    device: &raw::Device,
    execution_graph: &raw::Pipeline,
    p_node_info: &PipelineShaderStageNodeCreateInfoAMDX,
    dispatcher: &CommandsDispatcher,
) -> Result<u32> {
    let vulkan_command = dispatcher
        .get_execution_graph_pipeline_node_index_amdx
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_node_index = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { execution_graph.clone() }),
        ptr::from_ref(p_node_info),
        p_node_index.as_mut_ptr(),
    );
    vk_status.map_success(|| p_node_index.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdInitializeGraphScratchMemoryAMDX.html>"]
#[doc(alias = "vkCmdInitializeGraphScratchMemoryAMDX")]
pub unsafe fn cmd_initialize_graph_scratch_memory_amdx(
    command_buffer: &raw::CommandBuffer,
    scratch: DeviceAddress,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_initialize_graph_scratch_memory_amdx
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), scratch)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchGraphAMDX.html>"]
#[doc(alias = "vkCmdDispatchGraphAMDX")]
pub unsafe fn cmd_dispatch_graph_amdx(
    command_buffer: &raw::CommandBuffer,
    scratch: DeviceAddress,
    p_count_info: &DispatchGraphCountInfoAMDX,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_dispatch_graph_amdx
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        scratch,
        ptr::from_ref(p_count_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchGraphIndirectAMDX.html>"]
#[doc(alias = "vkCmdDispatchGraphIndirectAMDX")]
pub unsafe fn cmd_dispatch_graph_indirect_amdx(
    command_buffer: &raw::CommandBuffer,
    scratch: DeviceAddress,
    p_count_info: &DispatchGraphCountInfoAMDX,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_dispatch_graph_indirect_amdx
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        scratch,
        ptr::from_ref(p_count_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchGraphIndirectCountAMDX.html>"]
#[doc(alias = "vkCmdDispatchGraphIndirectCountAMDX")]
pub unsafe fn cmd_dispatch_graph_indirect_count_amdx(
    command_buffer: &raw::CommandBuffer,
    scratch: DeviceAddress,
    count_info: DeviceAddress,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_dispatch_graph_indirect_count_amdx
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), scratch, count_info)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEXT.html>"]
#[doc(alias = "vkCmdSetSampleLocationsEXT")]
pub unsafe fn cmd_set_sample_locations_ext(
    command_buffer: &raw::CommandBuffer,
    p_sample_locations_info: &SampleLocationsInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_sample_locations_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_sample_locations_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_multisample_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_multisample_properties);
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        samples,
        S::get_uninit_head_ptr(p_multisample_properties.as_mut_ptr()),
    );
    S::setup_cleanup(p_multisample_properties.as_mut_ptr());
    p_multisample_properties.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureKHR.html>"]
#[doc(alias = "vkCreateAccelerationStructureKHR")]
pub unsafe fn create_acceleration_structure_khr(
    device: &raw::Device,
    p_create_info: &AccelerationStructureCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<AccelerationStructureKHR> {
    let vulkan_command = dispatcher
        .create_acceleration_structure_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_acceleration_structure = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_acceleration_structure.as_mut_ptr(),
    );
    vk_status.map_success(|| p_acceleration_structure.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureKHR.html>"]
#[doc(alias = "vkDestroyAccelerationStructureKHR")]
pub unsafe fn destroy_acceleration_structure_khr(
    device: &raw::Device,
    acceleration_structure: Option<&raw::AccelerationStructureKHR>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_acceleration_structure_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        acceleration_structure.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructuresKHR.html>"]
#[doc(alias = "vkCmdBuildAccelerationStructuresKHR")]
pub unsafe fn cmd_build_acceleration_structures_khr<'a>(
    command_buffer: &raw::CommandBuffer,
    p_infos: impl AsSlice<'a, AccelerationStructureBuildGeometryInfoKHR<'a>>,
    pp_build_range_infos: &&AccelerationStructureBuildRangeInfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_build_acceleration_structures_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_infos.as_slice().len() as _,
        p_infos.as_slice().as_ptr().cast(),
        ptr::from_ref(pp_build_range_infos).cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructuresIndirectKHR.html>"]
#[doc(alias = "vkCmdBuildAccelerationStructuresIndirectKHR")]
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
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_indirect_strides.as_slice().len() as _,
        p_infos.as_slice().as_ptr().cast(),
        p_indirect_device_addresses.as_slice().as_ptr().cast(),
        p_indirect_strides.as_slice().as_ptr().cast(),
        ptr::from_ref(pp_max_primitive_counts).cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBuildAccelerationStructuresKHR.html>"]
#[doc(alias = "vkBuildAccelerationStructuresKHR")]
pub unsafe fn build_acceleration_structures_khr<'a>(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    p_infos: impl AsSlice<'a, AccelerationStructureBuildGeometryInfoKHR<'a>>,
    pp_build_range_infos: &&AccelerationStructureBuildRangeInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .build_acceleration_structures_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        deferred_operation.map(|v| unsafe { v.clone() }),
        p_infos.as_slice().len() as _,
        p_infos.as_slice().as_ptr().cast(),
        ptr::from_ref(pp_build_range_infos).cast(),
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureKHR.html>"]
#[doc(alias = "vkCopyAccelerationStructureKHR")]
pub unsafe fn copy_acceleration_structure_khr(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    p_info: &CopyAccelerationStructureInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .copy_acceleration_structure_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        deferred_operation.map(|v| unsafe { v.clone() }),
        ptr::from_ref(p_info),
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureToMemoryKHR.html>"]
#[doc(alias = "vkCopyAccelerationStructureToMemoryKHR")]
pub unsafe fn copy_acceleration_structure_to_memory_khr(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    p_info: &CopyAccelerationStructureToMemoryInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .copy_acceleration_structure_to_memory_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        deferred_operation.map(|v| unsafe { v.clone() }),
        ptr::from_ref(p_info),
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToAccelerationStructureKHR.html>"]
#[doc(alias = "vkCopyMemoryToAccelerationStructureKHR")]
pub unsafe fn copy_memory_to_acceleration_structure_khr(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    p_info: &CopyMemoryToAccelerationStructureInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .copy_memory_to_acceleration_structure_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        deferred_operation.map(|v| unsafe { v.clone() }),
        ptr::from_ref(p_info),
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html>"]
#[doc(alias = "vkWriteAccelerationStructuresPropertiesKHR")]
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
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        p_acceleration_structures.as_slice().len() as _,
        p_acceleration_structures.as_slice().as_ptr().cast(),
        query_type,
        data_size,
        p_data,
        stride,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html>"]
#[doc(alias = "vkCmdCopyAccelerationStructureKHR")]
pub unsafe fn cmd_copy_acceleration_structure_khr(
    command_buffer: &raw::CommandBuffer,
    p_info: &CopyAccelerationStructureInfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_acceleration_structure_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html>"]
#[doc(alias = "vkCmdCopyAccelerationStructureToMemoryKHR")]
pub unsafe fn cmd_copy_acceleration_structure_to_memory_khr(
    command_buffer: &raw::CommandBuffer,
    p_info: &CopyAccelerationStructureToMemoryInfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_acceleration_structure_to_memory_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html>"]
#[doc(alias = "vkCmdCopyMemoryToAccelerationStructureKHR")]
pub unsafe fn cmd_copy_memory_to_acceleration_structure_khr(
    command_buffer: &raw::CommandBuffer,
    p_info: &CopyMemoryToAccelerationStructureInfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_memory_to_acceleration_structure_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html>"]
#[doc(alias = "vkGetAccelerationStructureDeviceAddressKHR")]
pub unsafe fn get_acceleration_structure_device_address_khr(
    device: &raw::Device,
    p_info: &AccelerationStructureDeviceAddressInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> DeviceAddress {
    let vulkan_command = dispatcher
        .get_acceleration_structure_device_address_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), ptr::from_ref(p_info))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html>"]
#[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesKHR")]
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
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_acceleration_structures.as_slice().len() as _,
        p_acceleration_structures.as_slice().as_ptr().cast(),
        query_type,
        Some(unsafe { query_pool.clone() }),
        first_query,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html>"]
#[doc(alias = "vkGetDeviceAccelerationStructureCompatibilityKHR")]
pub unsafe fn get_device_acceleration_structure_compatibility_khr(
    device: &raw::Device,
    p_version_info: &AccelerationStructureVersionInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> AccelerationStructureCompatibilityKHR {
    let vulkan_command = dispatcher
        .get_device_acceleration_structure_compatibility_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_compatibility = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_version_info),
        p_compatibility.as_mut_ptr(),
    );
    p_compatibility.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureBuildSizesKHR.html>"]
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
    let vulkan_command = dispatcher
        .get_acceleration_structure_build_sizes_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_size_info = MaybeUninit::uninit();
    S::setup_uninit(&mut p_size_info);
    vulkan_command(
        Some(unsafe { device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysKHR.html>"]
#[doc(alias = "vkCmdTraceRaysKHR")]
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
    let vulkan_command = dispatcher
        .cmd_trace_rays_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_raygen_shader_binding_table),
        ptr::from_ref(p_miss_shader_binding_table),
        ptr::from_ref(p_hit_shader_binding_table),
        ptr::from_ref(p_callable_shader_binding_table),
        width,
        height,
        depth,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesKHR.html>"]
#[doc(alias = "vkCreateRayTracingPipelinesKHR")]
pub unsafe fn create_ray_tracing_pipelines_khr<'a, R: DynamicArray<Pipeline>>(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    pipeline_cache: Option<&raw::PipelineCache>,
    p_create_infos: impl AsSlice<'a, RayTracingPipelineCreateInfoKHR<'a>>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, R)> {
    let vulkan_command = dispatcher
        .create_ray_tracing_pipelines_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_pipelines = R::create_with_capacity(p_create_infos.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        deferred_operation.map(|v| unsafe { v.clone() }),
        pipeline_cache.map(|v| unsafe { v.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html>"]
#[doc(alias = "vkGetRayTracingShaderGroupHandlesKHR")]
pub unsafe fn get_ray_tracing_shader_group_handles_khr(
    device: &raw::Device,
    pipeline: &raw::Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_ray_tracing_shader_group_handles_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { pipeline.clone() }),
        first_group,
        group_count,
        data_size,
        p_data,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesNV.html>"]
#[doc(alias = "vkGetRayTracingShaderGroupHandlesNV")]
pub unsafe fn get_ray_tracing_shader_group_handles_nv(
    device: &raw::Device,
    pipeline: &raw::Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_ray_tracing_shader_group_handles_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { pipeline.clone() }),
        first_group,
        group_count,
        data_size,
        p_data,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html>"]
#[doc(alias = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")]
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
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { pipeline.clone() }),
        first_group,
        group_count,
        data_size,
        p_data,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirectKHR.html>"]
#[doc(alias = "vkCmdTraceRaysIndirectKHR")]
pub unsafe fn cmd_trace_rays_indirect_khr(
    command_buffer: &raw::CommandBuffer,
    p_raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
    p_miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
    p_hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
    p_callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
    indirect_device_address: DeviceAddress,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_trace_rays_indirect_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_raygen_shader_binding_table),
        ptr::from_ref(p_miss_shader_binding_table),
        ptr::from_ref(p_hit_shader_binding_table),
        ptr::from_ref(p_callable_shader_binding_table),
        indirect_device_address,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html>"]
#[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
pub unsafe fn get_ray_tracing_shader_group_stack_size_khr(
    device: &raw::Device,
    pipeline: &raw::Pipeline,
    group: u32,
    group_shader: ShaderGroupShaderKHR,
    dispatcher: &CommandsDispatcher,
) -> DeviceSize {
    let vulkan_command = dispatcher
        .get_ray_tracing_shader_group_stack_size_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { pipeline.clone() }),
        group,
        group_shader,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html>"]
#[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
pub unsafe fn cmd_set_ray_tracing_pipeline_stack_size_khr(
    command_buffer: &raw::CommandBuffer,
    pipeline_stack_size: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_ray_tracing_pipeline_stack_size_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), pipeline_stack_size)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { image.clone() }),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_properties.as_mut_ptr());
        p_properties.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateValidationCacheEXT.html>"]
#[doc(alias = "vkCreateValidationCacheEXT")]
pub unsafe fn create_validation_cache_ext(
    device: &raw::Device,
    p_create_info: &ValidationCacheCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<ValidationCacheEXT> {
    let vulkan_command = dispatcher
        .create_validation_cache_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_validation_cache = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_validation_cache.as_mut_ptr(),
    );
    vk_status.map_success(|| p_validation_cache.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyValidationCacheEXT.html>"]
#[doc(alias = "vkDestroyValidationCacheEXT")]
pub unsafe fn destroy_validation_cache_ext(
    device: &raw::Device,
    validation_cache: Option<&raw::ValidationCacheEXT>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_validation_cache_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        validation_cache.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMergeValidationCachesEXT.html>"]
#[doc(alias = "vkMergeValidationCachesEXT")]
pub unsafe fn merge_validation_caches_ext<'a, V3: Alias<raw::ValidationCacheEXT> + 'a>(
    device: &raw::Device,
    dst_cache: &raw::ValidationCacheEXT,
    p_src_caches: impl AsSlice<'a, V3>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .merge_validation_caches_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { dst_cache.clone() }),
        p_src_caches.as_slice().len() as _,
        p_src_caches.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetValidationCacheDataEXT.html>"]
#[doc(alias = "vkGetValidationCacheDataEXT")]
pub unsafe fn get_validation_cache_data_ext(
    device: &raw::Device,
    validation_cache: &raw::ValidationCacheEXT,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<usize> {
    let vulkan_command = dispatcher
        .get_validation_cache_data_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_data_size = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { validation_cache.clone() }),
        p_data_size.as_mut_ptr(),
        p_data,
    );
    vk_status.map_success(|| p_data_size.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindShadingRateImageNV.html>"]
#[doc(alias = "vkCmdBindShadingRateImageNV")]
pub unsafe fn cmd_bind_shading_rate_image_nv(
    command_buffer: &raw::CommandBuffer,
    image_view: Option<&raw::ImageView>,
    image_layout: ImageLayout,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_shading_rate_image_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        image_view.map(|v| unsafe { v.clone() }),
        image_layout,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportShadingRatePaletteNV.html>"]
#[doc(alias = "vkCmdSetViewportShadingRatePaletteNV")]
pub unsafe fn cmd_set_viewport_shading_rate_palette_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    first_viewport: u32,
    p_shading_rate_palettes: impl AsSlice<'a, ShadingRatePaletteNV<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_viewport_shading_rate_palette_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        first_viewport,
        p_shading_rate_palettes.as_slice().len() as _,
        p_shading_rate_palettes.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoarseSampleOrderNV.html>"]
#[doc(alias = "vkCmdSetCoarseSampleOrderNV")]
pub unsafe fn cmd_set_coarse_sample_order_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    sample_order_type: CoarseSampleOrderTypeNV,
    p_custom_sample_orders: impl AsSlice<'a, CoarseSampleOrderCustomNV<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_coarse_sample_order_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        sample_order_type,
        p_custom_sample_orders.as_slice().len() as _,
        p_custom_sample_orders.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureNV.html>"]
#[doc(alias = "vkCreateAccelerationStructureNV")]
pub unsafe fn create_acceleration_structure_nv(
    device: &raw::Device,
    p_create_info: &AccelerationStructureCreateInfoNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<AccelerationStructureNV> {
    let vulkan_command = dispatcher
        .create_acceleration_structure_nv
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_acceleration_structure = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_acceleration_structure.as_mut_ptr(),
    );
    vk_status.map_success(|| p_acceleration_structure.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureNV.html>"]
#[doc(alias = "vkDestroyAccelerationStructureNV")]
pub unsafe fn destroy_acceleration_structure_nv(
    device: &raw::Device,
    acceleration_structure: Option<&raw::AccelerationStructureNV>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_acceleration_structure_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        acceleration_structure.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html>"]
#[doc(alias = "vkGetAccelerationStructureMemoryRequirementsNV")]
pub unsafe fn get_acceleration_structure_memory_requirements_nv<
    S: StructureChainOut<MemoryRequirements2KHR<'static>>,
>(
    device: &raw::Device,
    p_info: &AccelerationStructureMemoryRequirementsInfoNV,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_acceleration_structure_memory_requirements_nv
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindAccelerationStructureMemoryNV.html>"]
#[doc(alias = "vkBindAccelerationStructureMemoryNV")]
pub unsafe fn bind_acceleration_structure_memory_nv<'a>(
    device: &raw::Device,
    p_bind_infos: impl AsSlice<'a, BindAccelerationStructureMemoryInfoNV<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .bind_acceleration_structure_memory_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        p_bind_infos.as_slice().len() as _,
        p_bind_infos.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructureNV.html>"]
#[doc(alias = "vkCmdBuildAccelerationStructureNV")]
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
    let vulkan_command = dispatcher
        .cmd_build_acceleration_structure_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_info),
        instance_data.map(|v| unsafe { v.clone() }),
        instance_offset,
        update.into(),
        Some(unsafe { dst.clone() }),
        src.map(|v| unsafe { v.clone() }),
        Some(unsafe { scratch.clone() }),
        scratch_offset,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureNV.html>"]
#[doc(alias = "vkCmdCopyAccelerationStructureNV")]
pub unsafe fn cmd_copy_acceleration_structure_nv(
    command_buffer: &raw::CommandBuffer,
    dst: &raw::AccelerationStructureNV,
    src: &raw::AccelerationStructureNV,
    mode: CopyAccelerationStructureModeKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_acceleration_structure_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { dst.clone() }),
        Some(unsafe { src.clone() }),
        mode,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysNV.html>"]
#[doc(alias = "vkCmdTraceRaysNV")]
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
    let vulkan_command = dispatcher
        .cmd_trace_rays_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { raygen_shader_binding_table_buffer.clone() }),
        raygen_shader_binding_offset,
        miss_shader_binding_table_buffer.map(|v| unsafe { v.clone() }),
        miss_shader_binding_offset,
        miss_shader_binding_stride,
        hit_shader_binding_table_buffer.map(|v| unsafe { v.clone() }),
        hit_shader_binding_offset,
        hit_shader_binding_stride,
        callable_shader_binding_table_buffer.map(|v| unsafe { v.clone() }),
        callable_shader_binding_offset,
        callable_shader_binding_stride,
        width,
        height,
        depth,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesNV.html>"]
#[doc(alias = "vkCreateRayTracingPipelinesNV")]
pub unsafe fn create_ray_tracing_pipelines_nv<'a, R: DynamicArray<Pipeline>>(
    device: &raw::Device,
    pipeline_cache: Option<&raw::PipelineCache>,
    p_create_infos: impl AsSlice<'a, RayTracingPipelineCreateInfoNV<'a>>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, R)> {
    let vulkan_command = dispatcher
        .create_ray_tracing_pipelines_nv
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_pipelines = R::create_with_capacity(p_create_infos.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        pipeline_cache.map(|v| unsafe { v.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureHandleNV.html>"]
#[doc(alias = "vkGetAccelerationStructureHandleNV")]
pub unsafe fn get_acceleration_structure_handle_nv(
    device: &raw::Device,
    acceleration_structure: &raw::AccelerationStructureNV,
    data_size: usize,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_acceleration_structure_handle_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { acceleration_structure.clone() }),
        data_size,
        p_data,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html>"]
#[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesNV")]
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
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_acceleration_structures.as_slice().len() as _,
        p_acceleration_structures.as_slice().as_ptr().cast(),
        query_type,
        Some(unsafe { query_pool.clone() }),
        first_query,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCompileDeferredNV.html>"]
#[doc(alias = "vkCompileDeferredNV")]
pub unsafe fn compile_deferred_nv(
    device: &raw::Device,
    pipeline: &raw::Pipeline,
    shader: u32,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .compile_deferred_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { pipeline.clone() }),
        shader,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html>"]
#[doc(alias = "vkGetMemoryHostPointerPropertiesEXT")]
pub unsafe fn get_memory_host_pointer_properties_ext<
    S: StructureChainOut<MemoryHostPointerPropertiesEXT<'static>>,
>(
    device: &raw::Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    p_host_pointer: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_memory_host_pointer_properties_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_host_pointer_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_host_pointer_properties);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        handle_type,
        p_host_pointer,
        S::get_uninit_head_ptr(p_memory_host_pointer_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_memory_host_pointer_properties.as_mut_ptr());
        p_memory_host_pointer_properties.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarkerAMD.html>"]
#[doc(alias = "vkCmdWriteBufferMarkerAMD")]
pub unsafe fn cmd_write_buffer_marker_amd(
    command_buffer: &raw::CommandBuffer,
    pipeline_stage: PipelineStageFlags,
    dst_buffer: &raw::Buffer,
    dst_offset: DeviceSize,
    marker: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_write_buffer_marker_amd
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        pipeline_stage,
        Some(unsafe { dst_buffer.clone() }),
        dst_offset,
        marker,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksNV.html>"]
#[doc(alias = "vkCmdDrawMeshTasksNV")]
pub unsafe fn cmd_draw_mesh_tasks_nv(
    command_buffer: &raw::CommandBuffer,
    task_count: u32,
    first_task: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_draw_mesh_tasks_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        task_count,
        first_task,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html>"]
#[doc(alias = "vkCmdDrawMeshTasksIndirectNV")]
pub unsafe fn cmd_draw_mesh_tasks_indirect_nv(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_draw_mesh_tasks_indirect_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { buffer.clone() }),
        offset,
        draw_count,
        stride,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html>"]
#[doc(alias = "vkCmdDrawMeshTasksIndirectCountNV")]
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
    let vulkan_command = dispatcher
        .cmd_draw_mesh_tasks_indirect_count_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { buffer.clone() }),
        offset,
        Some(unsafe { count_buffer.clone() }),
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorEnableNV.html>"]
#[doc(alias = "vkCmdSetExclusiveScissorEnableNV")]
pub unsafe fn cmd_set_exclusive_scissor_enable_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    first_exclusive_scissor: u32,
    p_exclusive_scissor_enables: impl AsSlice<'a, Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_exclusive_scissor_enable_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        first_exclusive_scissor,
        p_exclusive_scissor_enables.as_slice().len() as _,
        p_exclusive_scissor_enables.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorNV.html>"]
#[doc(alias = "vkCmdSetExclusiveScissorNV")]
pub unsafe fn cmd_set_exclusive_scissor_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    first_exclusive_scissor: u32,
    p_exclusive_scissors: impl AsSlice<'a, Rect2D>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_exclusive_scissor_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        first_exclusive_scissor,
        p_exclusive_scissors.as_slice().len() as _,
        p_exclusive_scissors.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCheckpointNV.html>"]
#[doc(alias = "vkCmdSetCheckpointNV")]
pub unsafe fn cmd_set_checkpoint_nv(
    command_buffer: &raw::CommandBuffer,
    p_checkpoint_marker: VoidPtr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_checkpoint_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), p_checkpoint_marker)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointDataNV.html>"]
#[doc(alias = "vkGetQueueCheckpointDataNV")]
pub unsafe fn get_queue_checkpoint_data_nv<R: DynamicArray<CheckpointDataNV<'static>>>(
    queue: &raw::Queue,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher
        .get_queue_checkpoint_data_nv
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_checkpoint_data_count = vk_len.as_mut_ptr();
    let p_checkpoint_data = ptr::null_mut();
    vulkan_command(
        Some(unsafe { queue.clone() }),
        p_checkpoint_data_count,
        p_checkpoint_data,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_checkpoint_data_count = ptr::from_mut(&mut vk_len);
    let mut p_checkpoint_data = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(unsafe { queue.clone() }),
        p_checkpoint_data_count,
        p_checkpoint_data,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkInitializePerformanceApiINTEL.html>"]
#[doc(alias = "vkInitializePerformanceApiINTEL")]
pub unsafe fn initialize_performance_api_intel(
    device: &raw::Device,
    p_initialize_info: &InitializePerformanceApiInfoINTEL,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .initialize_performance_api_intel
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_initialize_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUninitializePerformanceApiINTEL.html>"]
#[doc(alias = "vkUninitializePerformanceApiINTEL")]
pub unsafe fn uninitialize_performance_api_intel(
    device: &raw::Device,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .uninitialize_performance_api_intel
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html>"]
#[doc(alias = "vkCmdSetPerformanceMarkerINTEL")]
pub unsafe fn cmd_set_performance_marker_intel(
    command_buffer: &raw::CommandBuffer,
    p_marker_info: &PerformanceMarkerInfoINTEL,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .cmd_set_performance_marker_intel
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_marker_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html>"]
#[doc(alias = "vkCmdSetPerformanceStreamMarkerINTEL")]
pub unsafe fn cmd_set_performance_stream_marker_intel(
    command_buffer: &raw::CommandBuffer,
    p_marker_info: &PerformanceStreamMarkerInfoINTEL,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .cmd_set_performance_stream_marker_intel
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_marker_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html>"]
#[doc(alias = "vkCmdSetPerformanceOverrideINTEL")]
pub unsafe fn cmd_set_performance_override_intel(
    command_buffer: &raw::CommandBuffer,
    p_override_info: &PerformanceOverrideInfoINTEL,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .cmd_set_performance_override_intel
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_override_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html>"]
#[doc(alias = "vkAcquirePerformanceConfigurationINTEL")]
pub unsafe fn acquire_performance_configuration_intel(
    device: &raw::Device,
    p_acquire_info: &PerformanceConfigurationAcquireInfoINTEL,
    dispatcher: &CommandsDispatcher,
) -> Result<PerformanceConfigurationINTEL> {
    let vulkan_command = dispatcher
        .acquire_performance_configuration_intel
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_configuration = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_acquire_info),
        p_configuration.as_mut_ptr(),
    );
    vk_status.map_success(|| p_configuration.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html>"]
#[doc(alias = "vkReleasePerformanceConfigurationINTEL")]
pub unsafe fn release_performance_configuration_intel(
    device: &raw::Device,
    configuration: Option<&raw::PerformanceConfigurationINTEL>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .release_performance_configuration_intel
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        configuration.map(|v| unsafe { v.clone() }),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html>"]
#[doc(alias = "vkQueueSetPerformanceConfigurationINTEL")]
pub unsafe fn queue_set_performance_configuration_intel(
    queue: &raw::Queue,
    configuration: &raw::PerformanceConfigurationINTEL,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .queue_set_performance_configuration_intel
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { queue.clone() }),
        Some(unsafe { configuration.clone() }),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPerformanceParameterINTEL.html>"]
#[doc(alias = "vkGetPerformanceParameterINTEL")]
pub unsafe fn get_performance_parameter_intel(
    device: &raw::Device,
    parameter: PerformanceParameterTypeINTEL,
    dispatcher: &CommandsDispatcher,
) -> Result<PerformanceValueINTEL> {
    let vulkan_command = dispatcher
        .get_performance_parameter_intel
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_value = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        parameter,
        p_value.as_mut_ptr(),
    );
    vk_status.map_success(|| p_value.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetLocalDimmingAMD.html>"]
#[doc(alias = "vkSetLocalDimmingAMD")]
pub unsafe fn set_local_dimming_amd(
    device: &raw::Device,
    swap_chain: &raw::SwapchainKHR,
    local_dimming_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .set_local_dimming_amd
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { swap_chain.clone() }),
        local_dimming_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImagePipeSurfaceFUCHSIA.html>"]
#[doc(alias = "vkCreateImagePipeSurfaceFUCHSIA")]
pub unsafe fn create_image_pipe_surface_fuchsia(
    instance: &raw::Instance,
    p_create_info: &ImagePipeSurfaceCreateInfoFUCHSIA,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher
        .create_image_pipe_surface_fuchsia
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMetalSurfaceEXT.html>"]
#[doc(alias = "vkCreateMetalSurfaceEXT")]
pub unsafe fn create_metal_surface_ext(
    instance: &raw::Instance,
    p_create_info: &MetalSurfaceCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher
        .create_metal_surface_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFragmentShadingRatesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceFragmentShadingRatesKHR")]
pub unsafe fn get_physical_device_fragment_shading_rates_khr<
    R: DynamicArray<PhysicalDeviceFragmentShadingRateKHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_fragment_shading_rates_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_fragment_shading_rate_count = vk_len.as_mut_ptr();
    let p_fragment_shading_rates = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateKHR.html>"]
#[doc(alias = "vkCmdSetFragmentShadingRateKHR")]
pub unsafe fn cmd_set_fragment_shading_rate_khr(
    command_buffer: &raw::CommandBuffer,
    p_fragment_size: &Extent2D,
    combiner_ops: [FragmentShadingRateCombinerOpKHR; 2u16 as _],
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_fragment_shading_rate_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_fragment_size),
        combiner_ops,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRenderingAttachmentLocationsKHR.html>"]
#[doc(alias = "vkCmdSetRenderingAttachmentLocationsKHR")]
pub unsafe fn cmd_set_rendering_attachment_locations_khr(
    command_buffer: &raw::CommandBuffer,
    p_location_info: &RenderingAttachmentLocationInfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_rendering_attachment_locations_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_location_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRenderingInputAttachmentIndicesKHR.html>"]
#[doc(alias = "vkCmdSetRenderingInputAttachmentIndicesKHR")]
pub unsafe fn cmd_set_rendering_input_attachment_indices_khr(
    command_buffer: &raw::CommandBuffer,
    p_input_attachment_index_info: &RenderingInputAttachmentIndexInfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_rendering_input_attachment_indices_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_input_attachment_index_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitForPresentKHR.html>"]
#[doc(alias = "vkWaitForPresentKHR")]
pub unsafe fn wait_for_present_khr(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    present_id: u64,
    timeout: u64,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .wait_for_present_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { swapchain.clone() }),
        present_id,
        timeout,
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html>"]
#[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV")]
pub unsafe fn get_physical_device_cooperative_matrix_properties_nv<
    R: DynamicArray<CooperativeMatrixPropertiesNV<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_cooperative_matrix_properties_nv
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html>"]
#[doc(alias = "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")]
pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv<
    R: DynamicArray<FramebufferMixedSamplesCombinationNV<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_combination_count = vk_len.as_mut_ptr();
    let p_combinations = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html>"]
#[doc(alias = "vkGetPhysicalDeviceSurfacePresentModes2EXT")]
pub unsafe fn get_physical_device_surface_present_modes2_ext<R: DynamicArray<PresentModeKHR>>(
    physical_device: &raw::PhysicalDevice,
    p_surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_surface_present_modes2_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_present_mode_count = vk_len.as_mut_ptr();
    let p_present_modes = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html>"]
#[doc(alias = "vkAcquireFullScreenExclusiveModeEXT")]
pub unsafe fn acquire_full_screen_exclusive_mode_ext(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .acquire_full_screen_exclusive_mode_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { swapchain.clone() }),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html>"]
#[doc(alias = "vkReleaseFullScreenExclusiveModeEXT")]
pub unsafe fn release_full_screen_exclusive_mode_ext(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .release_full_screen_exclusive_mode_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { swapchain.clone() }),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html>"]
#[doc(alias = "vkGetDeviceGroupSurfacePresentModes2EXT")]
pub unsafe fn get_device_group_surface_present_modes2_ext(
    device: &raw::Device,
    p_surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    dispatcher: &CommandsDispatcher,
) -> Result<DeviceGroupPresentModeFlagsKHR> {
    let vulkan_command = dispatcher
        .get_device_group_surface_present_modes2_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_modes = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_surface_info),
        p_modes.as_mut_ptr(),
    );
    vk_status.map_success(|| p_modes.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateHeadlessSurfaceEXT.html>"]
#[doc(alias = "vkCreateHeadlessSurfaceEXT")]
pub unsafe fn create_headless_surface_ext(
    instance: &raw::Instance,
    p_create_info: &HeadlessSurfaceCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher
        .create_headless_surface_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDeferredOperationKHR.html>"]
#[doc(alias = "vkCreateDeferredOperationKHR")]
pub unsafe fn create_deferred_operation_khr(
    device: &raw::Device,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<DeferredOperationKHR> {
    let vulkan_command = dispatcher
        .create_deferred_operation_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_deferred_operation = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_deferred_operation.as_mut_ptr(),
    );
    vk_status.map_success(|| p_deferred_operation.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDeferredOperationKHR.html>"]
#[doc(alias = "vkDestroyDeferredOperationKHR")]
pub unsafe fn destroy_deferred_operation_khr(
    device: &raw::Device,
    operation: Option<&raw::DeferredOperationKHR>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_deferred_operation_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        operation.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html>"]
#[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
pub unsafe fn get_deferred_operation_max_concurrency_khr(
    device: &raw::Device,
    operation: &raw::DeferredOperationKHR,
    dispatcher: &CommandsDispatcher,
) -> u32 {
    let vulkan_command = dispatcher
        .get_deferred_operation_max_concurrency_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { operation.clone() }),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationResultKHR.html>"]
#[doc(alias = "vkGetDeferredOperationResultKHR")]
pub unsafe fn get_deferred_operation_result_khr(
    device: &raw::Device,
    operation: &raw::DeferredOperationKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .get_deferred_operation_result_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { operation.clone() }),
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeferredOperationJoinKHR.html>"]
#[doc(alias = "vkDeferredOperationJoinKHR")]
pub unsafe fn deferred_operation_join_khr(
    device: &raw::Device,
    operation: &raw::DeferredOperationKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .deferred_operation_join_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { operation.clone() }),
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html>"]
#[doc(alias = "vkGetPipelineExecutablePropertiesKHR")]
pub unsafe fn get_pipeline_executable_properties_khr<
    R: DynamicArray<PipelineExecutablePropertiesKHR<'static>>,
>(
    device: &raw::Device,
    p_pipeline_info: &PipelineInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_pipeline_executable_properties_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_executable_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { device.clone() }),
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
            Some(unsafe { device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html>"]
#[doc(alias = "vkGetPipelineExecutableStatisticsKHR")]
pub unsafe fn get_pipeline_executable_statistics_khr<
    R: DynamicArray<PipelineExecutableStatisticKHR<'static>>,
>(
    device: &raw::Device,
    p_executable_info: &PipelineExecutableInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_pipeline_executable_statistics_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_statistic_count = vk_len.as_mut_ptr();
    let p_statistics = ptr::null_mut();
    vulkan_command(
        Some(unsafe { device.clone() }),
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
            Some(unsafe { device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_internal_representation_count = vk_len.as_mut_ptr();
    let p_internal_representations = ptr::null_mut();
    vulkan_command(
        Some(unsafe { device.clone() }),
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
            Some(unsafe { device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToImageEXT.html>"]
#[doc(alias = "vkCopyMemoryToImageEXT")]
pub unsafe fn copy_memory_to_image_ext(
    device: &raw::Device,
    p_copy_memory_to_image_info: &CopyMemoryToImageInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .copy_memory_to_image_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_copy_memory_to_image_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyImageToMemoryEXT.html>"]
#[doc(alias = "vkCopyImageToMemoryEXT")]
pub unsafe fn copy_image_to_memory_ext(
    device: &raw::Device,
    p_copy_image_to_memory_info: &CopyImageToMemoryInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .copy_image_to_memory_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_copy_image_to_memory_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyImageToImageEXT.html>"]
#[doc(alias = "vkCopyImageToImageEXT")]
pub unsafe fn copy_image_to_image_ext(
    device: &raw::Device,
    p_copy_image_to_image_info: &CopyImageToImageInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .copy_image_to_image_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_copy_image_to_image_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkTransitionImageLayoutEXT.html>"]
#[doc(alias = "vkTransitionImageLayoutEXT")]
pub unsafe fn transition_image_layout_ext<'a>(
    device: &raw::Device,
    p_transitions: impl AsSlice<'a, HostImageLayoutTransitionInfoEXT<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .transition_image_layout_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        p_transitions.as_slice().len() as _,
        p_transitions.as_slice().as_ptr().cast(),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMapMemory2KHR.html>"]
#[doc(alias = "vkMapMemory2KHR")]
pub unsafe fn map_memory2_khr(
    device: &raw::Device,
    p_memory_map_info: &MemoryMapInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher
        .map_memory2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut pp_data = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_memory_map_info),
        pp_data.as_mut_ptr(),
    );
    vk_status.map_success(|| pp_data.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUnmapMemory2KHR.html>"]
#[doc(alias = "vkUnmapMemory2KHR")]
pub unsafe fn unmap_memory2_khr(
    device: &raw::Device,
    p_memory_unmap_info: &MemoryUnmapInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .unmap_memory2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_memory_unmap_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseSwapchainImagesEXT.html>"]
#[doc(alias = "vkReleaseSwapchainImagesEXT")]
pub unsafe fn release_swapchain_images_ext(
    device: &raw::Device,
    p_release_info: &ReleaseSwapchainImagesInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .release_swapchain_images_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_release_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPreprocessGeneratedCommandsNV.html>"]
#[doc(alias = "vkCmdPreprocessGeneratedCommandsNV")]
pub unsafe fn cmd_preprocess_generated_commands_nv(
    command_buffer: &raw::CommandBuffer,
    p_generated_commands_info: &GeneratedCommandsInfoNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_preprocess_generated_commands_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_generated_commands_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteGeneratedCommandsNV.html>"]
#[doc(alias = "vkCmdExecuteGeneratedCommandsNV")]
pub unsafe fn cmd_execute_generated_commands_nv(
    command_buffer: &raw::CommandBuffer,
    is_preprocessed: impl Into<Bool32>,
    p_generated_commands_info: &GeneratedCommandsInfoNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_execute_generated_commands_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        is_preprocessed.into(),
        ptr::from_ref(p_generated_commands_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipelineShaderGroupNV.html>"]
#[doc(alias = "vkCmdBindPipelineShaderGroupNV")]
pub unsafe fn cmd_bind_pipeline_shader_group_nv(
    command_buffer: &raw::CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: &raw::Pipeline,
    group_index: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_pipeline_shader_group_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        pipeline_bind_point,
        Some(unsafe { pipeline.clone() }),
        group_index,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateIndirectCommandsLayoutNV.html>"]
#[doc(alias = "vkCreateIndirectCommandsLayoutNV")]
pub unsafe fn create_indirect_commands_layout_nv(
    device: &raw::Device,
    p_create_info: &IndirectCommandsLayoutCreateInfoNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<IndirectCommandsLayoutNV> {
    let vulkan_command = dispatcher
        .create_indirect_commands_layout_nv
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_indirect_commands_layout = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_indirect_commands_layout.as_mut_ptr(),
    );
    vk_status.map_success(|| p_indirect_commands_layout.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyIndirectCommandsLayoutNV.html>"]
#[doc(alias = "vkDestroyIndirectCommandsLayoutNV")]
pub unsafe fn destroy_indirect_commands_layout_nv(
    device: &raw::Device,
    indirect_commands_layout: Option<&raw::IndirectCommandsLayoutNV>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_indirect_commands_layout_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        indirect_commands_layout.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBias2EXT.html>"]
#[doc(alias = "vkCmdSetDepthBias2EXT")]
pub unsafe fn cmd_set_depth_bias2_ext(
    command_buffer: &raw::CommandBuffer,
    p_depth_bias_info: &DepthBiasInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_bias2_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_depth_bias_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireDrmDisplayEXT.html>"]
#[doc(alias = "vkAcquireDrmDisplayEXT")]
pub unsafe fn acquire_drm_display_ext(
    physical_device: &raw::PhysicalDevice,
    drm_fd: i32,
    display: &raw::DisplayKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .acquire_drm_display_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        drm_fd,
        Some(unsafe { display.clone() }),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDrmDisplayEXT.html>"]
#[doc(alias = "vkGetDrmDisplayEXT")]
pub unsafe fn get_drm_display_ext(
    physical_device: &raw::PhysicalDevice,
    drm_fd: i32,
    connector_id: u32,
    dispatcher: &CommandsDispatcher,
) -> Result<DisplayKHR> {
    let vulkan_command = dispatcher
        .get_drm_display_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut display = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { physical_device.clone() }),
        drm_fd,
        connector_id,
        display.as_mut_ptr(),
    );
    vk_status.map_success(|| display.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCudaModuleNV.html>"]
#[doc(alias = "vkCreateCudaModuleNV")]
pub unsafe fn create_cuda_module_nv(
    device: &raw::Device,
    p_create_info: &CudaModuleCreateInfoNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<CudaModuleNV> {
    let vulkan_command = dispatcher
        .create_cuda_module_nv
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_module = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_module.as_mut_ptr(),
    );
    vk_status.map_success(|| p_module.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetCudaModuleCacheNV.html>"]
#[doc(alias = "vkGetCudaModuleCacheNV")]
pub unsafe fn get_cuda_module_cache_nv(
    device: &raw::Device,
    module: &raw::CudaModuleNV,
    p_cache_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<usize> {
    let vulkan_command = dispatcher
        .get_cuda_module_cache_nv
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_cache_size = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { module.clone() }),
        p_cache_size.as_mut_ptr(),
        p_cache_data,
    );
    vk_status.map_success(|| p_cache_size.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCudaFunctionNV.html>"]
#[doc(alias = "vkCreateCudaFunctionNV")]
pub unsafe fn create_cuda_function_nv(
    device: &raw::Device,
    p_create_info: &CudaFunctionCreateInfoNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<CudaFunctionNV> {
    let vulkan_command = dispatcher
        .create_cuda_function_nv
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_function = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_function.as_mut_ptr(),
    );
    vk_status.map_success(|| p_function.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCudaModuleNV.html>"]
#[doc(alias = "vkDestroyCudaModuleNV")]
pub unsafe fn destroy_cuda_module_nv(
    device: &raw::Device,
    module: &raw::CudaModuleNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_cuda_module_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { module.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCudaFunctionNV.html>"]
#[doc(alias = "vkDestroyCudaFunctionNV")]
pub unsafe fn destroy_cuda_function_nv(
    device: &raw::Device,
    function: &raw::CudaFunctionNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_cuda_function_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { function.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCudaLaunchKernelNV.html>"]
#[doc(alias = "vkCmdCudaLaunchKernelNV")]
pub unsafe fn cmd_cuda_launch_kernel_nv(
    command_buffer: &raw::CommandBuffer,
    p_launch_info: &CudaLaunchInfoNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_cuda_launch_kernel_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_launch_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkExportMetalObjectsEXT.html>"]
#[doc(alias = "vkExportMetalObjectsEXT")]
pub unsafe fn export_metal_objects_ext<S: StructureChainOut<ExportMetalObjectsInfoEXT<'static>>>(
    device: &raw::Device,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .export_metal_objects_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_metal_objects_info = MaybeUninit::uninit();
    S::setup_uninit(&mut p_metal_objects_info);
    vulkan_command(
        Some(unsafe { device.clone() }),
        S::get_uninit_head_ptr(p_metal_objects_info.as_mut_ptr()),
    );
    S::setup_cleanup(p_metal_objects_info.as_mut_ptr());
    p_metal_objects_info.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarker2AMD.html>"]
#[doc(alias = "vkCmdWriteBufferMarker2AMD")]
pub unsafe fn cmd_write_buffer_marker2_amd(
    command_buffer: &raw::CommandBuffer,
    stage: PipelineStageFlags2,
    dst_buffer: &raw::Buffer,
    dst_offset: DeviceSize,
    marker: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_write_buffer_marker2_amd
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        stage,
        Some(unsafe { dst_buffer.clone() }),
        dst_offset,
        marker,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointData2NV.html>"]
#[doc(alias = "vkGetQueueCheckpointData2NV")]
pub unsafe fn get_queue_checkpoint_data2_nv<R: DynamicArray<CheckpointData2NV<'static>>>(
    queue: &raw::Queue,
    dispatcher: &CommandsDispatcher,
) -> R {
    let vulkan_command = dispatcher
        .get_queue_checkpoint_data2_nv
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_checkpoint_data_count = vk_len.as_mut_ptr();
    let p_checkpoint_data = ptr::null_mut();
    vulkan_command(
        Some(unsafe { queue.clone() }),
        p_checkpoint_data_count,
        p_checkpoint_data,
    );
    let mut vk_len = vk_len.assume_init();
    let mut vk_vec = R::create_with_capacity(vk_len as _);
    let mut p_checkpoint_data_count = ptr::from_mut(&mut vk_len);
    let mut p_checkpoint_data = vk_vec.get_content_mut_ptr();
    vulkan_command(
        Some(unsafe { queue.clone() }),
        p_checkpoint_data_count,
        p_checkpoint_data,
    );
    vk_vec.resize_with_len(vk_len as _);
    vk_vec
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSizeEXT.html>"]
#[doc(alias = "vkGetDescriptorSetLayoutSizeEXT")]
pub unsafe fn get_descriptor_set_layout_size_ext(
    device: &raw::Device,
    layout: &raw::DescriptorSetLayout,
    dispatcher: &CommandsDispatcher,
) -> DeviceSize {
    let vulkan_command = dispatcher
        .get_descriptor_set_layout_size_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_layout_size_in_bytes = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { layout.clone() }),
        p_layout_size_in_bytes.as_mut_ptr(),
    );
    p_layout_size_in_bytes.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutBindingOffsetEXT.html>"]
#[doc(alias = "vkGetDescriptorSetLayoutBindingOffsetEXT")]
pub unsafe fn get_descriptor_set_layout_binding_offset_ext(
    device: &raw::Device,
    layout: &raw::DescriptorSetLayout,
    binding: u32,
    dispatcher: &CommandsDispatcher,
) -> DeviceSize {
    let vulkan_command = dispatcher
        .get_descriptor_set_layout_binding_offset_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_offset = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { layout.clone() }),
        binding,
        p_offset.as_mut_ptr(),
    );
    p_offset.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorEXT.html>"]
#[doc(alias = "vkGetDescriptorEXT")]
pub unsafe fn get_descriptor_ext(
    device: &raw::Device,
    p_descriptor_info: &DescriptorGetInfoEXT,
    data_size: usize,
    p_descriptor: VoidPtr,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .get_descriptor_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_descriptor_info),
        data_size,
        p_descriptor,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBuffersEXT.html>"]
#[doc(alias = "vkCmdBindDescriptorBuffersEXT")]
pub unsafe fn cmd_bind_descriptor_buffers_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    p_binding_infos: impl AsSlice<'a, DescriptorBufferBindingInfoEXT<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_descriptor_buffers_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_binding_infos.as_slice().len() as _,
        p_binding_infos.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDescriptorBufferOffsetsEXT.html>"]
#[doc(alias = "vkCmdSetDescriptorBufferOffsetsEXT")]
pub unsafe fn cmd_set_descriptor_buffer_offsets_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: &raw::PipelineLayout,
    first_set: u32,
    p_buffer_indices: impl AsSlice<'a, u32>,
    p_offsets: impl AsSlice<'a, DeviceSize>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_descriptor_buffer_offsets_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        pipeline_bind_point,
        Some(unsafe { layout.clone() }),
        first_set,
        p_offsets.as_slice().len() as _,
        p_buffer_indices.as_slice().as_ptr().cast(),
        p_offsets.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html>"]
#[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplersEXT")]
pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers_ext(
    command_buffer: &raw::CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: &raw::PipelineLayout,
    set: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_descriptor_buffer_embedded_samplers_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        pipeline_bind_point,
        Some(unsafe { layout.clone() }),
        set,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureDescriptorDataEXT.html>"]
#[doc(alias = "vkGetBufferOpaqueCaptureDescriptorDataEXT")]
pub unsafe fn get_buffer_opaque_capture_descriptor_data_ext(
    device: &raw::Device,
    p_info: &BufferCaptureDescriptorDataInfoEXT,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_buffer_opaque_capture_descriptor_data_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        p_data,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageOpaqueCaptureDescriptorDataEXT.html>"]
#[doc(alias = "vkGetImageOpaqueCaptureDescriptorDataEXT")]
pub unsafe fn get_image_opaque_capture_descriptor_data_ext(
    device: &raw::Device,
    p_info: &ImageCaptureDescriptorDataInfoEXT,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_image_opaque_capture_descriptor_data_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        p_data,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html>"]
#[doc(alias = "vkGetImageViewOpaqueCaptureDescriptorDataEXT")]
pub unsafe fn get_image_view_opaque_capture_descriptor_data_ext(
    device: &raw::Device,
    p_info: &ImageViewCaptureDescriptorDataInfoEXT,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_image_view_opaque_capture_descriptor_data_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        p_data,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html>"]
#[doc(alias = "vkGetSamplerOpaqueCaptureDescriptorDataEXT")]
pub unsafe fn get_sampler_opaque_capture_descriptor_data_ext(
    device: &raw::Device,
    p_info: &SamplerCaptureDescriptorDataInfoEXT,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_sampler_opaque_capture_descriptor_data_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        p_data,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html>"]
#[doc(alias = "vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT")]
pub unsafe fn get_acceleration_structure_opaque_capture_descriptor_data_ext(
    device: &raw::Device,
    p_info: &AccelerationStructureCaptureDescriptorDataInfoEXT,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .get_acceleration_structure_opaque_capture_descriptor_data_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        p_data,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateEnumNV.html>"]
#[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
pub unsafe fn cmd_set_fragment_shading_rate_enum_nv(
    command_buffer: &raw::CommandBuffer,
    shading_rate: FragmentShadingRateNV,
    combiner_ops: [FragmentShadingRateCombinerOpKHR; 2u16 as _],
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_fragment_shading_rate_enum_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        shading_rate,
        combiner_ops,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksEXT.html>"]
#[doc(alias = "vkCmdDrawMeshTasksEXT")]
pub unsafe fn cmd_draw_mesh_tasks_ext(
    command_buffer: &raw::CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_draw_mesh_tasks_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        group_count_x,
        group_count_y,
        group_count_z,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectEXT.html>"]
#[doc(alias = "vkCmdDrawMeshTasksIndirectEXT")]
pub unsafe fn cmd_draw_mesh_tasks_indirect_ext(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_draw_mesh_tasks_indirect_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { buffer.clone() }),
        offset,
        draw_count,
        stride,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountEXT.html>"]
#[doc(alias = "vkCmdDrawMeshTasksIndirectCountEXT")]
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
    let vulkan_command = dispatcher
        .cmd_draw_mesh_tasks_indirect_count_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { buffer.clone() }),
        offset,
        Some(unsafe { count_buffer.clone() }),
        count_buffer_offset,
        max_draw_count,
        stride,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireWinrtDisplayNV.html>"]
#[doc(alias = "vkAcquireWinrtDisplayNV")]
pub unsafe fn acquire_winrt_display_nv(
    physical_device: &raw::PhysicalDevice,
    display: &raw::DisplayKHR,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .acquire_winrt_display_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        Some(unsafe { display.clone() }),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetWinrtDisplayNV.html>"]
#[doc(alias = "vkGetWinrtDisplayNV")]
pub unsafe fn get_winrt_display_nv(
    physical_device: &raw::PhysicalDevice,
    device_relative_id: u32,
    dispatcher: &CommandsDispatcher,
) -> Result<DisplayKHR> {
    let vulkan_command = dispatcher
        .get_winrt_display_nv
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_display = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { physical_device.clone() }),
        device_relative_id,
        p_display.as_mut_ptr(),
    );
    vk_status.map_success(|| p_display.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDirectFBSurfaceEXT.html>"]
#[doc(alias = "vkCreateDirectFBSurfaceEXT")]
pub unsafe fn create_direct_fbsurface_ext(
    instance: &raw::Instance,
    p_create_info: &DirectFBSurfaceCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher
        .create_direct_fbsurface_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html>"]
#[doc(alias = "vkGetPhysicalDeviceDirectFBPresentationSupportEXT")]
pub unsafe fn get_physical_device_direct_fbpresentation_support_ext(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    dfb: &VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> bool {
    let vulkan_command = dispatcher
        .get_physical_device_direct_fbpresentation_support_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        queue_family_index,
        ptr::from_ref(dfb),
    )
    .into()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetVertexInputEXT.html>"]
#[doc(alias = "vkCmdSetVertexInputEXT")]
pub unsafe fn cmd_set_vertex_input_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    p_vertex_binding_descriptions: impl AsSlice<'a, VertexInputBindingDescription2EXT<'a>>,
    p_vertex_attribute_descriptions: impl AsSlice<'a, VertexInputAttributeDescription2EXT<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_vertex_input_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_vertex_binding_descriptions.as_slice().len() as _,
        p_vertex_binding_descriptions.as_slice().as_ptr().cast(),
        p_vertex_attribute_descriptions.as_slice().len() as _,
        p_vertex_attribute_descriptions.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandleFUCHSIA.html>"]
#[doc(alias = "vkGetMemoryZirconHandleFUCHSIA")]
pub unsafe fn get_memory_zircon_handle_fuchsia(
    device: &raw::Device,
    p_get_zircon_handle_info: &MemoryGetZirconHandleInfoFUCHSIA,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher
        .get_memory_zircon_handle_fuchsia
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_zircon_handle = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_get_zircon_handle_info),
        p_zircon_handle.as_mut_ptr(),
    );
    vk_status.map_success(|| p_zircon_handle.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandlePropertiesFUCHSIA.html>"]
#[doc(alias = "vkGetMemoryZirconHandlePropertiesFUCHSIA")]
pub unsafe fn get_memory_zircon_handle_properties_fuchsia<
    S: StructureChainOut<MemoryZirconHandlePropertiesFUCHSIA<'static>>,
>(
    device: &raw::Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    zircon_handle: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_memory_zircon_handle_properties_fuchsia
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_zircon_handle_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_zircon_handle_properties);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        handle_type,
        zircon_handle,
        S::get_uninit_head_ptr(p_memory_zircon_handle_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_memory_zircon_handle_properties.as_mut_ptr());
        p_memory_zircon_handle_properties.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreZirconHandleFUCHSIA.html>"]
#[doc(alias = "vkImportSemaphoreZirconHandleFUCHSIA")]
pub unsafe fn import_semaphore_zircon_handle_fuchsia(
    device: &raw::Device,
    p_import_semaphore_zircon_handle_info: &ImportSemaphoreZirconHandleInfoFUCHSIA,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .import_semaphore_zircon_handle_fuchsia
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_import_semaphore_zircon_handle_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreZirconHandleFUCHSIA.html>"]
#[doc(alias = "vkGetSemaphoreZirconHandleFUCHSIA")]
pub unsafe fn get_semaphore_zircon_handle_fuchsia(
    device: &raw::Device,
    p_get_zircon_handle_info: &SemaphoreGetZirconHandleInfoFUCHSIA,
    dispatcher: &CommandsDispatcher,
) -> Result<VoidPtr> {
    let vulkan_command = dispatcher
        .get_semaphore_zircon_handle_fuchsia
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_zircon_handle = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_get_zircon_handle_info),
        p_zircon_handle.as_mut_ptr(),
    );
    vk_status.map_success(|| p_zircon_handle.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBufferCollectionFUCHSIA.html>"]
#[doc(alias = "vkCreateBufferCollectionFUCHSIA")]
pub unsafe fn create_buffer_collection_fuchsia(
    device: &raw::Device,
    p_create_info: &BufferCollectionCreateInfoFUCHSIA,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<BufferCollectionFUCHSIA> {
    let vulkan_command = dispatcher
        .create_buffer_collection_fuchsia
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_collection = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_collection.as_mut_ptr(),
    );
    vk_status.map_success(|| p_collection.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionImageConstraintsFUCHSIA.html>"]
#[doc(alias = "vkSetBufferCollectionImageConstraintsFUCHSIA")]
pub unsafe fn set_buffer_collection_image_constraints_fuchsia(
    device: &raw::Device,
    collection: &raw::BufferCollectionFUCHSIA,
    p_image_constraints_info: &ImageConstraintsInfoFUCHSIA,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .set_buffer_collection_image_constraints_fuchsia
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { collection.clone() }),
        ptr::from_ref(p_image_constraints_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetBufferCollectionBufferConstraintsFUCHSIA.html>"]
#[doc(alias = "vkSetBufferCollectionBufferConstraintsFUCHSIA")]
pub unsafe fn set_buffer_collection_buffer_constraints_fuchsia(
    device: &raw::Device,
    collection: &raw::BufferCollectionFUCHSIA,
    p_buffer_constraints_info: &BufferConstraintsInfoFUCHSIA,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .set_buffer_collection_buffer_constraints_fuchsia
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { collection.clone() }),
        ptr::from_ref(p_buffer_constraints_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferCollectionFUCHSIA.html>"]
#[doc(alias = "vkDestroyBufferCollectionFUCHSIA")]
pub unsafe fn destroy_buffer_collection_fuchsia(
    device: &raw::Device,
    collection: &raw::BufferCollectionFUCHSIA,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_buffer_collection_fuchsia
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { collection.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferCollectionPropertiesFUCHSIA.html>"]
#[doc(alias = "vkGetBufferCollectionPropertiesFUCHSIA")]
pub unsafe fn get_buffer_collection_properties_fuchsia<
    S: StructureChainOut<BufferCollectionPropertiesFUCHSIA<'static>>,
>(
    device: &raw::Device,
    collection: &raw::BufferCollectionFUCHSIA,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_buffer_collection_properties_fuchsia
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { collection.clone() }),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_properties.as_mut_ptr());
        p_properties.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html>"]
#[doc(alias = "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")]
pub unsafe fn get_device_subpass_shading_max_workgroup_size_huawei<R: DynamicArray<Extent2D>>(
    device: &raw::Device,
    renderpass: &raw::RenderPass,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_device_subpass_shading_max_workgroup_size_huawei
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_max_workgroup_size = R::create_with_capacity(1 as _);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { renderpass.clone() }),
        p_max_workgroup_size.get_content_mut_ptr(),
    );
    vk_status.map_success(|| {
        p_max_workgroup_size.resize_with_len(1 as _);
        p_max_workgroup_size
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSubpassShadingHUAWEI.html>"]
#[doc(alias = "vkCmdSubpassShadingHUAWEI")]
pub unsafe fn cmd_subpass_shading_huawei(
    command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_subpass_shading_huawei
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindInvocationMaskHUAWEI.html>"]
#[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
pub unsafe fn cmd_bind_invocation_mask_huawei(
    command_buffer: &raw::CommandBuffer,
    image_view: Option<&raw::ImageView>,
    image_layout: ImageLayout,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_invocation_mask_huawei
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        image_view.map(|v| unsafe { v.clone() }),
        image_layout,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryRemoteAddressNV.html>"]
#[doc(alias = "vkGetMemoryRemoteAddressNV")]
pub unsafe fn get_memory_remote_address_nv(
    device: &raw::Device,
    p_memory_get_remote_address_info: &MemoryGetRemoteAddressInfoNV,
    dispatcher: &CommandsDispatcher,
) -> Result<RemoteAddressNV> {
    let vulkan_command = dispatcher
        .get_memory_remote_address_nv
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_address = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_memory_get_remote_address_info),
        p_address.as_mut_ptr(),
    );
    vk_status.map_success(|| p_address.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelinePropertiesEXT.html>"]
#[doc(alias = "vkGetPipelinePropertiesEXT")]
pub unsafe fn get_pipeline_properties_ext(
    device: &raw::Device,
    p_pipeline_info: &PipelineInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<BaseOutStructure<'static>> {
    let vulkan_command = dispatcher
        .get_pipeline_properties_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_pipeline_properties = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_pipeline_info),
        p_pipeline_properties.as_mut_ptr(),
    );
    vk_status.map_success(|| p_pipeline_properties.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPatchControlPointsEXT.html>"]
#[doc(alias = "vkCmdSetPatchControlPointsEXT")]
pub unsafe fn cmd_set_patch_control_points_ext(
    command_buffer: &raw::CommandBuffer,
    patch_control_points: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_patch_control_points_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        patch_control_points,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEXT.html>"]
#[doc(alias = "vkCmdSetLogicOpEXT")]
pub unsafe fn cmd_set_logic_op_ext(
    command_buffer: &raw::CommandBuffer,
    logic_op: LogicOp,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_logic_op_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), logic_op)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateScreenSurfaceQNX.html>"]
#[doc(alias = "vkCreateScreenSurfaceQNX")]
pub unsafe fn create_screen_surface_qnx(
    instance: &raw::Instance,
    p_create_info: &ScreenSurfaceCreateInfoQNX,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<SurfaceKHR> {
    let vulkan_command = dispatcher
        .create_screen_surface_qnx
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_surface = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { instance.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_surface.as_mut_ptr(),
    );
    vk_status.map_success(|| p_surface.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceScreenPresentationSupportQNX.html>"]
#[doc(alias = "vkGetPhysicalDeviceScreenPresentationSupportQNX")]
pub unsafe fn get_physical_device_screen_presentation_support_qnx(
    physical_device: &raw::PhysicalDevice,
    queue_family_index: u32,
    window: &VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> bool {
    let vulkan_command = dispatcher
        .get_physical_device_screen_presentation_support_qnx
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
        queue_family_index,
        ptr::from_ref(window),
    )
    .into()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteEnableEXT.html>"]
#[doc(alias = "vkCmdSetColorWriteEnableEXT")]
pub unsafe fn cmd_set_color_write_enable_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    p_color_write_enables: impl AsSlice<'a, Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_color_write_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_color_write_enables.as_slice().len() as _,
        p_color_write_enables.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirect2KHR.html>"]
#[doc(alias = "vkCmdTraceRaysIndirect2KHR")]
pub unsafe fn cmd_trace_rays_indirect2_khr(
    command_buffer: &raw::CommandBuffer,
    indirect_device_address: DeviceAddress,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_trace_rays_indirect2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        indirect_device_address,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiEXT.html>"]
#[doc(alias = "vkCmdDrawMultiEXT")]
pub unsafe fn cmd_draw_multi_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    p_vertex_info: impl AsSlice<'a, MultiDrawInfoEXT>,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_draw_multi_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_vertex_info.as_slice().len() as _,
        p_vertex_info.as_slice().as_ptr().cast(),
        instance_count,
        first_instance,
        stride,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMultiIndexedEXT.html>"]
#[doc(alias = "vkCmdDrawMultiIndexedEXT")]
pub unsafe fn cmd_draw_multi_indexed_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    p_index_info: impl AsSlice<'a, MultiDrawIndexedInfoEXT>,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
    p_vertex_offset: Option<&i32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_draw_multi_indexed_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateMicromapEXT.html>"]
#[doc(alias = "vkCreateMicromapEXT")]
pub unsafe fn create_micromap_ext(
    device: &raw::Device,
    p_create_info: &MicromapCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<MicromapEXT> {
    let vulkan_command = dispatcher
        .create_micromap_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_micromap = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_micromap.as_mut_ptr(),
    );
    vk_status.map_success(|| p_micromap.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyMicromapEXT.html>"]
#[doc(alias = "vkDestroyMicromapEXT")]
pub unsafe fn destroy_micromap_ext(
    device: &raw::Device,
    micromap: Option<&raw::MicromapEXT>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_micromap_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        micromap.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildMicromapsEXT.html>"]
#[doc(alias = "vkCmdBuildMicromapsEXT")]
pub unsafe fn cmd_build_micromaps_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    p_infos: impl AsSlice<'a, MicromapBuildInfoEXT<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_build_micromaps_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_infos.as_slice().len() as _,
        p_infos.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBuildMicromapsEXT.html>"]
#[doc(alias = "vkBuildMicromapsEXT")]
pub unsafe fn build_micromaps_ext<'a>(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    p_infos: impl AsSlice<'a, MicromapBuildInfoEXT<'a>>,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .build_micromaps_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        deferred_operation.map(|v| unsafe { v.clone() }),
        p_infos.as_slice().len() as _,
        p_infos.as_slice().as_ptr().cast(),
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMicromapEXT.html>"]
#[doc(alias = "vkCopyMicromapEXT")]
pub unsafe fn copy_micromap_ext(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    p_info: &CopyMicromapInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .copy_micromap_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        deferred_operation.map(|v| unsafe { v.clone() }),
        ptr::from_ref(p_info),
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMicromapToMemoryEXT.html>"]
#[doc(alias = "vkCopyMicromapToMemoryEXT")]
pub unsafe fn copy_micromap_to_memory_ext(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    p_info: &CopyMicromapToMemoryInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .copy_micromap_to_memory_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        deferred_operation.map(|v| unsafe { v.clone() }),
        ptr::from_ref(p_info),
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToMicromapEXT.html>"]
#[doc(alias = "vkCopyMemoryToMicromapEXT")]
pub unsafe fn copy_memory_to_micromap_ext(
    device: &raw::Device,
    deferred_operation: Option<&raw::DeferredOperationKHR>,
    p_info: &CopyMemoryToMicromapInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> Result<Status> {
    let vulkan_command = dispatcher
        .copy_memory_to_micromap_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        deferred_operation.map(|v| unsafe { v.clone() }),
        ptr::from_ref(p_info),
    )
    .into_result()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWriteMicromapsPropertiesEXT.html>"]
#[doc(alias = "vkWriteMicromapsPropertiesEXT")]
pub unsafe fn write_micromaps_properties_ext<'a, V2: Alias<raw::MicromapEXT> + 'a>(
    device: &raw::Device,
    p_micromaps: impl AsSlice<'a, V2>,
    query_type: QueryType,
    data_size: usize,
    p_data: VoidPtr,
    stride: usize,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .write_micromaps_properties_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        p_micromaps.as_slice().len() as _,
        p_micromaps.as_slice().as_ptr().cast(),
        query_type,
        data_size,
        p_data,
        stride,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMicromapEXT.html>"]
#[doc(alias = "vkCmdCopyMicromapEXT")]
pub unsafe fn cmd_copy_micromap_ext(
    command_buffer: &raw::CommandBuffer,
    p_info: &CopyMicromapInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_micromap_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMicromapToMemoryEXT.html>"]
#[doc(alias = "vkCmdCopyMicromapToMemoryEXT")]
pub unsafe fn cmd_copy_micromap_to_memory_ext(
    command_buffer: &raw::CommandBuffer,
    p_info: &CopyMicromapToMemoryInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_micromap_to_memory_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToMicromapEXT.html>"]
#[doc(alias = "vkCmdCopyMemoryToMicromapEXT")]
pub unsafe fn cmd_copy_memory_to_micromap_ext(
    command_buffer: &raw::CommandBuffer,
    p_info: &CopyMemoryToMicromapInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_memory_to_micromap_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteMicromapsPropertiesEXT.html>"]
#[doc(alias = "vkCmdWriteMicromapsPropertiesEXT")]
pub unsafe fn cmd_write_micromaps_properties_ext<'a, V2: Alias<raw::MicromapEXT> + 'a>(
    command_buffer: &raw::CommandBuffer,
    p_micromaps: impl AsSlice<'a, V2>,
    query_type: QueryType,
    query_pool: &raw::QueryPool,
    first_query: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_write_micromaps_properties_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_micromaps.as_slice().len() as _,
        p_micromaps.as_slice().as_ptr().cast(),
        query_type,
        Some(unsafe { query_pool.clone() }),
        first_query,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMicromapCompatibilityEXT.html>"]
#[doc(alias = "vkGetDeviceMicromapCompatibilityEXT")]
pub unsafe fn get_device_micromap_compatibility_ext(
    device: &raw::Device,
    p_version_info: &MicromapVersionInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> AccelerationStructureCompatibilityKHR {
    let vulkan_command = dispatcher
        .get_device_micromap_compatibility_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_compatibility = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_version_info),
        p_compatibility.as_mut_ptr(),
    );
    p_compatibility.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMicromapBuildSizesEXT.html>"]
#[doc(alias = "vkGetMicromapBuildSizesEXT")]
pub unsafe fn get_micromap_build_sizes_ext<
    S: StructureChainOut<MicromapBuildSizesInfoEXT<'static>>,
>(
    device: &raw::Device,
    build_type: AccelerationStructureBuildTypeKHR,
    p_build_info: &MicromapBuildInfoEXT,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_micromap_build_sizes_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_size_info = MaybeUninit::uninit();
    S::setup_uninit(&mut p_size_info);
    vulkan_command(
        Some(unsafe { device.clone() }),
        build_type,
        ptr::from_ref(p_build_info),
        S::get_uninit_head_ptr(p_size_info.as_mut_ptr()),
    );
    S::setup_cleanup(p_size_info.as_mut_ptr());
    p_size_info.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawClusterHUAWEI.html>"]
#[doc(alias = "vkCmdDrawClusterHUAWEI")]
pub unsafe fn cmd_draw_cluster_huawei(
    command_buffer: &raw::CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_draw_cluster_huawei
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        group_count_x,
        group_count_y,
        group_count_z,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawClusterIndirectHUAWEI.html>"]
#[doc(alias = "vkCmdDrawClusterIndirectHUAWEI")]
pub unsafe fn cmd_draw_cluster_indirect_huawei(
    command_buffer: &raw::CommandBuffer,
    buffer: &raw::Buffer,
    offset: DeviceSize,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_draw_cluster_indirect_huawei
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { buffer.clone() }),
        offset,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDeviceMemoryPriorityEXT.html>"]
#[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
pub unsafe fn set_device_memory_priority_ext(
    device: &raw::Device,
    memory: &raw::DeviceMemory,
    priority: f32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .set_device_memory_priority_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { memory.clone() }),
        priority,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_host_mapping = MaybeUninit::uninit();
    S::setup_uninit(&mut p_host_mapping);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_binding_reference),
        S::get_uninit_head_ptr(p_host_mapping.as_mut_ptr()),
    );
    S::setup_cleanup(p_host_mapping.as_mut_ptr());
    p_host_mapping.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetHostMappingVALVE.html>"]
#[doc(alias = "vkGetDescriptorSetHostMappingVALVE")]
pub unsafe fn get_descriptor_set_host_mapping_valve(
    device: &raw::Device,
    descriptor_set: &raw::DescriptorSet,
    dispatcher: &CommandsDispatcher,
) -> VoidPtr {
    let vulkan_command = dispatcher
        .get_descriptor_set_host_mapping_valve
        .get()
        .expect("Vulkan command not loaded.");
    let mut pp_data = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { descriptor_set.clone() }),
        pp_data.as_mut_ptr(),
    );
    pp_data.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryIndirectNV.html>"]
#[doc(alias = "vkCmdCopyMemoryIndirectNV")]
pub unsafe fn cmd_copy_memory_indirect_nv(
    command_buffer: &raw::CommandBuffer,
    copy_buffer_address: DeviceAddress,
    copy_count: u32,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_memory_indirect_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        copy_buffer_address,
        copy_count,
        stride,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToImageIndirectNV.html>"]
#[doc(alias = "vkCmdCopyMemoryToImageIndirectNV")]
pub unsafe fn cmd_copy_memory_to_image_indirect_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    copy_buffer_address: DeviceAddress,
    stride: u32,
    dst_image: &raw::Image,
    dst_image_layout: ImageLayout,
    p_image_subresources: impl AsSlice<'a, ImageSubresourceLayers>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_copy_memory_to_image_indirect_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        copy_buffer_address,
        p_image_subresources.as_slice().len() as _,
        stride,
        Some(unsafe { dst_image.clone() }),
        dst_image_layout,
        p_image_subresources.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDecompressMemoryNV.html>"]
#[doc(alias = "vkCmdDecompressMemoryNV")]
pub unsafe fn cmd_decompress_memory_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    p_decompress_memory_regions: impl AsSlice<'a, DecompressMemoryRegionNV>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_decompress_memory_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_decompress_memory_regions.as_slice().len() as _,
        p_decompress_memory_regions.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDecompressMemoryIndirectCountNV.html>"]
#[doc(alias = "vkCmdDecompressMemoryIndirectCountNV")]
pub unsafe fn cmd_decompress_memory_indirect_count_nv(
    command_buffer: &raw::CommandBuffer,
    indirect_commands_address: DeviceAddress,
    indirect_commands_count_address: DeviceAddress,
    stride: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_decompress_memory_indirect_count_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        indirect_commands_address,
        indirect_commands_count_address,
        stride,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineIndirectMemoryRequirementsNV.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdUpdatePipelineIndirectBufferNV.html>"]
#[doc(alias = "vkCmdUpdatePipelineIndirectBufferNV")]
pub unsafe fn cmd_update_pipeline_indirect_buffer_nv(
    command_buffer: &raw::CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: &raw::Pipeline,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_update_pipeline_indirect_buffer_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        pipeline_bind_point,
        Some(unsafe { pipeline.clone() }),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineIndirectDeviceAddressNV.html>"]
#[doc(alias = "vkGetPipelineIndirectDeviceAddressNV")]
pub unsafe fn get_pipeline_indirect_device_address_nv(
    device: &raw::Device,
    p_info: &PipelineIndirectDeviceAddressInfoNV,
    dispatcher: &CommandsDispatcher,
) -> DeviceAddress {
    let vulkan_command = dispatcher
        .get_pipeline_indirect_device_address_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), ptr::from_ref(p_info))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClampEnableEXT.html>"]
#[doc(alias = "vkCmdSetDepthClampEnableEXT")]
pub unsafe fn cmd_set_depth_clamp_enable_ext(
    command_buffer: &raw::CommandBuffer,
    depth_clamp_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_clamp_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        depth_clamp_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPolygonModeEXT.html>"]
#[doc(alias = "vkCmdSetPolygonModeEXT")]
pub unsafe fn cmd_set_polygon_mode_ext(
    command_buffer: &raw::CommandBuffer,
    polygon_mode: PolygonMode,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_polygon_mode_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), polygon_mode)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationSamplesEXT.html>"]
#[doc(alias = "vkCmdSetRasterizationSamplesEXT")]
pub unsafe fn cmd_set_rasterization_samples_ext(
    command_buffer: &raw::CommandBuffer,
    rasterization_samples: SampleCountFlags,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_rasterization_samples_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        rasterization_samples,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleMaskEXT.html>"]
#[doc(alias = "vkCmdSetSampleMaskEXT")]
pub unsafe fn cmd_set_sample_mask_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    samples: SampleCountFlags,
    p_sample_mask: impl AsSlice<'a, SampleMask>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_sample_mask_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        samples,
        p_sample_mask.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToCoverageEnableEXT.html>"]
#[doc(alias = "vkCmdSetAlphaToCoverageEnableEXT")]
pub unsafe fn cmd_set_alpha_to_coverage_enable_ext(
    command_buffer: &raw::CommandBuffer,
    alpha_to_coverage_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_alpha_to_coverage_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        alpha_to_coverage_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToOneEnableEXT.html>"]
#[doc(alias = "vkCmdSetAlphaToOneEnableEXT")]
pub unsafe fn cmd_set_alpha_to_one_enable_ext(
    command_buffer: &raw::CommandBuffer,
    alpha_to_one_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_alpha_to_one_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        alpha_to_one_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEnableEXT.html>"]
#[doc(alias = "vkCmdSetLogicOpEnableEXT")]
pub unsafe fn cmd_set_logic_op_enable_ext(
    command_buffer: &raw::CommandBuffer,
    logic_op_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_logic_op_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        logic_op_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEnableEXT.html>"]
#[doc(alias = "vkCmdSetColorBlendEnableEXT")]
pub unsafe fn cmd_set_color_blend_enable_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    first_attachment: u32,
    p_color_blend_enables: impl AsSlice<'a, Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_color_blend_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        first_attachment,
        p_color_blend_enables.as_slice().len() as _,
        p_color_blend_enables.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEquationEXT.html>"]
#[doc(alias = "vkCmdSetColorBlendEquationEXT")]
pub unsafe fn cmd_set_color_blend_equation_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    first_attachment: u32,
    p_color_blend_equations: impl AsSlice<'a, ColorBlendEquationEXT>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_color_blend_equation_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        first_attachment,
        p_color_blend_equations.as_slice().len() as _,
        p_color_blend_equations.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteMaskEXT.html>"]
#[doc(alias = "vkCmdSetColorWriteMaskEXT")]
pub unsafe fn cmd_set_color_write_mask_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    first_attachment: u32,
    p_color_write_masks: impl AsSlice<'a, ColorComponentFlags>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_color_write_mask_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        first_attachment,
        p_color_write_masks.as_slice().len() as _,
        p_color_write_masks.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetTessellationDomainOriginEXT.html>"]
#[doc(alias = "vkCmdSetTessellationDomainOriginEXT")]
pub unsafe fn cmd_set_tessellation_domain_origin_ext(
    command_buffer: &raw::CommandBuffer,
    domain_origin: TessellationDomainOrigin,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_tessellation_domain_origin_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), domain_origin)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationStreamEXT.html>"]
#[doc(alias = "vkCmdSetRasterizationStreamEXT")]
pub unsafe fn cmd_set_rasterization_stream_ext(
    command_buffer: &raw::CommandBuffer,
    rasterization_stream: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_rasterization_stream_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        rasterization_stream,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetConservativeRasterizationModeEXT.html>"]
#[doc(alias = "vkCmdSetConservativeRasterizationModeEXT")]
pub unsafe fn cmd_set_conservative_rasterization_mode_ext(
    command_buffer: &raw::CommandBuffer,
    conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_conservative_rasterization_mode_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        conservative_rasterization_mode,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html>"]
#[doc(alias = "vkCmdSetExtraPrimitiveOverestimationSizeEXT")]
pub unsafe fn cmd_set_extra_primitive_overestimation_size_ext(
    command_buffer: &raw::CommandBuffer,
    extra_primitive_overestimation_size: f32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_extra_primitive_overestimation_size_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        extra_primitive_overestimation_size,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipEnableEXT.html>"]
#[doc(alias = "vkCmdSetDepthClipEnableEXT")]
pub unsafe fn cmd_set_depth_clip_enable_ext(
    command_buffer: &raw::CommandBuffer,
    depth_clip_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_clip_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        depth_clip_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEnableEXT.html>"]
#[doc(alias = "vkCmdSetSampleLocationsEnableEXT")]
pub unsafe fn cmd_set_sample_locations_enable_ext(
    command_buffer: &raw::CommandBuffer,
    sample_locations_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_sample_locations_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        sample_locations_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendAdvancedEXT.html>"]
#[doc(alias = "vkCmdSetColorBlendAdvancedEXT")]
pub unsafe fn cmd_set_color_blend_advanced_ext<'a>(
    command_buffer: &raw::CommandBuffer,
    first_attachment: u32,
    p_color_blend_advanced: impl AsSlice<'a, ColorBlendAdvancedEXT>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_color_blend_advanced_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        first_attachment,
        p_color_blend_advanced.as_slice().len() as _,
        p_color_blend_advanced.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetProvokingVertexModeEXT.html>"]
#[doc(alias = "vkCmdSetProvokingVertexModeEXT")]
pub unsafe fn cmd_set_provoking_vertex_mode_ext(
    command_buffer: &raw::CommandBuffer,
    provoking_vertex_mode: ProvokingVertexModeEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_provoking_vertex_mode_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        provoking_vertex_mode,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineRasterizationModeEXT.html>"]
#[doc(alias = "vkCmdSetLineRasterizationModeEXT")]
pub unsafe fn cmd_set_line_rasterization_mode_ext(
    command_buffer: &raw::CommandBuffer,
    line_rasterization_mode: LineRasterizationModeEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_line_rasterization_mode_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        line_rasterization_mode,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEnableEXT.html>"]
#[doc(alias = "vkCmdSetLineStippleEnableEXT")]
pub unsafe fn cmd_set_line_stipple_enable_ext(
    command_buffer: &raw::CommandBuffer,
    stippled_line_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_line_stipple_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        stippled_line_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipNegativeOneToOneEXT.html>"]
#[doc(alias = "vkCmdSetDepthClipNegativeOneToOneEXT")]
pub unsafe fn cmd_set_depth_clip_negative_one_to_one_ext(
    command_buffer: &raw::CommandBuffer,
    negative_one_to_one: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_clip_negative_one_to_one_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        negative_one_to_one.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingEnableNV.html>"]
#[doc(alias = "vkCmdSetViewportWScalingEnableNV")]
pub unsafe fn cmd_set_viewport_wscaling_enable_nv(
    command_buffer: &raw::CommandBuffer,
    viewport_wscaling_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_viewport_wscaling_enable_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        viewport_wscaling_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportSwizzleNV.html>"]
#[doc(alias = "vkCmdSetViewportSwizzleNV")]
pub unsafe fn cmd_set_viewport_swizzle_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    first_viewport: u32,
    p_viewport_swizzles: impl AsSlice<'a, ViewportSwizzleNV>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_viewport_swizzle_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        first_viewport,
        p_viewport_swizzles.as_slice().len() as _,
        p_viewport_swizzles.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorEnableNV.html>"]
#[doc(alias = "vkCmdSetCoverageToColorEnableNV")]
pub unsafe fn cmd_set_coverage_to_color_enable_nv(
    command_buffer: &raw::CommandBuffer,
    coverage_to_color_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_coverage_to_color_enable_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        coverage_to_color_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorLocationNV.html>"]
#[doc(alias = "vkCmdSetCoverageToColorLocationNV")]
pub unsafe fn cmd_set_coverage_to_color_location_nv(
    command_buffer: &raw::CommandBuffer,
    coverage_to_color_location: u32,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_coverage_to_color_location_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        coverage_to_color_location,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationModeNV.html>"]
#[doc(alias = "vkCmdSetCoverageModulationModeNV")]
pub unsafe fn cmd_set_coverage_modulation_mode_nv(
    command_buffer: &raw::CommandBuffer,
    coverage_modulation_mode: CoverageModulationModeNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_coverage_modulation_mode_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        coverage_modulation_mode,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableEnableNV.html>"]
#[doc(alias = "vkCmdSetCoverageModulationTableEnableNV")]
pub unsafe fn cmd_set_coverage_modulation_table_enable_nv(
    command_buffer: &raw::CommandBuffer,
    coverage_modulation_table_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_coverage_modulation_table_enable_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        coverage_modulation_table_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableNV.html>"]
#[doc(alias = "vkCmdSetCoverageModulationTableNV")]
pub unsafe fn cmd_set_coverage_modulation_table_nv<'a>(
    command_buffer: &raw::CommandBuffer,
    p_coverage_modulation_table: impl AsSlice<'a, f32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_coverage_modulation_table_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_coverage_modulation_table.as_slice().len() as _,
        p_coverage_modulation_table.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetShadingRateImageEnableNV.html>"]
#[doc(alias = "vkCmdSetShadingRateImageEnableNV")]
pub unsafe fn cmd_set_shading_rate_image_enable_nv(
    command_buffer: &raw::CommandBuffer,
    shading_rate_image_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_shading_rate_image_enable_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        shading_rate_image_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRepresentativeFragmentTestEnableNV.html>"]
#[doc(alias = "vkCmdSetRepresentativeFragmentTestEnableNV")]
pub unsafe fn cmd_set_representative_fragment_test_enable_nv(
    command_buffer: &raw::CommandBuffer,
    representative_fragment_test_enable: impl Into<Bool32>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_representative_fragment_test_enable_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        representative_fragment_test_enable.into(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageReductionModeNV.html>"]
#[doc(alias = "vkCmdSetCoverageReductionModeNV")]
pub unsafe fn cmd_set_coverage_reduction_mode_nv(
    command_buffer: &raw::CommandBuffer,
    coverage_reduction_mode: CoverageReductionModeNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_coverage_reduction_mode_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        coverage_reduction_mode,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderModuleIdentifierEXT.html>"]
#[doc(alias = "vkGetShaderModuleIdentifierEXT")]
pub unsafe fn get_shader_module_identifier_ext<
    S: StructureChainOut<ShaderModuleIdentifierEXT<'static>>,
>(
    device: &raw::Device,
    shader_module: &raw::ShaderModule,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_shader_module_identifier_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_identifier = MaybeUninit::uninit();
    S::setup_uninit(&mut p_identifier);
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { shader_module.clone() }),
        S::get_uninit_head_ptr(p_identifier.as_mut_ptr()),
    );
    S::setup_cleanup(p_identifier.as_mut_ptr());
    p_identifier.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderModuleCreateInfoIdentifierEXT.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_identifier = MaybeUninit::uninit();
    S::setup_uninit(&mut p_identifier);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        S::get_uninit_head_ptr(p_identifier.as_mut_ptr()),
    );
    S::setup_cleanup(p_identifier.as_mut_ptr());
    p_identifier.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_format_count = vk_len.as_mut_ptr();
    let p_image_format_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateOpticalFlowSessionNV.html>"]
#[doc(alias = "vkCreateOpticalFlowSessionNV")]
pub unsafe fn create_optical_flow_session_nv(
    device: &raw::Device,
    p_create_info: &OpticalFlowSessionCreateInfoNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<OpticalFlowSessionNV> {
    let vulkan_command = dispatcher
        .create_optical_flow_session_nv
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_session = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_session.as_mut_ptr(),
    );
    vk_status.map_success(|| p_session.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyOpticalFlowSessionNV.html>"]
#[doc(alias = "vkDestroyOpticalFlowSessionNV")]
pub unsafe fn destroy_optical_flow_session_nv(
    device: &raw::Device,
    session: &raw::OpticalFlowSessionNV,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_optical_flow_session_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { session.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindOpticalFlowSessionImageNV.html>"]
#[doc(alias = "vkBindOpticalFlowSessionImageNV")]
pub unsafe fn bind_optical_flow_session_image_nv(
    device: &raw::Device,
    session: &raw::OpticalFlowSessionNV,
    binding_point: OpticalFlowSessionBindingPointNV,
    view: Option<&raw::ImageView>,
    layout: ImageLayout,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .bind_optical_flow_session_image_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { session.clone() }),
        binding_point,
        view.map(|v| unsafe { v.clone() }),
        layout,
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdOpticalFlowExecuteNV.html>"]
#[doc(alias = "vkCmdOpticalFlowExecuteNV")]
pub unsafe fn cmd_optical_flow_execute_nv(
    command_buffer: &raw::CommandBuffer,
    session: &raw::OpticalFlowSessionNV,
    p_execute_info: &OpticalFlowExecuteInfoNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_optical_flow_execute_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        Some(unsafe { session.clone() }),
        ptr::from_ref(p_execute_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindIndexBuffer2KHR.html>"]
#[doc(alias = "vkCmdBindIndexBuffer2KHR")]
pub unsafe fn cmd_bind_index_buffer2_khr(
    command_buffer: &raw::CommandBuffer,
    buffer: Option<&raw::Buffer>,
    offset: DeviceSize,
    size: DeviceSize,
    index_type: IndexType,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_index_buffer2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        buffer.map(|v| unsafe { v.clone() }),
        offset,
        size,
        index_type,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRenderingAreaGranularityKHR.html>"]
#[doc(alias = "vkGetRenderingAreaGranularityKHR")]
pub unsafe fn get_rendering_area_granularity_khr(
    device: &raw::Device,
    p_rendering_area_info: &RenderingAreaInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> Extent2D {
    let vulkan_command = dispatcher
        .get_rendering_area_granularity_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_granularity = MaybeUninit::uninit();
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_rendering_area_info),
        p_granularity.as_mut_ptr(),
    );
    p_granularity.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSubresourceLayoutKHR.html>"]
#[doc(alias = "vkGetDeviceImageSubresourceLayoutKHR")]
pub unsafe fn get_device_image_subresource_layout_khr<
    S: StructureChainOut<SubresourceLayout2KHR<'static>>,
>(
    device: &raw::Device,
    p_info: &DeviceImageSubresourceInfoKHR,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_device_image_subresource_layout_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_layout = MaybeUninit::uninit();
    S::setup_uninit(&mut p_layout);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_layout.as_mut_ptr()),
    );
    S::setup_cleanup(p_layout.as_mut_ptr());
    p_layout.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout2KHR.html>"]
#[doc(alias = "vkGetImageSubresourceLayout2KHR")]
pub unsafe fn get_image_subresource_layout2_khr<
    S: StructureChainOut<SubresourceLayout2KHR<'static>>,
>(
    device: &raw::Device,
    image: &raw::Image,
    p_subresource: &ImageSubresource2KHR,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_image_subresource_layout2_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_layout = MaybeUninit::uninit();
    S::setup_uninit(&mut p_layout);
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { image.clone() }),
        ptr::from_ref(p_subresource),
        S::get_uninit_head_ptr(p_layout.as_mut_ptr()),
    );
    S::setup_cleanup(p_layout.as_mut_ptr());
    p_layout.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout2EXT.html>"]
#[doc(alias = "vkGetImageSubresourceLayout2EXT")]
pub unsafe fn get_image_subresource_layout2_ext<
    S: StructureChainOut<SubresourceLayout2KHR<'static>>,
>(
    device: &raw::Device,
    image: &raw::Image,
    p_subresource: &ImageSubresource2KHR,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_image_subresource_layout2_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_layout = MaybeUninit::uninit();
    S::setup_uninit(&mut p_layout);
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { image.clone() }),
        ptr::from_ref(p_subresource),
        S::get_uninit_head_ptr(p_layout.as_mut_ptr()),
    );
    S::setup_cleanup(p_layout.as_mut_ptr());
    p_layout.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAntiLagUpdateAMD.html>"]
#[doc(alias = "vkAntiLagUpdateAMD")]
pub unsafe fn anti_lag_update_amd(
    device: &raw::Device,
    p_data: &AntiLagDataAMD,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .anti_lag_update_amd
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { device.clone() }), ptr::from_ref(p_data))
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateShadersEXT.html>"]
#[doc(alias = "vkCreateShadersEXT")]
pub unsafe fn create_shaders_ext<'a, R: DynamicArray<ShaderEXT>>(
    device: &raw::Device,
    p_create_infos: impl AsSlice<'a, ShaderCreateInfoEXT<'a>>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, R)> {
    let vulkan_command = dispatcher
        .create_shaders_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_shaders = R::create_with_capacity(p_create_infos.as_slice().len() as _);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderEXT.html>"]
#[doc(alias = "vkDestroyShaderEXT")]
pub unsafe fn destroy_shader_ext(
    device: &raw::Device,
    shader: Option<&raw::ShaderEXT>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_shader_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        shader.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderBinaryDataEXT.html>"]
#[doc(alias = "vkGetShaderBinaryDataEXT")]
pub unsafe fn get_shader_binary_data_ext(
    device: &raw::Device,
    shader: &raw::ShaderEXT,
    p_data: VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<usize> {
    let vulkan_command = dispatcher
        .get_shader_binary_data_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_data_size = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { shader.clone() }),
        p_data_size.as_mut_ptr(),
        p_data,
    );
    vk_status.map_success(|| p_data_size.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindShadersEXT.html>"]
#[doc(alias = "vkCmdBindShadersEXT")]
pub unsafe fn cmd_bind_shaders_ext<'a, V3: Alias<raw::ShaderEXT> + 'a>(
    command_buffer: &raw::CommandBuffer,
    p_stages: impl AsSlice<'a, ShaderStageFlags>,
    p_shaders: impl AsSlice<'a, V3>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_shaders_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        p_shaders.as_slice().len() as _,
        p_stages.as_slice().as_ptr().cast(),
        p_shaders.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClampRangeEXT.html>"]
#[doc(alias = "vkCmdSetDepthClampRangeEXT")]
pub unsafe fn cmd_set_depth_clamp_range_ext(
    command_buffer: &raw::CommandBuffer,
    depth_clamp_mode: DepthClampModeEXT,
    p_depth_clamp_range: Option<&DepthClampRangeEXT>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_depth_clamp_range_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        depth_clamp_mode,
        p_depth_clamp_range
            .map(|v| ptr::from_ref(v))
            .unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineBinariesKHR.html>"]
#[doc(alias = "vkCreatePipelineBinariesKHR")]
pub unsafe fn create_pipeline_binaries_khr<
    S: StructureChainOut<PipelineBinaryHandlesInfoKHR<'static>>,
>(
    device: &raw::Device,
    p_create_info: &PipelineBinaryCreateInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<(Status, S)> {
    let vulkan_command = dispatcher
        .create_pipeline_binaries_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_binaries = MaybeUninit::uninit();
    S::setup_uninit(&mut p_binaries);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        S::get_uninit_head_ptr(p_binaries.as_mut_ptr()),
    );
    vk_status.map_successes(|| {
        S::setup_cleanup(p_binaries.as_mut_ptr());
        p_binaries.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineBinaryKHR.html>"]
#[doc(alias = "vkDestroyPipelineBinaryKHR")]
pub unsafe fn destroy_pipeline_binary_khr(
    device: &raw::Device,
    pipeline_binary: Option<&raw::PipelineBinaryKHR>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_pipeline_binary_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        pipeline_binary.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineKeyKHR.html>"]
#[doc(alias = "vkGetPipelineKeyKHR")]
pub unsafe fn get_pipeline_key_khr<S: StructureChainOut<PipelineBinaryKeyKHR<'static>>>(
    device: &raw::Device,
    p_pipeline_create_info: Option<&PipelineCreateInfoKHR>,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_pipeline_key_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_pipeline_key = MaybeUninit::uninit();
    S::setup_uninit(&mut p_pipeline_key);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseCapturedPipelineDataKHR.html>"]
#[doc(alias = "vkReleaseCapturedPipelineDataKHR")]
pub unsafe fn release_captured_pipeline_data_khr(
    device: &raw::Device,
    p_info: &ReleaseCapturedPipelineDataInfoKHR,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .release_captured_pipeline_data_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFramebufferTilePropertiesQCOM.html>"]
#[doc(alias = "vkGetFramebufferTilePropertiesQCOM")]
pub unsafe fn get_framebuffer_tile_properties_qcom<R: DynamicArray<TilePropertiesQCOM<'static>>>(
    device: &raw::Device,
    framebuffer: &raw::Framebuffer,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_framebuffer_tile_properties_qcom
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_properties_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { framebuffer.clone() }),
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
            Some(unsafe { device.clone() }),
            Some(unsafe { framebuffer.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDynamicRenderingTilePropertiesQCOM.html>"]
#[doc(alias = "vkGetDynamicRenderingTilePropertiesQCOM")]
pub unsafe fn get_dynamic_rendering_tile_properties_qcom<
    S: StructureChainOut<TilePropertiesQCOM<'static>>,
>(
    device: &raw::Device,
    p_rendering_info: &RenderingInfo,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_dynamic_rendering_tile_properties_qcom
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_rendering_info),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_properties.as_mut_ptr());
        p_properties.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetLatencySleepModeNV.html>"]
#[doc(alias = "vkSetLatencySleepModeNV")]
pub unsafe fn set_latency_sleep_mode_nv(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    p_sleep_mode_info: &LatencySleepModeInfoNV,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .set_latency_sleep_mode_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { swapchain.clone() }),
        ptr::from_ref(p_sleep_mode_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkLatencySleepNV.html>"]
#[doc(alias = "vkLatencySleepNV")]
pub unsafe fn latency_sleep_nv(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    p_sleep_info: &LatencySleepInfoNV,
    dispatcher: &CommandsDispatcher,
) -> Result<()> {
    let vulkan_command = dispatcher
        .latency_sleep_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { swapchain.clone() }),
        ptr::from_ref(p_sleep_info),
    )
    .map_success(|| ())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetLatencyMarkerNV.html>"]
#[doc(alias = "vkSetLatencyMarkerNV")]
pub unsafe fn set_latency_marker_nv(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    p_latency_marker_info: &SetLatencyMarkerInfoNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .set_latency_marker_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { swapchain.clone() }),
        ptr::from_ref(p_latency_marker_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetLatencyTimingsNV.html>"]
#[doc(alias = "vkGetLatencyTimingsNV")]
pub unsafe fn get_latency_timings_nv<S: StructureChainOut<GetLatencyMarkerInfoNV<'static>>>(
    device: &raw::Device,
    swapchain: &raw::SwapchainKHR,
    dispatcher: &CommandsDispatcher,
) -> S {
    let vulkan_command = dispatcher
        .get_latency_timings_nv
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_latency_marker_info = MaybeUninit::uninit();
    S::setup_uninit(&mut p_latency_marker_info);
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { swapchain.clone() }),
        S::get_uninit_head_ptr(p_latency_marker_info.as_mut_ptr()),
    );
    S::setup_cleanup(p_latency_marker_info.as_mut_ptr());
    p_latency_marker_info.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueNotifyOutOfBandNV.html>"]
#[doc(alias = "vkQueueNotifyOutOfBandNV")]
pub unsafe fn queue_notify_out_of_band_nv(
    queue: &raw::Queue,
    p_queue_type_info: &OutOfBandQueueTypeInfoNV,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .queue_notify_out_of_band_nv
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { queue.clone() }),
        ptr::from_ref(p_queue_type_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR")]
pub unsafe fn get_physical_device_cooperative_matrix_properties_khr<
    R: DynamicArray<CooperativeMatrixPropertiesKHR<'static>>,
>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_cooperative_matrix_properties_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_property_count = vk_len.as_mut_ptr();
    let p_properties = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetAttachmentFeedbackLoopEnableEXT.html>"]
#[doc(alias = "vkCmdSetAttachmentFeedbackLoopEnableEXT")]
pub unsafe fn cmd_set_attachment_feedback_loop_enable_ext(
    command_buffer: &raw::CommandBuffer,
    aspect_mask: ImageAspectFlags,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_attachment_feedback_loop_enable_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(Some(unsafe { command_buffer.clone() }), aspect_mask)
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetScreenBufferPropertiesQNX.html>"]
#[doc(alias = "vkGetScreenBufferPropertiesQNX")]
pub unsafe fn get_screen_buffer_properties_qnx<
    S: StructureChainOut<ScreenBufferPropertiesQNX<'static>>,
>(
    device: &raw::Device,
    buffer: &VoidPtr,
    dispatcher: &CommandsDispatcher,
) -> Result<S> {
    let vulkan_command = dispatcher
        .get_screen_buffer_properties_qnx
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_properties = MaybeUninit::uninit();
    S::setup_uninit(&mut p_properties);
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(buffer),
        S::get_uninit_head_ptr(p_properties.as_mut_ptr()),
    );
    vk_status.map_success(|| {
        S::setup_cleanup(p_properties.as_mut_ptr());
        p_properties.assume_init()
    })
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleKHR.html>"]
#[doc(alias = "vkCmdSetLineStippleKHR")]
pub unsafe fn cmd_set_line_stipple_khr(
    command_buffer: &raw::CommandBuffer,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_line_stipple_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        line_stipple_factor,
        line_stipple_pattern,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEXT.html>"]
#[doc(alias = "vkCmdSetLineStippleEXT")]
pub unsafe fn cmd_set_line_stipple_ext(
    command_buffer: &raw::CommandBuffer,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_line_stipple_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        line_stipple_factor,
        line_stipple_pattern,
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html>"]
#[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsKHR")]
pub unsafe fn get_physical_device_calibrateable_time_domains_khr<R: DynamicArray<TimeDomainKHR>>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_calibrateable_time_domains_khr
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_time_domain_count = vk_len.as_mut_ptr();
    let p_time_domains = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html>"]
#[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")]
pub unsafe fn get_physical_device_calibrateable_time_domains_ext<R: DynamicArray<TimeDomainKHR>>(
    physical_device: &raw::PhysicalDevice,
    dispatcher: &CommandsDispatcher,
) -> Result<R> {
    let vulkan_command = dispatcher
        .get_physical_device_calibrateable_time_domains_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut vk_len = MaybeUninit::uninit();
    let p_time_domain_count = vk_len.as_mut_ptr();
    let p_time_domains = ptr::null_mut();
    vulkan_command(
        Some(unsafe { physical_device.clone() }),
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
            Some(unsafe { physical_device.clone() }),
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
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorSets2KHR.html>"]
#[doc(alias = "vkCmdBindDescriptorSets2KHR")]
pub unsafe fn cmd_bind_descriptor_sets2_khr(
    command_buffer: &raw::CommandBuffer,
    p_bind_descriptor_sets_info: &BindDescriptorSetsInfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_descriptor_sets2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_bind_descriptor_sets_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants2KHR.html>"]
#[doc(alias = "vkCmdPushConstants2KHR")]
pub unsafe fn cmd_push_constants2_khr(
    command_buffer: &raw::CommandBuffer,
    p_push_constants_info: &PushConstantsInfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_push_constants2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_push_constants_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSet2KHR.html>"]
#[doc(alias = "vkCmdPushDescriptorSet2KHR")]
pub unsafe fn cmd_push_descriptor_set2_khr(
    command_buffer: &raw::CommandBuffer,
    p_push_descriptor_set_info: &PushDescriptorSetInfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_push_descriptor_set2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_push_descriptor_set_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetWithTemplate2KHR.html>"]
#[doc(alias = "vkCmdPushDescriptorSetWithTemplate2KHR")]
pub unsafe fn cmd_push_descriptor_set_with_template2_khr(
    command_buffer: &raw::CommandBuffer,
    p_push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfoKHR,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_push_descriptor_set_with_template2_khr
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_push_descriptor_set_with_template_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDescriptorBufferOffsets2EXT.html>"]
#[doc(alias = "vkCmdSetDescriptorBufferOffsets2EXT")]
pub unsafe fn cmd_set_descriptor_buffer_offsets2_ext(
    command_buffer: &raw::CommandBuffer,
    p_set_descriptor_buffer_offsets_info: &SetDescriptorBufferOffsetsInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_set_descriptor_buffer_offsets2_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_set_descriptor_buffer_offsets_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html>"]
#[doc(alias = "vkCmdBindDescriptorBufferEmbeddedSamplers2EXT")]
pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers2_ext(
    command_buffer: &raw::CommandBuffer,
    p_bind_descriptor_buffer_embedded_samplers_info: &BindDescriptorBufferEmbeddedSamplersInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_bind_descriptor_buffer_embedded_samplers2_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_bind_descriptor_buffer_embedded_samplers_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsEXT.html>"]
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
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_memory_requirements = MaybeUninit::uninit();
    S::setup_uninit(&mut p_memory_requirements);
    vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_info),
        S::get_uninit_head_ptr(p_memory_requirements.as_mut_ptr()),
    );
    S::setup_cleanup(p_memory_requirements.as_mut_ptr());
    p_memory_requirements.assume_init()
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPreprocessGeneratedCommandsEXT.html>"]
#[doc(alias = "vkCmdPreprocessGeneratedCommandsEXT")]
pub unsafe fn cmd_preprocess_generated_commands_ext(
    command_buffer: &raw::CommandBuffer,
    p_generated_commands_info: &GeneratedCommandsInfoEXT,
    state_command_buffer: &raw::CommandBuffer,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_preprocess_generated_commands_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        ptr::from_ref(p_generated_commands_info),
        Some(unsafe { state_command_buffer.clone() }),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteGeneratedCommandsEXT.html>"]
#[doc(alias = "vkCmdExecuteGeneratedCommandsEXT")]
pub unsafe fn cmd_execute_generated_commands_ext(
    command_buffer: &raw::CommandBuffer,
    is_preprocessed: impl Into<Bool32>,
    p_generated_commands_info: &GeneratedCommandsInfoEXT,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .cmd_execute_generated_commands_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { command_buffer.clone() }),
        is_preprocessed.into(),
        ptr::from_ref(p_generated_commands_info),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateIndirectCommandsLayoutEXT.html>"]
#[doc(alias = "vkCreateIndirectCommandsLayoutEXT")]
pub unsafe fn create_indirect_commands_layout_ext(
    device: &raw::Device,
    p_create_info: &IndirectCommandsLayoutCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<IndirectCommandsLayoutEXT> {
    let vulkan_command = dispatcher
        .create_indirect_commands_layout_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_indirect_commands_layout = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_indirect_commands_layout.as_mut_ptr(),
    );
    vk_status.map_success(|| p_indirect_commands_layout.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyIndirectCommandsLayoutEXT.html>"]
#[doc(alias = "vkDestroyIndirectCommandsLayoutEXT")]
pub unsafe fn destroy_indirect_commands_layout_ext(
    device: &raw::Device,
    indirect_commands_layout: Option<&raw::IndirectCommandsLayoutEXT>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_indirect_commands_layout_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        indirect_commands_layout.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateIndirectExecutionSetEXT.html>"]
#[doc(alias = "vkCreateIndirectExecutionSetEXT")]
pub unsafe fn create_indirect_execution_set_ext(
    device: &raw::Device,
    p_create_info: &IndirectExecutionSetCreateInfoEXT,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) -> Result<IndirectExecutionSetEXT> {
    let vulkan_command = dispatcher
        .create_indirect_execution_set_ext
        .get()
        .expect("Vulkan command not loaded.");
    let mut p_indirect_execution_set = MaybeUninit::uninit();
    let vk_status = vulkan_command(
        Some(unsafe { device.clone() }),
        ptr::from_ref(p_create_info),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
        p_indirect_execution_set.as_mut_ptr(),
    );
    vk_status.map_success(|| p_indirect_execution_set.assume_init())
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyIndirectExecutionSetEXT.html>"]
#[doc(alias = "vkDestroyIndirectExecutionSetEXT")]
pub unsafe fn destroy_indirect_execution_set_ext(
    device: &raw::Device,
    indirect_execution_set: Option<&raw::IndirectExecutionSetEXT>,
    p_allocator: Option<&AllocationCallbacks>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .destroy_indirect_execution_set_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        indirect_execution_set.map(|v| unsafe { v.clone() }),
        p_allocator.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateIndirectExecutionSetPipelineEXT.html>"]
#[doc(alias = "vkUpdateIndirectExecutionSetPipelineEXT")]
pub unsafe fn update_indirect_execution_set_pipeline_ext<'a>(
    device: &raw::Device,
    indirect_execution_set: &raw::IndirectExecutionSetEXT,
    p_execution_set_writes: impl AsSlice<'a, WriteIndirectExecutionSetPipelineEXT<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .update_indirect_execution_set_pipeline_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { indirect_execution_set.clone() }),
        p_execution_set_writes.as_slice().len() as _,
        p_execution_set_writes.as_slice().as_ptr().cast(),
    )
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateIndirectExecutionSetShaderEXT.html>"]
#[doc(alias = "vkUpdateIndirectExecutionSetShaderEXT")]
pub unsafe fn update_indirect_execution_set_shader_ext<'a>(
    device: &raw::Device,
    indirect_execution_set: &raw::IndirectExecutionSetEXT,
    p_execution_set_writes: impl AsSlice<'a, WriteIndirectExecutionSetShaderEXT<'a>>,
    dispatcher: &CommandsDispatcher,
) {
    let vulkan_command = dispatcher
        .update_indirect_execution_set_shader_ext
        .get()
        .expect("Vulkan command not loaded.");
    vulkan_command(
        Some(unsafe { device.clone() }),
        Some(unsafe { indirect_execution_set.clone() }),
        p_execution_set_writes.as_slice().len() as _,
        p_execution_set_writes.as_slice().as_ptr().cast(),
    )
}
